use std::sync::LazyLock;

pub static POSSIBLE_TEMPLATES: LazyLock<Vec<&str>> = LazyLock::new(|| {
    let mut templates = Vec::new();
    templates.extend(POSSIBLE_TEX_TEMPLATES);
    templates.extend(POSSIBLE_TYPST_TEMPLATES);
    templates.extend(POSSIBLE_EPUB_TEMPLATES);
    templates
});

pub const POSSIBLE_TEX_TEMPLATES: [&str; 4] = [
    "template.tex",
    "booklet.tex",
    "lix_novel_a4.tex",
    "lix_novel_book.tex",
];

pub const POSSIBLE_TYPST_TEMPLATES: [&str; 1] = ["template_typ.typ"];

pub const POSSIBLE_EPUB_TEMPLATES: [&str; 1] = ["default_epub"];

pub const CURRENT_MANIFEST_VERSION: u32 = 3;
