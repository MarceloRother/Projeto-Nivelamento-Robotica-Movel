use crate::sensor::Sensor;

#[derive(Clone, Copy)]
enum Status{
    menu,
    jogando_sem_caixa,
    jogando_com_caixa,
    fim
}

pub struct Carteiro{
    pos_x: i32,
    pos_y: i32,
    status: Status,
    sensor: Sensor

}

impl Carteiro{
    pub fn new(x: i32, y: i32) -> Self {
        Self { pos_x: x, pos_y: y, status: Status::menu, sensor: Sensor::new()}
    }

    pub fn get_pos_x(&self) -> i32{
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32{
        self.pos_x
    }

    pub fn get_status(&self) -> Status{
        self.status
    }
}