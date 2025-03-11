use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use color_eyre::eyre::{Result, eyre};
use pandoc::{OutputFormat, Pandoc};

use crate::{
    manifest_model::TemplateMapping,
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_latex(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    ));

    convert_md_to_tex(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
    )?;

    compile_latex(compiled_directory_path, &template_path)?;
    compile_latex(compiled_directory_path, &template_path)?;

    let template_path = compiled_directory_path.join(template_path.with_extension("pdf"));
    if template_path.exists() && template_path.as_os_str() != output_path.as_os_str() {
        fs::copy(&template_path, &output_path)?;
    }

    Ok(output_path)
}

fn convert_md_to_tex(
    template: &TemplateMapping,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    combined_markdown_path: &Path,
) -> Result<()> {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&combined_markdown_path);
    pandoc.add_option(pandoc::PandocOption::Listings);
    // TODO: Output per template (use template name)
    pandoc.set_output(pandoc::OutputKind::File(
        compiled_directory_path.join("output.tex"),
    ));

    add_lua_filters(template, project_directory_path, &mut pandoc)?;

    pandoc.execute()?;

    Ok(())
}

// NOTE: This requires xelatex to be installed. I don't particularly like that, but I tried tectonic and it didn't work.
//       For now we'll keep it simple and just use xelatex. I'm not sure if there's a way to get tectonic to work with the current setup.
fn compile_latex(compiled_directory_path: &Path, template_path: &Path) -> Result<()> {
    Command::new("xelatex")
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template_path)
        .stdout(Stdio::null())
        .status()?;

    Ok(())
}

pub(crate) fn convert_epub(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
) -> Result<PathBuf> {
    let template_path = compiled_directory_path.join(get_template_path(
        template.template_file.clone(),
        &template.name,
    ));
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    ));

    let mut pandoc = Pandoc::new();
    pandoc.add_input(combined_markdown_path);
    pandoc.set_output(pandoc::OutputKind::File(output_path.clone()));
    pandoc.set_output_format(OutputFormat::Epub3, vec![]);

    add_lua_filters(template, project_directory_path, &mut pandoc)?;

    pandoc.execute()?;

    Ok(output_path)
}

pub(crate) fn convert_typst(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
) -> Result<PathBuf> {
    let template_path = compiled_directory_path.join(get_template_path(
        template.template_file.clone(),
        &template.name,
    ));
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    ));

    convert_md_to_typst(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
    )?;

    Command::new("typst")
        .current_dir(compiled_directory_path)
        .arg("compile")
        .arg(template_path)
        .arg(&output_path)
        .stdout(Stdio::null())
        .status()?;

    Ok(output_path)
}

fn convert_md_to_typst(
    template: &TemplateMapping,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    combined_markdown_path: &Path,
) -> Result<()> {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&combined_markdown_path);
    // TODO: Output per template (use template name)
    pandoc.set_output(pandoc::OutputKind::File(
        compiled_directory_path.join("output.typ"),
    ));

    add_lua_filters(template, project_directory_path, &mut pandoc)?;

    pandoc.execute()?;

    Ok(())
}

fn add_lua_filters(
    template: &TemplateMapping,
    project_directory_path: &Path,
    pandoc: &mut Pandoc,
) -> Result<()> {
    for filter in template.filters.clone().unwrap_or_default() {
        let filter = project_directory_path.join(&filter);

        if !filter.exists() {
            return Err(eyre!("Filter file does not exist: {}", filter.display()));
        }

        add_lua_filter_or_directory(filter, pandoc)?;
    }

    Ok(())
}

fn add_lua_filter_or_directory(filter: PathBuf, pandoc: &mut Pandoc) -> Result<()> {
    if filter.is_dir() {
        for entry in fs::read_dir(filter)? {
            let entry = entry?.path();

            add_lua_filter_or_directory(entry, pandoc)?;
        }
    } else if filter.is_file() && filter.extension().unwrap_or_default() == "lua" {
        pandoc.add_option(pandoc::PandocOption::LuaFilter(filter));
    }

    Ok(())
}
