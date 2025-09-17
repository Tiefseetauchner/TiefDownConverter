# Tiefdown Project Support

VS Code integration for Tiefdown projects. The extension scans opened workspace folders for a `manifest.toml` and exposes a command to run `tiefdownconverter convert` directly from the editor.

## Features

- Detects Tiefdown projects automatically when a `manifest.toml` is present in any workspace folder
- Provides a `Tiefdown: Convert Project` command that runs the CLI and streams logs to a dedicated output channel
- Lets you choose between multiple Tiefdown projects in the same workspace
- Allows selecting specific templates before running a conversion (or run them all by skipping the selection)

## Requirements

- `tiefdownconverter` CLI must be installed and available in your system `PATH`
- VS Code 1.104.0 or later

## Usage

1. Open a folder that contains a Tiefdown `manifest.toml`
2. Run the `Tiefdown: Convert Project` command from the Command Palette
3. Pick the project you want to convert if more than one manifest is available
4. (Optional) Select which templates to convert; leave the picker empty to run every template
5. Watch conversion logs in the `Tiefdown Converter` output channel

## Contributing

Please file issues or pull requests in the main Tiefdown Converter repository.
