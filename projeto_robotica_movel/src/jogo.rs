use crate::caixa::Caixa;
use crate::carteiro;
use crate::caixa;
use crate::carteiro::Carteiro;

pub struct Jogo{
    carteiro: carteiro::Carteiro,
    caixa: caixa::Caixa
}

impl Jogo {
    pub fn joga(&self){
    
    }

    pub fn cria_jogo(self, matriz: [[char; 20]; 20]){
        
    }
}

impl Default for Jogo {
    fn default() -> Self {
        Self {carteiro: Carteiro::default(), caixa: Caixa::default()}
    }
}