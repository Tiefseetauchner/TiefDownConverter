# Injections

Injections are a project-driven way to insert input files at either the top,
inside, or bottom of a document, correspondingly named header, body and footer
injections.

Injections serve as a template scoped way to add content to a conversion. An
injection is defined once in the manifest and then assigned to a template as
any of the aforementioned methods.

During conversion, the injected files are resolved and placed in the list of
input files to be converted to the intermediary format.

## Header and footer injections

Header injections are inserted at the top of the document, while footer 
injections are inserted at the bottom, both in the order as
defined in the manifest. The first file defined in the first referenced
injection is placed first, the last file of the last referenced injection
last.

## Body injections

Body injections are injected before the main sorting algorithm as any
file in the input directory would be. That means they get sorted in accordance
with the primary sorting algorithm.