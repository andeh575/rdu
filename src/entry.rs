struct Entry {
    size:           u64,                
    n_components:   u32,                // Component count for this entry
    path:           String,             // Current entry path
    components:     Vec<String>,        // Components of this entry
    depth:          u32,                // Current depth of this entry in the tree
    max_depth:      u32,                // Depth of the tree at this entry
    n_children:     u32,                // Child count for this entry
    children:       Option<Box<Entry>>  // Child entries for this entry
}


