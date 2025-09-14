use std::str::FromStr;

use rstest::rstest;

use crate::template_type::TemplateType;

#[rstest]
#[case("tex", TemplateType::Tex)]
#[case("typst", TemplateType::Typst)]
#[case("epub", TemplateType::Epub)]
#[case("custompreprocessors", TemplateType::CustomPreprocessors)]
#[case("customprocessor", TemplateType::CustomProcessor)]
fn explicit_from_str(#[case] name: &str, #[case] expected_template_type: TemplateType) {
    let template_type = TemplateType::from_str(name).expect("from_str returned non-ok result");

    assert_eq!(template_type, expected_template_type);
}

#[rstest]
#[case("tex", TemplateType::Tex)]
#[case("typst", TemplateType::Typst)]
#[case("epub", TemplateType::Epub)]
#[case("custompreprocessors", TemplateType::CustomPreprocessors)]
#[case("customprocessor", TemplateType::CustomProcessor)]
fn from_str(#[case] name: &str, #[case] expected_template_type: TemplateType) {
    let template_type = TemplateType::from(name);

    assert_eq!(template_type, expected_template_type);
}

#[rstest]
#[case(0, TemplateType::Tex)]
#[case(1, TemplateType::Typst)]
#[case(2, TemplateType::Epub)]
#[case(3, TemplateType::CustomPreprocessors)]
#[case(4, TemplateType::CustomProcessor)]
fn from_usize(#[case] value: usize, #[case] expected_template_type: TemplateType) {
    let template_type = TemplateType::from(value);

    assert_eq!(template_type, expected_template_type);
}

#[rstest]
#[case(TemplateType::Tex, "Tex")]
#[case(TemplateType::Typst, "Typst")]
#[case(TemplateType::Epub, "Epub")]
#[case(TemplateType::CustomPreprocessors, "CustomPreprocessors")]
#[case(TemplateType::CustomProcessor, "CustomProcessor")]
fn as_str(#[case] value: TemplateType, #[case] expected_string: &str) {
    let template_name = value.as_str();

    assert_eq!(template_name, expected_string);
}
