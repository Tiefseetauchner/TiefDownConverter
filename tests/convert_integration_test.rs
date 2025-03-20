use assert_cmd::Command;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::tempdir;

const VALID_TEMPLATE_CONTENT_TEX: &str = r#"\documentclass[a4paper,12pt]{article}

\begin{document}

This is a test document, and this text is needed as otherwise the document would be empty and no pages would be generated.

\input{./output.tex}

\end{document}"#;

const VALID_TEMPLATE_CONTENT_TYP: &str = r#"#include "output.typ""#;

const VALID_MARKDOWN_CONTENT: &str = r#"# Chapter 1
Basic test content"#;

fn create_empty_project(temp_dir: &Path) -> PathBuf {
    let project_path = temp_dir.join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");
    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("init")
        .arg("-n")
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
        .arg("add-template")
        .arg(template_name)
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

fn create_markdown_file(project_path: &Path, filename: &str, content: &str) {
    let markdown_dir = project_path.join("Markdown");
    let markdown_path = markdown_dir.join(filename);
    fs::create_dir_all(&markdown_dir).expect("Failed to create markdown directory");
    fs::write(markdown_path, content).expect("Failed to write markdown file");
}

#[rstest]
fn test_convert() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_empty_project(&temp_dir.path());

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

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

    let project_path = create_empty_project(&temp_dir.path());

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

    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

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

    let project_path = create_empty_project(&temp_dir.path());

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);
    add_tex_template(
        &project_path,
        "Template 2",
        "templ2.tex",
        Some("custom_out.pdf"),
    );
    add_typst_template(&project_path, "Template 3", "templ3.typ", None);

    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

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

    let project_path = create_empty_project(&temp_dir.path());

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

    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

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

    let project_path = create_empty_project(&temp_dir.path());

    add_epub_template(&project_path, "Epub Template", "epub_template", None);

    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

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

    let project_path = create_empty_project(&temp_dir.path());

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    let content = include_str!("testdata/large_document_markdown.md");
    for i in 0..5 {
        create_markdown_file(&project_path, &format!("Chapter {}.md", i), content);
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

    let project_path = create_empty_project(&temp_dir.path());

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    for i in 1..=1000 {
        create_markdown_file(
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

    let project_path = create_empty_project(&temp_dir.path());

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

    let project_path = create_empty_project(&temp_dir.path());

    add_tex_template(&project_path, "Template 1", "templ1.tex", None);

    let long_markdown_file_name =
        "Chapter 0001 - This is a very long chapter name and might cause issues.md";
    create_markdown_file(
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

    let project_path = create_empty_project(&temp_dir.path());

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

// #[rstest]
// fn test_convert_custom_pandoc_conversion() {
//     let temp_dir = tempdir().expect("Failed to create temporary directory");
// }
