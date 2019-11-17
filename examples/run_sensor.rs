use rppal::i2c::I2c;
use simple_signal::{self, Signal};

use tsl2591_sensor::TSL2591Sensor;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let i2c = I2c::new().unwrap();

    let lux_dev = TSL2591Sensor::new(i2c).expect("Unable to open lux device: robbie");

    /*   let led = LED::new(26);
    //let foo  = LinuxI2CDevice::new("device ", 0x23a3).unwrap();
    loop {
        led.on();
        sleep(Duration::from_secs(1));
        led.off();
        sleep(Duration::from_secs(1));
    } */

    println!(
        "Gain is: {}",
        lux_dev.get_gain().expect("Unable to get gain")
    );
    println!(
        "Integration time is: {}",
        lux_dev
            .get_integration_time()
            .expect("Unable to get integration time")
    );

    let running = Arc::new(AtomicBool::new(true));

    simple_signal::set_handler(&[Signal::Int, Signal::Term], {
        let r = running.clone();
        move |_signals| {
            r.store(false, Ordering::SeqCst);
        }
    });

    while running.load(Ordering::SeqCst) {
        let visible = lux_dev.visible().unwrap();
        let infrared = lux_dev.infrared().unwrap();
        let full_spectrum = lux_dev.full_spectrum().unwrap();
        let lux = lux_dev.lux().unwrap();
        println!("Visible: {}", visible);
        println!("Infrared: {}", infrared);
        println!("Full Spectrum: {}", full_spectrum);
        println!("Lux: {}", lux);
        println!("");
        sleep(Duration::from_secs(1));
    }

    println!("Shutting down");
    let _ = lux_dev.disable();
}
