use std::path::PathBuf;

use bpaf::Bpaf;
use make::Make;
use registry::Registry;
use script::ScriptOutput;

pub mod make;
pub mod registry;
pub mod script;
use make::make;

pub struct ScriptEnv {
    /// Path to where the environment's library and script cache are stored.
    /// Default value: $CARGO_HOME.
    pub path: PathBuf,
    /// Map from the name of a script, to its URL.
    pub registry: Registry,
}

impl Default for ScriptEnv {
    fn default() -> Self {
        unimplemented!()
    }
}

impl ScriptEnv {
    pub fn run<O: ScriptOutput>(&self, script: &str) -> anyhow::Result<O> {
        if let Some(_url) = self.registry.get(script) {}
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
