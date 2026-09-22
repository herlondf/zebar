use serde::Serialize;

#[cfg(any(target_os = "macos", windows))]
use super::komorebi::KomorebiOutput;
#[cfg(windows)]
use super::{
  audio::AudioOutput, bluetooth::BluetoothOutput,
  focused_window::FocusedWindowOutput, gpu::GpuOutput,
  keyboard::KeyboardOutput, media::MediaOutput, systray::SystrayOutput,
};
use super::{
  battery::BatteryOutput, command::CommandOutput, cpu::CpuOutput,
  disk::DiskOutput, host::HostOutput, http::HttpOutput, ip::IpOutput,
  memory::MemoryOutput, network::NetworkOutput,
  temperature::TemperatureOutput, weather::WeatherOutput,
};

/// Implements `From<T>` for `ProviderOutput` for each given variant.
macro_rules! impl_provider_output {
  ($($variant:ident($type:ty)),* $(,)?) => {
    $(
      impl From<$type> for ProviderOutput {
        fn from(value: $type) -> Self {
          Self::$variant(value)
        }
      }
    )*
  };
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
#[allow(
  clippy::large_enum_variant,
  reason = "produced one at a time, never held in bulk"
)]
pub enum ProviderOutput {
  #[cfg(windows)]
  Audio(AudioOutput),
  Battery(BatteryOutput),
  Command(CommandOutput),
  Cpu(CpuOutput),
  #[cfg(windows)]
  FocusedWindow(FocusedWindowOutput),
  #[cfg(windows)]
  Bluetooth(BluetoothOutput),
  #[cfg(windows)]
  Gpu(GpuOutput),
  Host(HostOutput),
  Http(HttpOutput),
  Ip(IpOutput),
  #[cfg(any(target_os = "macos", windows))]
  Komorebi(KomorebiOutput),
  #[cfg(windows)]
  Media(MediaOutput),
  Memory(MemoryOutput),
  Disk(DiskOutput),
  Network(NetworkOutput),
  #[cfg(windows)]
  Systray(SystrayOutput),
  Temperature(TemperatureOutput),
  Weather(WeatherOutput),
  #[cfg(windows)]
  Keyboard(KeyboardOutput),
}

impl_provider_output! {
  Battery(BatteryOutput),
  Command(CommandOutput),
  Cpu(CpuOutput),
  Host(HostOutput),
  Http(HttpOutput),
  Ip(IpOutput),
  Memory(MemoryOutput),
  Disk(DiskOutput),
  Network(NetworkOutput),
  Temperature(TemperatureOutput),
  Weather(WeatherOutput)
}

#[cfg(target_os = "macos")]
impl_provider_output! {
  Komorebi(KomorebiOutput),
}

#[cfg(windows)]
impl_provider_output! {
  Audio(AudioOutput),
  Bluetooth(BluetoothOutput),
  FocusedWindow(FocusedWindowOutput),
  Gpu(GpuOutput),
  Media(MediaOutput),
  Keyboard(KeyboardOutput),
  Komorebi(KomorebiOutput),
  Systray(SystrayOutput),
}
