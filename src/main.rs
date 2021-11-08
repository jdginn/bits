use clap::{App, Arg};

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
    let mut bits = Vec::with_capacity(hex_string.len() * 4);
    for c in hex_string.chars() {
        println!("{:?}", c);
        let digit = c.to_digit(16).unwrap();
        bits.push(digit >> 3 & 0b1);
        bits.push(digit >> 2 & 0b1);
        bits.push(digit >> 1 & 0b1);
        bits.push(digit & 0b1);
    println!("{:?}", bits);
    }
}
