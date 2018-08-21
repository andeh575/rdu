extern crate clap;
use clap::{Arg, App, SubCommand};

mod entry;

const INDENT: i32 = 2;
const DU_INIT_ENTRIES: i32 = (128 * 1024);

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

    if matches.is_present("pre-order") {
        println!("Sorting entries...");

        println!("Building tree (pre-order)...");
    } else {
        println!("Build tree (post-order)...");
    }
}

