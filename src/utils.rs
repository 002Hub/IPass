use std::io::Write;


pub fn get_args() -> Vec<String> {
    std::env::args().collect() // [0] = file path; [n>0] = argument
}

fn encrypt_pass(pass: &mut String,_mpw: String) -> &mut String {
    return pass;
}

fn decrypt_pass(pass: &mut String, _mpw: String) -> &mut String {
    return pass;
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

pub fn create_entry(name:&String, pw:&mut String) -> bool {
    if std::path::Path::new(&(get_ipass_folder()+name+".ipass")).exists() {
        return false;
    }
    edit_entry(name, pw);
    return true;
}

pub fn get_entry(filepath:&String) -> String {
    let content = &mut std::fs::read_to_string(filepath).expect("Should have been able to read the file");
    let mpw = ask_for_pw();
    return decrypt_pass(content,mpw).to_owned();
}

pub fn edit_entry(name:&String,mut pw:&mut String) {
    let mpw = ask_for_pw();
    pw = encrypt_pass(pw,mpw);
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