use clap::{Parser, Subcommand, builder::PossibleValuesParser};
use color_eyre::eyre::{Result, eyre};
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
If using a LiX template, make sure to install the corresponding .sty and .cls files from https://github.com/NicklasVraa/LiX. Adjust the metadata in template/meta.tex accordingly.
"#,
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
    Project {
        #[arg(help = "The project to edit. If not provided, the current directory will be used.")]
        project: Option<String>,
        #[command(subcommand)]
        command: ProjectCommands,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    #[command(about = "Add a new template to the project.")]
    AddTemplate {
        #[arg(
            help = r#"The name of the template to create. If using a LiX template, make sure to install the corresponding .sty and .cls files from https://github.com/NicklasVraa/LiX. Adjust the metadata in template/meta.tex accordingly."#
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
        #[arg(short, long, help = r#"The template to remove."#)]
        template: String,
    },
    #[command(about = "Update a template in the project.")]
    UpdateTemplate {
        #[arg(help = r#"The template to update."#)]
        template: String,
        #[arg(
            long,
            help = "The file to use as the template. If not provided, the template name will be used."
        )]
        template_file: Option<PathBuf>,
        #[arg(
            long,
            help = r#"The type of the template. If not provided, the type will be inferred from the template file.
Changing this is not recommended, as it is highly unlikely the type and only the type has changed. It is recommended to create a new template instead."#
        )]
        template_type: Option<TemplateType>,
        #[arg(
            long,
            help = "The output file. If not provided, the template name will be used."
        )]
        output: Option<PathBuf>,
        #[arg(long,help = "The luafilters to use for pandoc conversion of this templates markdown.", num_args = 1.., value_delimiter = ',')]
        filters: Option<Vec<String>>,
        #[arg(
            long,
            help = "The luafilters add to the template.",
            num_args = 1..,
            value_delimiter = ',',
        )]
        add_filters: Option<Vec<String>>,
        #[arg(
            long,
            help = "The luafilters to remove from the template.",
            num_args = 1..,
            value_delimiter = ',',
        )]
        remove_filters: Option<Vec<String>>,
    },
    #[command(about = "Update the project manifest.")]
    UpdateManifest {
        #[arg(
            help = "The project to manipulate. If not provided, the current directory will be used."
        )]
        project: Option<String>,
        #[arg(
            short,
            long,
            help = "The directory where the Markdown files are located."
        )]
        markdown_dir: Option<String>,
    },
    #[command(about = "List the templates in the project.")]
    ListTemplates {
        #[arg(
            help = "The project to manipulate. If not provided, the current directory will be used."
        )]
        project: Option<String>,
    },
    #[command(about = "Validate the TiefDown project structure and metadata.")]
    Validate {
        #[arg(
            help = "The project to validate. If not provided, the current directory will be used."
        )]
        project: Option<String>,
    },
    #[command(about = "Clean temporary files from the TiefDown project.")]
    Clean {
        #[arg(help = "The project to clean. If not provided, the current directory will be used.")]
        project: Option<String>,
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
        Commands::Project { project, command } => match command {
            ProjectCommands::AddTemplate {
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
            ProjectCommands::RemoveTemplate { template } => {
                project_management::remove_template(project, template)?
            }
            ProjectCommands::UpdateTemplate {
                template,
                template_file,
                template_type,
                output,
                filters,
                add_filters,
                remove_filters,
            } => {
                if filters.is_some() && (add_filters.is_some() || remove_filters.is_some()) {
                    return Err(eyre!("Cannot specify both filters and add/remove filters."));
                }

                project_management::update_template(
                    project,
                    template,
                    template_type,
                    template_file,
                    output,
                    filters,
                    add_filters,
                    remove_filters,
                )?
            }
            ProjectCommands::UpdateManifest {
                project,
                markdown_dir,
            } => project_management::update_manifest(project, markdown_dir)?,
            ProjectCommands::ListTemplates { project } => {
                project_management::list_templates(project)?
            }
            ProjectCommands::Validate { project } => project_management::validate(project)?,
            ProjectCommands::Clean { project } => project_management::clean(project)?,
        },
    }

    Ok(())
}
