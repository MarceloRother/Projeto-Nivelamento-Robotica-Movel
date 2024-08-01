use rand::Rng;

pub struct Sensor {
    pub pitch: f64,
    pub roll: f64,
}

impl Sensor {
    pub fn new() -> Self {
        Sensor { pitch: 0.0, roll: 0.0 }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.pitch += rng.gen_range(-1.0..1.0);
        self.roll += rng.gen_range(-1.0..1.0);
    }
}
