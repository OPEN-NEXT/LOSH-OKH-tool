<!--
SPDX-FileCopyrightText: 2021 - 2023 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# LOSH OKH tool

[![License: AGPL-3.0-or-later](
    https://img.shields.io/badge/License-AGPL%203.0+-blue.svg)](
    LICENSE.txt)
[![REUSE status](
    https://api.reuse.software/badge/github.com/OPEN-NEXT/LOSH-OKH-tool)](
    https://api.reuse.software/info/github.com/OPEN-NEXT/LOSH-OKH-tool)
[![Repo](
    https://img.shields.io/badge/Repo-GitHub-555555&logo=github.svg)](
    https://github.com/OPEN-NEXT/LOSH-OKH-tool)
[![Package Releases](
    https://img.shields.io/crates/v/okh_tool.svg)](
    https://crates.io/crates/okh_tool)
[![Documentation Releases](
    https://docs.rs/okh_tool/badge.svg)](
    https://docs.rs/okh_tool)
[![Dependency Status](
    https://deps.rs/repo/github/OPEN-NEXT/LOSH-OKH-tool/status.svg)](
    https://deps.rs/repo/github/OPEN-NEXT/LOSH-OKH-tool)
[![Build Status](
    https://github.com/OPEN-NEXT/LOSH-OKH-tool/workflows/build/badge.svg)](
    https://github.com/OPEN-NEXT/LOSH-OKH-tool/actions)

[![In cooperation with FabCity Hamburg](
    https://raw.githubusercontent.com/osegermany/tiny-files/master/res/media/img/badge-fchh.svg)](
    https://fabcity.hamburg)
[![In cooperation with Open Source Ecology Germany](
    https://raw.githubusercontent.com/osegermany/tiny-files/master/res/media/img/badge-oseg.svg)](
    https://opensourceecology.de)

A CLI tool to deal with OKH data files.
Its main functionalities are:

- validation of, and
- conversion between

different formats of OKH

## Definitions

* LOSH: _A **L**ibrary of **O**pen **S**ource **H**ardware -
  technical documentation in an open graph database._
* OKH: _**O**pen **K**now-**H**ow - A standard for OSH project meta-data._

## Intro

This repo contains two pieces of software:

* `okh-tool`
* `fetch-n-conv-v1`

and it uses the [OKH JSON-Schemas repo](
https://github.com/OPEN-NEXT/LOSH-OKH-JSON-Schemas)
as submodule.

* [okh-losh.schema.json](
  https://github.com/OPEN-NEXT/LOSH-OKH-JSON-Schemas/blob/master/okh-losh.schema.json)
* [okh-v1.schema.json](
  https://github.com/OPEN-NEXT/LOSH-OKH-JSON-Schemas/blob/master/okh-v1.schema.json)

The `okh-tool` can:

* convert manifest files from one version of the OKH standard to an other
* validate manifest files of the different OKH standard versions.
  It does so by using the above mentioned JSON Schema files.

The `fetch-n-conv-v1` script will:

1. fetch publicly registered OKH v1 manifest files
2. clean them up
3. validate them
4. convert them to OKH LOSH (aka "v2")
5. validate the generated OKH LOSH manifest files

## Usage

### Building and Running

The `okh-tool` is written in rust, and can be compiled like this:

```bash
cargo build --release
```

This will produce the stand-alone binary tool in `target/release/okh-tool`.

To run it, you can either use the binary you just built,
or run it directly from the sources with:

```bash
cargo run --
```

#### Conversion

You have these options for conversion:

```bash
$ okh-tool conv --help
okh-tool-conv 0.1.0



Converts one format into an other (currently only OKH-v1 to OKH-LOSH)

USAGE:
    okh-tool conv [OPTIONS] <INPUT> [OUTPUT]

ARGS:
    <INPUT>     The input file or dir path
    <OUTPUT>    The output file or dir path

OPTIONS:
    -c, --continue     If the input path is a directory, continue processing further files, even
                       after an error
    -h, --help         Print help information
    -o, --overwrite    If the outout file alreayd exists, overwrite it, instead of skipping the
                       conversion
    -r, --recursive    If the input path is a directory, search for input files recursively
    -V, --version      Print version information
```

#### Validation

You have these options for validation:

```bash
$ okh-tool val --help
okh-tool-val 0.1.0



Validates manifest files for validity using JSON Schema (currently supports OKH-v1 and OKH-LOSH)

USAGE:
    okh-tool val [OPTIONS] <INPUT>

ARGS:
    <INPUT>    The input file or dir path

OPTIONS:
    -c, --continue                     If the input path is a directory, continue processing further
                                       files, even after an error
    -h, --help                         Print help information
    -o, --okh-version <okh-version>    If the input path is a directory, search for input files
                                       recursively [possible values: v1, losh]
    -r, --recursive                    If the input path is a directory, search for input files
                                       recursively
    -V, --version                      Print version information
```

#### Fetching and converting all OKH v1 data

Once you have the above described `okh-tool` ready
(it is required by this script),
you may just start the whole process like follows;
but be wary, this may take around 3h
(99+% of which is spent downloading).
The already downloaded files will not be re-downloaded
when you abort the process and start a-new.

```bash
./fetch-n-conv-v1
```

## Funding

This project was funded by:

* the European Union's [Horizon 2020](
      https://research-and-innovation.ec.europa.eu/funding/funding-opportunities/funding-programmes-and-open-calls/horizon-2020_en)
  research and innovation program,
  under grant agreement no. 869984,
  in the context of the [OPEN!NEXT Project](https://opennext.eu/),
  from November 2021 (project start)
  until July 2022.

  ![Logo of the European Commission](
      https://commission.europa.eu/sites/default/files/styles/oe_theme_medium_no_crop/public/2016-09/ec-logo-st-rvb-web_en.jpg)

* the European Regional Development Fund (ERDF)
  in the context of the [INTERFACER Project](https://www.interfacerproject.eu/),
  from August 2022
  until March 2023.

  ![Logo of the EU ERDF program](
      https://cloud.fabcity.hamburg/s/TopenKEHkWJ8j5P/download/logo-eu-erdf.png)
