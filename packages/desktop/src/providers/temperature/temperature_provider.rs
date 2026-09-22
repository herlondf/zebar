use serde::{Deserialize, Serialize};
use sysinfo::Components;

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureOutput {
  /// Every sensor the OS reports.
  pub components: Vec<TemperatureComponent>,

  /// Hottest sensor, or `None` when the OS reports no sensors at all.
  ///
  /// Which sensor that is varies by machine, so a bar that just wants one
  /// number has something to show without knowing the labels.
  pub hottest: Option<TemperatureComponent>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemperatureComponent {
  /// Sensor name as the OS reports it, e.g. `CPU`.
  pub label: String,

  /// Current temperature in celsius.
  pub celsius_temp: f32,

  /// Current temperature in fahrenheit.
  pub fahrenheit_temp: f32,

  /// Highest temperature seen since the process started.
  pub max_celsius_temp: f32,

  /// Temperature the OS considers critical, if it reports one.
  pub critical_celsius_temp: Option<f32>,
}

pub struct TemperatureProvider {
  config: TemperatureProviderConfig,
  common: CommonProviderState,
  components: Components,
}

impl TemperatureProvider {
  pub fn new(
    config: TemperatureProviderConfig,
    common: CommonProviderState,
  ) -> TemperatureProvider {
    TemperatureProvider {
      config,
      common,
      // Not the shared `sysinfo::System`: components are a separate list
      // that has to be refreshed on its own.
      components: Components::new_with_refreshed_list(),
    }
  }

  fn run_interval(&mut self) -> anyhow::Result<TemperatureOutput> {
    self.components.refresh_list();

    let components = self
      .components
      .iter()
      .map(|component| {
        let celsius_temp = component.temperature();

        TemperatureComponent {
          label: component.label().to_string(),
          celsius_temp,
          fahrenheit_temp: celsius_temp * 9.0 / 5.0 + 32.0,
          max_celsius_temp: component.max(),
          critical_celsius_temp: component.critical(),
        }
      })
      .collect::<Vec<_>>();

    let hottest = components
      .iter()
      .max_by(|a, b| a.celsius_temp.total_cmp(&b.celsius_temp))
      .cloned();

    Ok(TemperatureOutput {
      components,
      hottest,
    })
  }
}

impl Provider for TemperatureProvider {
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
