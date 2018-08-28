mod entries;

extern crate clap;
use clap::{Arg, App};

use std::io::{self, Read};

fn read_du() -> io::Result<(String)> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer)?;
    
    Ok(buffer)
}

fn construct_entries(buffer: String, entries: &mut entries::Entries) {
    for line in buffer.lines() {
        let data:Vec<_> = line.split_whitespace().collect();
        let size = data[0]; // Acquire the size of the item
        let path = data[1]; // Acquire the path of the item
        let components:Vec<_> = path.split("/").collect();
        let depth = components.iter().count();


        println!("Item: {}; size: {}", path, size);
        println!("\tcomponents: {:?}; depth: {}", components, depth);
    }
}

fn build_tree_postorder(tree: &mut entries::Entries) {
    
}

fn build_tree_preorder(tree: &mut entries::Entries) {
    
}

fn main() {
    let mut raw_entries = entries::Entries::new();
    let mut built_tree = entries::Entries::new();

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
    let buffer = read_du().unwrap();
    construct_entries(buffer, &mut raw_entries);

    if matches.is_present("pre-order") {
        println!("Sorting entries...");

        println!("Building tree (pre-order)...");
        build_tree_preorder(&mut built_tree);
    } else {
        println!("Build tree (post-order)...");
        build_tree_postorder(&mut built_tree);
    }

    println!("Rendering tree...");
    built_tree.show_entries();
}

