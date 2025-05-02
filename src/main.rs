use clap::Parser;
use cli::*;
use colog::format::CologStyle;
use color_eyre::{
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};
use env_logger::fmt::Formatter;
use log::Level;
use std::io::Write;

mod cli;
mod consts;
mod conversion;
mod conversion_decider;
mod converters;
mod manifest_model;
mod markdown_project_management;
mod metadata_management;
mod project_management;
mod template_management;
mod template_type;

pub struct NoPrefixToken;

impl CologStyle for NoPrefixToken {
    fn prefix_token(&self, _level: &Level) -> String {
        "".to_string()
    }

    fn format(
        &self,
        buf: &mut Formatter,
        record: &log::Record<'_>,
    ) -> std::result::Result<(), std::io::Error> {
        writeln!(buf, "{}", record.args(),)
    }
}

pub struct ErrorStyle;

impl CologStyle for ErrorStyle {
    fn prefix_token(&self, _level: &Level) -> String {
        "ERR ".red().to_string()
    }

    fn format(
        &self,
        buf: &mut Formatter,
        record: &log::Record<'_>,
    ) -> std::result::Result<(), std::io::Error> {
        writeln!(buf, "{}", record.args(),)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let log_level_filter = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    colog::default_builder()
        .filter_level(log_level_filter)
        .format(colog::formatter(NoPrefixToken))
        .target(env_logger::Target::Stdout)
        .init();

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
                smart_clean,
                smart_clean_threshold,
            } => project_management::update_manifest(project, smart_clean, smart_clean_threshold)?,
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
            ProjectCommands::SharedMeta { command } => match command {
                ManageMetadataCommand::Set { key, value } => {
                    metadata_management::set_metadata(project, key, value)?
                }
                ManageMetadataCommand::Remove { key } => {
                    metadata_management::remove_metadata(project, key)?
                }
                ManageMetadataCommand::List => metadata_management::list_metadata(project)?,
            },
            ProjectCommands::Markdown { command } => match command {
                ManageMarkdownProjectsCommand::Add {
                    name,
                    path,
                    output,
                    default_profile,
                } => markdown_project_management::add_markdown_project(
                    project,
                    name,
                    path,
                    output,
                    default_profile,
                )?,
                ManageMarkdownProjectsCommand::Remove { name } => {
                    markdown_project_management::remove_markdown_project(project, name)?
                }
                ManageMarkdownProjectsCommand::Update {
                    name,
                    path,
                    output,
                    default_profile,
                } => markdown_project_management::update_markdown_project(
                    project,
                    name,
                    path,
                    output,
                    default_profile,
                )?,
                ManageMarkdownProjectsCommand::Meta { name, command } => match command {
                    ManageMetadataCommand::Set { key, value } => {
                        markdown_project_management::set_metadata(project, name, key, value)?
                    }
                    ManageMetadataCommand::Remove { key } => {
                        markdown_project_management::remove_metadata(project, name, key)?
                    }
                    ManageMetadataCommand::List => {
                        markdown_project_management::list_metadata(project, name)?
                    }
                },
                ManageMarkdownProjectsCommand::Resources { name, command } => match command {
                    ManageResourcesCommand::Add { paths } => {
                        markdown_project_management::add_resources(project, name, paths)?
                    }
                    ManageResourcesCommand::Remove { path } => {
                        markdown_project_management::remove_resource(project, name, path)?
                    }
                    ManageResourcesCommand::List => {
                        markdown_project_management::list_resources(project, name)?
                    }
                },
                ManageMarkdownProjectsCommand::List => {
                    markdown_project_management::list_markdown_projects(project)?
                }
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
