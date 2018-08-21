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

    pub fn compare_entries() {

    }

    pub fn compare_sizes() {

    }

    pub fn compare_subtrees() {

    }

    pub fn show_entries() {

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
    pub fn new(path: String) -> Entry {
        Entry {
            size: 0,
            path: path,
            depth: 0,
            components: vec![],
            children: Box::new(vec![])
        }
    }

    pub fn show_entry() {

    }
}