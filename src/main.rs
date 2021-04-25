use std::env;

use crate::cert_info::Certificate;
use crate::extract_info::extract_info;
use crate::write_info::{write, write_parts_only};

mod extract_info;
mod write_info;
mod cert_info;
mod versions;
mod title;
mod biblio;
mod revision;
mod tools;
mod table_of_contents;
mod config;

fn print_help() {
    println!("Usage: cargo run -- [OPTION | FILE] ...\n");
    println!("OPTION:\n\t--title\tExtracts title and pretty prints");
    println!("\t--content\tExtracts table of contents and pretty prints");
    println!("\t--biblio\tExtracts bibliography and pretty prints");
    println!("\t--versions\tExtracts versions and pretty prints");
    println!("\t--revisions\tExtracts revisions and pretty prints");
    println!("\nFILE:\tName of the file to be parsed. If no OPTION is given, the output is saved in FILE.json file. \n");
}

fn process_argument(arg: &str, conf: &mut config::Config) -> bool {
    match arg.chars().nth(0).unwrap() {
        '-' => {
            match arg {
                "--title" => {
                    conf.pretty = true;
                    conf.pretty_title = true;
                }
                "--content" => {
                    conf.pretty = true;
                    conf.pretty_content = true;
                }
                "--biblio" => {
                    conf.pretty = true;
                    conf.pretty_biblio = true;
                }
                "--versions" => {
                    conf.pretty = true;
                    conf.pretty_versions = true;
                }
                "--revisions" => {
                    conf.pretty = true;
                    conf.pretty_revisions = true;
                }
                &_ => {
                    print_help();
                    return false;
                }
            }
        }
        _ => { conf.input_files.push(arg.to_string()); }
    }
    true
}

fn extract_and_write_info(conf: &config::Config) {
    for filename in conf.input_files.iter() {
        let certificate = extract_info(&filename, &conf);
        if conf.pretty {
            write_parts_only(&certificate, filename, &conf);
        } else {
            let mut output_filename = filename.to_string();
            output_filename.push_str(".json");
            write(&certificate, &output_filename)
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = config::Config::new(&args);
    let mut first_arg = true;
    for arg in args {
        if first_arg {
            first_arg = false;
        } else {
            if !process_argument(&arg, &mut config) {
                return;
            }
        }
    }
    extract_and_write_info(&config);
}



