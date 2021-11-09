use clap::{App, Arg};

fn get_bitvector(hex_string: &str) -> Vec<u32> {
    let mut bits = Vec::<u32>::with_capacity(hex_string.len() * 4);
    for c in hex_string.chars() {
        println!("{:?}", c);
        let digit = c.to_digit(16).unwrap();
        bits.push(digit >> 3 & 0b1);
        bits.push(digit >> 2 & 0b1);
        bits.push(digit >> 1 & 0b1);
        bits.push(digit & 0b1);
    println!("{:?}", bits);
    }
    return bits
}

// fn print_bits(bits: &Vec<u32>) {
fn print_bits(bits: &[u32]) {
   let mut rows = vec!["".to_string(), "".to_string(), "".to_string()];
   // let s: String = "".to_string();
   for i in 0..bits.len() + 1 {
        let s = format!("{:0>3}", i.to_string());
        println!("{:?}", s);
        for j in 0..s.len() {
            println!("{:?}", rows[j]);
            rows[j].push_str(&s[j..j+1])
        }
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
    print_bits(&get_bitvector(hex_string));
}
