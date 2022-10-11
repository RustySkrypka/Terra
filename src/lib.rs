use peripheral::dht11::HTSensor;
use peripheral::dht11::SensorData;
use peripheral::relay::Relay;

use chrono::{Timelike, Utc};
use std::fmt;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod peripheral;

#[derive(Debug)]
pub enum TerraType {
    Turttle,
}

pub struct Terra {
    env: TerraType,
}

impl fmt::Display for TerraType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TerraType::Turttle => write!(f, "Terrarium configured for turttle environment"),
        }
    }
}

impl Terra {
    pub fn new(env: TerraType) -> Self {
        Terra { env }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let mut handles = Vec::new();
        let relay = Arc::new(Mutex::new(Relay::new()));

        println!("{}", self.env);

        let (sender, receiver) = mpsc::channel();
        let receiver_instance = Arc::new(Mutex::new(receiver));

        let mut htsensor = HTSensor::new(sender);
        let sensor_handle = thread::spawn(move || {
            htsensor.run();
        });

        handles.push(sensor_handle);

        let humidity_relay = Arc::clone(&relay);
        let humidity_receiver = Arc::clone(&receiver_instance);
        let humidity_relay_handle = thread::spawn(move || {
            Self::humidity_control(humidity_relay, humidity_receiver);
        });

        handles.push(humidity_relay_handle);

        let day_lamp_relay = Arc::clone(&relay);
        let day_lamp_relay_handle = thread::spawn(move || {
            Self::day_lamp_control(day_lamp_relay);
        });

        handles.push(day_lamp_relay_handle);

        let night_lamp_relay = Arc::clone(&relay);
        let night_lamp_receiver = Arc::clone(&receiver_instance);
        let night_lamp_relay_handle = thread::spawn(move || {
            Self::night_lamp_control(night_lamp_relay, night_lamp_receiver);
        });

        handles.push(night_lamp_relay_handle);

        let water_pump_relay = Arc::clone(&relay);
        let water_pump_relay_handle = thread::spawn(move || {
            Self::water_pump_control(water_pump_relay);
        });

        handles.push(water_pump_relay_handle);

        for handle in handles {
            handle.join().unwrap();
        }

        Ok(())
    }

    fn humidity_control(relay: Arc<Mutex<Relay>>, receiver: Arc<Mutex<Receiver<SensorData>>>) {
        let config = 70.0;
        loop {
            println!("humidity control loop");

            let sensor_data = receiver.lock().unwrap().recv().unwrap();

            if sensor_data.humidity < config {
                println!("enable humidifier");

                relay.lock().unwrap().open_humidifier_channel();
            } else {
                relay.lock().unwrap().close_humidifier_channel();
            }

            sleep(Duration::from_secs(60));
        }
    }

    fn day_lamp_control(relay: Arc<Mutex<Relay>>) {
        loop {
            println!("day lamp control loop");

            let now = Utc::now();
            let hour = now.hour();

            if hour >= 7 && hour < 23 {
                println!("enable day lamp");

                relay.lock().unwrap().open_day_lamp_channel();
            } else {
                relay.lock().unwrap().close_day_lamp_channel();
            }

            sleep(Duration::from_secs(60));
        }
    }

    fn night_lamp_control(relay: Arc<Mutex<Relay>>, receiver: Arc<Mutex<Receiver<SensorData>>>) {
        let config = 23.0;

        loop {
            println!("night lamp control loop");

            let now = Utc::now();
            let hour = now.hour();

            if hour >= 23 && hour < 7 {
                let sensor_data = receiver.lock().unwrap().recv().unwrap();

                if sensor_data.temperature < config {
                    println!("enable night lamp");

                    relay.lock().unwrap().open_night_lamp_channel();
                }
            } else {
                relay.lock().unwrap().close_night_lamp_channel();
            }

            sleep(Duration::from_secs(60));
        }
    }

    fn water_pump_control(relay: Arc<Mutex<Relay>>) {
        loop {
            println!("water pump control loop");

            let now = Utc::now();
            let hour = now.hour();

            if hour >= 7 && hour < 23 {
                println!("enable water pump");

                relay.lock().unwrap().open_water_pump_channel();
            } else {
                relay.lock().unwrap().close_water_pump_channel();
            }

            sleep(Duration::from_secs(60));
        }
    }
}
