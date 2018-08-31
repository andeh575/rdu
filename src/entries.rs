/// Struct containing a vector of `Entry`
#[derive(PartialEq, PartialOrd,Debug)]
pub struct Entries {
    entries: Vec<Entry>
}

impl Entries {
    /// Create a new `Entries` struct
    pub fn new() -> Entries {
        Entries {
            entries: vec![]
        }
    }

    /// Add an `Entry` to the struct's `entries` vector
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Compares two `Entry` elements in `self.entries`
    pub fn compare_entries(&self) {

    }

    /// Compares the `size` of two `Entry elements in `self.entries`
    pub fn compare_sizes(&self) {

    }

    /// Compares the `children` of two `Entry` elements in `self.entries`
    pub fn compare_subtrees(&self) {

    }

    /// Prints the contents of `entries`
    pub fn show_entries(&self) {
        for entry in &self.entries {
            entry.show_entry();
        }
    }
}

/// An individual `Entry`
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Entry {
    size: u64,                  // Size of this entry
    path: String,               // Current entry path
    depth: u32,                 // Depth of this entry in the tree
    components: Vec<String>,    // Components of this entry
    children: Box<Vec<Entry>>   // Child entries for this entry
}

impl Entry {
    /// Create a new `Entry` struct
    pub fn new(path: String, size: u64) -> Entry {
        
        let components: Vec<String> = path.split("/").map(String::from).collect();
        let depth: u32 = components.iter().count() as u32;
        
        Entry {
            size: size,
            path: path,
            components: components,
            depth: depth,
            children: Box::new(vec![])
        }
    }

    /// Print the `components` of this `Entry`
    pub fn show_entry(&self) {
        self.indent();
        println!("{} {}", self.components[self.components.len() - 1 ], self.size);
    }

    /// Helper function to indent text
    fn indent(&self) {
        match self.depth {
            1 => print!("{:width$}", "", width = (0) as usize),
            _ => print!("{:width$}/", "", width = (4 * (self.depth - 1)) as usize)
        }
    }
}

#[cfg(test)]
mod tests {

    use entries::*;    

    /// Helper function to generate some test `Entry`'s
    fn generate_entry() -> (Entry, Entry) {
        let e1 = Entry::new("./some/small/path".to_string(), 10);
        let e2 = Entry::new("./some/long/er/path".to_string(), 5);

        (e1, e2)
    }

    /// Helper function to generate some test `Entries`'s
    fn generate_entries() -> (Entries, Entries) {
        let entries1 = Entries::new();
        let entries2 = Entries::new();

        (entries1, entries2)
    }

    #[test]
    fn create_new_entry() {
        let (e1, e2) = generate_entries();
    }

    #[test]
    fn create_new_entries() {
        let (entries1, entries2) = generate_entries();
    }

    #[test]
    fn add_entries() {
        let (mut entries1, _) = generate_entries();
        let (entry1, entry2) = generate_entry();

        entries1.add_entry(entry1);
        entries1.add_entry(entry2);

        assert_eq!(entries1.entries.len(), 2)
    }

    #[test]
    fn greater_than_entry() {
        let (entry1, entry2) = generate_entry();

        assert!(entry1 > entry2)
    }

    #[test]
    #[should_panic]
    fn less_than_entry_panic() {
        let (entry1, entry2) = generate_entry();

        assert!(entry1 < entry2)
    }

    #[test]
    fn equal_entry() {
        let e1 = Entry::new("./some/small/path".to_string(), 1);
        let e2 = Entry::new("./some/small/path".to_string(), 1);

        assert!(e1 == e2)
    }

    #[test]
    fn greater_than_entries() {
        let (mut entries1, entries2) = generate_entries();
        let (entry1, entry2) = generate_entry();

        entries1.add_entry(entry1);
        entries1.add_entry(entry2);

        assert!(entries1 > entries2)
    }

    #[test]
    #[should_panic]
    fn less_than_entries_panic() {
        let (mut entries1, entries2) = generate_entries();
        let (entry1, entry2) = generate_entry();

        entries1.add_entry(entry1);
        entries1.add_entry(entry2);

        assert!(entries1 < entries2)
    }

    #[test]
    fn equal_entries() {
        let (entries1, entries2) = generate_entries();

        assert!(entries1 == entries2)
    }
}