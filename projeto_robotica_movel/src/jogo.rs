use std::os::windows::process;
use std::{sync::RwLockWriteGuard, process::Command};
use crate::caixa::Caixa;
use crate::carteiro::Carteiro;

pub struct Jogo{
    carteiro: Carteiro,
    caixa: Caixa,
    mapa: Vec<Vec<char>>
}

impl Jogo {
    // Construtor
    pub fn new(novo_cateiro: Carteiro, novo_caixa: Caixa, mapa: Vec<Vec<char>>) -> Self {
        Self {carteiro: novo_cateiro, caixa: novo_caixa, mapa: mapa}
    }

    // Funcao para limpar terminal
    pub fn limpa_terminal(&self) {
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap();
        } else {
            // Comando para outros sistemas operacionais, como Linux e macOS
            Command::new("clear").status().unwrap();
        }
    }

    // Imprimi e atualiza mapa
    pub fn imprime_mapa(&mut self){
        // Imprime mapa
        for (i, row) in self.mapa.iter().enumerate(){
            for (j, elem) in row.iter().enumerate(){
                print!("{}", elem);
            }
            print!("\n");
        }
        println!("\nPitch: {}\t Roll: {}\n", self.carteiro.get_sensor_pitch(), self.carteiro.get_sensor_roll());
    }

    pub fn update(&mut self){
        // Update dos sensores do carteiro
        self.carteiro.update_sensor();
        if self.carteiro.get_sensor_pitch() > 1.0 || self.carteiro.get_sensor_pitch() < -1.0 ||
         self.carteiro.get_sensor_roll() > 1.0 || self.carteiro.get_sensor_roll() < -1.0 {
            println!("Ops!! O Carteiro caiu e seu jogo acabou!!\n");
            self.imprime_mapa();
            std::process::exit(0);
        }

        // Atualiza local do carteiro
        for (i_usize, row) in self.mapa.iter_mut().enumerate() {
            let i = i_usize as i32;
            if i == self.carteiro.get_pos_x() {
                for (j_usize, elem) in  row.iter_mut().enumerate(){
                    let j = j_usize as i32;
                    if j == self.carteiro.get_pos_y() {
                        *elem = '&';
                    }
                }
            }  
        }

        // Atualiza local da caixa
        for (i_usize, row) in self.mapa.iter_mut().enumerate() {
            let i = i_usize as i32;
            if i == self.caixa.get_pos_x() {
                for (j_usize, elem) in  row.iter_mut().enumerate(){
                    let j = j_usize as i32;
                    if j == self.caixa.get_pos_y() {
                        *elem = '@';
                    }
                }
            }  
        }
    }

    pub fn joga(&mut self){
        self.limpa_terminal();
        loop {
            self.update();
            self.imprime_mapa();
        }
    }
}