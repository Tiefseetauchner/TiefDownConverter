use clap::Parser;
use cli::*;
use colog::format::CologStyle;
use color_eyre::eyre::{Result, eyre};
use env_logger::fmt::Formatter;
use log::Level;
use std::io::Write;
use tiefdownlib::{
    consts, conversion, injections, markdown_project_management, metadata_management,
    project_handle::ProjectHandle, project_management,
};

mod cli;
mod cli_meta_generation_feature;
mod cli_meta_generation_format;
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
        Commands::Project { project, command } => {
            let mut handle = ProjectHandle::open(project)?;

            match command {
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
                        multi_file_output,
                        output_extension,
                        meta_gen_feature,
                        nav_meta_gen_output,
                        metadata_meta_gen_output,
                        meta_gen_format,
                    } => project_management::add_template(
                        &mut handle,
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
                        multi_file_output,
                        output_extension,
                        meta_gen_feature.map(|t| t.into()),
                        nav_meta_gen_output,
                        metadata_meta_gen_output,
                        meta_gen_format.map(|t| t.into()),
                    )?,
                    TemplatesCommands::Remove => {
                        project_management::remove_template(&mut handle, template)?
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
                        multi_file_output,
                        output_extension,
                        meta_gen_feature,
                        nav_meta_gen_output,
                        metadata_meta_gen_output,
                        meta_gen_format,
                    } => {
                        if filters.is_some() && (add_filters.is_some() || remove_filters.is_some())
                        {
                            return Err(eyre!(
                                "Cannot specify both filters or add/remove filters."
                            ));
                        }

                        if preprocessors.is_some()
                            && (add_preprocessors.is_some() || remove_preprocessors.is_some())
                        {
                            return Err(eyre!(
                                "Cannot specify both preprocessors and add/remove preprocessors."
                            ));
                        }

                        project_management::update_template(
                            &mut handle,
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
                            multi_file_output,
                            output_extension,
                            meta_gen_feature.map(|t| t.into()),
                            nav_meta_gen_output,
                            metadata_meta_gen_output,
                            meta_gen_format.map(|t| t.into()),
                        )?
                    }
                },
                ProjectCommands::UpdateSettings {
                    smart_clean,
                    smart_clean_threshold,
                } => project_management::update_settings(
                    &mut handle,
                    smart_clean,
                    smart_clean_threshold,
                )?,
                ProjectCommands::PreProcessors { command } => match command {
                    PreProcessorsCommands::Add {
                        name,
                        filter,
                        cli,
                        cli_args,
                    } => project_management::add_preprocessor(
                        &mut handle,
                        name,
                        filter,
                        cli,
                        cli_args,
                    )?,
                    PreProcessorsCommands::Remove { name } => {
                        project_management::remove_preprocessor(&mut handle, name)?
                    }
                    PreProcessorsCommands::List => project_commands::list_preprocessors(&handle)?,
                },
                ProjectCommands::Processors { command } => match command {
                    ProcessorsCommands::Add {
                        name,
                        processor_args,
                    } => project_management::add_processor(&mut handle, name, processor_args)?,
                    ProcessorsCommands::Remove { name } => {
                        project_management::remove_processor(&mut handle, name)?
                    }
                    ProcessorsCommands::List => project_commands::list_processors(&handle)?,
                },
                ProjectCommands::Profiles { command } => match command {
                    ProfilesCommands::Add { name, templates } => {
                        project_management::add_profile(&mut handle, name, templates)?
                    }
                    ProfilesCommands::Remove { name } => {
                        project_management::remove_profile(&mut handle, name)?
                    }
                    ProfilesCommands::List => project_commands::list_profiles(&handle)?,
                },
                ProjectCommands::Injections { command } => match command {
                    ManageInjectionsCommand::Add { name, files } => {
                        injections::add_injection(&mut handle, name, files)?
                    }
                    ManageInjectionsCommand::Remove { name } => {
                        injections::remove_injection(&mut handle, name)?
                    }
                    ManageInjectionsCommand::AddFiles { name, files } => {
                        injections::add_files_to_injection(&mut handle, name, files)?
                    }
                    ManageInjectionsCommand::List {} => {
                        project_commands::list_injections(&mut handle)?;
                    }
                },
                ProjectCommands::SharedMeta { command } => match command {
                    ManageMetadataCommand::Set { key, value } => {
                        metadata_management::set_metadata(&mut handle, key, value)?
                    }
                    ManageMetadataCommand::Remove { key } => {
                        metadata_management::remove_metadata(&mut handle, key)?
                    }
                    ManageMetadataCommand::List => {
                        project_commands::list_shared_metadata(&mut handle)?
                    }
                },
                ProjectCommands::Markdown { command } => match command {
                    ManageMarkdownProjectsCommand::Add {
                        name,
                        path,
                        output,
                        default_profile,
                    } => markdown_project_management::add_markdown_project(
                        &mut handle,
                        name,
                        path,
                        output,
                        default_profile,
                    )?,
                    ManageMarkdownProjectsCommand::Remove { name } => {
                        markdown_project_management::remove_markdown_project(&mut handle, name)?
                    }
                    ManageMarkdownProjectsCommand::Update {
                        name,
                        path,
                        output,
                        default_profile,
                    } => markdown_project_management::update_markdown_project(
                        &mut handle,
                        name,
                        path,
                        output,
                        default_profile,
                    )?,
                    ManageMarkdownProjectsCommand::Meta { name, command } => match command {
                        ManageMetadataCommand::Set { key, value } => {
                            markdown_project_management::set_metadata(
                                &mut handle,
                                name,
                                key,
                                value,
                            )?
                        }
                        ManageMetadataCommand::Remove { key } => {
                            markdown_project_management::remove_metadata(&mut handle, name, key)?
                        }
                        ManageMetadataCommand::List => {
                            project_commands::list_markdown_project_metadata(&mut handle, name)?
                        }
                    },
                    ManageMarkdownProjectsCommand::Resources { name, command } => match command {
                        ManageResourcesCommand::Add { paths } => {
                            markdown_project_management::add_resources(&mut handle, name, paths)?
                        }
                        ManageResourcesCommand::Remove { path } => {
                            markdown_project_management::remove_resource(&mut handle, name, path)?
                        }
                        ManageResourcesCommand::List => {
                            project_commands::list_resources(&mut handle, name)?
                        }
                    },
                    ManageMarkdownProjectsCommand::List => {
                        project_commands::list_markdown_projects(&mut handle)?
                    }
                },
                ProjectCommands::ListTemplates => project_commands::list_templates(&mut handle)?,
                ProjectCommands::Clean => project_management::clean(&mut handle)?,
                ProjectCommands::SmartClean => project_management::smart_clean(&mut handle)?,
            }

            handle.save_if_dirty()?;
        }
        Commands::CheckDependencies => {
            project_management::check_dependencies(vec!["pandoc", "xelatex", "typst"])?
        }
    }

    Ok(())
}
