#![warn(missing_docs, single_use_lifetimes, unused_lifetimes, trivial_casts, trivial_numeric_casts)]
#![forbid(unsafe_code)]

//! IPass Password Manager

extern crate ip_lib;

use std::collections::HashMap;
use std::io::Write;

fn main() {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("x.x.x");
    println!("IPass v{}\n", version);

    let args = ip_lib::get_args();

    if args.len() < 2 {
        help_message(&args);
        return;
    }

    let mode: &str = &args[1].trim().to_lowercase().to_owned();

    let sync_file_loc = ip_lib::get_home_folder_str()+"/.sync.ipass";
    let sync_file_path = std::path::Path::new(&sync_file_loc);

    let sync_enabled = sync_file_path.clone().exists();
    let sync_loc: String;

    if sync_enabled {
        // println!("Sync enabled, syncing...");
        sync_loc = std::fs::read_to_string(sync_file_path).expect("Should have been able to read the sync file");
        ip_lib::import_file(&sync_loc);
    } else {
        sync_loc = "".to_string(); //sync is disabled, no location needed
    }
    
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
        "sync" => sync(&args),
        "isync" => isync(&args),
        "clear" => clear(),
        _ => help_message(&args),
    }
    if sync_enabled {
        ip_lib::export_file(&sync_loc);
    }
}

