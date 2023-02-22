<!--
SPDX-FileCopyrightText: 2021-2023 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# LOSH OKH tool

[![GitHub license](
    https://img.shields.io/github/license/OPEN-NEXT/LOSH-OKH-tool.svg?style=flat)](
    ./LICENSE)
[![REUSE status](
    https://api.reuse.software/badge/github.com/OPEN-NEXT/LOSH-OKH-tool)](
    https://api.reuse.software/info/github.com/OPEN-NEXT/LOSH-OKH-tool)

[![In cooperation with FabCity Hamburg](
    https://custom-icon-badges.demolab.com/badge/-FCHH-dddddd.svg?logo=fc_logo)](
    https://fabcity.hamburg)
[![In cooperation with Open Source Ecology Germany](
    https://custom-icon-badges.demolab.com/badge/-OSEG-555555.svg?logo=oseg_logo)](
    https://opensourceecology.de)

* LOSH: _A **L**ibrary of **O**pen **S**ource **H**ardware -
  technical documentation in an open graph database._
* OKH: _**O**pen **K**now-**H**ow - A standard for OSH project meta-data._

This repo contains two pieces of software:

* `okh-tool`
* `fetch-n-conv-v1`

and it uses the [OKH JSON-Schemas repo](
https://github.com/OpenKnowHow/okh-search/blob/master/projects_okhs.csv)
as submodule.

* [okh-losh.schema.json](src/okh-losh.schema.json)
* [okh-v1.schema.json](src/okh-v1.schema.json)

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

### Conversion and Validation

The `okh-tool` is written in rust, and can be compiled like this:

```bash
cargo build --release
```

This will produce the stand-alone binary tool in `target/release/okh-tool`.

To run it, you have these options for conversion:

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

and these for validation:

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

### Fetching and converting OKH v1

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
