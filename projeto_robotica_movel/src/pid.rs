// src/pid.rs
pub struct PID {
    kp: f64, // Ganho Proporcional
    ki: f64, // Ganho Integral
    kd: f64, // Ganho Derivativo
    previous_error: f64,
    integral: f64,
}

impl PID {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        PID {
            kp,
            ki,
            kd,
            previous_error: 0.0,
            integral: 0.0,
        }
    }

    pub fn update(&mut self, setpoint: f64, measured_value: f64) -> f64 {
        let error = setpoint - measured_value;
        self.integral += error;
        let derivative = error - self.previous_error;
        self.previous_error = error;
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}
