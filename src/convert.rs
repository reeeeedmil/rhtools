use crate::ConvertGroup;

pub fn convert(input: ConvertGroup) {
    if input.binary != None {
        show_binary(input.binary);
    } else if input.hexadecimal != None {
        show_hexadecimal(input.hexadecimal);
    } else {
        show_octal(input.octal);
    }
}

fn show_binary(input: Option<u32>) {
    let input = match input {
        Some(num) => num,
        None => {
            println!("number is none");
            std::process::exit(0);
        }
    };
    let bin = String::from(format!("{:b}", input));
    let mut empty_string: String = String::new();
    for n in 0..bin.len() {
        if n % 4 == 0 && n != 0 {
            empty_string.push(char::from(32));
        }
        empty_string.push(bin.chars().nth(n).unwrap());
    }
    println!("Input: {}\nBin: {}", input, empty_string);
}

fn show_hexadecimal(input: Option<u32>) {
    let input = match input {
        Some(num) => num,
        None => {
            println!("number is none");
            std::process::exit(0);
        }
    };
    let bin = String::from(format!("{:x}", input));
    let mut empty_string: String = String::new();
    for n in 0..bin.len() {
        if n % 2 == 0 && n != 0 {
            empty_string.push(char::from(32));
        }
        empty_string.push(bin.chars().nth(n).unwrap());
    }
    println!("Input: {}\nHex: {}", input, empty_string);
}
fn show_octal(input: Option<u32>) {
    let input = match input {
        Some(num) => num,
        None => {
            println!("number is none");
            std::process::exit(0);
        }
    };
    let bin = String::from(format!("{:o}", input));
    let mut empty_string: String = String::new();
    for n in 0..bin.len() {
        if n % 3 == 0 && n != 0 {
            empty_string.push(char::from(32));
        }
        empty_string.push(bin.chars().nth(n).unwrap());
    }
    println!("Input: {}\nOctal: {}", input, empty_string);
}
