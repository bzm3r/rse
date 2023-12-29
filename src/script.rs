use url::Url;

#[doc(hidden)]
macro_rules! doc_warn_impermanence {
    () => {
        "\n**Warning:** this does not add the script to `/nix/store/rse/declared_scripts` and you are at risk of losing this information upon a system rebuilds! To declaratively add a script, you must list the scripts's URL in the `declaredScripts` option for the scripting environment's NixOS module."
    }
}

// `cargo-script` will cache local instances of a script, in order to allow for
// incremental rebuilding and other build related optimizations: https://github.com/rust-lang/cargo/issues/12207#issuecomment-1776089794

pub enum Script {
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

impl Script {
    fn name(&self) -> &str {
        unimplemented!()
    }
}

pub trait ScriptOutput {}
