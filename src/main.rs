mod parser;
mod type_checker;

use clap::{App, Arg};
use std::{fs::File, io::Read};

use crate::{parser::parse, type_checker::type_check};

fn main() {
    let matches = App::new("rust_python_type_checker")
        .version("0.1")
        .author("Zomatree")
        .about("Static python type checker in rust")
        .arg(Arg::with_name("file").help("The file to run the type checker on"))
        .get_matches();

    let file_name = matches.value_of("file").expect("No file given");

    let mut file_content = String::new();

    File::open(file_name)
        .expect("No file found")
        .read_to_string(&mut file_content)
        .expect("Error reading file");

    println!("{}", file_content);
    let module = parse(file_content).expect("Invalid syntax in python code");
    println!("{:?}", module);
    type_check(module).unwrap();
}
