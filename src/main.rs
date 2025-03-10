use clap::{Parser, Subcommand, builder::PossibleValuesParser};
use color_eyre::eyre::Result;
use consts::POSSIBLE_TEMPLATES;
use manifest_model::TemplateType;
use std::path::PathBuf;
mod consts;
mod conversion;
mod conversion_decider;
mod converters;
mod manifest_model;
mod project_management;
mod template_management;

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
            help = r#"The preset templates to use. If not provided, the default template.tex will be used.
For custom templates, use the update command after initializing the project.
If using a LiX template, make sure to install the corresponding .sty and .cls files from https://github.com/NicklasVraa/LiX. Adjust the metadata in template/meta.tex accordingly."#,
            value_parser = PossibleValuesParser::new(&*POSSIBLE_TEMPLATES),
            use_value_delimiter = true,
            value_delimiter = ',',
            num_args = 1..,
        )]
        templates: Option<Vec<String>>,
        #[arg(
            short,
            long,
            help = "Do not include the default templates. You will need to add templates manually with Update"
        )]
        no_templates: bool,
        #[arg(short, long, help = "Delete the project if it already exists.")]
        force: bool,
        #[arg(
            short,
            long,
            help = "The directory where the Markdown files are located. If not provided, Markdown/ will be used."
        )]
        markdown_dir: Option<String>,
    },
    #[command(about = "Update the TiefDown project.")]
    Update {
        #[arg(
            help = "The project to initialize. If not provided, the current directory will be used."
        )]
        project: Option<String>,
        #[command(subcommand)]
        command: UpdateCommands,
    },
}

#[derive(Subcommand)]
enum UpdateCommands {
    #[command(about = "Add a new template to the project.")]
    AddTemplate {
        #[arg(
            help = r#"The templates to use. If not provided, the default template.tex will be used. If using a LiX template, make sure to install the corresponding .sty and .cls files from https://github.com/NicklasVraa/LiX. Adjust the metadata in template/meta.tex accordingly."#
        )]
        template: String,
        #[arg(
            help = "The file to use as the template. If not provided, the template name will be used."
        )]
        template_file: Option<PathBuf>,
        #[arg(
            help = "The type of the template. If not provided, the type will be inferred from the template file."
        )]
        template_type: Option<TemplateType>,
        #[arg(help = "The output file. If not provided, the template name will be used.")]
        output: Option<PathBuf>,
        #[arg(help = "The luafilters to use for pandoc conversion of this templates markdown.", num_args = 1.., value_delimiter = ',')]
        filters: Option<Vec<String>>,
    },
    #[command(about = "Remove a template from the project.")]
    RemoveTemplate {
        #[arg(
            short,
            long,
            help = r#"The templates to remove."#,
            value_parser = PossibleValuesParser::new(&*POSSIBLE_TEMPLATES),
        )]
        template: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    match args.command {
        Commands::Convert { project, templates } => conversion::convert(project, templates)?,
        Commands::Init {
            project,
            templates,
            no_templates,
            force,
            markdown_dir,
        } => project_management::init(project, templates, no_templates, force, markdown_dir)?,
        Commands::Update { project, command } => match command {
            UpdateCommands::AddTemplate {
                template,
                template_file,
                template_type,
                output,
                filters,
            } => project_management::add_template(
                project,
                template,
                template_type,
                template_file,
                output,
                filters,
            )?,
            UpdateCommands::RemoveTemplate { template } => {
                project_management::remove_template(project, template)?
            }
        },
    }

    Ok(())
}
