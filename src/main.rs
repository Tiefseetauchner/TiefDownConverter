use clap::{Parser, Subcommand};

mod conversion;
mod manifest_model;
mod project_management;

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
    #[command(about = "Initialize a new TiefDown project.")]
    Init {
        #[arg(
            short,
            long,
            help = "The project to initialize. If not provided, the current directory will be used."
        )]
        project: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    if let Err(e) = match args.command {
        Commands::Convert { project, templates } => conversion::convert(project, templates),
        Commands::Init { project } => project_management::init(project),
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
