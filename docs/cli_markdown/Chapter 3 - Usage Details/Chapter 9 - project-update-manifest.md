## `tiefdownconverter project update-manifest` {#projectupdate-manifest}

**Version:** `tiefdownconverter 0.8.0`

### Usage:

```
Update the project manifest.

Usage: tiefdownconverter project update-manifest [OPTIONS]

Options:
      --smart-clean <SMART_CLEAN>
          Enables smart clean for the project with a default threshold of 5.
          If the number of conversion folders in the project is above the smart_clean_threshold, old folders will be cleaned, leaving only the threshold amount of folders.

          [possible values: true, false]

  -v, --verbose
          Enable verbose output.

      --smart-clean-threshold <SMART_CLEAN_THRESHOLD>
          The threshold for smart clean. If not provided, the default threshold of 5 will be used.
          If the number of conversion folders in the project is above this threshold, old folders will be cleaned, leaving only the threshold amount of folders.

  -h, --help
          Print help (see a summary with '-h')
```
