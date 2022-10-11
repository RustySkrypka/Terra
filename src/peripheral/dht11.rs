use dht_hal_drv::{dht_read, DhtError, DhtType, DhtValue};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rppal::gpio::{Gpio, IoPin, Mode};

use serde::{Deserialize, Serialize};
use spin_sleep;
use void;

use std::result::Result;
use std::sync::mpsc::Sender;
use std::thread::sleep;
use std::time::Duration;

use super::GpioConnections::DHT11;

#[derive(Serialize, Deserialize)]
pub struct SensorData {
    pub temperature: f32,
    pub humidity: f32,
}

pub struct HTSensor {
    pin: OpenPin,
    sender: Sender<SensorData>,
}

impl HTSensor {
    pub fn new(sender: Sender<SensorData>) -> Self {
        let iopin = Gpio::new()
            .unwrap()
            .get(DHT11 as u8)
            .unwrap()
            .into_io(Mode::Input);

        let opin = OpenPin::new(iopin);

        HTSensor {
            pin: opin,
            sender: sender,
        }
    }

    fn perform_measurment(&mut self) -> Result<DhtValue, DhtError> {
        let readings = dht_read(DhtType::DHT11, &mut self.pin, &mut |d| {
            spin_sleep::sleep(Duration::from_micros(d as u64))
        });

        readings
    }

    pub fn run(&mut self) {
        loop {
            let res = self.perform_measurment();
            match res {
                Ok(data) => {
                    let sensor_data = SensorData {
                        temperature: data.temperature(),
                        humidity: data.humidity(),
                    };
                    println!(
                        "Temperature: {}, Humidity: {}",
                        sensor_data.temperature, sensor_data.humidity
                    );
                    self.sender.send(sensor_data);
                }
                Err(err) => {
                    println!("DHT ERROR: {:?}", err);
                }
            };

            sleep(Duration::from_secs(60));
        }
    }
}

struct OpenPin {
    iopin: IoPin,
    mode: Mode,
}

impl OpenPin {
    fn new(mut pin: IoPin) -> OpenPin {
        pin.set_mode(Mode::Input);
        OpenPin {
            iopin: pin,
            mode: Mode::Input,
        }
    }

    fn switch_input(&mut self) {
        if self.mode != Mode::Input {
            self.mode = Mode::Input;
            self.iopin.set_mode(Mode::Input);
        }
    }

    fn switch_output(&mut self) {
        if self.mode != Mode::Output {
            self.mode = Mode::Output;
            self.iopin.set_mode(Mode::Output);
        }
    }
}

impl InputPin for OpenPin {
    type Error = void::Void;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.iopin.is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.iopin.is_low())
    }
}

impl OutputPin for OpenPin {
    type Error = void::Void;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.switch_output();
        self.iopin.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.iopin.set_high();
        self.switch_input();
        Ok(())
    }
}
