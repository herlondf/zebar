use serde::Deserialize;

#[cfg(any(target_os = "macos", windows))]
use super::komorebi::KomorebiProviderConfig;
#[cfg(windows)]
use super::{
  audio::AudioProviderConfig, bluetooth::BluetoothProviderConfig,
  focused_window::FocusedWindowProviderConfig,
  gpu::GpuProviderConfig, keyboard::KeyboardProviderConfig, media::MediaProviderConfig,
  systray::SystrayProviderConfig,
};
use super::{
  battery::BatteryProviderConfig, command::CommandProviderConfig,
  cpu::CpuProviderConfig, disk::DiskProviderConfig,
  host::HostProviderConfig, http::HttpProviderConfig,
  ip::IpProviderConfig, memory::MemoryProviderConfig,
  network::NetworkProviderConfig, temperature::TemperatureProviderConfig,
  weather::WeatherProviderConfig,
};

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderConfig {
  #[cfg(windows)]
  Audio(AudioProviderConfig),
  Battery(BatteryProviderConfig),
  Command(CommandProviderConfig),
  Cpu(CpuProviderConfig),
  #[cfg(windows)]
  #[serde(rename = "focusedWindow")]
  FocusedWindow(FocusedWindowProviderConfig),
  #[cfg(windows)]
  Bluetooth(BluetoothProviderConfig),
  #[cfg(windows)]
  Gpu(GpuProviderConfig),
  Host(HostProviderConfig),
  Http(HttpProviderConfig),
  Ip(IpProviderConfig),
  #[cfg(any(target_os = "macos", windows))]
  Komorebi(KomorebiProviderConfig),
  #[cfg(windows)]
  Media(MediaProviderConfig),
  Memory(MemoryProviderConfig),
  Disk(DiskProviderConfig),
  Network(NetworkProviderConfig),
  #[cfg(windows)]
  Systray(SystrayProviderConfig),
  Temperature(TemperatureProviderConfig),
  Weather(WeatherProviderConfig),
  #[cfg(windows)]
  Keyboard(KeyboardProviderConfig),
}
