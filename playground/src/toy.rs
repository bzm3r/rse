use cache::{
    anyhow,
    xshell::{cmd, Shell},
};

fn main() -> anyhow::Result<()> {
    let sh = Shell::new()?;
    cmd!(sh, "echo \"I am the toy bin!\"").run()?;
    cmd!(sh, "echo \"Wooooooo!\"").run()?;
    cmd!(sh, "rm -rf /home/bzm3r/rse/mini_rse/toy/my_inc").run()?;
    cmd!(sh, "rm -rf /home/bzm3r/rse/mini_rse/toy/my_out").run()?;
    cmd!(sh, "rm -rf /home/bzm3r/rse/mini_rse/toy/asfdasdfasdf").run()?;
    cmd!(sh, "rm -rf /home/bzm3r/rse/mini_rse/toy/asfsdsfzxg4r3w").run()?;
    Ok(())
}
