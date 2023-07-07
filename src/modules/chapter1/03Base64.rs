use base64::{engine::general_purpose, Engine as _};

fn base64ToData() {
    let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let decoded = hex::decode(input).expect("Decoding failed");

    println!("{* Here is your flag: }", general_purpose::STANDARD.encode(&decoded));
}