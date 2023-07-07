// ASCII is a 7-bit encoding standard which allows the representation of text using the integers 0-127.
use ascii_converter::*;
fn ascii2char() {
    let v = vec![99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125];
    match decimals_to_string(&v) {
        Ok(num) => println!("* Here is your flag: {}", num),
        Err(e) => println!("* Error: {}", e),
    }
}