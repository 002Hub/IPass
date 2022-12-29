use std::collections::HashMap;
use std::io::{Read, Write};
use rand::rngs::OsRng;
use rand::RngCore;
mod utils;

fn main() {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("x.x.x");
    println!("IPass v{}\n", version);

    let args = utils::get_args();

    if args.len() < 2 {
        help_message(&args);
        return;
    }

    let mode: &str = &args[1].trim().to_lowercase().to_owned();
    
    match mode {
        "list" => list(),
        "add" => add(&args),
        "get" => get(&args),
        "changepw" => changepw(&args),
        "changeuser" => changeuser(&args),
        "remove" => remove(&args),
        "import" => import(&args),
        "export" => export(&args),
        "rename" => rename(&args),
        "version" => version_help(version),
        "clear" => clear(),
        _ => help_message(&args),
    }
}

fn version_help(version: &str) {
    let mut data = version.split(".");
    print!("Major {} ", data.next().unwrap());
    print!("Sub {} ", data.next().unwrap());
    println!("Bugfix {}", data.next().unwrap());
}

fn help_message(args: &Vec<String>) {
    let mut help_messages:HashMap<String, String> = HashMap::new();
    help_messages.insert(
        "list".to_string(),
        "returns a list of all saved entries".to_string(),
    );
    help_messages.insert(
        "add".to_string(),
        "creates a new entry taking a [name] and an optional {password} ".to_string(),
    );
    help_messages.insert(
        "get".to_string(),
        "returns a specific entry, selected by the provided [name]".to_string(),
    );
    help_messages.insert(
        "help".to_string(),
        "tells you this message, takes an optional {command name}".to_string(),
    );
    help_messages.insert(
        "changepw".to_string(),
        "changes the password of the specified entry".to_string(),
    );
    help_messages.insert(
        "remove".to_string(),
        "removes an existing entry".to_string(),
    );
    help_messages.insert(
        "rename".to_string(),
        "renames an existing entry".to_string(),
    );
    help_messages.insert(
        "import".to_string(),
        "import a .ipassx file to your current entries from an optional {directory}".to_string(),
    );
    help_messages.insert(
        "export".to_string(),
        "exports your current entries as an .ipassx file to an optional {directory}".to_string(),
    );
    help_messages.insert(
        "version".to_string(),
        "explains the current version".to_string()
    );
    help_messages.insert(
        "changeuser".to_string(),
        "changes the username of the specified entry".to_string(),
    );
    help_messages.insert(
        "clear".to_string(),
        "clears all entries".to_string(),
    );

    if args.len() < 3 {
        println!("You can use the following commands:");
        for (cmd, expl) in &help_messages {
            println!("\"{cmd}\"{}-  {expl}"," ".repeat(12-cmd.len()));
        }
        return;
    }

    println!("{} {}", &args[2], &help_messages[&args[2]])    
}

fn list() {
    let mut paths = std::fs::read_dir(utils::get_ipass_folder()).unwrap();
    
    let mut has_entry:bool = false;

    println!("Total entries: {}\n", paths.count());

    paths = std::fs::read_dir(utils::get_ipass_folder()).unwrap();

    for path in paths {
        has_entry = true;
        println!("Entry: \"{}\"", path.unwrap().file_name().into_string().unwrap().replace(".ipass", ""));
    }
    
    if !has_entry {
        println!("No entries yet!");
        return;
    }
}

fn add(args: &Vec<String>) {

    if args.len() < 4 || args.len() > 5 {
        println!("Incorrect usage of \"add\"");
        return;
    }

    let pw: String;
    let username:String = args[3].to_string();

    if args.len() > 4 {
        pw = username+";"+args[4].trim();
    } else {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!\"$%&/()=?{[]}\\,.-;:_><|+*#'";
        let alph_len = alphabet.chars().count();
        let char_set:Vec<char> = alphabet.chars().collect();
        let mut chars_index: Vec<u8> = vec![0;20];
        OsRng.fill_bytes(&mut chars_index);
        let mut chars = String::new();

        for index in chars_index {
            // println!("{} - {} - {}",index,(index as usize)%(alph_len-1),alph_len);
            chars += &char_set[(index as usize)%(alph_len-1)].to_string();
        }
        pw = username+";"+chars.as_str();

        println!("Using auto generated password");
        // println!("pw: {pw}");
        
    }

    println!("Adding password for {}",args[2]);
    let succeeded = utils::create_entry(&args[2], pw);
    if !succeeded {
        println!("You already have an entry with that name! Did you mean to use \"edit\"?");
        return;
    }
    println!("Added password for {}",args[2])
}

