#![feature(stdin_forwarders)]
use std::{collections::HashSet, io::Read};

fn main() {
    let sampletext = include_str!("../sample-text");

    // set of cursed character modifiers
    let set = sampletext.chars().filter(not_alpha).filter(|c| *c != '\n' && *c != '\r' ).collect::<HashSet<char>>();
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

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
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

        println!();
    }
}

fn not_alpha(c: &char) -> bool {
    !c.is_alphabetic()
}