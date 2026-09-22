use anyhow::Context;
use nvml_wrapper::{
  enum_wrappers::device::{Clock, TemperatureSensor},
  Nvml,
};
use serde::{Deserialize, Serialize};

use crate::{
  common::SyncInterval,
  providers::{
    CommonProviderState, Provider, ProviderInputMsg, RuntimeType,
  },
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GpuProviderConfig {
  pub refresh_interval: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuOutput {
  /// Every NVIDIA GPU the driver reports.
  pub devices: Vec<GpuDevice>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GpuDevice {
  /// Model name, e.g. `NVIDIA GeForce RTX 3050 Laptop GPU`.
  pub name: String,

  /// Percentage of time the GPU was busy over the last sample period.
  pub usage: u32,

  /// Percentage of time GPU memory was being read or written.
  pub memory_usage: u32,

  /// Memory in use, in bytes.
  pub used_memory: u64,

  /// Total memory, in bytes.
  pub total_memory: u64,

  /// Core temperature in celsius.
  pub celsius_temp: u32,

  /// Fan speed as a percentage, or `None` on cards with no controllable
  /// fan, which includes many laptop GPUs.
  pub fan_speed: Option<u32>,

  /// Current core clock in MHz.
  pub core_clock_mhz: Option<u32>,

  /// Current power draw in milliwatts.
  pub power_usage_mw: Option<u32>,
}

pub struct GpuProvider {
  config: GpuProviderConfig,
  common: CommonProviderState,

  /// Kept across ticks because initialising NVML means loading and
  /// handshaking with the driver library.
  ///
  /// `None` until the first tick, and an error on a machine with no NVIDIA
  /// driver, which is reported rather than retried silently.
  nvml: Option<Nvml>,
}

impl GpuProvider {
  pub fn new(
    config: GpuProviderConfig,
    common: CommonProviderState,
  ) -> GpuProvider {
    GpuProvider {
      config,
      common,
      nvml: None,
    }
  }

  fn run_interval(&mut self) -> anyhow::Result<GpuOutput> {
    let nvml = match &self.nvml {
      Some(nvml) => nvml,
      None => self.nvml.insert(
        Nvml::init().context("Unable to load NVIDIA's NVML library.")?,
      ),
    };

    let count = nvml.device_count()?;
    let mut devices = Vec::with_capacity(count as usize);

    for index in 0..count {
      let device = nvml.device_by_index(index)?;
      let utilization = device.utilization_rates()?;
      let memory = device.memory_info()?;

      devices.push(GpuDevice {
        name: device.name()?,
        usage: utilization.gpu,
        memory_usage: utilization.memory,
        used_memory: memory.used,
        total_memory: memory.total,
        celsius_temp: device.temperature(TemperatureSensor::Gpu)?,
        // These three are not reported by every card, so a card that
        // withholds one still reports the rest.
        fan_speed: device.fan_speed(0).ok(),
        core_clock_mhz: device.clock_info(Clock::Graphics).ok(),
        power_usage_mw: device.power_usage().ok(),
      });
    }

    Ok(GpuOutput { devices })
  }
}

impl Provider for GpuProvider {
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
