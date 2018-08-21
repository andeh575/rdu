mod entries;

extern crate clap;
use clap::{Arg, App};

use std::io::{self, Read};

const INDENT: i32 = 2;
const DU_INIT_ENTRIES: i32 = (128 * 1024);

fn build_tree_postorder() {

}

fn build_tree_preorder() {

}

fn parse_du() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let matches = App::new("duvis")
                    .version("0.1.0")
                    .author("Andrew Graham <andrew.t.graham@live.com>")
                    .about("A fast(er) xdu replacement")
                    .arg(Arg::with_name("pre-order")
                        .short("p")
                        .long("pre-order")
                        .help("Enable pre-order sorting"))
                    .arg(Arg::with_name("verbose")
                        .short("v")
                        .long("verbose")
                        .help("Enable verbose output"))
                    .get_matches();

    println!("Parsing du file...");
    let buffer = parse_du().unwrap();

    println!("{}", buffer);

    if matches.is_present("pre-order") {
        println!("Sorting entries...");

        println!("Building tree (pre-order)...");
    } else {
        println!("Build tree (post-order)...");
    }
}

