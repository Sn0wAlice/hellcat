extern crate hellcat;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    // first step is to detect the OS
    let os = std::env::consts::OS;

    // show the OS
    println!("OS: {}", os);

    match os {
        "linux" => {
            hellcat::machine::linux::hellcat_linux().await;
        },
        "macos" => {
            hellcat::machine::macos::hellcat_macos().await;
        },
        "windows" => {
            hellcat::machine::windows::hellcat_windows().await;
        },
        _ => {
            println!("Unknown OS");
            // kill the program due to unknown OS
            std::process::exit(1);
        }
    }

    Ok(())
}
