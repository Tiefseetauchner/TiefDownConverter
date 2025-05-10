use clap::{Parser, Subcommand, builder::PossibleValuesParser, command};
use std::path::PathBuf;

use crate::{consts::POSSIBLE_TEMPLATES, cli_template_type::CliTemplateType};

#[derive(Parser)]
#[command(
    name = "tiefdownconverter",
    version,
    author = env!("CARGO_PKG_AUTHORS"),
    about = "A CLI tool for managing TiefDown Projects",
    long_about = r#"TiefDownConverter manages TiefDown projects.
TiefDown is a project structure meant to simplify the conversion process from Markdown to PDFs.
TiefDownConverter consolidates multiple conversion processes and templating systems to generate a configurable set or subset of output documents.
It is not in itself a converter, but a wrapper around pandoc, xelatex and typst. As such, it requires these dependencies to be installed."#
)]
pub struct Cli {
    #[arg(short, long, help = "Enable verbose output.")]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum ProjectCommands {
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
            long,
            help = r#"Enables smart clean for the project with a default threshold of 5."#,
            long_help = r#"Enables smart clean for the project with a default threshold of 5.
If the number of conversion folders in the project is above the smart_clean_threshold, old folders will be cleaned, leaving only the threshold amount of folders."#
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
    #[command(
        about = "Manage the preprocessors of the project.",
        long_about = r#"Manage the preprocessors of the project.
A preprocessor defines the arguments passed to the pandoc conversion from markdown.
If using a CustomPandoc template, a preprocessor is required.
Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
For templates, that is the file imported by the template.
Preprocessors are incompatible with epub conversion. Use processors instead."#
    )]
    PreProcessors {
        #[command(subcommand)]
        command: PreProcessorsCommands,
    },
    #[command(
        about = "Manage the processors of the project.",
        long_about = r#"Manage the processors of the project.
A processor defines additional arguments passed to the conversion command.
For LaTeX and typst templates, this allows extending the respective conversion parameters.
For epub templates, this allows adding custom pandoc parameters.
Processors are incompatible with CustomPandoc conversions. Use preprocessors instead."#
    )]
    Processors {
        #[command(subcommand)]
        command: ProcessorsCommands,
    },
    #[command(
        about = "Manage the conversion profiles of the project.",
        long_about = r#"Manage the conversion profiles of the project.
A conversion profile defines a collection of templates to be converted at the same time.
This can be used to prepare presets (for example, web export, PDF export, ...).
It can also be used for defining default templates for markdown projects."#
    )]
    Profiles {
        #[command(subcommand)]
        command: ProfilesCommands,
    },
    #[command(
        about = "Manage the shared metadata of the project.",
        long_about = r#"Manage the shared metadata of the project.
This Metadata is shared between all markdown projects.
When converting, it is merged with the markdown project specific metadata.
When using the same key for shared and project metadata, the project metadata overrides the shared metadata."#
    )]
    SharedMeta {
        #[command(subcommand)]
        command: ManageMetadataCommand,
    },
    #[command(
        about = "Manage the markdown projects of the project.",
        long_about = r#"Manage the markdown projects of the project.
A markdown project defines the markdown conversion process for a project.
There can be multiple markdown projects with different markdown files.
Each markdown project also has a seperate output folder ('.' per default).
A markdown project can have seperate metadata.
A markdown project can have resources that are copied to the respective conversion folder."#
    )]
    Markdown {
        #[command(subcommand)]
        command: ManageMarkdownProjectsCommand,
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
pub enum TemplatesCommands {
    #[command(
        about = "Add a new template to the project.",
        long_about = format!(r#"Add a new template to the project.
If using a preset template name, the preset will be copied to the template folder.
If using a custom template, make sure to add the respective files to the template folder.
Available preset templates are: {}"#, POSSIBLE_TEMPLATES.join(", "))
    )]
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
        template_type: Option<CliTemplateType>,
        #[arg(
            short,
            long,
            help = "The output file. If not provided, the template name will be used."
        )]
        output: Option<PathBuf>,
        #[arg(
            long, 
            help = "The luafilters to use for pandoc conversion of this templates markdown.", 
            long_help = r#"The luafilters to use for pandoc conversion of this templates markdown.
Luafilters are lua scripts applied during the pandoc conversion.
You can add a folder or a filename. If adding a folder, it will be traversed recursively, and any .lua file will be added.
See the pandoc documentation and 'Writing filters' of the TiefDownConverter documentation for more details."#,
            num_args = 1.., 
            value_delimiter = ','
        )]
        filters: Option<Vec<String>>,
        #[arg(
            long, 
            help = "The preprocessor to use for this template.",
            long_help = r#"The preprocessor to use for this template.
A preprocessor defines the arguments passed to the pandoc conversion from markdown.
If using a CustomPandoc template, a preprocessor is required.
Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
For templates, that is the file imported by the template.
Preprocessors are incompatible with epub conversion. Use processors instead."#
        )]
        preprocessor: Option<String>,
        #[arg(
            long, 
            help = "The processor to use for this template.",
            long_help = r#"The processor to use for this template.
