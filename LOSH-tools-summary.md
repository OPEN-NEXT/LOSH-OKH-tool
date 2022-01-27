<!--
SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# The software tools (and similar things) that came out of LOSH

- [`okh-tool`](https://github.com/OPEN-NEXT/LOSH-OKH-tool)
  \- Rust CLI tool with various sub-commands:
  - `val` - validates manifest files:
    - OKH v1, YAML
    - OKH LOSHv1, TOML
  - `conv` - converts manifest files from OKH v1 to LOSHv1 (lossy)
  - `gen` - (**WIP**) generates an OKH LOSHv1 file,
    given a (preferably git) local checkout of an OSH project
- [appropedia-scraper](https://github.com/OPEN-NEXT/LOSH-Appropedia-Scraper)
  \- Fetches the list of project names hosted on [appropedia.org](https://appropedia.org),
  and makes them available as (updated weekly):
  - [`appro_proj_names.csv`](https://open-next.github.io/LOSH-Appropedia-Scraper/appro_proj_names.csv)
    \- the list of project names, in the same format as
    [*OpenKnowHow/okh-search/projects_okhs.csv*](https://github.com/OpenKnowHow/okh-search/blob/master/projects_okhs.csv)
  - [`appro_yaml_urls.csv`](https://open-next.github.io/LOSH-Appropedia-Scraper/appro_yaml_urls.csv)
    \- the table containing only the OKH v1 file URLs
- [OKH LOSHv1 RDF tripple-store tester](https://github.com/OPEN-NEXT/LOSH-RDF-DB-tester)
  \- Allows to easily test an OKH RDF DB locally,
   with a fully automated setup,
   allowing for SPARQL querries on the data.
- [OKH-JSON-Schemas](https://github.com/OPEN-NEXT/LOSH-OKH-JSON-Schemas)
  \- A repo containing only the [JSON-Schema](https://json-schema.org/)s
  for OKH v1 and LOSHv1 manifests.
  These allow to validate manifest files, and to generate examples.
  They are also (in my (Robins) opinion) the best way
  to document a manifest file format,
  as they are:
  - targeting JSON, YAML, TOML and some other formats
  - videly supported
  - machine readable &
  - human oriented
  - extendable, as in: unknown keys are ignored by tools that don't know them
  - allow IDEs to show context-menu help or indicate errors

