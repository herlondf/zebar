use std::path::PathBuf;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shell_util::{CommandOptions, Shell};
use tokio::time::{self, Duration};

use crate::providers::{
  CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandProviderConfig {
  /// Program name (if in PATH) or full path to the program.
  pub program: String,

  /// Arguments to pass to the program.
  #[serde(default)]
  pub args: Vec<String>,

  /// Directory to run the program in.
  pub cwd: Option<PathBuf>,

  /// How often the program is run, in milliseconds.
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandOutput {
  /// What the program wrote to stdout, with trailing newlines removed.
  pub stdout: String,

  /// What the program wrote to stderr, with trailing newlines removed.
  pub stderr: String,

  /// Exit code, or `None` if the program was killed by a signal.
  pub exit_code: Option<i32>,

  /// Whether the program exited successfully.
  pub success: bool,
}

pub struct CommandProvider {
  config: CommandProviderConfig,
  common: CommonProviderState,
}

impl CommandProvider {
  pub fn new(
    config: CommandProviderConfig,
    common: CommonProviderState,
  ) -> CommandProvider {
    CommandProvider { config, common }
  }

  async fn run_interval(&self) -> anyhow::Result<CommandOutput> {
    let options = CommandOptions {
      cwd: self.config.cwd.clone(),
      ..CommandOptions::default()
    };

    let output =
      Shell::exec(&self.config.program, &self.config.args, &options)
        .await?;

    Ok(CommandOutput {
      stdout: trim_output(output.stdout.as_str().unwrap_or_default()),
      stderr: trim_output(output.stderr.as_str().unwrap_or_default()),
      exit_code: output.status.code,
      success: output.status.success,
    })
  }
}

/// Strips the trailing newline that almost every program adds, so that a
/// widget can print the output without trimming it first.
fn trim_output(output: &str) -> String {
  output.trim_end_matches(['\r', '\n']).to_string()
}

#[async_trait]
impl Provider for CommandProvider {
  fn runtime_type(&self) -> RuntimeType {
    RuntimeType::Async
  }

  async fn start_async(&mut self) {
    let mut interval =
      time::interval(Duration::from_millis(self.config.refresh_interval));

    loop {
      tokio::select! {
        _ = interval.tick() => {
          let output = self.run_interval().await;
          self.common.emitter.emit_output(output);
        }
        Some(input) = self.common.input.async_rx.recv() => {
          if let ProviderInputMsg::Stop = input {
            break;
          }
        }
      }
    }
  }
}
