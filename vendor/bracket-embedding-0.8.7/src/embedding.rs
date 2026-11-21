use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::collections::HashMap;

lazy_static! {
    pub static ref EMBED: Mutex<Dictionary> = Mutex::new(Dictionary::new());
}

/// Stores a dictionary of resources, generally added via `embedded_resource!` and `link_resource!` macros.
#[derive(Default)]
pub struct Dictionary {
    entries: HashMap<String, &'static [u8]>,
}

impl Dictionary {
    /// Create a new, empty dictionary.
    #[must_use]
    pub fn new() -> Dictionary {
        Self::default()
    }

    /// Request a resource, returning either a byte array or `None`.
    #[must_use]
    pub fn get_resource(&self, path: String) -> Option<&'static [u8]> {
        let fixed_path = if std::path::MAIN_SEPARATOR == '/' {
            path 
        } else {
            path.replace(std::path::MAIN_SEPARATOR, "/")
        };

        if self.entries.contains_key(&fixed_path) {
            return Some(self.entries[&fixed_path]);
        }
        None
    }

    /// Insert a resource into the dictionary.
    pub fn add_resource(&mut self, path: String, bytes: &'static [u8]) {
        self.entries.insert(path, bytes);
    }
}
