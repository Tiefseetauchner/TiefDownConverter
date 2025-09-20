## tiefdownconverter convert

```
Convert a TiefDown project. By default, it will convert the current directory.

Usage: tiefdownconverter convert [OPTIONS]

Options:
  -p, --project <PROJECT>
          The project to convert. If not provided, the current directory will be used.
  -v, --verbose
          Enable verbose output.
  -t, --templates <TEMPLATES>...
          The templates to use. If not provided, the default templates from the manifest file will be used. Cannot be used with --profile.
  -P, --profile <PROFILE>
          The conversion profile to use. Cannot be used with --templates.
  -m, --markdown-projects <MARKDOWN_PROJECTS>...
          The markdown projects to convert. If not provided, all markdown projects will be converted.
  -h, --help
          Print help
```

