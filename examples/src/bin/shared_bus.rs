//! Connect SDA to P0.03, SCL to P0.04
//! $ DEFMT_LOG=info cargo rb shared_bus
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use nrf_embassy as _; // global logger + panicking-behavior

use core::f32::consts::PI;
use defmt::*;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_executor::_export::StaticCell;
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{interrupt, peripherals::TWISPI0};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Delay, Duration, Timer};
use libm::atan2;
use mpu6050_async::*;
use qmc5883l_async::*;

// Need correct magnetic declination for your location for accurate
// readings. See http://www.magnetic-declination.com/
const DECLINATION_RADS: f32 = 0.024434609;

#[embassy_executor::task]
async fn compass_task(
    mut compass: QMC5883L<I2cDevice<'static, ThreadModeRawMutex, Twim<'_, TWISPI0>>>,
) {
    compass.continuous().await.unwrap();
    loop {
        if let Ok((x, y, z)) = compass.mag().await {
            let mut heading = atan2(y as f64, x as f64) as f32 + DECLINATION_RADS;
            if heading < 0.0 {
                heading += 2.0 * PI;
            } else if heading > 2.0 * PI {
                heading -= 2.0 * PI;
            }
            let heading_degrees = heading * 180.0 / PI;
            info!(
                "x={}, y={}, z={}: heading={} degrees",
                x, y, z, heading_degrees
            );
            Timer::after(Duration::from_millis(1000)).await;
        }
    }
}

#[embassy_executor::task]
async fn gyro_task(mut mpu: Mpu6050<I2cDevice<'static, ThreadModeRawMutex, Twim<'_, TWISPI0>>>) {
    mpu.init(&mut Delay).await.unwrap();
    loop {
        // Get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().await.unwrap();
        info!("gyro: {:?}", gyro);
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    static I2C_BUS: StaticCell<Mutex<ThreadModeRawMutex, Twim<TWISPI0>>> = StaticCell::new();
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let i2c = Twim::new(p.TWISPI0, irq, p.P0_03, p.P0_04, config);
    let i2c_bus = Mutex::<ThreadModeRawMutex, _>::new(i2c);
    let i2c_bus = I2C_BUS.init(i2c_bus);

    let i2c_dev1 = I2cDevice::new(i2c_bus);
    let compass = QMC5883L::new(i2c_dev1).await.unwrap();
    unwrap!(spawner.spawn(compass_task(compass)));

    let i2c_dev2 = I2cDevice::new(i2c_bus);
    let mpu = Mpu6050::new(i2c_dev2);
    unwrap!(spawner.spawn(gyro_task(mpu)));
}
