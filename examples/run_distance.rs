use std::error::Error;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use rppal::gpio;

const ECHO: u8 = 17;
const TRIGGER: u8 = 4;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello");

    let timeout = Duration::from_millis(100);

    let gpio = gpio::Gpio::new()?;
    let mut trigger_pin = gpio.get(TRIGGER)?.into_output();

    trigger_pin.set_low();
    thread::sleep(Duration::from_micros(10));

    let echo_pin = gpio.get(ECHO)?.into_input();

    trigger_pin.set_high();
    thread::sleep(Duration::from_micros(10));
    trigger_pin.set_low();

    let start_wait = Instant::now();
    // Hang out while the echo pin is low
    while echo_pin.is_low() {
        if start_wait.elapsed() > timeout {
            panic!("Abort due to timeout while hangout")
        }
    }

    // Track how ling the pin in high
    let start_instance = Instant::now();
    while echo_pin.is_high() {
        if start_wait.elapsed() > timeout {
            panic!("Abort due to timeout")
        }
    }

    let duration = start_instance.elapsed();

    println!("Duration is : {} nano seconds", duration.as_nanos());
    println!(
        "Distance is : {}cm",
        (duration.as_secs_f64() * 170.0 * 100.0)
    );

    Ok(())
}
