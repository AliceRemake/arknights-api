use rand::Rng;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();

    if args.len() <= 1 {
        panic!("Usage: {} <token_length>", &args[0]);
    }

    let mut length: u8 = args[1].parse()?;

    let mut token: Vec<char> = Vec::new();

    let mut rng = rand::thread_rng();

    while length > 0 {
        match rng.gen_range(0..3) {
            0 => token.push(rng.gen_range(48u8..58u8) as char),
            1 => token.push(rng.gen_range(65u8..91u8) as char),
            2 => token.push(rng.gen_range(97u8..123u8) as char),
            _ => panic!("rand out of range"),
        }
        length = length - 1;
    }

    println!("{}", token.iter().collect::<String>());

    Ok(())
}
