## tiefdownconverter project injections create

```
Creates a new injection.
Fails if an injection with that name already exists.

Usage: tiefdownconverter project injections create [OPTIONS] --name <NAME>

Options:
  -n, --name <NAME>
          The name of the injection to create.
          Must be unique per project.

  -v, --verbose
          Enable verbose output.

  -f, --files <FILES>...
          The files to be used for the injections.
          Can be a directory.
          The order of the files here defines the order for header and footer injections.
          For body injections, the files are ordered as per the default algorithm.
          Files in directories are ordered as per the default algorithm.
          Duplicate files will be added twice.

  -h, --help
          Print help (see a summary with '-h')
```

