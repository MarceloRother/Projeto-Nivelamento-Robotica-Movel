use core::num;
use std::{io::{self, Write}, num::ParseIntError};
use caixa::Caixa;
use carteiro::Carteiro;
use sensor::Sensor;
use pid::PID;
use jogo::Jogo;

mod sensor;
mod pid;
mod jogo;
mod carteiro;
mod caixa;

//  JOGO DA ENTREGA
//
// O jogo consiste em fazer com que o carteiro ('&') leve a caixa ('@') até ó ponto desejado ('X') em um campo que será uma matriz 20x20, onde ('+') representa um espaço válido..
// Para desenvolver tal projeto vocês terão de usar/desenvolver as estruturas 'carteiro', 'caixa', e 'jogo'.
// Obs:
//   - O carteiro só pode andar um 'índice' por iteração
//   - Apliquem a ideia de Encapsulamento
//   - O código tera uma mapa de exemplo para o teste enquanto estiver em desenvolvimento
//   - No dia da apresentação o código será posto em prática com um código diferente 

fn le_terminal() -> Vec<i32> {
    let mut num = Vec::new();

    print!("X: ");
    io::stdout().flush().unwrap();
    let x = aux_le_terminal();
    num.push(x);

    print!("Y: ");
    io::stdout().flush().unwrap();
    let y = aux_le_terminal();
    num.push(y);

    num

}

fn aux_le_terminal() -> i32{
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                match trimmed_input.parse::<i32>() {
                    Ok(num) => return num,
                    Err(_) => {
                        println!("Número inválido. Por favor, digite novamente.");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("Erro ao ler a entrada. Por favor, tente novamente.");
                continue;
            }
        }
    }
}

fn main() {
    // Setup
    let mut matriz: Vec<Vec<char>> = vec![vec!['+'; 20]; 20];

    
    // Declarando Carteiro
    println!("Digite as coordenadas do carteiro.");
    let aux = le_terminal();
    let mut carteiro = Carteiro::new(aux[0], aux[1]);

    // Declarando Caixa
    println!("Digite as coordenadas da caixa.");
    let aux = le_terminal();
    let mut caixa = Caixa::new(aux[0], aux[1]);

    // Coletando coordenadas do destino
    println!("Digite as coordenadas do destino.");
    let aux = le_terminal();

    let mut jogo = Jogo::new(carteiro, caixa, matriz);
    jogo.cria_jogo();
}
