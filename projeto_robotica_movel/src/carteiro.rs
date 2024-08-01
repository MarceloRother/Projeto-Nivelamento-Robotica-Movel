enum Status{
    menu,
    jogando_sem_caixa,
    jogando_com_caixa,
    fim
}

pub struct Carteiro{
    pos_x: i32,
    pos_y: i32,
    status: Status
}

impl Carteiro{

}

impl Default for Carteiro {
    fn default() -> Self {
        Self { pos_x: 0, pos_y: 0, status: Status::menu}
    }
}