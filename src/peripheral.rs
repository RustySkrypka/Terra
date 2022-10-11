pub mod dht11;
pub mod relay;

#[repr(u8)]
pub enum GpioConnections {
    DHT11 = 4,
    RelayModuleIN1 = 22,
    RelayModuleIN2 = 23,
    RelayModuleIN3 = 24,
    RelayModuleIN4 = 27,
}
