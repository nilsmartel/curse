// #![feature(stdin_forwarders)]
use std::{collections::HashSet, io::Read};

fn main() {
    let sampletext = include_str!("../sample-text");

    // set of cursed character modifiers
    let set = sampletext
        .chars()
        .filter(not_alpha)
        .filter(|c| *c != '\n' && *c != '\r')
        .collect::<HashSet<char>>();
    let set = set.into_iter().collect::<Vec<char>>();

    let intensity = if let Some(intensity) = std::env::args().skip(1).next() {
        match intensity.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("please specify intensity as argument");
                std::process::exit(1);
            }
        }
    } else {
        4
    };

    // read text from stdin.

    let mut line = String::with_capacity(64);
    loop {
        let bytes_read = std::io::stdin().read_line(&mut line);
        match bytes_read {
            Ok(0) => break,
            Err(_) => break,
            _ => {},
        }

        for c in line.chars() {
            print!("{}", c);

            // print a couple cursed characters
            for _ in 0..intensity {
                let elem = {
                    let n = rand::random::<f32>() * set.len() as f32;

                    set[n as usize]
                };
                print!("{}", elem);
            }
        }

        line.clear();
        println!();
    }
}

fn not_alpha(c: &char) -> bool {
    !c.is_alphabetic()
}
