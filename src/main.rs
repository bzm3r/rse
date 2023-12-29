use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use bpaf::Bpaf;
use make::Make;
use url::Url;

pub mod make;
pub mod script;
use make::make;


/// This should use: https://doc.rust-lang.org/cargo/reference/registries.html
/// Should use `cargo`'s existing features as much as possible, to minimize
/// effort required to build NixOS support *into* `cargo script`.
///
/// 0) https://doc.rust-lang.org/cargo/reference/registries.html#registries
/// 1) https://doc.rust-lang.org/cargo/reference/running-a-registry.html
///     * in particular, see: https://github.com/rust-lang/cargo/wiki/Third-party-registries
pub struct Registry {
    pub table: HashMap<String, ScriptUrl>,
}

impl Registry {
    fn get(&self, _name: &str) -> Option<bool> {
        unimplemented!()
    }
}

pub struct ScriptName<'a>(Cow<'a, str>);

pub enum Status {
    /// The registry knows where to fetch this script from, but it has not been
    /// instantiated on disk.
    Declared,
    /// The registry has previously instantiated this script on disk.
    Instantiated,
    /// The script is not known to the registry.
    Unknown,
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

pub struct ScriptEnv {
    /// Path to where the environments dynamic library and script cache are stored.
    /// Default value: $CARGO_HOME.
    pub path: PathBuf,
    /// Map from the name of a script, to its URL.
    registry: Registry,
}

impl Default for ScriptEnv {
    fn default() -> Self {
        unimplemented!()
    }
}

impl ScriptEnv {
    pub fn run<O: ScriptOutput>(&self, script: &Script) -> anyhow::Result<O> {
        if let Some(_url) = self.registry.get(script.name()) {}
        unimplemented!()
    }
}

impl Script {
    fn name(&self) -> &str {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum Interface {
    /// Make a script project.
    #[bpaf(command)]
    Make(#[bpaf(external(make))] Make),
}

fn main() -> anyhow::Result<()> {
    let opts = interface().run();
    println!("{opts:?}");

    Ok(())
}
