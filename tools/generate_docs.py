#!/usr/bin/env python3

import os
import subprocess
import re
import textwrap

# CLI executable path (adjust if necessary)
CLI_EXEC = "tiefdownconverter"

# Output directory
DOCS_DIR = "docs/Markdown/Chapter 3 - Usage Details/"
os.makedirs(DOCS_DIR, exist_ok=True)

chapter_counter = 1
MAX_LINE_LENGTH = 75  # Wrap long lines at this length

def extract_help(command):
    """Runs the command with --help and extracts output."""
    try:
        result = subprocess.run([*command, "--help"], capture_output=True, text=True, check=True)
        return result.stdout
    except subprocess.CalledProcessError:
        print(f"Warning: Failed to get help for {' '.join(command)}")
        return ""

def extract_version():
    """Gets the version of the CLI tool."""
    try:
        result = subprocess.run([CLI_EXEC, "--version"], capture_output=True, text=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError:
        return "Unknown"

def parse_subcommands(help_text):
    """Extracts subcommands from the help text using regex."""
    subcommands = []
    for line in help_text.splitlines():
        match = re.match(r"^\s{2}([\w-]+)\s{2,}.*$", line)
        if match and match.group(1) != "help":
            subcommands.append(match.group(1))
    return subcommands

def wrap_text(text, indent=4):
    """Wraps text for LaTeX-friendly output, indenting continuation lines."""
    return textwrap.fill(text, width=MAX_LINE_LENGTH, subsequent_indent=" " * indent)

def generate_markdown(command, subcommand_chain=""):
    """Generates Markdown for the given command and its subcommands recursively."""
    global chapter_counter

    chapter_number = chapter_counter
    chapter_counter += 1

    filename = os.path.join(DOCS_DIR, f"Chapter {chapter_number} - {subcommand_chain.replace(' ', '-') or 'main'}.md")

    help_text = extract_help(command)
    version = extract_version()
    subcommands = parse_subcommands(help_text)

    # Write Markdown documentation
    with open(filename, "w", encoding="utf-8") as f:
        f.write(f"## `{CLI_EXEC} {subcommand_chain}` {{#{subcommand_chain.replace(' ', '') or 'main'}}}\n\n")
        f.write(f"**Version:** `{version}`\n\n")
        f.write("### Usage:\n```\n")

        # Wrap and format the help output
        for line in help_text.splitlines():
            f.write(wrap_text(line) + "\n")

        f.write("```\n\n")

        if subcommands:
            f.write("### Subcommands:\n")
            for subcmd in subcommands:
                f.write(f"- [{subcmd}](#{subcommand_chain.replace(' ', '')}{subcmd})\n")
            f.write("\n")

    print(f"Generated: {filename}")

    # Recursively generate docs for each subcommand
    for subcmd in subcommands:
        generate_markdown(command + [subcmd], f"{subcommand_chain} {subcmd}".strip())

if __name__ == "__main__":
    generate_markdown([CLI_EXEC])
    print(f"Markdown documentation generated in `{DOCS_DIR}`!")
