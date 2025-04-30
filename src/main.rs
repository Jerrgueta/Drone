mod kernel;
mod drivers;
mod control;
mod fusion;

use std::{thread, time};

fn main() {
    println!("RT-DOS Booting...");

    // Simulate a real-time control loop running at 1 Hz (1 task per second)
    loop {
        kernel::init_scheduler();
        drivers::init_imu();
        control::start_motor_control();
        fusion::run_sensor_fusion();

        // Simulated task timing (wait 1 second before repeating)
        thread::sleep(time::Duration::from_secs(1));
    }
}
