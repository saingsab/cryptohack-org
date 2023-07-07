use ascii_converter::*;

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

fn convert_hex_to_decimals(_v: &str){
    
}
fn main() {

    // let v = vec![72, 69, 76, 76, 79];
    // let v = vec![0x48, 0x45, 0x4c, 0x4c, 0x4f];
    let input = "48454c4c4f";
    // convert_hex_bytes_to_ascii(input);
    convert_hex_to_decimals(input);
}
