use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::path::Path;
use std::process::Command;

mod config;
use config::Config;

fn main() {
    let config = Config::new();

    /* Check that config.toml's length is not more than 10, so that I can reset it */
    let file_path = "my_file.txt";
    let metadata_result = fs::metadata(file_path);
    let is_file_empty = match metadata_result {
        Ok(metadata) => metadata.len() == 0,
        Err(_) => false,
    };

    if is_file_empty {
        config.modify("0.0.1", format!("{}/scripts", env::current_dir().unwrap().display().to_string()).as_str()).unwrap();
    }

    let logo = format!(".----------------.\n| uScript {}  |\n'----------------'", config.version).blue().bold();
    let args: Vec<String> = env::args().skip(1).collect();

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) || args.contains(&"help".to_string()) || args.len() <= 0 {
        println!("{}", logo);
        println!("{}", "Usage: uscript <OPTIONS> <NAME>".blue().bold());
        println!("{}", "OPTIONS:".green().bold());
        println!("{}", "\t-h(--help)\t\tShow this help message.");
        println!("{}", "\t-v(--version)\t\tShow version.");
        println!("{}", "\t-r(--run) <NAME>\t\tRuns a script\n\t\tNAME: Name of script. You can check all script by writing --list or -l.");
        println!("{}", "\t-l(--list)\t\tList all scripts.");
        println!("{}", "\t-p(--path) <PATH>\t\tSet path to scripts directory.");

        return;
    }
    if args.contains(&"--version".to_string()) || args.contains(&"-v".to_string()) {
        println!("{}", logo);
        println!("{}", "Version: 0.0.1".green().bold());
        return;
    }

    if args.contains(&"--path".to_string()) || args.contains(&"-p".to_string()) {
        let mut path = String::new();
        let mut args = env::args();

        while let Some(arg) = args.next() {
            if arg == "--path" {
                path = args.next().unwrap_or(String::new());
            }
        }
        config.modify(config.version.as_str(), &path).unwrap();
    }

    else {
        let mut dir_path = PathBuf::new();
        dir_path.push(config.path.clone());

        if !dir_path.exists() {
            eprintln!("Directory not found: {:?}", dir_path);
            std::process::exit(1);
        }
        let paths: Vec<PathBuf> = fs::read_dir(dir_path)
            .unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path.is_file() && path.extension().unwrap_or_default() == "sh" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        if args.contains(&"--list".to_string()) || args.contains(&"-l".to_string()) || args.contains(&"-L".to_string()) {
            list_scripts(&paths);
            return;
        }
        if args.contains(&"--run".to_string()) || args.contains(&"-r".to_string()) {
            let mut name = String::new();
            let mut args = env::args();

            while let Some(arg) = args.next() {
                if arg == "--run" {
                    name = args.next().unwrap_or(String::new());
                }
            }
            let contains_file = paths.iter().any(|p| p.file_name().unwrap() == format!("{}.sh", name).as_str());
            if contains_file == false {
                print!("{}", "Script not found: ".red().bold().to_string());
                print!("{}\n", name.bold());
                println!("You can use --list to list available scripts");
                return;
            }

            println!("Runnig script: {}", name);


            Command::new("sh")
                .arg(format!("{}/{}.sh", config.path, name))
                .spawn()
                .expect("sh command failed to start");
        }
    }
}

fn read_description(script: &str) -> Result<String, std::io::Error> {
    let path = Path::new(script);
    fs::read_to_string(path)
}

fn list_scripts(paths: &Vec<PathBuf>) {
    let logo = ".-----------------.\n| List of Scripts |\n'-----------------'".blue().bold();
    println!("{}\n", logo);

    for path in paths {
        let script_name = path.file_stem().unwrap().to_str().unwrap();
        let script_description = read_description(format!("{}.desc", path.to_string_lossy()).as_str()).unwrap_or_else(|error| {
            format!("No description found")
        });
        println!("{} - {}", script_name.green().bold(), script_description);
    }
}
