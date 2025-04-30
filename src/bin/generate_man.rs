use clap::{Command, CommandFactory};
use clap_mangen::Man;
use color_eyre::eyre::Result;
use fs_extra::dir;
use std::fs;
use tiefdownconverter::cli::Cli;

fn main() -> Result<()> {
    dir::create_all("docs/man", false)?;
    let root = Cli::command();
    render_recursive(&root, "docs/man", vec![root.get_name().to_string()])?;
    Ok(())
}

fn render_recursive(cmd: &Command, out_dir: &str, name_parts: Vec<String>) -> Result<()> {
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;

    let filename = format!("{}.1", name_parts.join("-"));
    let filepath = std::path::Path::new(out_dir).join(filename);
    fs::write(filepath, buffer)?;
    println!("Generated man page for: {}", name_parts.join(" "));

    for sub in cmd.get_subcommands() {
        let mut parts = name_parts.clone();
        parts.push(sub.get_name().to_string());
        render_recursive(sub, out_dir, parts)?;
    }

    Ok(())
}
