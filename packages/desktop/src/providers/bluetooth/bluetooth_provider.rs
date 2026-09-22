use serde::{Deserialize, Serialize};
use windows::{
  core::{Interface, HSTRING},
  Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED},
  Devices::Enumeration::{DeviceInformation, DeviceInformationKind},
  Foundation::{Collections::IIterable, IPropertyValue},
};

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

/// Matches paired Bluetooth association endpoints, both classic and LE.
const BLUETOOTH_AQS_FILTER: &str = concat!(
  "System.Devices.Aep.ProtocolId:=\"{e0cbf06c-cd8b-4647-bb8a-263b43f0f974}\"",
  " OR ",
  "System.Devices.Aep.ProtocolId:=\"{bb7bb05e-5972-42b5-94fc-76eaa7084d49}\"",
);

/// Whether the endpoint is currently connected.
const IS_CONNECTED_KEY: &str = "System.Devices.Aep.IsConnected";

/// Battery percentage. Only present for devices that report one, which is
/// a minority of Bluetooth hardware.
const BATTERY_KEY: &str = "{104EA319-6EE2-4701-BD47-8DDBF425BBE5} 2";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothOutput {
  /// Every paired Bluetooth device.
  pub devices: Vec<BluetoothDevice>,

  /// The subset that is connected right now.
  pub connected_devices: Vec<BluetoothDevice>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothDevice {
  /// Device name as Windows shows it.
  pub name: String,

  /// Whether the device is connected right now.
  pub is_connected: bool,

  /// Battery percentage, or `None` for the many devices that do not
  /// report one.
  pub battery_percent: Option<u8>,
}

pub struct BluetoothProvider {
  config: BluetoothProviderConfig,
  common: CommonProviderState,
}

impl BluetoothProvider {
  pub fn new(
    config: BluetoothProviderConfig,
    common: CommonProviderState,
  ) -> BluetoothProvider {
    BluetoothProvider { config, common }
  }

  fn run_interval(&self) -> anyhow::Result<BluetoothOutput> {
    let extra_properties = IIterable::<HSTRING>::try_from(vec![
      HSTRING::from(IS_CONNECTED_KEY),
      HSTRING::from(BATTERY_KEY),
    ])?;

    // The kind matters: a protocol filter only matches association
    // endpoints, and the default kind is device interfaces, which is why
    // the obvious call returns nothing.
    let found =
      DeviceInformation::FindAllAsyncWithKindAqsFilterAndAdditionalProperties(
        &HSTRING::from(BLUETOOTH_AQS_FILTER),
        &extra_properties,
        DeviceInformationKind::AssociationEndpoint,
      )?
      .get()?;

    let mut devices = Vec::new();

    for info in found {
      let properties = info.Properties()?;

      // Properties come back boxed as `IInspectable`, so each has to be
      // cast before it can be read.
      let is_connected = properties
        .Lookup(&HSTRING::from(IS_CONNECTED_KEY))
        .and_then(|value| value.cast::<IPropertyValue>())
        .and_then(|value| value.GetBoolean())
        .unwrap_or(false);

      let battery_percent = properties
        .Lookup(&HSTRING::from(BATTERY_KEY))
        .and_then(|value| value.cast::<IPropertyValue>())
        .and_then(|value| value.GetUInt8())
        .ok();

      devices.push(BluetoothDevice {
        name: info.Name()?.to_string(),
        is_connected,
        battery_percent,
      });
    }

    let connected_devices = devices
      .iter()
      .filter(|device| device.is_connected)
      .cloned()
      .collect();

    Ok(BluetoothOutput {
      devices,
      connected_devices,
    })
  }
}

impl Provider for BluetoothProvider {
  fn runtime_type(&self) -> RuntimeType {
    // Sync, because `IAsyncOperation::get` blocks the calling thread.
    // On the async runtime it blocks a worker and nothing is ever
    // emitted; the media provider takes the same route for the same
    // reason.
    RuntimeType::Sync
  }

  fn start_sync(&mut self) {
    // Device enumeration completes on a background thread, and waiting on
    // it needs this thread to have an apartment. Provider threads come
    // from a pool with none, so the first call would otherwise never
    // return.
    let _ = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

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
