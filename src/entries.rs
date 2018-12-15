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

    /// Display in `pre-ordered` fashion, where children are displayed above their parents
    pub fn print_pre_ordered(&self) {
        for child in &self.children {
            child.print_pre_ordered()
        }
        self.show_entry();
    }

    /// Display in `post-ordered` fashion, where children are displayed below their parents
    pub fn print_post_ordered(&self) {
        self.show_entry();

        for child in &self.children {
            child.print_post_ordered()
        }
    }

    /// Print the `components` of this `Entry`
    pub fn show_entry(&self) {
        self.indent();
        println!(
            "{} {}",
            self.components[self.components.len() - 1],
            self.size
        );
    }

    /// Helper function to indent text
    fn indent(&self) {
        let depth = self.components.len();
        match depth {
            1 => print!("{:width$}", "", width = (0) as usize),
            _ => print!("{:width$}/", "", width = (4 * (depth - 1))),
        }
    }
}

#[cfg(test)]
mod tests {

    use entries::*;

    /// Helper function to generate some test `Entry`s
    fn generate_entries() -> (Entry, Entry) {
        let e1 = Entry::new("./some/small/path".to_string(), 10, vec![]);
        let e2 = Entry::new("./some/long/er/path".to_string(), 5, vec![]);

        (e1, e2)
    }

    #[test]
    fn greater_than_entry() {
        let (entry1, entry2) = generate_entries();

        assert!(entry1 > entry2)
    }

    #[test]
    #[should_panic]
    fn less_than_entry_panic() {
        let (entry1, entry2) = generate_entries();

        assert!(entry1 < entry2)
    }

    #[test]
    fn equal_entry() {
        let e1 = Entry::new("./some/small/path".to_string(), 1, vec![]);
        let e2 = Entry::new("./some/small/path".to_string(), 1, vec![]);

        assert!(e1 == e2)
    }
}