A processor defines additional arguments passed to the conversion command.
For LaTeX and typst templates, this allows extending the respective conversion parameters.
For epub templates, this allows adding custom pandoc parameters.
Processors are incompatible with CustomPandoc conversions. Use preprocessors instead."#
        )]
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
        template_type: Option<CliTemplateType>,
        #[arg(
            long,
            help = "The output file. If not provided, the template name will be used."
        )]
        output: Option<PathBuf>,
        #[arg(
            long,
            help = "The luafilters to use for pandoc conversion of this templates markdown.", 
            long_help = r#"The luafilters to use for pandoc conversion of this templates markdown.
This replaces all existing filters."#,
            num_args = 1.., 
            value_delimiter = ','
        )]
        filters: Option<Vec<String>>,
        #[arg(
            long,
            help = "The luafilters to add to the template.",
            long_help = r#"The luafilters to use for pandoc conversion of this templates markdown.
This adds to the existing filters."#,
            num_args = 1..,
            value_delimiter = ',',
        )]
        add_filters: Option<Vec<String>>,
        #[arg(
            long,
            help = "The luafilters to remove from the template.",
            long_help = r#"The luafilters to use for pandoc conversion of this templates markdown.
This removes the filter from the existing filters."#,
            num_args = 1..,
            value_delimiter = ',',
        )]
        remove_filters: Option<Vec<String>>,
        #[arg(
            long, 
            help = "The preprocessor to use for this template.",
            long_help = r#"The preprocessor to use for this template.
A preprocessor defines the arguments passed to the pandoc conversion from markdown.
If using a CustomPandoc template, a preprocessor is required.
Preprocessors replace all arguments. Thus, with preprocessors, you need to define the output file and format.
For templates, that is the file imported by the template.
Preprocessors are incompatible with epub conversion. Use processors instead."#
        )]
        preprocessor: Option<String>,
        #[arg(
            long, 
            help = "The processor to use for this template.",
            long_help = r#"The processor to use for this template.
A processor defines additional arguments passed to the conversion command.
For LaTeX and typst templates, this allows extending the respective conversion parameters.
For epub templates, this allows adding custom pandoc parameters.
Processors are incompatible with CustomPandoc conversions. Use preprocessors instead."#
        )]
        processor: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum PreProcessorsCommands {
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
pub enum ProcessorsCommands {
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
pub enum ProfilesCommands {
    #[command(
        about = "Add a new conversion profile to the project.",
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
pub enum ManageMarkdownProjectsCommand {
    #[command(about = "Add a new markdown project to the project.")]
    Add {
        #[arg(help = "The name of the markdown project to create.")]
        name: String,
        #[arg(help = "The path to the markdown project.")]
        path: PathBuf,
        #[arg(help = "The output folder.")]
        output: PathBuf,
        #[arg(long, help = "The default profile to use for converting this project.")]
        default_profile: Option<String>,
    },
    #[command(about = "Update a markdown project in the project.")]
    Update {
        #[arg(help = "The name of the markdown project to update.")]
        name: String,
        #[arg(long, help = "The path to the markdown project.")]
        path: Option<PathBuf>,
        #[arg(long, help = "The output folder.")]
        output: Option<PathBuf>,
        #[arg(long, help = "The default profile to use for converting this project.")]
        default_profile: Option<String>,
    },
    #[command(
        about = "Manage the metadata of a markdown project.",
        long_about = r#"Manage the metadata of a markdown project.
This metadata is markdown project specific and is not shared between projects.
This metadata takes precedence over the shared metadata."#
    )]
    Meta {
        #[arg(help = "The name of the markdown project to update.")]
        name: String,
        #[command(subcommand)]
        command: ManageMetadataCommand,
    },
    #[command(
        about = "Manage the resources of a markdown project.",
        long_about = r#"Manage the resources of a markdown project.
Resources are a way to include meta information and resources on a per project basis.
This is helpful for example for including a custom css file for a project, as that is not possible purely with metadata.
Resources are stored in the markdown folder and copied to the conversion directory for that profile before conversion."#
    )]
    Resources {
        #[arg(help = "The name of the markdown project to update.")]
        name: String,
        #[command(subcommand)]
        command: ManageResourcesCommand,
    },
    #[command(about = "Remove a markdown project from the project.")]
    Remove {
        #[arg(help = "The name of the markdown project to remove.")]
        name: String,
    },
    #[command(about = "List the markdown projects in the project.")]
    List,
}

#[derive(Subcommand)]
pub enum ManageMetadataCommand {
    #[command(about = "Add or change the metadata. Overrides previous keys.")]
    Set {
        #[arg(help = "The key to set.")]
        key: String,
        #[arg(help = "The value to set.")]
        value: String,
    },
    #[command(about = "Remove metadata.")]
    Remove {
        #[arg(help = "The key to remove.")]
        key: String,
    },
    #[command(about = "List the metadata.")]
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

#[derive(Subcommand)]
pub enum ManageResourcesCommand {
    #[command(about = "Add a new resource to the project.")]
    Add {
        #[arg(help = "The paths to the resources. Seperated by spaces.", num_args = 1.., value_delimiter = ' ', last = true, allow_hyphen_values = true)]
        paths: Vec<PathBuf>,
    },
    #[command(about = "Remove a resource from the project.")]
    Remove {
        #[arg(help = "The path to the resource.")]
        path: PathBuf,
    },
    #[command(about = "List the resources in the project.")]
    List,
}
