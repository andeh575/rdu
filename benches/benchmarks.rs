#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::io::{self, BufRead, Read};
    use std::fs::File;
    use test::{Bencher, black_box};

    /// Representation of an individual line item from `du` output
    #[derive(PartialEq, PartialOrd, Debug)]
    pub struct Entry {
        size: u64,                   // Size of this entry
        path: String,                // Current entry path
        pub components: Vec<String>, // Components of this entry
        children: Vec<Entry>,        // Child entries for this entry
    }

    impl Entry {
        /// Create a new `Entry` struct
        pub fn new(path: String, size: u64, children: Vec<Entry>) -> Entry {
            let components: Vec<String> = path.split('/').map(String::from).collect();

            Entry {
                size,
                path,
                components,
                children,
            }
        }
    }

    // https://doc.rust-lang.org/1.16.0/book/benchmark-tests.html
    // https://doc.rust-lang.org/stable/book/2018-edition/ch11-03-test-organization.html
    // https://doc.rust-lang.org/std/fs/struct.File.html

    #[bench]
    fn bench_process_from_intermediary_buffer(b: &mut Bencher) {
        b.iter(|| {
            // Inner closure, the actual test
            let mut file = File::open("test-du-output.txt").unwrap();
            let mut stack: Vec<Entry> = vec![];
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            for line in buffer.lines() {
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
        });
    }

    #[bench]
    fn bench_process_directly_by_line(b: &mut Bencher) {
        b.iter(|| {
            // Inner closure, the actual test
            let file = File::open("test-du-output.txt").unwrap();
            let mut stack: Vec<Entry> = vec![];
            for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
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
        });
    }
}
