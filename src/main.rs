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
mod metadata_management;
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
            help = "The templates to use. If not provided, the default templates from the manifest file will be used. Cannot be used with --profile.",
            use_value_delimiter = true,
            value_delimiter = ',',
            num_args = 1..,
        )]
        templates: Option<Vec<String>>,
        #[arg(
            short = 'P',
            long,
            help = "The conversion profile to use. Cannot be used with --templates."
        )]
        profile: Option<String>,
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
    #[command(about = "Add or modify templates in the project.")]
    Templates {
        #[arg(help = "The template name to edit or add.")]
        template: String,
        #[command(subcommand)]
        command: TemplatesCommands,
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
    #[command(about = "Manage the preprocessors of the project.")]
    PreProcessors {
        #[command(subcommand)]
        command: PreProcessorsCommands,
    },
    #[command(about = "Manage the preprocessors of the project.")]
    Processors {
        #[command(subcommand)]
        command: ProcessorsCommands,
    },
    #[command(about = "Manage the conversion profiles of the project.")]
    Profiles {
        #[command(subcommand)]
        command: ProfilesCommands,
    },
    #[command(about = "Manage the manifest metadata of the project.")]
    ManageMetadata {
        #[command(subcommand)]
        command: ManageMetadataCommand,
    },
    #[command(about = "List the templates in the project.")]
    ListTemplates,
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

#[derive(Subcommand)]
enum TemplatesCommands {
    #[command(about = "Add a new template to the project.")]
    Add {
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
        #[arg(long, help = "The processor to use for this template.")]
        processor: Option<String>,
    },
    #[command(about = "Remove a template from the project.")]
    Remove,
    #[command(about = "Update a template in the project.")]
    Update {
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
        #[arg(long, help = "The processor to use for this template.")]
        processor: Option<String>,
    },
}

#[derive(Subcommand)]
enum PreProcessorsCommands {
    #[command(about = "Add a new preprocessor to the project.")]
    Add {
        #[arg(help = "The name of the preprocessor to create.")]
        name: String,
        #[arg(help = "The arguments to pass to the preprocessor.", num_args = 1.., value_delimiter = ' ', last = true, allow_hyphen_values = true)]
        pandoc_args: Vec<String>,
    },
    #[command(about = "Remove a preprocessor from the project.")]
    Remove {
        #[arg(help = "The name of the preprocessor to remove.")]
        name: String,
    },
    #[command(about = "List the preprocessors in the project.")]
    List,
}

#[derive(Subcommand)]
enum ProcessorsCommands {
    #[command(about = "Add a new processor to the project.")]
    Add {
        #[arg(help = "The name of the processor to create.")]
        name: String,
        #[arg(help = "The arguments to pass to the processor.", num_args = 1.., value_delimiter = ' ', last = true, allow_hyphen_values = true)]
        processor_args: Vec<String>,
    },
    #[command(about = "Remove a processor from the project.")]
    Remove {
        #[arg(help = "The name of the processor to remove.")]
        name: String,
    },
    #[command(about = "List the processors in the project.")]
    List,
}

#[derive(Subcommand)]
enum ProfilesCommands {
    #[command(
        about = "Add a new conversion profile to the project.",
        long_about = "Add a new conversion profile to the project. These profiles contain a list of templates to preset conversion workflows."
    )]
    Add {
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
    Remove {
        #[arg(help = "The name of the profile to remove.")]
        name: String,
    },
    #[command(about = "List the conversion profiles in the project.")]
    List,
}

#[derive(Subcommand)]
enum ManageMetadataCommand {
    #[command(about = "Add or change the metadata of the project.")]
    Set {
        #[arg(help = "The key to set.")]
        key: String,
        #[arg(help = "The value to set.")]
        value: String,
    },
    #[command(about = "Remove metadata from the project.")]
    Remove {
        #[arg(help = "The key to remove.")]
        key: String,
    },
    #[command(about = "List the metadata of the project.")]
    List,
    // #[command(about = "Import metadata from a JSON file.")]
    // Import {
    //     #[arg(help = "The path to the JSON file.")]
    //     path: String,
    // },
    // #[command(about = "Export metadata to a JSON file.")]
    // Export {
    //     #[arg(help = "The path to the JSON file.")]
    //     path: String,
    // },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    match args.command {
        Commands::Convert {
            project,
            templates,
            profile,
        } => {
            if profile.is_some() && templates.is_some() {
                return Err(eyre!("Cannot specify both templates and a profile."));
            }

            conversion::convert(project, templates, profile)?
        }
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
            ProjectCommands::Templates { template, command } => match command {
                TemplatesCommands::Add {
                    template_file,
                    template_type,
                    output,
                    filters,
                    preprocessor,
                    processor,
                } => project_management::add_template(
                    project,
                    template,
                    template_type,
                    template_file,
                    output,
                    filters,
                    preprocessor,
                    processor,
                )?,
                TemplatesCommands::Remove => {
                    project_management::remove_template(project, template)?
                }
                TemplatesCommands::Update {
                    template_file,
                    template_type,
                    output,
                    filters,
                    add_filters,
                    remove_filters,
                    preprocessor,
                    processor,
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
                        processor,
                    )?
                }
            },
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
            ProjectCommands::PreProcessors { command } => match command {
                PreProcessorsCommands::Add { name, pandoc_args } => {
                    project_management::add_preprocessor(project, name, pandoc_args)?
                }
                PreProcessorsCommands::Remove { name } => {
                    project_management::remove_preprocessor(project, name)?
                }
                PreProcessorsCommands::List => project_management::list_preprocessors(project)?,
            },
            ProjectCommands::Processors { command } => match command {
                ProcessorsCommands::Add {
                    name,
                    processor_args,
                } => project_management::add_processor(project, name, processor_args)?,
                ProcessorsCommands::Remove { name } => {
                    project_management::remove_processor(project, name)?
                }
                ProcessorsCommands::List => project_management::list_processors(project)?,
            },
            ProjectCommands::Profiles { command } => match command {
                ProfilesCommands::Add { name, templates } => {
                    project_management::add_profile(project, name, templates)?
                }
                ProfilesCommands::Remove { name } => {
                    project_management::remove_profile(project, name)?
                }
                ProfilesCommands::List => project_management::list_profiles(project)?,
            },
            ProjectCommands::ManageMetadata { command } => match command {
                ManageMetadataCommand::Set { key, value } => {
                    metadata_management::set_metadata(project, key, value)?
                }
                ManageMetadataCommand::Remove { key } => {
                    metadata_management::remove_metadata(project, key)?
                }
                ManageMetadataCommand::List => metadata_management::list_metadata(project)?,
            },
            ProjectCommands::ListTemplates => project_management::list_templates(project)?,
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
