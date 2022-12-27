use std::collections::HashMap;
use rand::rngs::OsRng;
use rand::RngCore;
use std::io::Write;
mod utils;

fn main() {
    let version = "0.1.1";
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
        "edit" => edit(&args),
        "remove" => remove(&args),
        "import" => import(&args),
        "export" => export(&args),
        "rename" => rename(&args),
        "version" => version_help(version),
        _ => help_message(&args),
    }
}

fn version_help(version: &str) {
    let mut data = version.split(".");
    print!("Major {} ", data.next().unwrap());
    print!("Sub {} ", data.next().unwrap());
    print!("Bugfix {}", data.next().unwrap());
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
        "edit".to_string(),
        "lets you edit an existing entry, given the name and the new password".to_string(),
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


    if args.len() < 3 {
        println!("You can use the following commands:");
        for i in help_messages.keys() {
            println!("\"{i}\"{}-  {}"," ".repeat(8-i.len()),help_messages[i]);
        }
        return;
    }

    println!("{} {}", &args[2], &help_messages[&args[2]])    
}

fn list() {
    let paths = std::fs::read_dir(utils::get_ipass_folder()).unwrap();

    for path in paths {
        println!("Entry: \"{}\"", path.unwrap().file_name().into_string().unwrap().replace(".ipass", ""));
    }
}

fn add(args: &Vec<String>) {

    if args.len() < 3 || args.len() > 4 {
        println!("Incorrect usage of \"add\"");
        return;
    }

    let mut pw: String;

    if args.len() > 3 {
        pw = args[3].to_owned();
    } else {
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!\"§$%&/()=?´`²³{[]}\\,.-;:_><|+*#'";
        let char_set:Vec<char> = alphabet.clone().chars().collect();
        let mut chars_index: Vec<u8> = vec![0;20];
        OsRng.fill_bytes(&mut chars_index);
        let mut chars = String::new();

        for index in chars_index {
            // println!("index: {index} : {}",(index%((alphabet.len()-1) as u8)));
            chars += &char_set[(index%((alphabet.len()-1) as u8)) as usize].to_string();
        }

        pw = chars;
        
    }

    println!("Adding password for {}",args[2]);
    let succeeded = utils::create_entry(&args[2],&mut pw);
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
        println!("{}",utils::get_entry(filepath));
    } else {
        println!("No such entry!");
        return;
    }
}

fn edit(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Invalid usage of \"edit\"");
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
        
        let mut file = std::fs::File::create(format!("{}/{}.ipass",utils::get_ipass_folder(),args[2])).unwrap();
        file.write_all(output.replace("\n", "").replace("\r","").as_bytes()).unwrap();

        println!("Changed Password of {}!", args[2]);
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
        std::fs::rename(format!("{}/{}.ipass",utils::get_ipass_folder(),args[2]), format!("{}/{}.ipass",utils::get_ipass_folder(),args[3])).unwrap();
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
    if utils::prompt_answer(format!("Are you sure you want to delete {}? [y/N]", name)) == "y" {
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
        let content = &mut std::fs::read_to_string(location.clone()+"/export.ipassx").expect("Should have been able to read the file");
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
        print!("Imported all entries!");
    } else {
        println!("No such file found!");
    }

}

fn export(args: &Vec<String>) { //TODO: compress data
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

    let mut file = std::fs::File::create(location.clone()+"/export.ipassx").unwrap();
    file.write_all(collected_data.as_bytes()).unwrap();

    println!("Saved at: {}/export.ipassx", location);
}