fn ask_for_pw() -> String {
    let output = rpassword::prompt_password("Please enter the master password: ").unwrap();
    return output.replace("\n", "").replace("\r","");
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
    help_messages.insert(
        "sync".to_string(),
        "automatically sync your data with a specified file".to_string(),
    );
    help_messages.insert(
        "isync".to_string(),
        "not fully implemented yet; ignore this for now  | Syncs the database to IPass servers".to_string(),
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
    let mut paths = ip_lib::get_entries();
    
    let mut has_entry:bool = false;

    println!("Total entries: {}\n", paths.count());

    paths = ip_lib::get_entries();

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
    //! arguments: program add [name] {password}

    if args.len() < 4 || args.len() > 5 {
        println!("Incorrect usage of \"add\"");
        return;
    }

    let pw: String;
    let username:String = args[3].to_string();

    if args.len() > 4 {
        pw = username+";"+args[4].trim();
    } else {
        let chars = ip_lib::random_password();
        pw = username+";"+chars.as_str();

        println!("Using auto generated password");
        // println!("pw: {pw}");
        
    }

    println!("Adding password for {}",args[2]);
    let succeeded: bool = ip_lib::create_entry(&args[2], pw, ask_for_pw());
    if !succeeded {
        println!("You already have an entry with that name! Did you mean to use \"edit\"?");
        return;
    }
    println!("Added password for {}",args[2])
}

fn get(args: &Vec<String>) {
    //! arguments: program get [name]
    if args.len() < 3 {
        println!("Invalid usage of \"get\"");
        return;
    }
    let name: &String = &args[2];
    let filepath = &(ip_lib::get_ipass_folder()+name+".ipass");
    if std::path::Path::new(filepath).exists() {
        println!("Getting entry");
        let entry: String = ip_lib::get_entry(name, ask_for_pw());
        let mut data = entry.split(";");
        println!("Username: '{}' Password: '{}'",data.next().unwrap(),data.next().unwrap());
    } else {
        println!("No such entry!");
        return;
    }
}

fn changepw(args: &Vec<String>) { 
    //! arguments: program changepw [name] {new_password}
    if args.len() < 3 {
        println!("Invalid usage of \"changepw\"");
        return;
    }
    let filepath = &(ip_lib::get_ipass_folder()+&args[2]+".ipass");
    if std::path::Path::new(filepath).exists() {
        let output: String;
        if args.len() != 4 {
            output = rpassword::prompt_password("Please enter the new password: ").unwrap();
        } else {
            output = args[3].clone();
        }
        
        ip_lib::edit_password(&args[2], output, ask_for_pw());

        println!("Changed Password of {}!", args[2]);
    } else {
        println!("No such file!");
    }
}

fn changeuser(args: &Vec<String>) {
    //! arguments: program changeuser [name] {new_username}
    if args.len() < 3 {
        println!("Invalid usage of \"changeuser\"");
        return;
    }
    let filepath = &(ip_lib::get_ipass_folder()+&args[2]+".ipass");
    if std::path::Path::new(filepath).exists() {
        let output: String;
        if args.len() != 4 {
            output = ip_lib::prompt_answer("Enter new Username: ".to_string());
        } else {
            output = args[3].clone();
        }
        
        ip_lib::edit_username(&args[2], output, ask_for_pw());

        println!("Changed Username of {}!", args[2]);
    } else {
        println!("No such file!");
    }
}

fn rename(args: &Vec<String>) {
    //! arguments: program rename [name] [new_name]
    if args.len() < 4 {
        println!("Invalid usage of \"rename\"");
        return;
    }
    let filepath = &(ip_lib::get_ipass_folder()+&args[2]+".ipass");
    if std::path::Path::new(filepath).exists() {
        ip_lib::rename(&args[2],&args[3], ask_for_pw());
        println!("Renamed {} to {}", args[2], args[3]);
    } else {
        println!("No such file");
    }
}

fn remove(args: &Vec<String>) {
    //! arguments: program remove [name]
    if args.len() < 3 {
        println!("Invalid usage of \"remove\"");
        return;
    }
    let name = &args[2];
    let filepath = &(ip_lib::get_ipass_folder()+name+".ipass");
    if std::path::Path::new(filepath).exists() {
    if ip_lib::prompt_answer(format!("Are you sure you want to delete {}? [y/N] ", name)) == "y" {
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
    //! arguments: program import {location}
    let mut location = ip_lib::get_home_folder_str();
    if args.len() == 3 {
        location = args[2].clone();
    } else {
        if ip_lib::prompt_answer(format!("No location specified, defaulting to {} continue? [Y/n] ", location.clone())) == "n" {
            println!("Operation cancelled");
            return;
        }
    }
    if ip_lib::import_file(&location) {
        println!("Imported all entries!");
    } else {
        println!("No such file found!");
    }

}

fn export(args: &Vec<String>) {
    //! arguments: program export {location}
    let mut location = ip_lib::get_home_folder_str();
    if args.len() == 3 {
        location = args[2].clone();
    } else {
        if ip_lib::prompt_answer(format!("No location specified, defaulting to {} continue? [Y/n] ", location.clone())) == "n" {
            println!("Operation cancelled");
            return;
        }
    }
    if ip_lib::export_file(&location) {
        println!("Saved at: '{}/export.ipassx'", location);
    } else {
        println!("Failed saving at '{}/export.ipassx' does it exist?",location);
    }
}

fn clear() {
    if ip_lib::prompt_answer("Are you sure you want to clear everything? [y/N] ".to_string()) != "y" {
        println!("operation cancelled!");
        return;
    }

    let paths = std::fs::read_dir(ip_lib::get_ipass_folder()).unwrap();

    for path in paths {
        if let Ok(p) = path {
            std::fs::remove_file(ip_lib::get_ipass_folder()+"/"+p.file_name().into_string().unwrap().as_str()).unwrap();
        }
    }
    println!("Cleared all entries!");
}

fn sync(args: &Vec<String>) { 
    //! arguments: program sync [on/off] {location if on}
    let arg: String;
    if args.len() < 3 {
        println!("Invalid usage of \"sync\"");
        return;
    } else {
        arg = args[2].to_lowercase();
    }

    match arg.as_str() {
        "on" => {
            let location: String;

            if args.len() < 4 {
                location = ip_lib::prompt_answer_nolower("No location specified, please provide the location of the file you want to sync: ".to_string());
            } else {
                location = args[3].clone();
            }

            let mut sync_file = std::fs::File::create(ip_lib::get_home_folder_str()+"/.sync.ipass").expect("could not open sync file");
            sync_file.write(location.as_bytes()).expect("could not write to sync file");

            if !ip_lib::export_file(&location) {
                eprintln!("Test sync error, make sure you specified a valid folder!");
            } else {
                println!("Sync is now Enabled!");
                println!("Sync file: {}\\export.ipassx",location);
            }

            

            
        },
        "off" => {

            let sync_file_loc = ip_lib::get_home_folder_str()+"/.sync.ipass";
            let sync_file_path = std::path::Path::new(&sync_file_loc);

            let sync_enabled = sync_file_path.clone().exists();

            if sync_enabled {

                std::fs::remove_file(ip_lib::get_home_folder_str()+"/.sync.ipass").expect("could not disable sync, is it already disabled?");

                println!("Sync is now Disabled!");
            } else {
                println!("Sync is already disabled!");
            }
        },
        _ => {
            println!("Invalid argument, check \"help\" for help");
        }
    }
}

fn isync(args: &Vec<String>) { 
    //! arguments: program isync [on/off]
    if args.len() > 2 {
        println!("Invalid usage of \"isync\"");
        return;
    }
    let arg: String = args[2].clone().to_lowercase();

    match arg.as_str() {
        "on" => {
            todo!("ISync");

            // println!("ISync is now Enabled!");
        },
        "off" => {
            println!("ISync is now Disabled!");
        },
        _ => {
            println!("Invalid argument, check \"help\" for help");
        }
    }
}