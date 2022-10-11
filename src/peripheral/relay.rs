use super::GpioConnections::*;
use rppal::gpio::{Gpio, OutputPin, Pin};

use std::thread::sleep;
use std::time::Duration;

// output active low
pub struct Relay {
    water_pump_pin: OutputPin,
    humidifier_pin: OutputPin,
    day_lamp_pin: OutputPin,
    night_lamp_pin: OutputPin,
}

impl Relay {
    pub fn new() -> Self {
        let water_pump_pin = Gpio::new()
            .expect("Failed to create gpio")
            .get(RelayModuleIN3 as u8)
            .expect("Failed to get water pump gpio")
            .into_output();

        let humidifier_pin = Gpio::new()
            .expect("Failed to create gpio")
            .get(RelayModuleIN2 as u8)
            .expect("Failed to get humidifier gpio")
            .into_output();

        let day_lamp_pin = Gpio::new()
            .expect("Failed to create gpio")
            .get(RelayModuleIN4 as u8)
            .expect("Failed to get day lamp gpio")
            .into_output();

        let night_lamp_pin = Gpio::new()
            .expect("Failed to create gpio")
            .get(RelayModuleIN1 as u8)
            .expect("Failed to get night lamp gpio")
            .into_output();

        Relay {
            water_pump_pin: water_pump_pin,
            humidifier_pin: humidifier_pin,
            day_lamp_pin: day_lamp_pin,
            night_lamp_pin: night_lamp_pin,
        }
    }

    // opens and closes only if set twice, bug?
    pub fn open_water_pump_channel(&mut self) {
        self.water_pump_pin.set_low();
        sleep(Duration::from_secs(1));
        self.water_pump_pin.set_low();
    }

    pub fn close_water_pump_channel(&mut self) {
        self.water_pump_pin.set_high();
        sleep(Duration::from_secs(1));
        self.water_pump_pin.set_high();
    }

    pub fn open_humidifier_channel(&mut self) {
        self.humidifier_pin.set_low();
        sleep(Duration::from_secs(1));
        self.humidifier_pin.set_low();
    }

    pub fn close_humidifier_channel(&mut self) {
        self.humidifier_pin.set_high();
        sleep(Duration::from_secs(1));
        self.humidifier_pin.set_high();
    }

    pub fn open_day_lamp_channel(&mut self) {
        self.day_lamp_pin.set_low();
        sleep(Duration::from_secs(1));
        self.day_lamp_pin.set_low();
    }

    pub fn close_day_lamp_channel(&mut self) {
        self.day_lamp_pin.set_high();
        sleep(Duration::from_secs(1));
        self.day_lamp_pin.set_high();
    }

    pub fn open_night_lamp_channel(&mut self) {
        self.night_lamp_pin.set_low();
        sleep(Duration::from_secs(1));
        self.night_lamp_pin.set_low();
    }

    pub fn close_night_lamp_channel(&mut self) {
        self.night_lamp_pin.set_high();
        sleep(Duration::from_secs(1));
        self.night_lamp_pin.set_high();
    }
}
