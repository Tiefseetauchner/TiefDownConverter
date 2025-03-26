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
#[command(name = "tiefdownconverter", version)]
#[command(
    about = "A CLI tool for managing TiefDown Projects",
    long_about = r#"TiefDownConverter manages TiefDown projects.
TiefDown is a project structure meant to simplify the conversion process from Markdown to PDFs.
TiefDownConverter consolidates multiple conversion processes and templating systems to generate a configurable set or subset of output documents.
It is not in itself a converter, but a wrapper around pandoc, xelatex and typst. As such, it requires these dependencies to be installed."#
)]
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
        #[arg(
            long,
            action,
            help = r#"Enables smart clean for the project with a default threshold of 5."#,
            long_help = r#"Enables smart clean for the project with a default threshold of 5.
If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders."#
        )]
        smart_clean: bool,
        #[arg(
            long,
            help = r#"The threshold for smart clean. If not provided, the default threshold of 5 will be used."#,
            long_help = r#"The threshold for smart clean. If not provided, the default threshold of 5 will be used.
If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders."#
        )]
        smart_clean_threshold: Option<u32>,
    },
    #[command(about = "Update the TiefDown project.")]
    Project {
        #[arg(help = "The project to edit. If not provided, the current directory will be used.")]
        project: Option<String>,
        #[command(subcommand)]
        command: ProjectCommands,
    },
    #[command(about = "Validate dependencies are installed.")]
    CheckDependencies,
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
            short = 'f',
            long,
            help = "The file to use as the template. If not provided, the template name will be used."
        )]
        template_file: Option<PathBuf>,
        #[arg(
            short,
            long,
            help = "The type of the template. If not provided, the type will be inferred from the template file."
        )]
        template_type: Option<TemplateType>,
        #[arg(
            short,
            long,
            help = "The output file. If not provided, the template name will be used."
        )]
        output: Option<PathBuf>,
        #[arg(long, help = "The luafilters to use for pandoc conversion of this templates markdown.", num_args = 1.., value_delimiter = ',')]
        filters: Option<Vec<String>>,
        #[arg(long, help = "The preprocessor to use for this template.")]
        preprocessor: Option<String>,
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
        #[arg(long, help = "The preprocessor to use for this template.")]
        preprocessor: Option<String>,
    },
    #[command(about = "Update the project manifest.")]
    UpdateManifest {
        #[arg(
            short,
            long,
            help = "The directory where the Markdown files are located."
        )]
        markdown_dir: Option<String>,
        #[arg(
            long,
            help = r#"Enables smart clean for the project with a default threshold of 5."#,
            long_help = r#"Enables smart clean for the project with a default threshold of 5.
If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders."#
        )]
        smart_clean: Option<bool>,
        #[arg(
            long,
            help = r#"The threshold for smart clean. If not provided, the default threshold of 5 will be used."#,
            long_help = r#"The threshold for smart clean. If not provided, the default threshold of 5 will be used.
If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders."#
        )]
        smart_clean_threshold: Option<u32>,
    },
    #[command(about = "Add a new preprocessor to the project.")]
    AddPreprocessor {
        #[arg(help = "The name of the preprocessor to create.")]
        name: String,
        #[arg(help = "The arguments to pass to the preprocessor.", num_args = 1.., value_delimiter = ' ', last = true, allow_hyphen_values = true)]
        pandoc_args: Vec<String>,
    },
    #[command(about = "Remove a preprocessor from the project.")]
    RemovePreprocessor {
        #[arg(help = "The name of the preprocessor to remove.")]
        name: String,
    },
    #[command(
        about = "Add a new conversion profile to the project.",
        long_about = "Add a new conversion profile to the project. These profiles contain a list of templates to preset conversion workflows."
    )]
    AddProfile {
        #[arg(help = "The name of the profile to create.")]
        name: String,
        #[arg(
            help = "The templates to add to the profile.",
            num_args = 1..,
            value_delimiter = ',',
        )]
        templates: Vec<String>,
    },
    #[command(about = "Remove a conversion profile from the project.")]
    RemoveProfile {
        #[arg(help = "The name of the profile to remove.")]
        name: String,
    },
    #[command(about = "List the templates in the project.")]
    ListTemplates,
    #[command(about = "List the conversion profiles in the project.")]
    ListProfiles,
    #[command(about = "List the preprocessors in the project.")]
    ListPreprocessors,
    #[command(about = "Validate the TiefDown project structure and metadata.")]
    Validate,
    #[command(about = "Clean temporary files from the TiefDown project.")]
    Clean,
    #[command(
        about = "Clean temporary files from the TiefDown project, leaving only the threshold amount of folders.",
        long_about = r#"Clean temporary files from the TiefDown project.
If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders.
The threshold is set to 5 by default, and is overwritten by the threshold in the manifest."#
    )]
    SmartClean,
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
            smart_clean,
            smart_clean_threshold,
        } => project_management::init(
            project,
            templates,
            no_templates,
            force,
            markdown_dir,
            smart_clean,
            smart_clean_threshold,
        )?,
        Commands::Project { project, command } => match command {
            ProjectCommands::AddTemplate {
                template,
                template_file,
                template_type,
                output,
                filters,
                preprocessor,
            } => project_management::add_template(
                project,
                template,
                template_type,
                template_file,
                output,
                filters,
                preprocessor,
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
                preprocessor,
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
                    preprocessor,
                )?
            }
            ProjectCommands::UpdateManifest {
                markdown_dir,
                smart_clean,
                smart_clean_threshold,
            } => project_management::update_manifest(
                project,
                markdown_dir,
                smart_clean,
                smart_clean_threshold,
            )?,
            ProjectCommands::AddPreprocessor { name, pandoc_args } => {
                project_management::add_preprocessor(project, name, pandoc_args)?
            }
            ProjectCommands::RemovePreprocessor { name } => {
                project_management::remove_preprocessor(project, name)?
            }
            ProjectCommands::AddProfile { name, templates } => {
                project_management::add_profile(project, name, templates)?
            }
            ProjectCommands::RemoveProfile { name } => {
                project_management::remove_profile(project, name)?
            }
            ProjectCommands::ListTemplates => project_management::list_templates(project)?,
            ProjectCommands::ListProfiles => project_management::list_profiles(project)?,
            ProjectCommands::ListPreprocessors => project_management::list_preprocessors(project)?,
            ProjectCommands::Validate => project_management::validate(project)?,
            ProjectCommands::Clean => project_management::clean(project)?,
            ProjectCommands::SmartClean => project_management::smart_clean(project)?,
        },
        Commands::CheckDependencies => {
            project_management::check_dependencies(vec!["pandoc", "xelatex", "typst"])?
        }
    }

    Ok(())
}
