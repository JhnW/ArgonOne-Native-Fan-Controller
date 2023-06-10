mod configuration;
use crate::configuration::Configuration;
use std::thread;
use rppal::i2c::{Error, I2c};
use systemstat::Platform;
use std::time::Duration;

fn control(configuration: &Configuration) -> Result<(), Error> {
    let mut i2c = I2c::with_bus(1)?; //to support RPI 1 just change bus to 0
    i2c.set_slave_address(0x1a)?;
    thread::sleep(Duration::from_millis(100)); //to avoid 121 error
    control_fan(i2c, &configuration.curve, configuration.interval)
}

fn control_fan(i2c: I2c, curve: &[u8], wake_up_interval: Duration) -> Result<(), Error> {
    let sys = systemstat::System::new();
    loop {
        let temperature = sys.cpu_temp()?;
        let speed = calculate_fan_speed(temperature, curve);
        i2c.smbus_send_byte(speed)?;
        thread::sleep(wake_up_interval);
    }
}

fn calculate_fan_speed(temperature: f32, curve: &[u8]) -> u8 {
    let mut result = 0;
    for i in 0..(curve.len()/2) {
        let edge_temperature = curve[i];
        if edge_temperature as f32 > temperature {
            return result;
        }
        result = curve[i+1];
    }
    result
}

fn main() {
    let configuration = Configuration::load();
    loop {
        let error = control(&configuration).expect_err("Unexpected restart with no fail code.");
        eprintln!("Unable to control fan due to: {error}. Restarting service loop.");
        thread::sleep(configuration.interval * 2);
    }
}