use rand::rng;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
use zxcvbn::zxcvbn;

const UPPERCASE_CHARS: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWERCASE_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &[u8] = b"123456789";
const SYMBOLS: &[u8] = b"~!@#$%&*_+";

pub fn process_genpass(length: u8, no_uppercase: bool, no_lowercase: bool, no_numbers: bool, no_symbols: bool) -> anyhow::Result<()> {
    let mut rng = rng();
    let mut password = Vec::with_capacity(length as usize);
    let mut chars = Vec::new();

    if !no_uppercase {
        password.push(UPPERCASE_CHARS.choose(&mut rng).expect("UPPERCASE_CHARS is empty").clone());
        chars.extend_from_slice(UPPERCASE_CHARS);
    }
    if !no_lowercase {
        password.push(LOWERCASE_CHARS.choose(&mut rng).expect("LOWERCASE_CHARS is empty").clone());
        chars.extend_from_slice(LOWERCASE_CHARS);
    }
    if !no_numbers {
        password.push(NUMBERS.choose(&mut rng).expect("NUMBERS is empty").clone());
        chars.extend_from_slice(NUMBERS);
    }
    if !no_symbols {
        password.push(SYMBOLS.choose(&mut rng).expect("SYMBOLS is empty").clone());
        chars.extend_from_slice(SYMBOLS);
    }

    for _ in 0..length - password.len() as u8 {
        password.push(chars.choose(&mut rng).expect("chars is empty").clone());
    }

    password.shuffle(&mut rng);

    let password_str = String::from_utf8(password).unwrap();
    let result = zxcvbn(&password_str, &[]);
    eprintln!("{:?}", result.score());
    println!("{}", password_str);
    Ok(())
}   
