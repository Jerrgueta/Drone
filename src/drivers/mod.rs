use rand::Rng;

pub fn init_imu() {
    let mut rng = rand::thread_rng();

    let acc_x: f32 = rng.gen_range(-1.0..1.0);
    let acc_y: f32 = rng.gen_range(-1.0..1.0);
    let acc_z: f32 = rng.gen_range(0.8..1.2);  // simulate gravity along z

    println!("IMU: Accel = ({:.2}, {:.2}, {:.2})", acc_x, acc_y, acc_z);
}
