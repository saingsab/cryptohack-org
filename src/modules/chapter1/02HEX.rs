/*
Hexadecimal can be used in such a way to represent ASCII strings. 
First each letter is converted to an ordinal number according to the ASCII table (as in the previous challenge). 
Then the decimal numbers are converted to base-16 numbers, otherwise known as hexadecimal. 
The numbers can be combined together, into one long hex string.
*/
use ascii_converter::*;

fn hext2string() {
    let input = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let decoded = hex::decode(input).expect("Decoding failed");

    match decimals_to_string(&decoded) {
        Ok(num) => println!("* Here is your flag: {}", num),
        Err(e) => println!("* Error: {}", e),
    }
}