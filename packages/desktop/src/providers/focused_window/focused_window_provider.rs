use std::path::Path;

use serde::{Deserialize, Serialize};
use windows::Win32::{
  Foundation::{CloseHandle, HWND, MAX_PATH},
  System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_FORMAT,
    PROCESS_QUERY_LIMITED_INFORMATION,
  },
  UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
    GetWindowThreadProcessId,
  },
};

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FocusedWindowProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusedWindowOutput {
  /// Details of the focused window, or `None` when nothing has focus.
  ///
  /// Alt-tabbing and clicking the desktop both leave the foreground
  /// window briefly unset, so a bar should expect this to be empty at
  /// times.
  pub window: Option<FocusedWindow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusedWindow {
  /// Window title.
  pub title: String,

  /// Executable name, e.g. `firefox.exe`.
  pub process_name: String,

  /// Full path to the executable, if it can be read.
  ///
  /// Empty for a process this one is not allowed to open, which is the
  /// case for anything running elevated.
  pub process_path: String,

  /// ID of the process that owns the window.
  pub process_id: u32,
}

pub struct FocusedWindowProvider {
  config: FocusedWindowProviderConfig,
  common: CommonProviderState,
}

impl FocusedWindowProvider {
  pub fn new(
    config: FocusedWindowProviderConfig,
    common: CommonProviderState,
  ) -> FocusedWindowProvider {
    FocusedWindowProvider { config, common }
  }

  fn run_interval(&mut self) -> anyhow::Result<FocusedWindowOutput> {
    let handle = unsafe { GetForegroundWindow() };

    if handle.0.is_null() {
      return Ok(FocusedWindowOutput { window: None });
    }

    let mut process_id = 0;
    unsafe { GetWindowThreadProcessId(handle, Some(&raw mut process_id)) };

    let process_path = process_path(process_id);

    let process_name = Path::new(&process_path)
      .file_name()
      .map(|name| name.to_string_lossy().to_string())
      .unwrap_or_default();

    Ok(FocusedWindowOutput {
      window: Some(FocusedWindow {
        title: window_title(handle),
        process_name,
        process_path,
        process_id,
      }),
    })
  }
}

/// Reads a window's title.
fn window_title(handle: HWND) -> String {
  let length = unsafe { GetWindowTextLengthW(handle) };

  if length <= 0 {
    return String::new();
  }

  // One extra for the null terminator that `GetWindowTextW` writes.
  let mut buffer = vec![0u16; length as usize + 1];
  let copied = unsafe { GetWindowTextW(handle, &mut buffer) };

  String::from_utf16_lossy(&buffer[..copied as usize])
}

/// Reads the full path of a process' executable.
///
/// Returns an empty string when the process cannot be opened, which is
/// what happens for anything running elevated.
fn process_path(process_id: u32) -> String {
  let Ok(handle) = (unsafe {
    OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id)
  }) else {
    return String::new();
  };

  let mut buffer = [0u16; MAX_PATH as usize];
  let mut length = buffer.len() as u32;

  let result = unsafe {
    QueryFullProcessImageNameW(
      handle,
      PROCESS_NAME_FORMAT(0),
      windows::core::PWSTR(buffer.as_mut_ptr()),
      &raw mut length,
    )
  };

  let _ = unsafe { CloseHandle(handle) };

  match result {
    Ok(()) => String::from_utf16_lossy(&buffer[..length as usize]),
    Err(_) => String::new(),
  }
}

impl Provider for FocusedWindowProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    let mut interval = SyncInterval::new(self.config.refresh_interval);

    loop {
      crossbeam::select! {
        recv(interval.tick()) -> _ => {
          let output = self.run_interval();
          self.common.emitter.emit_output(output);
        }
        recv(self.common.input.sync_rx) -> input => {
          if let Ok(ProviderInputMsg::Stop) = input {
            break;
          }
        }
      }
    }
  }
}
