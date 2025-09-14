use rstest::rstest;

use crate::manifest_model::{
    upgrade_manifest_v0_to_v1, upgrade_manifest_v1_to_v2, upgrade_manifest_v2_to_v3,
    upgrade_manifest_v3_to_v4, upgrade_manifest_v4_to_v5,
};

#[rstest]
fn test_upgrade_manifest_v0_to_v1() {
    let manifest_content = r#"
markdown_dir = "Custom Markdown Directory"
templates = ["template1.tex", "template2.typ"]"#;
    let mut manifest = toml::from_str(manifest_content).unwrap();

    let result = upgrade_manifest_v0_to_v1(&mut manifest);

    assert!(result.is_ok());

    let expected_manifest = r#"markdown_dir = "Custom Markdown Directory"
version = 1

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let actual_manifest = toml::to_string(&manifest).unwrap();
    assert_eq!(expected_manifest, actual_manifest);
}

#[rstest]
fn test_upgrade_manifest_v1_to_v2() {
    let manifest_content = r#"
markdown_dir = "Custom Markdown Directory"
version = 1

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let mut manifest = toml::from_str(manifest_content).unwrap();
    let result = upgrade_manifest_v1_to_v2(&mut manifest);
    assert!(result.is_ok());

    let expected_manifest = r#"markdown_dir = "Custom Markdown Directory"
version = 2

[custom_processors]
preprocessors = []

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let actual_manifest = toml::to_string(&manifest).unwrap();
    assert_eq!(expected_manifest, actual_manifest);
}

#[rstest]
fn test_upgrade_manifest_v2_to_v3() {
    let manifest_content = r#"markdown_dir = "Custom Markdown Directory"
version = 2

[custom_processors]
preprocessors = []

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let mut manifest = toml::from_str(manifest_content).unwrap();
    let result = upgrade_manifest_v2_to_v3(&mut manifest);
    assert!(result.is_ok());

    let expected_manifest = r#"markdown_dir = "Custom Markdown Directory"
version = 3

[custom_processors]
preprocessors = []
processors = []

[metadata_fields]

[metadata_settings]

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let actual_manifest = toml::to_string(&manifest).unwrap();
    assert_eq!(expected_manifest, actual_manifest);
}

#[rstest]
fn test_upgrade_manifest_v3_to_v4() {
    let manifest_content = r#"markdown_dir = "Custom Markdown Directory"
version = 3

[custom_processors]
preprocessors = []
processors = []

[metadata_fields]
author = "Author Name"
title = "Document Title"

[metadata_settings]
metadata_prefix = "supermeta"

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let mut manifest = toml::from_str(manifest_content).unwrap();
    let result = upgrade_manifest_v3_to_v4(&mut manifest);
    assert!(result.is_ok());

    let expected_manifest = r#"version = 4

[custom_processors]
preprocessors = []
processors = []

[[markdown_projects]]
name = "Custom Markdown Directory"
output = "."
path = "Custom Markdown Directory"

[metadata_settings]
metadata_prefix = "supermeta"

[shared_metadata]
author = "Author Name"
title = "Document Title"

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

    let actual_manifest = toml::to_string(&manifest).unwrap();
    assert_eq!(expected_manifest, actual_manifest);
}

#[rstest]
fn test_upgrade_manifest_v4_to_v5() {
    let manifest_content = r#"version = 4

[custom_processors]
processors = []

[[custom_processors.preprocessors]]
name = "custom preprocessor"
pandoc_args = ["-o", "combined.tex", "-t", "tex", "--listings"]

[[markdown_projects]]
name = "Custom Markdown Directory"
output = "."
path = "Custom Markdown Directory"

[metadata_settings]
metadata_prefix = "supermeta"

[shared_metadata]
author = "Author Name"
title = "Document Title"

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"

[[templates]]
name = "template3"
preprocessor = "custom preprocessor"
template_type = "CustomPandoc"
"#;

    let mut manifest = toml::from_str(manifest_content).unwrap();
    let result = upgrade_manifest_v4_to_v5(&mut manifest);
    assert!(result.is_ok());

    let expected_manifest = r#"version = 5

[custom_processors]
processors = []

[[custom_processors.preprocessors]]
cli_args = ["-t", "tex", "--listings"]
name = "custom preprocessor"

[[markdown_projects]]
name = "Custom Markdown Directory"
output = "."
path = "Custom Markdown Directory"

[metadata_settings]
metadata_prefix = "supermeta"

[shared_metadata]
author = "Author Name"
title = "Document Title"

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"

[[templates]]
name = "template3"
preprocessor = "custom preprocessor"
template_type = "CustomPreprocessors"

[templates.preprocessors]
combined_output = "combined.tex"
preprocessors = ["custom preprocessor"]
"#;

    let actual_manifest = toml::to_string(&manifest).unwrap();
    assert_eq!(expected_manifest, actual_manifest);
}
