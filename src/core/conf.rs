use std::{env, process};
use clap::{Command, Arg, ArgAction};
use colored::Colorize;

#[derive(Debug)]
pub struct Conf {
    pub filename: String,
    pub dir_path: String,
    pub follow_links: bool,
    pub skip_hidden: bool,
}

fn get_default_root() -> Option<String> {
    match env::consts::OS {
        "macos" | "linux" => Some(String::from("/")),
        "windows" => Some(String::from("C:\\")),
        _ => None,
    }
}

impl Conf {
    pub fn build() -> Self {

        let matches = Command::new("spot")
        .version("1.0")
        .author("Shiva <siv6146@gmail.com>")
        .about("Spot file(s) or folder(s) in your filesystem by its name")
        .arg(Arg::new("file_name").help("Name or regex pattern of file(s) or folder(s)").required(true))
        .arg(Arg::new("dir_path").long("dir_path").help("Path to dir from where search should begin"))
        .arg(Arg::new("follow_links").long("follow_links").help("Whether to follow symbolic links or NOT in the search").action(ArgAction::SetTrue))
        .arg(Arg::new("skip_hidden").long("skip_hidden").help("Whether to include hidden files or NOT in the search").action(ArgAction::SetTrue))
        .get_matches();

        Conf { 
            filename: matches.get_one::<String>("file_name").expect("`file_name` is required").to_string(), 
            dir_path: match matches.get_one::<String>("dir_path") {
                Some(path) => path.to_string(),
                None => {
                    let path = get_default_root();
                    match path {
                        Some(path) => {
                            println!("{} {}", "No path specified! So, starting search from ".yellow(), path.yellow());
                            path
                        },
                        None => {
                            eprintln!("{}", "Unable to determine root of your filesystem!".red());
                            process::exit(1);
                        }
                    }
                }
            },
            follow_links: matches.get_flag("follow_links"),
            skip_hidden: matches.get_flag("skip_hidden"),
        }
    }
}