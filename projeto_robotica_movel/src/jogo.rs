use std::sync::RwLockWriteGuard;
use crate::caixa::Caixa;
use crate::carteiro::Carteiro;

pub struct Jogo{
    carteiro: Carteiro,
    caixa: Caixa,
    mapa: Vec<Vec<char>>
}

impl Jogo {
    pub fn new(novo_cateiro: Carteiro, novo_caixa: Caixa, mapa: Vec<Vec<char>>) -> Self {
        Self {carteiro: novo_cateiro, caixa: novo_caixa, mapa: mapa}
    }

    pub fn imprime_mapa(&self){
        for (i, row) in self.mapa.iter().enumerate(){
            for (j, elem) in row.iter().enumerate(){
                print!("{}", elem);
            }
            print!("\n");
        }
    }

    pub fn cria_jogo(&mut self){

        // Definindo local inicial do carteiro
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

        // Definindo local da caixa
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
        self.joga();
    }

    pub fn joga(&self){
        self.imprime_mapa();
    }
}