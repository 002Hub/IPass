use std::io::{Read, Write};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce
};
use sha2::{Sha256, Digest};

pub fn get_args() -> Vec<String> {
    std::env::args().collect() // [0] = file path; [n>0] = argument
}

pub fn import_file(location:&String) -> bool {
    if std::path::Path::new(&(location.clone()+"/export.ipassx")).exists() {
        let mut reader = brotli::Decompressor::new(
            std::fs::File::open(location.clone()+"/export.ipassx").unwrap(),
            4096, // buffer size
        );
        let mut content: String = String::new();
        let mut buf = [0u8; 4096];
        loop {
            match reader.read(&mut buf[..]) {
                Err(e) => {
                    if let std::io::ErrorKind::Interrupted = e.kind() {
                        continue;
                    }
                    panic!("{}", e);
                }
                Ok(size) => {
                    if size == 0 {
                        break;
                    }
                    content += &std::str::from_utf8(&buf[..size]).unwrap();
                }
            }
        }

        let lines = content.lines();
        let mut name = "";
        for i in lines {
            if name == "" {
                name = i;
                continue;
            }

            let mut file = std::fs::File::create(format!("{}/{}.ipass",get_ipass_folder(), name)).unwrap();
            file.write_all(i.as_bytes()).unwrap();
            name = "";
        }
        return true;
    } else {
        return false;
    }
}

pub fn export_file(location:&String) -> bool {
    let mut collected_data: String = String::new();

    let paths = std::fs::read_dir(get_ipass_folder()).unwrap();

    for p in paths {
        if let Ok(path) = p {
            let content = &mut std::fs::read_to_string(get_ipass_folder()+&path.file_name().into_string().unwrap()).expect("Should have been able to read the file");
            collected_data += format!("{}\n{}\n", path.file_name().into_string().unwrap().replace(".ipass", ""),content).as_str();
        }
    }

    if let Ok(file) = std::fs::File::create(location.clone()+"/export.ipassx") {
        let mut writer = brotli::CompressorWriter::new(
            file,
            4096,
            11,
            22);
        
        match writer.write_all(collected_data.as_bytes()) {
            Err(e) => panic!("{}", e),
            Ok(_) => {},
        }

        return true;
    } else {
        return false;
    }
}

fn vecu8_to_string(vec: Vec<u8>) -> String {
    let mut do_print_warning = false;
    let mut out: String = String::new();
    for ind in vec {
        if let Ok(a) = std::str::from_utf8(&[ind]) {
            out += a;
        } else {
            do_print_warning = true;
            eprintln!("[WARNING] malformed character {}",ind);
            let mut temp_vec: Vec<u8> = Vec::new();
            temp_vec.insert(0,ind%128);
            out += vecu8_to_string(temp_vec).as_str();
        }
    }
    if do_print_warning {
        println!("[WARNING] Output may be corrupt");
    }
    return out;
}

fn encrypt_pass(nonce_arg:String, pass: String,mpw: String) -> String {
    let mut nonce_argument = String::new();
    if nonce_arg.len() < 12 {
        nonce_argument = nonce_arg.clone() + &" ".repeat(12-nonce_arg.len());
    }
    if nonce_arg.len() > 12 {
        nonce_argument = nonce_arg[0..12].to_string();
    }

    let mut nonce_hasher = Sha256::new();
    nonce_hasher.update(nonce_argument.as_bytes());

    let nonce_final = &nonce_hasher.finalize()[0..12];


    let mut hasher = Sha256::new();
    hasher.update(mpw.as_bytes());

    let master_pw = &hasher.finalize();

    let cipher = Aes256Gcm::new(master_pw);
    let nonce = Nonce::from_slice(nonce_final); // 96-bits; unique per message
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

    let mut nonce_hasher = Sha256::new();
    nonce_hasher.update(nonce_argument.as_bytes());

    let nonce_final = &nonce_hasher.finalize()[0..12];

    let mut hasher = Sha256::new();
    hasher.update(mpw.as_bytes());

    let master_pw = &hasher.finalize();
    let cipher = Aes256Gcm::new(master_pw);
    let nonce = Nonce::from_slice(nonce_final); // 96-bits; unique per message

    let plaintext = cipher.decrypt(nonce, pass.as_ref());
    match plaintext {
        Ok(res) => {
            return vecu8_to_string(res);
        }
        Err(_) => {
            eprintln!("[ERROR] Error decrypting data, check your master password");
            std::process::exit(1);
        }
    }
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
    let mpw = ask_for_pw();
    // println!("{pw}");
    let pw = encrypt_pass(name.to_owned(), pw,mpw);
    let mut file = std::fs::File::create(get_ipass_folder()+name+".ipass").unwrap();
    file.write_all(pw.as_bytes()).unwrap();
    return true;
}

fn read_entry(name:&String,mpw:String) -> String {
    let content = &mut std::fs::read_to_string(get_ipass_folder()+name+".ipass").expect("Should have been able to read the file");
    return decrypt_pass(name.to_owned(),hex::decode(content).unwrap(),mpw).to_owned();
}

pub fn get_entry(name:&String) -> String {
    let mpw = ask_for_pw();
    return read_entry(name,mpw);
}

pub fn edit_password(name:&String, password:String) {
    let mpw = ask_for_pw();
    let entry = read_entry(name, mpw.clone());
    // println!("entry: {entry}");
    let mut parts = entry.split(";");
    let username = parts.next().unwrap().to_string();
    let _old_password = parts.next().unwrap();
    let data = encrypt_pass(name.to_owned(), username+";"+password.as_str(),mpw);
    let mut file = std::fs::File::create(get_ipass_folder()+name+".ipass").unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn edit_username(name:&String, username: String) {
    let mpw = ask_for_pw();
    let entry = read_entry(name, mpw.clone());
    // println!("entry: {entry}");
    let mut parts = entry.split(";");
    let _old_username = parts.next().unwrap();
    let password = parts.next().unwrap();
    let data = encrypt_pass(name.to_owned(), username+";"+password,mpw);
    let mut file = std::fs::File::create(get_ipass_folder()+name+".ipass").unwrap();
    file.write_all(data.as_bytes()).unwrap();
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