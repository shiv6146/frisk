use crate::core::Conf;
use std::{time::Instant, fmt, process};
use colored::Colorize;
use jwalk::{WalkDir};
use regex::Regex;

pub struct Spot {
    pub total_searched: u64,
    pub total_spotted: u64,
    pub time_elapsed: u64,
}

impl Spot {
    pub fn query(conf: Conf) -> Self {
        let dir_walker = WalkDir::new(conf.dir_path).skip_hidden(conf.skip_hidden).follow_links(conf.follow_links);

        let re = Regex::new(conf.filename.as_str());

        let expr = match re {
            Ok(expr) => expr,
            Err(_) => {
                eprintln!("{}", "Invalid regex!".red());
                process::exit(1);
            }
        };

        let mut tot_searched = 0;
        let mut tot_spotted = 0;

        let start_time = Instant::now();

        for item in dir_walker {
            if let Ok(entry) = item {
                tot_searched += 1;
                if expr.is_match(entry.file_name().to_str().unwrap()) {
                    tot_spotted += 1;
                    println!("{}", format!("{}", entry.path().display()).green());
                }
            }
        }

        Self {
            total_searched: tot_searched,
            total_spotted: tot_spotted,
            time_elapsed: start_time.elapsed().as_secs(),
        }
    }
}

impl fmt::Display for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "\n{} items have been searched in {} seconds to spot {} item(s)",
            self.total_searched.to_string().yellow(),
            self.time_elapsed.to_string().green(),
            self.total_spotted.to_string().green(),
        )
    }
}