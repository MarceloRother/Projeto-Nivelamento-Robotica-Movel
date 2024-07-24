// src/main.rs
mod sensor;
mod pid;

use sensor::Sensor;
use pid::PID;

fn main() {
    let mut sensor = Sensor::new();
    let mut pid_pitch = PID::new(1.0, 0.1, 0.01);
    let mut pid_roll = PID::new(1.0, 0.1, 0.01);

    let setpoint = 0.0;

    for _ in 0..100 {
        sensor.update();

        let pitch_adjustment = pid_pitch.update(setpoint, sensor.pitch);
        let roll_adjustment = pid_roll.update(setpoint, sensor.roll);

        sensor.pitch -= pitch_adjustment;
        sensor.roll -= roll_adjustment;

        println!("Pitch: {:.2}, Roll: {:.2}", sensor.pitch, sensor.roll);
    }
}
