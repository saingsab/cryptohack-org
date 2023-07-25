use ascii_converter::*;
use core::str;
use std::str::FromStr;
use hex::{FromHex, ToHex};

// Challange
// Given the string label, XOR each character with the integer 13. Convert these integers back to a string and submit the flag as crypto{new_string}.

// Todo
// label -> decimal and store in vec
// Each items in vec ^ by 13
// Convert these integers back to a string 

fn main() {
    let sentence_string = 
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
}
