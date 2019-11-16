use rust_gpiozero::*;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

mod tsl2591_lux {
    use i2cdev::core::I2CDevice;

    const TSL2591_ADDR: u8 = 0x79;
    const TSL2591_COMMAND_BIT: u8 = 0xA0;
    const TSL2591_ENABLE_POWEROFF: u8 = 0x00;

    const TSL2591_ENABLE_POWERON: u8 = 0x01;
    const TSL2591_ENABLE_AEN: u8 = 0x02;
    const TSL2591_ENABLE_AIEN: u8 = 0x10;
    const TSL2591_ENABLE_NPIEN: u8 = 0x80;
    const TSL2591_REGISTER_ENABLE: u8 = 0x00;
    const TSL2591_REGISTER_CONTROL: u8 = 0x01;
    const TSL2591_REGISTER_DEVICE_ID: u8 = 0x12;
    const TSL2591_REGISTER_CHAN0_LOW: u8 = 0x14;
    const TSL2591_REGISTER_CHAN1_LOW: u8 = 0x16;

    const TSL2591_LUX_DF: f32 = 408.0;
    const TSL2591_LUX_COEFB: f32 = 1.64;
    const TSL2591_LUX_COEFC: f32 = 0.59;
    const TSL2591_LUX_COEFD: f32 = 0.86;

    const TSL2591_MAX_COUNT_100MS: u16 = 0x8FFF;
    const TSL2591_MAX_COUNT_: u16 = 0xFFFF;

    pub enum TSL2591Error<E> {
        Error(E),
        ParseError,
    }

    pub struct TSL2591Lux<T: I2CDevice + Sized> {
        i2cdev: T,
    }

    impl<T> TSL2591Lux<T> 
    where
    T: I2CDevice + Sized
    {
        pub fn new(mut i2cdev: T) -> Result<TSL2591Lux<T>, T::Error> {
            //i2cdev.smbus_write_byte_data(register: u8, 0x00)?
            Ok(TSL2591Lux{ i2cdev: i2cdev})
        }
    }
}

use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    let led = LED::new(26);
    let _i2cdev = LinuxI2CDevice::new("a path", 0x07900);
    loop {
        led.on();
        sleep(Duration::from_secs(1));
        led.off();
        sleep(Duration::from_secs(1));
    }
}
