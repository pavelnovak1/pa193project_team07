# Security Certificate Parser Project

## Introduction

This project is an implementation of an Security Certificate Parser. This parser is able to extract important entries from the certificate and store them in a JSON format into the specified file (see more below). The implementation is done in the Rust language.

## Prerequisites

  - ```Rust``` language compiler
  - ```Regex``` and ```serde_json``` modules ( Already present as dependencies in ```Cargo.toml``` )
  - ```Python 3``` interpreter ( If you want to use automatic compare script )

## Project Structure

  - ```src/``` - Source code directory
      - ```main.rs``` - Main entry point
      - ```biblio.rs``` - Bibliography parser
      - ```cert_info.rs``` - Data structures
      - ```extract_info.rs``` - Module responsible for parsing and collecting all data
      - ```revision.rs``` - Revisions parser
      - ```table_of_contents.rs``` - TOC parser
      - ```title.rs``` - Title parser
      - ```tools.rs``` - Helper tools
      - ```versions.rs``` - Versions parser
      - ```write_info.rs``` - Module responsible for creating JSON and storing results into the file
  - ```test_dataset/``` - Directory with test certificates
      - ```output_compare.py``` - Compare script
      - ```*.txt *.json``` - Test certificates
  - ```testing/``` - Directory for testing purposes
      - ```output*.txt``` - Separate outputs for parsers
      - ```test_all.sh``` - Script that run application with all certificates and compare them with test certificates

## How to use

For running our tool, we are using rust cargo tool. Running it is pretty simple, just type:

```cargo run <output_file.txt> <input_file.txt>```

for example
```bash
cargo run output.json certificate.txt
```

Results are stored as a JSON file in the specified output file.
In the case of any error, an application is immediatelly terminated and you can see the description of an error.
