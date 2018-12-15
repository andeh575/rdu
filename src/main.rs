mod entries;

extern crate clap;

use clap::{App, Arg};
use entries::Entry;
use std::io::{self, BufRead};


/// Generic status printing function
fn status(step: &mut u8, msg: &str) {
    *step += 1;
    println!("({}) {}", step, msg);
}

/// Constructs a list of `Entry`s by parsing `stdin`
fn construct_entries() -> Option<Entry> {
    let mut stack: Vec<Entry> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let data: Vec<_> = line.split_whitespace().collect();
        let size = data[0].to_string().parse().unwrap();
        let path = data[1].trim_end_matches("/").to_string();
        let component_count = path.split('/').count();
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
        .arg(
            Arg::with_name("pre-order")
                .short("p")
                .long("pre-order")
                .help("Enable pre-order sorting"),
        ).arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose output"),
        ).arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug output"),
        ).get_matches();

    // Parse `du` data from `stdin`
    status(&mut step, "Parsing du file...");
    let tree = construct_entries().unwrap();

    // jsleeper:11-18-2018: I don't _think_ this is useful anymore?
    if matches.is_present("debug") {
        status(&mut step, "Received the following from `du`:");
        if matches.is_present("pre-order") {
            tree.print_pre_ordered();
        } else {
            tree.print_post_ordered();
        }
    }

    if matches.is_present("pre-order") {
        status(&mut step, "Building tree (pre-order)...");
        tree.print_pre_ordered();
    } else {
        status(&mut step, "Building tree (post-order)...");
        tree.print_post_ordered();
    }
}
