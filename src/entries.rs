struct Entries {
    entries: Vec<Entry>
}

impl Entries {
    fn new() -> Entries {
        Entries {
            entries: vec![]
        }
    }

    fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    fn compare_entries() {

    }

    fn compare_sizes() {

    }

    fn compare_subtrees() {

    }

    fn show_entries() {

    }
}

struct Entry {
    size: u64,                  // Size of this entry
    path: String,               // Current entry path
    depth: u32,                 // Depth of this entry in the tree
    components: Vec<String>,    // Components of this entry
    children: Box<Vec<Entry>>   // Child entries for this entry
}

impl Entry {
    fn new(path: String) -> Entry {
        Entry {
            size: 0,
            path: path,
            depth: 0,
            components: vec![],
            children: Box::new(vec![])
        }
    }

    fn show_entry() {

    }
}