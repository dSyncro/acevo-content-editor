<div align="center">

# Assetto Corsa Evo Content Editor
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/dSyncro/acevo-content-editor/blob/main/README.md) 
![Version](https://img.shields.io/badge/version-0.1.5-green) 
[![Build Target](https://img.shields.io/badge/Assetto%20Corsa%20Evo%20Build-0x312E30-red)](https://steamdb.info/depot/3058631/history/?changeid=M:2107359667722066372) [![Steam](https://img.shields.io/badge/steam-%23000000.svg?&logo=steam&logoColor=white)](https://store.steampowered.com/app/3058630)

A utility tool to quickly unpack Assetto Corsa Evo packed content packages (*.kspkg).

</div>

## 📖 Table of Contents

- [Dependencies and requirements](#dependencies-and-requirements)
- [Getting started](#getting-started)
- [Usage](#usage)
    - [Command `list`](#command-list)
    - [Command `unpack`](#command-unpack)
- [Acknowledgement](#acknowledgement)
- [Side notes](#side-notes)

## Dependencies and requirements

The requirements to build the project are:
- A [rust compiler](https://www.rust-lang.org/) supporting the edition `2021` of the language.
- Any OS supported by the base game

Keep in mind the project can potentially build and work on many other operating systems out of the box, however it was only tested on versions of Windows also supported by the game.

## Getting Started

To get started just clone the project:

```bash
git clone https://github.com/dSyncro/acevo-content-editor
```

Compile it:

```bash
cargo build             # Debug build
cargo build --release   # Release build
```

And finally run the compiled executable. Usually in `target/{debug|release}/acevo_content_editor.exe`.

```bash
acevo_content_editor.exe --help
```

## Usage
```bash
A utility tool to quickly unpack Assetto Corsa Evo packed content packages (*.kspkg).

Usage: acevo_content_editor.exe [OPTIONS] <COMMAND>

Commands:
  list    List content from package
  unpack  Unpack content from package
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...
          Verbosity level (can be specified multiple times)
  -c, --content-path <CONTENT_PATH>
          The path of the content package [default: content.kspkg]
  -o, --content-output <CONTENT_OUTPUT>
          The path where to extract content data [default: ./]
  -h, --help
          Print help
  -V, --version
          Print version
```

### Command `list`

```bash
List content from package

Usage: acevo_content_editor.exe list [OPTIONS] [GLOB]

Arguments:
  [GLOB]  Pattern of the elements to look for [default: *]

Options:
  -v, --verbose...
          Verbosity level (can be specified multiple times)
  -c, --content-path <CONTENT_PATH>
          The path of the content package [default: content.kspkg]
  -o, --content-output <CONTENT_OUTPUT>
          The path where to extract content data  [default: ./]
  -h, --help
          Print help
```

### Command `unpack`

```bash
Unpack content from package

Usage: acevo_content_editor.exe unpack [OPTIONS] <GLOB>

Arguments:
  <GLOB>  Pattern of the elements to look for

Options:
  -F, --force <FORCE>
          Pattern of elements to force extract, even if they are already present
  -v, --verbose...
          Verbosity level (can be specified multiple times)
  -c, --content-path <CONTENT_PATH>
          The path of the content package [default: content.kspkg]
  -o, --content-output <CONTENT_OUTPUT>
          The path where to extract content data [default: ./]
  -h, --help
          Print help
```
## Acknowledgement

This project would not have been possible without inspiration from these awesome projects:

- [ace-kspkg](https://github.com/ntpopgetdope/ace-kspkg) (Python)
- [kspkg-viewer](https://github.com/sa413x/kspkg-viewer) (C++)
- [ACEvo.Package](https://github.com/Nenkai/ACEvo.Package) (C#)

Please check them out and give the appropriate credit to them.

## Side notes

Please keep in mind that at the moment this is a side project developed with no planned continuity nor schedule. Therefore *support, fixes and new features can not be guaranteed*.

As stated in the [LICENSE](https://github.com/dSyncro/acevo-content-editor/blob/main/LICENSE), *no contributor must be considered liable for the use of this project*.
