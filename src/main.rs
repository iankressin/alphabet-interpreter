use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut alphabet: Vec<char> = Vec::new();
    let mut error = false;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        match index {
            0 => {
                create_alphabet(&line, &mut alphabet);
            }
            _ => {
                error = validate_word(&line, index, &alphabet);
            }
        } 
    }

    if !error {
        println!("Seu programa e valido!");
    }
}

fn create_alphabet(first_line: &str, alphabet: &mut Vec<char>) {
    let separator = ' ';

    for c in first_line.chars() {
        if c != separator {
            alphabet.push(c);
        }
    } 
}

fn validate_word(word: &str, line: usize, alphabet: &Vec<char>) -> bool {
    let mut error = false;

    for (i, c) in word.chars().enumerate() {
        if !alphabet.contains(&c) {
            error_handler(&line, &i, &c, &alphabet);
            error = true;
        }
    }

    error
} 

fn error_handler(line: &usize, col: &usize, character: &char, alphabet: &Vec<char>) {
    println!("O programa nao e valido pois apresenta caracter invalido:
    ---------------------
    | ==> Caracter: {}  |
    | ==> Linha: {}     |
    | ==> Coluna: {}    |
    ---------------------
Corrija o caracter indicado por alguma das possibilidades: {:?}",
        character, line, col, alphabet
    );

    std::process::exit(0);
}

