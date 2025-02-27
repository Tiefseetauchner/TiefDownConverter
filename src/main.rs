use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tiefdownconverter")]
#[command(about = "A CLI tool for managing TiefDown Projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        about = "Convert a TiefDown project. By default, it will convert the current directory."
    )]
    Convert {
        #[arg(
            short,
            long,
            help = "The project to convert. If not provided, the current directory will be used."
        )]
        project: Option<String>,
        #[arg(
            short,
            long,
            help = "The templates to use. If not provided, the default templates from the manifest file will be used.",
            use_value_delimiter = true,
            value_delimiter = ',',
            num_args = 1..,
        )]
        templates: Option<Vec<String>>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Convert { project, templates } => {
            convert(project, templates);
        }
    }
}

fn convert(project: Option<String>, templates: Option<Vec<String>>) -> () {
    let project = project.unwrap_or(".".to_string());

    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    if !manifest_path.exists() {
        panic!("No manifest file found. Please initialize a project first.");
    }

    let manifest = std::fs::read_to_string(manifest_path).unwrap();
    let manifest: toml::Value = toml::from_str(&manifest).unwrap();

    let mut templates = templates.unwrap_or(vec![]);
    if templates.is_empty() {
        templates = manifest["templates"]
            .as_table()
            .unwrap()
            .keys()
            .map(|s| s.to_string())
            .collect();
    }

    println!("Converting project: {}", project);
    println!("Using templates: {}", templates.join(", "));
}
