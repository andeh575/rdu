mod entries;

extern crate clap;
use clap::{Arg, App};

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
fn construct_entries(buffer: String, entries: &mut entries::Entries) {
    for line in buffer.lines() {
        let data:Vec<_> = line.split_whitespace().collect();
        let size: u64 = data[0].to_string().parse().unwrap();
        let path = data[1].to_string();

        entries.add_entry(entries::Entry::new(path, size))
    }
}

/// Construct an entry tree in postorder format
fn build_tree_postorder(tree: &mut entries::Entries, raw: entries::Entries) {
    
}

/// Construct an entry tree in preorder format
fn build_tree_preorder(tree: &mut entries::Entries, raw: entries::Entries) {
    
}

fn main() {
    let mut raw_entries = entries::Entries::new();
    let mut built_tree = entries::Entries::new();
    let mut step: u8 = 0;

    // Construct parser and parse command line arguments
    let matches = App::new("rdu")
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
                    .arg(Arg::with_name("debug")
                        .short("d")
                        .long("debug")
                        .help("Enable debug output"))
                    .get_matches();

    // Parse stdin into an input buffer
    status(&mut step, "Parsing du file...");
    let buffer = read_du().unwrap();
    construct_entries(buffer, &mut raw_entries);

    if matches.is_present("debug") {
        status(&mut step, "Recieved the following from `du`:");
        raw_entries.show_entries();
    }

    if matches.is_present("pre-order") {
        // output from `du` is already in post-order format; sort into pre-order format
        status(&mut step, "Sorting entries...");

        status(&mut step, "Building tree (pre-order)...");
        build_tree_preorder(&mut built_tree, raw_entries);
    } else {
        status(&mut step, "Build tree (post-order)...");
        build_tree_postorder(&mut built_tree, raw_entries);
    }

    status(&mut step, "Rendering tree...");
    built_tree.show_entries();
}

