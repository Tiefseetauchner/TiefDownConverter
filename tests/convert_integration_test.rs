use assert_cmd::Command;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
};
use tempfile::{TempDir, tempdir};

const VALID_MANIFEST_CONTENT_ONE_TEMPLATE: &str = r#"version = 1

[[templates]]
name = "Template for Testing"
output = "templ1.pdf"
template_file = "templ1.tex"
template_type = "Tex"
"#;

const VALID_MANIFEST_CONTENT_THREE_TEMPLATES: &str = r#"version = 1

[[templates]]
name = "Template for Testing"
template_file = "templ1.tex"
template_type = "Tex"

[[templates]]
name = "Template for Testing 2"
output = "custom_out.pdf"
template_file = "templ2.tex"
template_type = "Tex"

[[templates]]
name = "Template for Testing 3"
output = "templ3.pdf"
template_file = "templ3.typ"
template_type = "Typst"
"#;

const VALID_MANIFEST_CONTENT_EPUB_TEMPLATE: &str = r#"version = 1

[[templates]]
name = "Template for Testing"
output = "templ1.epub"
template_file = "template_epub"
template_type = "Epub"
"#;

const VALID_TEMPLATE_CONTENT_TEX: &str = r#"\documentclass[a4paper,12pt]{article}

\begin{document}

This is a test document, and this text is needed as otherwise the document would be empty and no pages would be generated.

\input{./output.tex}

\end{document}"#;

const VALID_TEMPLATE_CONTENT_TYP: &str = r#"#include "output.typ""#;

const VALID_MARKDOWN_CONTENT: &str = r#"# Chapter 1
Basic test content"#;

fn create_project_dir(temp_dir: &TempDir) -> PathBuf {
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).expect("Failed to create project directory");

    project_path
}

fn create_manifest_file(project_path: &Path, manifest_content: &str) {
    let manifest_path = project_path.join("manifest.toml");
    fs::write(manifest_path, manifest_content).expect("Failed to write manifest file");
}

fn create_template(project_path: &Path, template_name: &str, template_content: &str) {
    let template_dir = project_path.join("template");
    let template_path = template_dir.join(template_name);
    fs::create_dir_all(&template_dir).expect("Failed to create template directory");
    fs::write(template_path, template_content).expect("Failed to write template file");
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

    let project_path = create_project_dir(&temp_dir);

    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);

    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);

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

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_THREE_TEMPLATES);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
    create_template(&project_path, "templ2.tex", VALID_TEMPLATE_CONTENT_TEX);
    create_template(&project_path, "templ3.typ", VALID_TEMPLATE_CONTENT_TYP);
    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdfs = vec![
        project_path.join("templ1.pdf"),
        project_path.join("custom_out.pdf"),
        project_path.join("templ3.pdf"),
    ];

    for output_pdf in output_pdfs {
        assert!(output_pdf.exists(), "Output PDF should exist");
    }
}

#[rstest]
#[case("Template for Testing", "templ1.pdf", vec!["custom_out.pdf", "templ3.pdf"])]
#[case("Template for Testing 2", "custom_out.pdf", vec!["templ1.pdf", "templ3.pdf"])]
#[case("Template for Testing 3", "templ3.pdf", vec!["templ1.pdf", "custom_out.pdf"])]
fn test_convert_specific_template(
    #[case] template_name: &str,
    #[case] output_file: &str,
    #[case] non_convertered_template_output_files: Vec<&str>,
) {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_THREE_TEMPLATES);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
    create_template(&project_path, "templ2.tex", VALID_TEMPLATE_CONTENT_TEX);
    create_template(&project_path, "templ3.typ", VALID_TEMPLATE_CONTENT_TYP);
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

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&temp_dir)
        .arg("convert")
        .arg("--project")
        .arg(project_path_name)
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}

#[rstest]
fn test_convert_epub() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_EPUB_TEMPLATE);
    fs::create_dir_all(project_path.join("template").join("template_epub"))
        .expect("Failed to create template directory");
    create_markdown_file(&project_path, "Chapter 1.md", VALID_MARKDOWN_CONTENT);

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_epub = project_path.join("templ1.epub");
    assert!(output_epub.exists(), "Output EPUB should exist");
}

#[rstest]
fn test_convert_giant_file() {
    let temp_dir = tempdir().expect("Failed to create temporary directory");

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
    let content = include_str!("testdata/large_document_markdown.md");
    create_markdown_file(&project_path, "Chapter 1.md", content);
    create_markdown_file(&project_path, "Chapter 2.md", content);
    create_markdown_file(&project_path, "Chapter 3.md", content);
    create_markdown_file(&project_path, "Chapter 4.md", content);
    create_markdown_file(&project_path, "Chapter 5.md", content);
    create_markdown_file(&project_path, "Chapter 6.md", content);
    create_markdown_file(&project_path, "Chapter 7.md", content);
    create_markdown_file(&project_path, "Chapter 8.md", content);
    create_markdown_file(&project_path, "Chapter 9.md", content);
    create_markdown_file(&project_path, "Chapter 10.md", content);
    create_markdown_file(&project_path, "Chapter 11.md", content);
    create_markdown_file(&project_path, "Chapter 12.md", content);
    create_markdown_file(&project_path, "Chapter 13.md", content);
    create_markdown_file(&project_path, "Chapter 14.md", content);
    create_markdown_file(&project_path, "Chapter 15.md", content);

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

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
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

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);

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

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
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

    let project_path = create_project_dir(&temp_dir);
    create_manifest_file(&project_path, VALID_MANIFEST_CONTENT_ONE_TEMPLATE);
    create_template(&project_path, "templ1.tex", VALID_TEMPLATE_CONTENT_TEX);
    fs::create_dir_all(project_path.join("Markdown")).expect("Failed to create Markdown directory");

    let mut cmd = Command::cargo_bin("tiefdownconverter").expect("Failed to get cargo binary");
    cmd.current_dir(&project_path)
        .arg("convert")
        .assert()
        .success();

    let output_pdf = project_path.join("templ1.pdf");
    assert!(output_pdf.exists(), "Output PDF should exist");
}
