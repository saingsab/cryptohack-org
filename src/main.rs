use ascii_converter::*;
fn main() {
    // let decoded_string = hex::decode("63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d");
    // let v = vec![decoded_string];

   // let v1 = vec![99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125];
    // match decimals_to_string(v) {
    //     Ok(num) => println!("* Output: {}", num),
    //     Err(e) => println!("* Error: {}", e),
    // }
    // println!("{:?}", v1);
    let input = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let decoded = hex::decode(input).expect("Decoding failed");

    match decimals_to_string(&decoded) {
        Ok(num) => println!("* Output: {}", num),
        Err(e) => println!("* Error: {}", e),
    }
}
