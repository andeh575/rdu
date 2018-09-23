mod entries;

extern crate clap;

use clap::{Arg, App};
use entries::Entry;
use std::io::{self, Read};

/// Reads in data from `stdin`, assumes it's from `du` and inserts it into a buffer
fn read_du() -> io::Result<(String)> {
    // TODO: Needs to be updated to read from stdin as information becomes available
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;

    Ok(buffer)
}

/// Generic status printing function
fn status(step: &mut u8, msg: &str) {
    *step += 1;
    println!("({}) {}", step, msg);
}

/// Constructs a raw `vector` of `entries` by parsing an input buffer
fn construct_entries(buffer: String) -> Option<Entry> {
    let mut stack: Vec<Entry> = vec![];
    for line in buffer.lines() {
        let data: Vec<_> = line.split_whitespace().collect();
        let size: u64 = data[0].to_string().parse().unwrap();
        let path = data[1].to_string();
        let component_count = path.split("/").count();
        let mut children: Vec<Entry> = vec![];

        // Found a parent?
        while !stack.is_empty() && component_count < stack.last().unwrap().components.len() {
            children.insert(0, stack.pop().unwrap());
        }
        stack.push(Entry::new(path, size, children));
    }
    stack.pop()
}

fn main() {
    let mut step: u8 = 0;

    // Construct parser and parse command line arguments
    let matches = App::new("rdu")
                    .version("1.0.0")
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
                    .arg(Arg::with_name("debug")
                        .short("d")
                        .long("debug")
                        .help("Enable debug output"))
                    .get_matches();

    // Parse stdin into an input buffer
    status(&mut step, "Parsing du file...");
    let buffer = read_du().unwrap();
    let tree = construct_entries(buffer).unwrap();

    if matches.is_present("debug") {
        status(&mut step, "Received the following from `du`:");
        tree.print_post_ordered();
    }

    if matches.is_present("pre-order") {
        // output from `du` is already in post-order format; sort into pre-order format
        status(&mut step, "Sorting entries...");
        // TODO: actually sort 'em?

        status(&mut step, "Building tree (pre-order)...");
        tree.print_pre_ordered();
    } else {
        status(&mut step, "Building tree (post-order)...");
        tree.print_post_ordered();
    }
}

