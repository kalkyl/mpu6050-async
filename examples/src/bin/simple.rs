//! Connect SDA to P0.03, SCL to P0.04
//! $ DEFMT_LOG=info cargo rb simple

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use nrf_embassy as _; // global logger + panicking-behavior

use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Delay, Duration, Timer};
use embassy_nrf::twim::{self, Twim};
use embassy_nrf::{interrupt, Peripherals};
use mpu6050_async::*;

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let config = twim::Config::default();
    let irq = interrupt::take!(SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0);
    let i2c = Twim::new(p.TWISPI0, irq, p.P0_03, p.P0_04, config);

    let mut mpu = Mpu6050::new(i2c);
    mpu.init(&mut Delay).await.unwrap();

    loop {
        // get roll and pitch estimate
        let acc = mpu.get_acc_angles().await.unwrap();
        info!("r/p: {:?}", acc);

        // get temp
        let temp = mpu.get_temp().await.unwrap();
        info!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity
        let gyro = mpu.get_gyro().await.unwrap();
        info!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().await.unwrap();
        info!("acc: {:?}", acc);

        Timer::after(Duration::from_millis(100)).await;
    }
}
