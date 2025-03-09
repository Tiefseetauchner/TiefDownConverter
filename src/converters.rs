use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use color_eyre::eyre::Result;
use pandoc::{OutputFormat, Pandoc};

use crate::{
    manifest_model::TemplateMapping,
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_latex(
    combined_markdown_path: &PathBuf,
    compiled_directory_path: &PathBuf,
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

    convert_md_to_tex(template, compiled_directory_path, combined_markdown_path)?;

    compile_latex(compiled_directory_path, &template_path, &output_path)?;
    compile_latex(compiled_directory_path, &template_path, &output_path)?;

    Ok(output_path)
}

fn convert_md_to_tex(
    template: &TemplateMapping,
    compiled_directory_path: &PathBuf,
    combined_markdown_path: &PathBuf,
) -> Result<()> {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&combined_markdown_path);
    // TODO: Output per template (use template name)
    pandoc.set_output(pandoc::OutputKind::File(
        compiled_directory_path.join("output.tex"),
    ));

    for filter in template.filters.clone().unwrap_or_default() {
        pandoc.add_filter(move |_| filter.clone());
    }

    pandoc.execute()?;

    Ok(())
}

// NOTE: This requires xelatex to be installed. I don't particularly like that, but I tried tectonic and it didn't work.
//       For now we'll keep it simple and just use xelatex. I'm not sure if there's a way to get tectonic to work with the current setup.
fn compile_latex(
    compiled_directory_path: &std::path::PathBuf,
    template_path: &PathBuf,
    output_path: &PathBuf,
) -> Result<()> {
    Command::new("xelatex")
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template_path)
        .arg("-output")
        .arg(output_path)
        .stdout(Stdio::null())
        .status()?;
    Ok(())
}

pub(crate) fn convert_epub(
    combined_markdown_path: &PathBuf,
    compiled_directory_path: &PathBuf,
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

    pandoc.execute()?;

    Ok(output_path)
}

pub(crate) fn convert_typst(
    combined_markdown_path: &PathBuf,
    compiled_directory_path: &PathBuf,
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

    convert_md_to_typst(template, compiled_directory_path, combined_markdown_path)?;

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
    compiled_directory_path: &PathBuf,
    combined_markdown_path: &PathBuf,
) -> Result<()> {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&combined_markdown_path);
    // TODO: Output per template (use template name)
    pandoc.set_output(pandoc::OutputKind::File(
        compiled_directory_path.join("output.typ"),
    ));

    for filter in template.filters.clone().unwrap_or_default() {
        pandoc.add_filter(move |_| filter.clone());
    }

    pandoc.execute()?;

    Ok(())
}
