# `mpu6050`

> no_std async driver for the MPU6050 6-axis IMU, based on [`this driver`](https://github.com/juliangaal/mpu6050)

## What Works

- Reading the accelerometer, gyroscope, temperature sensor
  - raw
  - scaled
  - roll/pitch estimation
- Motion Detection
- Setting Accel/Gyro Ranges/Sensitivity
- Setting Accel HPF/LPF

## Basic usage

To use this driver you must provide a concrete `embedded_hal_async` implementation. Check the examples folder for an
[`embassy`](https://github.com/embassy-rs) example
