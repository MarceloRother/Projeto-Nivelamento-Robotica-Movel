use core::num;
use std::{io, num::ParseIntError};
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
    let x = aux_le_terminal();
    num.push(x);

    print!("Y: ");
    let y = aux_le_terminal();
    num.push(y);

    num

}

fn aux_le_terminal() -> i32{
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input.trim();

        match input.parse::<i32>(){
            Ok(num) => return num,
            Err(_) => continue,
        }
        println!("Numero invalido. Por favor digite novamente.")
    }
}

fn main() {
    // Setup
    let matriz: [[char; 20]; 20] = [['+'; 20]; 20];
    
    // Coordenadas do carteiro
    println!("Digite as coordenadas do carteiro.");
    let aux = le_terminal();

    // Coordenadas da caixa
    println!("Digite as coordenadas da caixa.");
    let aux = le_terminal();

    // Coordenadas do destino
    println!("Digite as coordenadas do destino.");
    let aux = le_terminal();

    let jogo = Jogo::default();
    jogo.cria_jogo(matriz);
}
