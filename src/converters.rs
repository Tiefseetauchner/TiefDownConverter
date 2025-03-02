use std::{
    error::Error,
    path::PathBuf,
    process::{Command, Stdio},
};

use pandoc::{OutputFormat, Pandoc};

pub(crate) fn convert_latex(
    compiled_directory_path: &PathBuf,
    template: &String,
) -> Result<PathBuf, Box<dyn Error>> {
    compile_latex(compiled_directory_path, template)?;
    compile_latex(compiled_directory_path, template)?;

    let result_file_name = template.replace(".tex", ".pdf");
    let output_path = compiled_directory_path.join(&result_file_name);
    Ok(output_path)
}

// NOTE: This requires xelatex to be installed. I don't particularly like that, but I tried tectonic and it didn't work.
//       For now we'll keep it simple and just use xelatex. I'm not sure if there's a way to get tectonic to work with the current setup.
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

pub(crate) fn convert_epub(
    compiled_directory_path: &PathBuf,
    template: &String,
) -> Result<PathBuf, Box<dyn Error>> {
    let result_file_name = format!("{}.epub", template.trim_matches('/'));
    let output_path = compiled_directory_path.join(&result_file_name);

    let mut pandoc = Pandoc::new();
    pandoc.add_input(&compiled_directory_path.join("combined.md"));
    pandoc.set_output(pandoc::OutputKind::File(output_path.clone()));
    pandoc.set_output_format(OutputFormat::Epub3, vec![]);

    pandoc.execute()?;

    Ok(output_path)
}
