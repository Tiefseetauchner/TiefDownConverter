## `tiefdownconverter project update-manifest` {#projectupdate-manifest}

**Version:** `tiefdownconverter 0.7.0`

### Usage:
```
Update the project manifest.

Usage: tiefdownconverter project update-manifest [OPTIONS]

Options:
  -m, --markdown-dir <MARKDOWN_DIR>
          The directory where the Markdown files are located.

      --smart-clean <SMART_CLEAN>
          Enables smart clean for the project with a default threshold of 5.
          If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders.
          
          [possible values: true, false]

      --smart-clean-threshold <SMART_CLEAN_THRESHOLD>
          The threshold for smart clean. If not provided, the default threshold of 5 will be used.
          If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders.

  -h, --help
          Print help (see a summary with '-h')
```

