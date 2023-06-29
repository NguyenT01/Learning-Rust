// EXERCISE 1: CONVERT TEMPERATURES BETWEEN FAHRENHEIT AND CELSIUS
use std::io;

fn main() {
    println!("Please type the temperature: ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    println!("Please select temperature unit to transform: [c stands for Celsius, f stands for Fahrenheit]");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let temp: u32 = temp.trim().parse().expect("Please type a number");
    let unit: char = unit.trim().parse().expect("Please type a character");

    let res = checking(temp, unit);
    if res.1 {
        println!("Temperate is: {}{unit}", res.0);
    }
}

fn checking(t: u32, unit: char) -> (f64, bool) {
    let result;
    let is_worked;
    
    if unit == 'f' {
        result = c_to_f(t);
        is_worked = true;
    } else if unit == 'c' {
        result = f_to_c(t);
        is_worked = true;
    } else {
        println!("Unit is not correct");
        is_worked = false;
        result = 0.0;
    }
    (result, is_worked)
}

fn c_to_f(c: u32) -> f64 {
    c as f64 * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: u32) -> f64 {
    (f as f64 - 32.0) * 5.0 / 9.0
}
