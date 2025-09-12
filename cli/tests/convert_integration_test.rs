use assert_cmd::Command;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::tempdir;

#[path = "assertions.rs"]
#[macro_use]
mod assertions;

const VALID_TEMPLATE_CONTENT_TEX: &str = r#"\documentclass[a4paper,12pt]{article}

\begin{document}

This is a test document, and this text is needed as otherwise the document would be empty and no pages would be generated.

\input{./output.tex}

\end{document}"#;

const VALID_TEMPLATE_CONTENT_TYP: &str = r#"#include "output.typ""#;

const VALID_MARKDOWN_CONTENT: &str = r#"# Chapter 1
Basic test content"#;

const VALID_HTML_CONTENT: &str = r#"<h1>Chapter 2</h1>
<p>Basic test content 02</p>"#;

fn create_empty_project(temp_dir: &Path, custom_args: Vec<&str>) -> PathBuf {
    let project_path = temp_dir.join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-n")
        .args(custom_args)
        .assert()
        .success();

    fs::create_dir_all(project_path.join("template")).expect("Failed to create template directory");

    project_path
}

fn add_tex_template(
    project_path: &Path,
    template_name: &str,
    template_file: &str,
    output_file: Option<&str>,
) {
    let template_file =
        create_template_file(project_path, template_file, VALID_TEMPLATE_CONTENT_TEX);
    add_template(
        project_path,
        template_name,
        &template_file,
        output_file,
        "tex",
    );
}

fn add_typst_template(
    project_path: &Path,
    template_name: &str,
    template_file: &str,
    output_file: Option<&str>,
) {
    let template_file =
        create_template_file(project_path, template_file, VALID_TEMPLATE_CONTENT_TYP);
    add_template(
        project_path,
        template_name,
        &template_file,
        output_file,
        "typst",
    );
}

fn add_epub_template(
    project_path: &Path,
    template_name: &str,
    template_file: &str,
    output_file: Option<&str>,
) {
    let template_file = PathBuf::from(template_file);
    fs::create_dir_all(project_path.join("template").join(&template_file))
        .expect("Failed to create template directory");
    add_template(
        project_path,
        template_name,
        &template_file,
        output_file,
        "epub",
    );
}

fn create_template_file(project_path: &Path, filename: &str, content: &str) -> PathBuf {
    let template_dir = project_path.join("template");
    let template_file = template_dir.join(filename);

    fs::create_dir_all(&template_dir).expect("Failed to create template directory");
    fs::write(template_file, content).expect("Failed to write template file");

    PathBuf::from(filename)
}

fn add_template(
    project_path: &Path,
    template_name: &str,
    template_file: &Path,
    output_file: Option<&str>,
    template_type: &str,
) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg(template_name)
        .arg("add")
        .arg("--template-file")
        .arg(template_file)
        .arg("--template-type")
        .arg(template_type);

    if let Some(output) = output_file {
        cmd.arg("--output").arg(output);
    }

    cmd.assert().success();

    let template_dir = project_path.join("template");
    assert!(template_dir.exists(), "Template directory should exist");
}

fn add_custom_preprocessors_template(
    project_path: &Path,
    template_name: &str,
    preprocessor: &str,
    preprocessor_args: &str,
    preprocessor_combined_path: &str,
    output_file: &str,
) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("templates")
        .arg(template_name)
        .arg("add")
        .arg("--preprocessors")
        .arg(preprocessor)
        .arg("--preprocessor-output")
        .arg(preprocessor_combined_path)
        .arg("--output")
        .arg(output_file)
        .arg("--template-type")
        .arg("custom-preprocessors")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("pre-processors")
        .arg("add")
        .arg(preprocessor)
        .arg("--")
        .arg(preprocessor_args)
        .assert()
        .success();
}

fn create_input_file(project_path: &Path, filename: &str, content: &str) {
    let markdown_dir = project_path.join("Markdown");
    let markdown_path = markdown_dir.join(filename);
    fs::create_dir_all(&markdown_dir).expect("Failed to create markdown directory");
    fs::write(markdown_path, content).expect("Failed to write markdown file");
}

