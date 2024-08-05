use crate::sensor::Sensor;

#[derive(Clone, Copy, PartialEq)]
enum Direcao{
    norte,
    sul,
    leste,
    oeste
}

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
    sensor: Sensor,
    direcao: Direcao

}

impl Carteiro{
    pub fn new(x: i32, y: i32) -> Self {
        Self { pos_x: x, pos_y: y, status: Status::menu, sensor: Sensor::new(), direcao: Direcao::norte}
    }

    pub fn get_pos_x(&self) -> i32{
        self.pos_x
    }

    pub fn get_pos_y(&self) -> i32{
        self.pos_y
    }

    pub fn get_sensor_pitch(&self) -> f64 {
        self.sensor.get_pitch()
    }

    pub fn get_sensor_roll(&self) -> f64 {
        self.sensor.get_roll()
    }

    pub fn update_sensor(&mut self) {
        self.sensor.update();
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_direcao(&self) -> Direcao {
        self.direcao
    }

    pub fn verifica_andar(&self) -> bool {
        match self.get_direcao() {
            Direcao::norte => {
                if self.get_pos_y() >= 19 {
                    false
                }
                else {
                    true
                }
            }
            Direcao::sul => {
                if self.get_pos_x() <= 0 {
                    false
                }
                else {
                    true
                }
            }
            Direcao::leste => {
                if self.get_pos_x() >= 19 {
                    false
                }
                else {
                    true
                }
            }
            Direcao::oeste => {
                if self.get_pos_x() <= 0 {
                    false
                }
                else {
                    true
                }
            }
        }
    }

    pub fn andar_x(&mut self) -> Option<bool>{
        match self.get_direcao() {
            Direcao::leste => { 
                self.pos_x += 1;
                None
            },
            Direcao::oeste => { 
                self.pos_x -= 1;
                None
            },
            _ => {
                Some(false)
            }
        }
    }

    pub fn andar_y(&mut self) -> Option<bool>{
        match self.get_direcao() {
            Direcao::norte => { 
                self.pos_y += 1;
                None
            },
            Direcao::sul => { 
                self.pos_y -= 1;
                None
            },
            _ => {
                Some(false)
            }
        }
    }

    pub fn andar(&mut self) -> bool{
        match self.get_direcao() {
            Direcao::norte => {
                if self.verifica_andar(){
                    self.andar_y();
                    true
                }
                else {
                    false
                }
            }
            Direcao::sul => {
                if self.verifica_andar(){
                    self.andar_y();
                    true
                }
                else {
                    false
                }
            }
            Direcao::leste => {
                if self.verifica_andar(){
                    self.andar_x();
                    true
                }
                else {
                    false
                }
            }
            Direcao::oeste => {
                if self.verifica_andar(){
                    self.andar_x();
                    true
                }
                else {
                    false
                }
            }
        }
    }

    
}