use clap::{builder::PossibleValuesParser, Parser, Subcommand};
use consts::POSSIBLE_TEMPLATES;

mod consts;
mod conversion;
mod conversion_decider;
mod converters;
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
            help = "The project to initialize. If not provided, the current directory will be used."
        )]
        project: Option<String>,
        #[arg(
            short,
            long,
            help = r#"The templates to use. If not provided, the default template.tex will be used. If using a LiX template, make sure to install the corresponding .sty and .cls files from https://github.com/NicklasVraa/LiX. Adjust the metadata in template/meta.tex accordingly."#,
            value_parser = PossibleValuesParser::new(POSSIBLE_TEMPLATES),
            use_value_delimiter = true,
            value_delimiter = ',',
            num_args = 1..,
        )]
        templates: Option<Vec<String>>,
        #[arg(short, long, help = "Delete the project if it already exists.")]
        force: bool,
        #[arg(
            short,
            long,
            help = "The directory where the Markdown files are located. If not provided, Markdown/ will be used."
        )]
        markdown_dir: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    if let Err(e) = match args.command {
        Commands::Convert { project, templates } => conversion::convert(project, templates),
        Commands::Init {
            project,
            templates,
            force,
            markdown_dir,
        } => project_management::init(project, templates, force, markdown_dir),
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
