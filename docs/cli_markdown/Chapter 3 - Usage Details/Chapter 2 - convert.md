## `tiefdownconverter convert` {#convert}

**Version:** `tiefdownconverter 0.9.0-ALPHA.1`

### Usage:
```
Convert a TiefDown project. By default, it will convert the current directory.

Usage: tiefdownconverter convert [OPTIONS]

Options:
  -p, --project <PROJECT>         The project to convert. If not provided, the current directory will be used.
  -v, --verbose                   Enable verbose output.
  -t, --templates <TEMPLATES>...  The templates to use. If not provided, the default templates from the manifest file will be used. Cannot be used with --profile.
  -P, --profile <PROFILE>         The conversion profile to use. Cannot be used with --templates.
  -h, --help                      Print help
```

