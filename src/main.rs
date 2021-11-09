use clap::{App, Arg};
use std::io::{Write};

fn get_bitvector(hex_string: &str) -> Vec<u32> {
    let mut bits = Vec::<u32>::with_capacity(hex_string.len() * 4);
    for c in hex_string.chars() {
        let digit = c.to_digit(16).unwrap();
        bits.push(digit >> 3 & 0b1);
        bits.push(digit >> 2 & 0b1);
        bits.push(digit >> 1 & 0b1);
        bits.push(digit & 0b1);
    }
    return bits
}

fn print_bits(hex_string: &str) {
   let bits = get_bitvector(hex_string);
   let mut rows = vec!["".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()];
   // let s: String = "".to_string();
   for i in 0..bits.len() {
        if i % 4 == 0 {
            for j in 0..rows.len() - 1 {
                rows[j].push_str(" ")
            }
            rows[4].push_str("    ");
            rows[4].push_str(&hex_string[i / 4 .. i / 4 + 1].to_uppercase());
        }

        let s = format!("{:0>2}", i.to_string());
        for j in 0..s.len() {
            rows[j].push_str(&s[j..j+1])
        }
        rows[2].push_str("");
        rows[3].push_str(&bits[i].to_string());
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    for row in rows.iter() {
        handle.write_all(format!("{:}\n", row).as_bytes()).unwrap();
        // println!("{}", row)
    }
}

fn main() {
    let matches = App::new("bits")
        .version("1.0")
        .author("Justin Ginn <justindavid.ginn@gmail.com")
        .about("Display hex strings bit by bit")
        .arg(
            Arg::with_name("hex_string")
                .help("String of hex digits to display bit by bit")
                .required(true),
        )
        .get_matches();

    let hex_string = matches.value_of("hex_string").unwrap();
    print_bits(hex_string);
}
