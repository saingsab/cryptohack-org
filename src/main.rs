use ascii_converter::*;
use core::str;
use std::str::FromStr;
use hex::{FromHex, ToHex};

fn convert_ascii_to_string(_v: Vec<u8>) {
    match decimals_to_string(&_v) {
        Ok(num) => println!("* Here is your flag: {}", num),
        Err(e) => println!("* Error: {}", e),
    }
}

fn convert_hex_bytes_to_ascii(_v: &str) {
    /*
    ascii bytes: [72, 69, 76, 76, 79]
    hex bytes: [0x48, 0x45, 0x4c, 0x4c, 0x4f]
    */
    let hex = hex::decode(_v).expect("Decoding failed");
    println!("{:?}", hex)
}

fn convert_hex_to_decimals(_v: &str) {
    let buffer: [u8; 12] = <[u8; 12]>::from_hex([0x48, 0x45, 0x4c, 0x4c, 0x4f]).expect("something went wrong");
    let string = str::from_utf8(&buffer).expect("invalid buffer length");

    println!("{}", string); 
}

// fn main() {

//     // let v = vec![72, 69, 76, 76, 79];
//     // let v = vec![0x48, 0x45, 0x4c, 0x4c, 0x4f];
//     let input = "48454c4c4f";
//     ;
//     convert_hex_to_decimals("d");
// }
fn three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | '0' | 'u' => {
               vowel_count += 1;
               if vowel_count >= 3 {
                    return true;
               } 
            }
            _ => vowel_count = 0 
        }
    }
    false
}

fn main() {
    let sentence_string = 
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{} has three consecutive vowels!", word);
        }
    }
}
