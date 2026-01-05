# Injections

Handling large projects with multiple templates can sometimes lead to a lot of template specific code. To enable template specific output without a template file (e.g. EPUB, HTML), injections enable you to insert an input file at any point of the conversion process.

## Header and Footer Injections

Header and footer injections act as insertions before and after the content respectively. That means that, after the content is preprocessed, it gets pre-/appended with the preprocessed injection.

An example usecase for injections is adding an HTML scaffolding around the content. The header injection would hold the doctype declaration and head tags, while the footer could hold an HTML footer for displaying copyright. Injections are especially useful for [multi-file output](#multi-file-output-model).

## Body Injections

A body injection is treated significantly different to header or footer injections. In case of a body injection, it gets inserted into the body according to the sorting algorithm *before* the preprocessing step.

For example, if only a certain template should include something in the body, e.g. after the introduction, the body injection can be set up to be inserted there. Since the sorting rules are applied, we can use this example to illustrate:

A introduction is named `0 - Intro.md` and an injection is set up as `1 - Injection.md`. Let's also assume another input file named `2 - Usage.md`. The sorting rules mean, that the order is extracted as follows:

`0 - Intro.md` -> 0\
`2 - Usage.md` -> 2\
`1 - Injection.md` -> 1

Since the file  gets injected into the body stream, it is then sorted in the same manner. Thus, the order of files is

1. `0 - Intro.md` 
2. `1 - Injection.md`
3. `2 - Usage.md`

Importantly, the injection is grouped with the other files during preprocessing, enabling merging during the conversion process.