fn get(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Invalid usage of \"get\"");
        return;
    }
    let name = &args[2];
    let filepath = &(utils::get_ipass_folder()+name+".ipass");
    if std::path::Path::new(filepath).exists() {
        println!("Getting entry");
        let entry = utils::get_entry(name);
        let mut data = entry.split(";");
        println!("Username: '{}' Password: '{}'",data.next().unwrap(),data.next().unwrap());
    } else {
        println!("No such entry!");
        return;
    }
}

fn changepw(args: &Vec<String>) { //rename func to changepw
    if args.len() < 3 {
        println!("Invalid usage of \"changepw\"");
        return;
    }
    let filepath = &(utils::get_ipass_folder()+&args[2]+".ipass");
    if std::path::Path::new(filepath).exists() {
        let output: String;
        if args.len() != 4 {
            output = rpassword::prompt_password("Please enter the new password: ").unwrap();
        } else {
            output = args[3].clone();
        }
        
        utils::edit_password(&args[2], output);

        println!("Changed Password of {}!", args[2]);
    } else {
        println!("No such file!");
    }
}

fn changeuser(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Invalid usage of \"changeuser\"");
        return;
    }
    let filepath = &(utils::get_ipass_folder()+&args[2]+".ipass");
    if std::path::Path::new(filepath).exists() {
        let output: String;
        if args.len() != 4 {
            output = utils::prompt_answer("Enter new Username: ".to_string());
        } else {
            output = args[3].clone();
        }
        
        utils::edit_username(&args[2], output);

        println!("Changed Username of {}!", args[2]);
    } else {
        println!("No such file!");
    }
}

fn rename(args: &Vec<String>) { // prog ren old new
    if args.len() < 4 {
        println!("Invalid usage of \"rename\"");
        return;
    }
    let filepath = &(utils::get_ipass_folder()+&args[2]+".ipass");
    if std::path::Path::new(filepath).exists() {
        utils::rename(&args[2],&args[3]);
        println!("Renamed {} to {}", args[2], args[3]);
    } else {
        println!("No such file");
    }
}

fn remove(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Invalid usage of \"remove\"");
        return;
    }
    let name = &args[2];
    let filepath = &(utils::get_ipass_folder()+name+".ipass");
    if std::path::Path::new(filepath).exists() {
    if utils::prompt_answer(format!("Are you sure you want to delete {}? [y/N] ", name)) == "y" {
        std::fs::remove_file(filepath).unwrap();
        println!("Removed entry \"{}\"", name);
    } else {
        println!("Operation cancelled!")
    }
    } else {
        println!("No entry named \"{}\"", name);
        return;
    }
}

fn import(args: &Vec<String>) {
    let mut location = utils::get_home_folder_str();
    if args.len() == 3 {
        location = args[2].clone();
    } else {
        if utils::prompt_answer(format!("No location specified, defaulting to {} continue? [Y/n] ", location.clone())) == "n" {
            println!("Operation cancelled");
            return;
        }
    }
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

            let mut file = std::fs::File::create(format!("{}/{}.ipass",utils::get_ipass_folder(), name)).unwrap();
            file.write_all(i.as_bytes()).unwrap();
            name = "";
        }
        println!("Imported all entries!");
    } else {
        println!("No such file found!");
    }

}

fn export(args: &Vec<String>) {
    let mut location = utils::get_home_folder_str();
    if args.len() == 3 {
        location = args[2].clone();
    } else {
        if utils::prompt_answer(format!("No location specified, defaulting to {} continue? [Y/n] ", location.clone())) == "n" {
            println!("Operation cancelled");
            return;
        }
    }

    let mut collected_data: String = String::new();

    let paths = std::fs::read_dir(utils::get_ipass_folder()).unwrap();

    for p in paths {
        if let Ok(path) = p {
            let content = &mut std::fs::read_to_string(utils::get_ipass_folder()+&path.file_name().into_string().unwrap()).expect("Should have been able to read the file");
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

        println!("Saved at: '{}/export.ipassx'", location);
    } else {
        println!("Failed saving at '{}/export.ipassx' does it exist?",location)
    }
    
}

fn clear() {
    if utils::prompt_answer("Are you sure you want to clear everything? [y/N] ".to_string()) != "y" {
        println!("operation cancelled!");
        return;
    }

    let paths = std::fs::read_dir(utils::get_ipass_folder()).unwrap();

    for path in paths {
        if let Ok(p) = path {
            std::fs::remove_file(utils::get_ipass_folder()+"/"+p.file_name().into_string().unwrap().as_str()).unwrap();
        }
    }
    println!("Cleared all entries!");
}