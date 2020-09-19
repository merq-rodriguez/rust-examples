extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero!");
    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    
    println!("El numero secreto es: {}", numero_secreto);

    println!("Por favor introduce tu corazonada");
    
    let mut corazonada = String::new();

    io::stdin().read_line(&mut corazonada)
        .ok()
        .expect("Falló al leer linea");
    
        
    let corazonada: u32 = corazonada.trim().parse()
        .ok()
        .expect("Falló al leer linea");
    
    println!("Tú corazonda fue: {}", corazonada);

    match corazonada.cmp(&numero_secreto) {
        Ordering::Less    => println!("Muy bueno!"),
        Ordering::Greater => println!("Muy grande!"),
        Ordering::Equal   => println!("Haz ganado campeon!!"),
    }
}
