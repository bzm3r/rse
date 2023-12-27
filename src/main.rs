use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use bpaf::{Bpaf, Parser};
use url::Url;
use xshell::{cmd, Shell};

// bpaf docs: https://docs.rs/bpaf/latest/bpaf/index.html
// xshell docs: https://docs.rs/xshell/latest/xshell/index.html

#[doc(hidden)]
macro_rules! doc_warn_impermanence {
    () => {
        "\n**Warning:** this does not add the script to `/nix/store/rse/declared_scripts` and you are at risk of losing this information upon a system rebuilds! To declaratively add a script, you must list the scripts's URL in the `declaredScripts` option for the scripting environment's NixOS module."
    }
}

enum Script {
    /// Name of a script that is assumed to already exist either in:
    ///     * already built and cached ("instantiated") as a directory in the
    ///     scripting environment's cache.
    ///     * associated with an uninstantiated URL in the `known_scripts` file,
    ///     stored in the `/nix/store` location for this scripting environment.
    Name(String),
    /// URL of a script in the form of a traditional `cargo` repository.
    #[doc = doc_warn_impermanence!()]
    Project(Url),
}

pub trait ScriptOutput {}

pub struct ScriptUrl {
    /// Note how `cargo-script` plans to store local instances of a cached
    /// script: https://github.com/rust-lang/cargo/issues/12207#issuecomment-1776089794
    instance: Option<PathBuf>,
    /// A [`Url`] to the source code of the script.
    source: Url,
}

/// This should use: https://doc.rust-lang.org/cargo/reference/registries.html
/// Should use `cargo`'s existing features as much as possible, to minimize
/// effort required to build NixOS support *into* `cargo script`.
///
/// 0) https://doc.rust-lang.org/cargo/reference/registries.html#registries
/// 1) https://doc.rust-lang.org/cargo/reference/running-a-registry.html
///     * in particular, see: https://github.com/rust-lang/cargo/wiki/Third-party-registries
pub struct Registry {
    table: HashMap<String, ScriptUrl>,
}

impl Registry {
    fn get(&self, name: &str) -> Option<bool> {
        unimplemented!()
    }
}

pub struct ScriptName<'a>(Cow<'a, str>);

pub enum Status {
    Declared { instantiated: bool },
    Instantiated { known: bool },
}

impl Registry {
    ///
    pub fn status<'a, HasName: Into<ScriptName<'a>>>(
        &self,
        name: HasName,
    ) -> Status {
        unimplemented!()
    }

    pub fn instantiated<'a, HasName: Into<ScriptName<'a>>>(
        &self,
        name: HasName,
    ) -> bool {
        unimplemented!()
    }
}

pub struct ScriptEnv {
    /// Path to where the environments dynamic library and script cache are stored.
    /// Default value: $CARGO_HOME.
    path: PathBuf,
    /// Map from the name of a script, to its URL.
    registry: Registry,
}

impl Default for ScriptEnv {
    fn default() -> Self {
        unimplemented!()
    }
}

impl ScriptEnv {
    pub fn is_instantiated(&self, script: &Script) {}
    pub fn run<O: ScriptOutput>(&self, script: &Script) -> anyhow::Result<O> {
        if let Some(url) = self.registry.get(script.name()) {}
        unimplemented!()
    }
}

impl Script {
    fn name(&self) -> &str {
        unimplemented!()
    }
}

/// CLI for the Rust Script Environment.
#[derive(Bpaf, Debug, Clone)]
struct Cli {
    /// Example of an optional flag.
    #[bpaf(short, long)]
    opt: bool,

    /// Example of an optional argument.
    #[bpaf(argument("OPTIONAL_ARG"), short, long)]
    arg: Option<usize>,

    /// Example of a positional argument.
    #[bpaf(positional("POSITIONAL"))]
    pos: String,
}

fn main() -> anyhow::Result<()> {
    let opts = cli().run();
    let greeting = if opts.opt { "goodbye" } else { "hello" };
    let thing = opts.pos.repeat(opts.arg.unwrap_or(1));
    let message = format!("{greeting} {thing}!");
    let sh = Shell::new()?;
    cmd!(sh, "echo \"{message}\"").run()?;

    Ok(())
}
