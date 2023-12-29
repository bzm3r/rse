use std::collections::HashMap;

use url::Url;

pub struct Registry {
    pub table: HashMap<String, Url>,
}

impl Registry {
    pub fn get(&self, _name: &str) -> Option<bool> {
        unimplemented!()
    }
}

impl Registry {
    /// Get the [`Status`] of a script in the registry.
    pub fn status(&self, name: &str) -> Status {
        unimplemented!()
    }

    /// Check if the script was previously instantiated in memory.
    pub fn is_instantiated(&self, name: &str) -> bool {
        unimplemented!()
    }

    /// Check if the registry knows about the script.
    pub fn is_known(&self, name: &str) -> bool {
        unimplemented!()
    }
}

pub enum Status {
    /// The registry knows where to fetch this script from, but it has not been
    /// instantiated on disk.
    Declared,
    /// The registry has previously instantiated this script on disk.
    Instantiated,
    /// The script is not known to the registry.
    Unknown,
}
