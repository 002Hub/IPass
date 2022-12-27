use std::collections::HashMap;
use home::home_dir;
use std::fs;

fn get_args() -> Vec<String> {
    std::env::args().collect() // [0] = file path; [n>0] = argument
}

fn get_home_folder_str() -> String {
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

fn get_ipass_folder() -> String {
    let path = get_home_folder_str()+"/.IPass/";
    fs::create_dir_all(&path).unwrap();
    return path;
}

fn main() {
    let args = get_args();

    if args.len()<2 {
        help_message()
    }

    let mode: &str = &args[1].trim().to_lowercase().to_owned();
    
    match mode {
        "list" => list(),
        "add" => add(),
        "get" => get(),
        "edit" => edit(),
        "remove" => remove(),
        _ => help_message(),
    }

    

}

fn help_message() {
    
    let version = "0.0.1";
    println!("IPass v{}\n", version);
    
    let args = get_args();

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
        "edit".to_string(),
        "lets you edit an existing entry, given the name and the new password".to_string(),
    );
    help_messages.insert(
        "remove".to_string(),
        "removes an existing entry".to_string(),
    );

    if args.len()<3 {
        print!("The possible commands are: ");
        for i in help_messages.keys() {
            print!("\"{i}\" ");
        }
        return;
    }



    println!("{} {}", &args[2], &help_messages[&args[2]])    
}

fn get_pw() -> String {
    let mut output: String = String::new();
    std::io::stdin().read_line(&mut output).expect("Failed to read line");
    return output.replace("\n", "").replace("\r","");
}

fn list() {
    todo!("List websites/names");
}

fn add() {
    todo!("Add password")
    // create_entry(args[3],encrypt_pass(args[4]));
}

fn get() {
    todo!("Get password")
}

fn edit() {
    todo!("Edit entry")
}

fn remove() {
    todo!("Removes entry")
}

fn encrypt_pass(pass: &str) -> &str {
    return pass;
}

/*
prog.exe list -> all saved entries
prog.exe add [Name] \n **in stars** {password}
prog.exe get [Name]
*/