#[rstest]
fn test_convert() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_with_multiple_templates() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);
    add_tex_template(
        &project_path,
        "Template 2",
        "templ2.tex",
        Some("custom_out.pdf"),
    );
    add_typst_template(&project_path, "Template 3", "templ3.typ", None);
    add_epub_template(
        &project_path,
        "Epub Template",
        "epub_template",
        Some("custom_epub_out.epub"),
    );
    add_custom_preprocessors_template(
        &project_path,
        "Template 4",
        "RTF Preprocessor",
        "-t rtf -o output.rtf",
        "output.rtf",
        "output.rtf",
    );

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_files = vec![
        project_path.join("templ1.pdf"),
        project_path.join("custom_out.pdf"),
        project_path.join("templ3.pdf"),
        project_path.join("custom_epub_out.epub"),
        project_path.join("output.rtf"),
    ];

    for output_file in output_files {
        assert!(
            output_file.exists(),
            "Output file {} should exist",
            output_file.display()
        );
    }
}

#[rstest]
#[case("Template 1", "templ1.pdf", vec!["custom_out.pdf", "templ3.pdf"])]
#[case("Template 2", "custom_out.pdf", vec!["templ1.pdf", "templ3.pdf"])]
#[case("Template 3", "templ3.pdf", vec!["templ1.pdf", "custom_out.pdf"])]
fn test_convert_specific_template(
    #[case] template_name: &str,
    #[case] output_file: &str,
    #[case] non_convertered_template_output_files: Vec<&str>,
) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);
    add_tex_template(
        &project_path,
        "Template 2",
        "templ2.tex",
        Some("custom_out.pdf"),
    );
    add_typst_template(&project_path, "Template 3", "templ3.typ", None);

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .arg("--templates")
        .arg(template_name)
        .assert()
        .success();

    let output_pdf = project_path.join(output_file);
    assert!(output_pdf.exists(), "Output PDF should exist");

    for file in non_convertered_template_output_files {
        let non_converted_output_pdf = project_path.join(file);
        assert!(
            !non_converted_output_pdf.exists(),
            "Non-converted output PDF should not exist"
        );
    }
}

#[rstest]
#[case("project")]
#[case("project/")]
#[case("./project")]
#[case("./project/")]
#[case("./project/../project")]
#[case("./project/../project/")]
fn test_convert_specific_project_folder(#[case] project_path_name: &str) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);
    add_tex_template(
        &project_path,
        "Template 2",
        "templ2.tex",
        Some("custom_out.pdf"),
    );
    add_typst_template(&project_path, "Template 3", "templ3.typ", None);
    add_epub_template(
        &project_path,
        "Epub Template",
        "epub_template",
        Some("custom_epub_out.epub"),
    );
    add_custom_preprocessors_template(
        &project_path,
        "Template 4",
        "RTF Preprocessor",
        "-t rtf -o output.rtf",
        "output.rtf",
        "output.rtf",
    );

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&temp_dir)
        .arg("convert")
        .arg("--project")
        .arg(project_path_name)
        .assert()
        .success();

    let output_files = vec![
        project_path.join("templ1.pdf"),
        project_path.join("custom_out.pdf"),
        project_path.join("templ3.pdf"),
        project_path.join("custom_epub_out.epub"),
        project_path.join("output.rtf"),
    ];

    for output_file in output_files {
        assert!(
            output_file.exists(),
            "Output file {} should exist",
            output_file.display()
        );
    }
}

#[rstest]
fn test_convert_epub() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_epub_template(&project_path, "Epub Template", "epub_template", None);

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_epub = project_path.join("epub_template.epub");
    assert!(output_epub.exists(), "Output EPUB should exist");
}

#[rstest]
fn test_convert_giant_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    let content = include_str!("testdata/large_document_markdown.md");
    for i in 0..5 {
        create_input_file(&project_path, &format!("Chapter {}.md", i), content);
    }

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_many_files() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    for i in 1..=1000 {
        create_input_file(
            &project_path,
            format!("Chapter {}.md", i).as_str(),
            VALID_MARKDOWN_CONTENT,
        );
    }

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_far_nested_markdown_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    let project_path = &project_path;
    let markdown_dir = project_path.join("Markdown");
    let mut current_markdown_path = markdown_dir.clone();
    for _ in 1..=5 {
        current_markdown_path = current_markdown_path.join("a");
        let markdown_path = current_markdown_path.join("Chapter 1.md");
        fs::create_dir_all(&current_markdown_path).expect("Failed to create markdown directory");
        fs::write(markdown_path, VALID_MARKDOWN_CONTENT).expect("Failed to write markdown file");
    }

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_long_markdown_file_name() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    let long_markdown_file_name =
        "Chapter 0001 - This is a very long chapter name and might cause issues.md";
    create_input_file(
        &project_path,
        long_markdown_file_name,
        VALID_MARKDOWN_CONTENT,
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_no_markdown_files() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    fs::create_dir_all(project_path.join("Markdown")).expect("Failed to create Markdown directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_custom_pandoc_conversion() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_custom_preprocessors_template(
        &project_path,
        "Template 1",
        "RTF Preprocessor",
        "-t rtf -o output.rtf",
        "output.rtf",
        "output.rtf",
    );

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_rtf = project_path.join("output.rtf");
    assert!(output_rtf.exists(), "Output RTF should exist");
}

