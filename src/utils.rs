use std::io::Write;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use sha2::{Sha256, Digest};

pub fn get_args() -> Vec<String> {
    std::env::args().collect() // [0] = file path; [n>0] = argument
}

fn vecu8_to_string(vec: Vec<u8>) -> String {
    let mut out: String = String::new();
    for ind in vec {
        if let Ok(a) = std::str::from_utf8(&[ind]) {
            out += a;
        } else {
            panic!("malformed character");
        }
    }
    return out
}

fn encrypt_pass(nonce_arg:String, pass: String,mpw: String) -> String {
    let mut nonce_argument = String::new();
    if nonce_arg.len() < 12 {
        nonce_argument = nonce_arg.clone() + &" ".repeat(12-nonce_arg.len());
    }
    if nonce_arg.len() > 12 {
        nonce_argument = nonce_arg[0..12].to_string();
    }
    let mut hasher = Sha256::new();
    hasher.update(mpw.as_bytes());

    let master_pw = &hasher.finalize();

    let cipher = Aes256Gcm::new(master_pw);
    let nonce = Nonce::from_slice(nonce_argument.as_bytes()); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(nonce, pass.as_ref()).unwrap();
    return hex::encode(ciphertext);
}

fn decrypt_pass(nonce_arg:String, pass: Vec<u8>,mpw: String) -> String {
    let mut nonce_argument = String::new();
    if nonce_arg.len() < 12 {
        nonce_argument = nonce_arg.clone() + &" ".repeat(12-nonce_arg.len());
    }
    if nonce_arg.len() > 12 {
        nonce_argument = nonce_arg[0..12].to_string();
    }
    let mut hasher = Sha256::new();
    hasher.update(mpw.as_bytes());

    let master_pw = &hasher.finalize();
    let cipher = Aes256Gcm::new(master_pw);
    let nonce = Nonce::from_slice(nonce_argument.as_bytes()); // 96-bits; unique per message

    let plaintext = cipher.decrypt(nonce, pass.as_ref()).unwrap();
    return vecu8_to_string(plaintext);
}

pub fn get_home_folder_str() -> String {
    match home::home_dir() {
        Some(path) => {
            let p = path.to_str();
            match p {
                Some(pa) => return pa.to_owned(),
                None => return "".to_owned(),
            }
        },
        None => return "".to_owned(),
    }
}

pub fn get_ipass_folder() -> String {
    let path = get_home_folder_str()+"/.IPass/";
    std::fs::create_dir_all(&path).unwrap();
    return path;
}

pub fn create_entry(name: &String, pw: String) -> bool {
    if std::path::Path::new(&(get_ipass_folder()+name+".ipass")).exists() {
        return false;
    }
    edit_entry(name, pw);
    return true;
}

pub fn get_entry(name:&String) -> String {
    let content = &mut std::fs::read_to_string(get_ipass_folder()+name+".ipass").expect("Should have been able to read the file");
    let mpw = ask_for_pw();
    return decrypt_pass(name.to_owned(),hex::decode(content).unwrap(),mpw).to_owned();
}

pub fn edit_entry(name:&String,mut pw:String) {
    let mpw = ask_for_pw();
    pw = encrypt_pass(name.to_owned(), pw,mpw);
    let mut file = std::fs::File::create(get_ipass_folder()+name+".ipass").unwrap();
    file.write_all(pw.as_bytes()).unwrap();
}

fn ask_for_pw() -> String {
    let output = rpassword::prompt_password("Please enter the master password: ").unwrap();
    return output.replace("\n", "").replace("\r","");
}

pub fn prompt_answer(toprint: String) -> String {
    print!("{toprint}");
    std::io::stdout().flush().unwrap();
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read choice");

    return choice.trim().to_lowercase();
}

pub fn rename(name: &String, new_name: &String) {
    if !std::path::Path::new(&(get_ipass_folder()+name+".ipass")).exists() {
        return;
    }
    if std::path::Path::new(&(get_ipass_folder()+new_name+".ipass")).exists() {
        return;
    }
    let content = &mut std::fs::read_to_string(get_ipass_folder()+name+".ipass").expect("Should have been able to read the file");
    let mpw = ask_for_pw();
    let mut pw = decrypt_pass(name.to_owned(),hex::decode(content).unwrap(),mpw.clone()).to_owned();

    pw = encrypt_pass(new_name.to_owned(), pw,mpw);
    let mut file = std::fs::File::create(get_ipass_folder()+new_name+".ipass").unwrap();
    file.write_all(pw.as_bytes()).unwrap();
}