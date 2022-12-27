//! # Spot
//!
//! `spot` is an efficient command-line tool to search for file(s) / folder(s) in your file system
mod core;

use crate::core::{Conf, Spot};
fn main() {
    let cfg = Conf::build();
    
    println!("{}", Spot::query(cfg));
}