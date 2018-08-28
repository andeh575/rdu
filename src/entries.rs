pub struct Entries {
    entries: Vec<Entry>
}

impl Entries {
    pub fn new() -> Entries {
        Entries {
            entries: vec![]
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn compare_entries(&self) {

    }

    pub fn compare_sizes(&self) {

    }

    pub fn compare_subtrees(&self) {

    }

    pub fn show_entries(&self) {
        for entry in &self.entries {
            entry.show_entry();
        }
    }
}

pub struct Entry {
    size: u64,                  // Size of this entry
    path: String,               // Current entry path
    depth: u32,                 // Depth of this entry in the tree
    components: Vec<String>,    // Components of this entry
    children: Box<Vec<Entry>>   // Child entries for this entry
}

impl Entry {
    pub fn new(path: String, size: u64, depth: u32, components: Vec<String>) -> Entry {
        Entry {
            size: size,
            path: path,
            depth: depth,
            components: components,
            children: Box::new(vec![])
        }
    }

    pub fn show_entry(&self) {
        self.indent();
        println!("/{} {}", self.components[self.components.len() - 1 ], self.size);
    }

    fn indent(&self) {
        match self.depth {
        1 => print!("{:width$}", "", width = (0) as usize),
        _ => print!("{:width$}", "", width = (4 * (self.depth - 1)) as usize)
        }
    }
}