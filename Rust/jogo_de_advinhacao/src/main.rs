extern crate rand;

use core::num;
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Advinhe um número!");

    let numero_secreto = rand::thread_rng().gen_range(1..101);

    //println!("O número secreto é: {}", numero_secreto);

    loop {
        println!("Digite seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada!");

        let palpite: i32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Palpite inválido. Por favor, digite um número válido.");
                return;
            }
        };

        println!("Você disse: {}", palpite);

        // Outra forma de comparação
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }

        // Outra forma de comparação
        if palpite == numero_secreto {
            println!("Parabéns! você acertou o número secreto!");
            break;
        } else {
            println!("Você não acertou o número secreto. Tente novamente.")
        }
    }
    
}