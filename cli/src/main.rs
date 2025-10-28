use clap::Parser;
use cli::*;
use colog::format::CologStyle;
use color_eyre::eyre::{Result, eyre};
use env_logger::fmt::Formatter;
use log::Level;
use std::io::Write;
use tiefdownlib::{
    consts, conversion, injections, markdown_project_management, metadata_management,
    project_management,
};

mod cli;
mod cli_template_type;
mod project_commands;

pub(crate) struct CustomLoggingStyle;

impl CologStyle for CustomLoggingStyle {
    fn prefix_token(&self, level: &Level) -> String {
        format!("{}", self.level_color(level, self.level_token(level)),)
    }

    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERR ",
            Level::Warn => "WRN ",
            Level::Debug => "DBG ",
            _ => "",
        }
    }

    fn format(
        &self,
        buf: &mut Formatter,
        record: &log::Record<'_>,
    ) -> std::result::Result<(), std::io::Error> {
        let prefix = self.prefix_token(&record.level());
        writeln!(buf, "{}{}", prefix, record.args().to_string(),)
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
        .format(colog::formatter(CustomLoggingStyle))
        .target(env_logger::Target::Stdout)
        .init();

    match args.command {
        Commands::Convert {
            project,
            templates,
            profile,
            markdown_projects,
        } => {
            if profile.is_some() && templates.is_some() {
                return Err(eyre!("Cannot specify both templates and a profile."));
            }

            let conversion_queue = conversion::get_conversion_queue(
                project.clone(),
                templates,
                profile,
                markdown_projects,
            )?;

            conversion::convert(project, conversion_queue)?
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
                    preprocessors,
                    preprocessor_output,
                    processor,
                    header_injections,
                    body_injections,
                    footer_injections,
                } => project_management::add_template(
                    project,
                    template,
                    template_type.map(|t| t.into()),
                    template_file,
                    output,
                    filters,
                    preprocessors,
                    preprocessor_output,
                    processor,
                    header_injections,
                    body_injections,
                    footer_injections,
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
                    preprocessors,
                    add_preprocessors,
                    remove_preprocessors,
                    preprocessor_output,
                    processor,
                    header_injections,
                    body_injections,
                    footer_injections,
                } => {
                    if filters.is_some() && (add_filters.is_some() || remove_filters.is_some()) {
                        return Err(eyre!("Cannot specify both filters or add/remove filters."));
                    }

                    if preprocessors.is_some()
                        && (add_preprocessors.is_some() || remove_preprocessors.is_some())
                    {
                        return Err(eyre!(
                            "Cannot specify both preprocessors or add/remove preprocessors."
                        ));
                    }

                    project_management::update_template(
                        project,
                        template,
                        template_type.map(|t| t.into()),
                        template_file,
                        output,
                        filters,
                        add_filters,
                        remove_filters,
                        preprocessors,
                        add_preprocessors,
                        remove_preprocessors,
                        preprocessor_output,
                        processor,
                        header_injections,
                        body_injections,
                        footer_injections,
                    )?
                }
            },
            ProjectCommands::UpdateSettings {
                smart_clean,
                smart_clean_threshold,
            } => project_management::update_settings(project, smart_clean, smart_clean_threshold)?,
            ProjectCommands::PreProcessors { command } => match command {
                PreProcessorsCommands::Add {
                    name,
                    filter,
                    cli,
                    cli_args,
                } => project_management::add_preprocessor(project, name, filter, cli, cli_args)?,
                PreProcessorsCommands::Remove { name } => {
                    project_management::remove_preprocessor(project, name)?
                }
                PreProcessorsCommands::List => project_commands::list_preprocessors(project)?,
            },
            ProjectCommands::Processors { command } => match command {
                ProcessorsCommands::Add {
                    name,
                    processor_args,
                } => project_management::add_processor(project, name, processor_args)?,
                ProcessorsCommands::Remove { name } => {
                    project_management::remove_processor(project, name)?
                }
                ProcessorsCommands::List => project_commands::list_processors(project)?,
            },
            ProjectCommands::Profiles { command } => match command {
                ProfilesCommands::Add { name, templates } => {
                    project_management::add_profile(project, name, templates)?
                }
                ProfilesCommands::Remove { name } => {
                    project_management::remove_profile(project, name)?
                }
                ProfilesCommands::List => project_commands::list_profiles(project)?,
            },
            ProjectCommands::Injections { command } => match command {
                ManageInjectionsCommand::Create { name, files } => {
                    injections::add_injection(project, name, files)?
                }
                ManageInjectionsCommand::Remove { name } => {
                    injections::remove_injection(project, name)?
                }
                ManageInjectionsCommand::AddFiles { name, files } => {
                    injections::add_files_to_injection(project, name, files)?
                }
            },
            ProjectCommands::SharedMeta { command } => match command {
                ManageMetadataCommand::Set { key, value } => {
                    metadata_management::set_metadata(project, key, value)?
                }
                ManageMetadataCommand::Remove { key } => {
                    metadata_management::remove_metadata(project, key)?
                }
                ManageMetadataCommand::List => project_commands::list_shared_metadata(project)?,
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
                        project_commands::list_markdown_project_metadata(project, name)?
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
                        project_commands::list_resources(project, name)?
                    }
                },
                ManageMarkdownProjectsCommand::List => {
                    project_commands::list_markdown_projects(project)?
                }
            },
            ProjectCommands::ListTemplates => project_commands::list_templates(project)?,
            ProjectCommands::Clean => project_management::clean(project)?,
            ProjectCommands::SmartClean => project_management::smart_clean(project)?,
        },
        Commands::CheckDependencies => {
            project_management::check_dependencies(vec!["pandoc", "xelatex", "typst"])?
        }
    }

    Ok(())
}