fn get_conversion_folders(project_path: &Path) -> Vec<PathBuf> {
    let regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}$")
        .expect("Failed to compile regex");
    let conversion_folders = fs::read_dir(project_path)
        .expect("Failed to read project directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir()
                && regex.is_match(
                    path.file_name()
                        .expect("Failed to get file name")
                        .to_str()
                        .expect("Failed to convert file name to string"),
                )
            {
                Some(path)
            } else {
                None
            }
        });

    conversion_folders.collect()
}

#[rstest]
fn test_convert_smart_clean() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(
        &temp_dir.path(),
        vec!["--smart-clean", "--smart-clean-threshold", "2"],
    );

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    for _ in 1..5 {
        let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
        cmd.current_dir(&project_path)
            .arg("convert")
            .assert()
            .success();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    let conversion_folders = get_conversion_folders(&project_path);
    assert_eq!(conversion_folders.len(), 2);

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_profile() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);
    add_tex_template(&project_path, "Template 2", "templ2.tex", None);
    add_tex_template(&project_path, "Template 3", "templ3.typ", None);

    add_profile(&project_path, "Profile 1", vec!["Template 1", "Template 3"]);

    create_input_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");

    cmd.current_dir(&project_path)
        .arg("convert")
        .arg("--profile")
        .arg("Profile 1")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");

    let output_pdf_2 = project_path.join("templ2.pdf");
    assert!(!output_pdf_2.exists(), "Output PDF should not exist");

    let output_pdf_3 = project_path.join("templ3.pdf");
    assert!(output_pdf_3.exists(), "Output PDF should exist");
}

fn add_profile(project_path: &Path, profile_name: &str, templates: Vec<&str>) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("profiles")
        .arg("add")
        .arg(profile_name)
        .arg(templates.join(","))
        .assert()
        .success();
}

#[rstest]
fn test_convert_multiple_markdown_projects() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);
    add_tex_template(&project_path, "Template 2", "templ2.tex", None);

    add_markdown_project(&project_path, "Project 1", "markdown_dir1", "out1");
    add_markdown_project(&project_path, "Project 2", "markdown_dir2", "out2");

    create_input_file(
        &project_path.join("markdown_dir1"),
        "test.md",
        VALID_MARKDOWN_CONTENT,
    );
    create_input_file(
        &project_path.join("markdown_dir2"),
        "test.md",
        VALID_MARKDOWN_CONTENT,
    );

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");

    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf_1 = project_path.join("out1").join("templ1.pdf");
    assert!(output_pdf_1.exists(), "Output PDF should exist");

    let output_pdf_2 = project_path.join("out1").join("templ2.pdf");
    assert!(output_pdf_2.exists(), "Output PDF should exist");

    let output_pdf_3 = project_path.join("out2").join("templ1.pdf");
    assert!(output_pdf_3.exists(), "Output PDF should exist");

    let output_pdf_4 = project_path.join("out2").join("templ2.pdf");
    assert!(output_pdf_4.exists(), "Output PDF should exist");
}

fn add_markdown_project(
    project_path: &Path,
    project_name: &str,
    markdown_dir: &str,
    output_dir: &str,
) {
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("project")
        .arg("markdown")
        .arg("add")
        .arg(project_name)
        .arg(markdown_dir)
        .arg(output_dir)
        .assert()
        .success();
}

#[rstest]
fn test_convert_mixed_input_formats() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path(), vec![]);

    add_custom_preprocessors_template(
        &project_path,
        "Template 1",
        "test_preprocessor",
        "-t markdown",
        "test.md",
        "test.md",
    );

    create_input_file(&project_path, "Chapter 2.md", VALID_MARKDOWN_CONTENT);
    create_input_file(&project_path, "Chapter 3.html", VALID_HTML_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output = project_path.join("test.md");
    assert!(output.exists(), "Output should exist");

    let output_content = fs::read_to_string(&output).expect("Failed to read output file");

    assert_contains!(
        output_content,
        r#"# Chapter 1

Basic test content


# Chapter 2

Basic test content 02"#
    );
}
