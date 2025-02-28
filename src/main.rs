use chrono::prelude::{DateTime, Utc};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use toml;

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
}

fn main() {
    let args = Cli::parse();

    if let Err(e) = match args.command {
        Commands::Convert { project, templates } => convert(project, templates),
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn convert(project: Option<String>, templates: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = Path::new(&project);

    if !project_path.exists() {
        return Err("Project path does not exist.".into());
    }

    let manifest_path = project_path.join("manifest.toml");
    if !manifest_path.exists() {
        return Err("No manifest file found. Please initialize a project first.".into());
    }

    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: toml::Value = toml::from_str(&manifest_content)?;

    println!("Converting project: {}", project);

    let current_time = std::time::SystemTime::now();
    let current_time: DateTime<Utc> = current_time.into();
    let current_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let compiled_directory_path = project_path.join(current_time);
    copy_dir::copy_dir(project_path.join("template/"), &compiled_directory_path)?;

    let combined_markdown_path = compiled_directory_path.join("combined.md");
    let markdown_dir = project_path.join(manifest["markdown_dir"].as_str().unwrap_or("Markdown/"));
    let mut combined_content = String::new();

    // NOTE: This expects the markdown files to be named "Chapter NR.md" to order them correctly.
    //       Not the nicest solution, but it works for now.
    let chapter_name_regex = regex::Regex::new(r"Chapter (\d+).*").unwrap();

    let mut markdown_files: Vec<_> = fs::read_dir(markdown_dir)?.filter_map(Result::ok).collect();

    markdown_files.sort_by_key(|entry| {
        let binding = entry.file_name();
        let filename = binding.to_string_lossy();
        chapter_name_regex
            .captures(&filename)
            .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok()) // Extract and parse number
            .unwrap_or(0) // Default to 0 if no number is found
    });

    for entry in markdown_files {
        if entry.path().extension() == Some("md".as_ref()) {
            combined_content.push_str(&fs::read_to_string(entry.path())?);
            combined_content.push_str("\n\n"); // Add spacing between files
        }
    }

    fs::write(&combined_markdown_path, combined_content)?;

    let lua_filter_args = get_lua_filters(project_path)?;

    let pandoc_status = Command::new("pandoc")
        .arg(&combined_markdown_path)
        .arg("-o")
        .arg(compiled_directory_path.join("output.tex"))
        .args(lua_filter_args)
        .status()?;
    if !pandoc_status.success() {
        return Err("Pandoc failed to convert the markdown.".into());
    }

    let templates = templates.unwrap_or_else(|| {
        manifest
            .get("templates")
            .and_then(|t| t.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default()
    });

    for template in &templates {
        let template_path = compiled_directory_path.join(template);
        if !template_path.exists() {
            eprintln!("Warning: Template path does not exist: {}", template);
            continue;
        }

        if template_path.extension() == Some("tex".as_ref()) {
            println!("Converting using XeTeX...");

            // NOTE: This is a little bit of a hack to get around the fact that for the first compile, the toc index is not yet generated.
            compile_latex(&compiled_directory_path, template)?;
            compile_latex(&compiled_directory_path, template)?;

            let result_file_name = format!("{}.pdf", template.replace(".tex", ""));

            let output_path = compiled_directory_path.join(&result_file_name);
            fs::copy(output_path, project_path.join(&result_file_name))?;
        } else {
            eprintln!(
                "Warning: The template type '{}' is not yet supported.",
                template
            );
            continue;
        }

        println!("Converted template: {}", template);
    }

    Ok(())
}

fn get_lua_filters(project_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let lua_filters = fs::read_dir(project_path.join("luafilters"))?;
    let mut lua_filter_args = vec![];
    for lua_filter in lua_filters {
        lua_filter_args.push("--lua-filter".to_string());
        lua_filter_args.push(lua_filter?.path().to_str().unwrap().to_string());
    }
    Ok(lua_filter_args)
}

fn compile_latex(
    compiled_directory_path: &std::path::PathBuf,
    template: &String,
) -> Result<(), Box<dyn Error>> {
    Command::new("xelatex")
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template)
        .stdout(Stdio::null())
        .status()?;
    Ok(())
}
