/*
An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.

For example:

9 is an Armstrong number, because 9 = 9^1 = 9
10 is not an Armstrong number, because 10 != 1^2 + 0^2 = 1
153 is an Armstrong number, because: 153 = 1^3 + 5^3 + 3^3 = 1 + 125 + 27 = 153
154 is not an Armstrong number, because: 154 != 1^3 + 5^3 + 4^3 = 1 + 125 + 64 = 190

*/

use std::io;

fn is_armstrong_number (number : &i32) -> bool {
    let number_str = number.to_string();
    let length = number_str.len();
    let sum : i32 = number_str.chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|n| n.pow(length as u32))
        .map(|number| number as i32)
        .sum();

    sum == *number
}


fn number_input_kb() -> i32 {
    let mut input = String::new();
    println!("Inserisci un numero:");
    io::stdin().read_line(&mut input)
        .expect("Errore durante la lettura da tastiera");

    input.trim();

    match input.parse::<i32>(){
        Ok(number) => return number,
        _ => panic!("Errore durante il parsing dell'input")
    }
}


fn main() {

    let input = number_input_kb();
    match is_armstrong_number(&input) {
        true => println!("il valore inserito è un numero di Armstrong"),
        false => print!("Il valore inserito non è un numero di Armstrong")
    }
}