# Check dependency data from [clearlydefined.io](https://clearlydefined.io)

[![Build Status](https://github.com/ctron/cargo-clearlydefined/workflows/CI/badge.svg)](https://github.com/ctron/cargo-clearlydefined/actions?workflow=CI)
[![Crates.io](https://img.shields.io/crates/v/cargo-clearlydefined.svg)](https://crates.io/crates/cargo-clearlydefined)

This is small extension to `cargo`, to check your dependency tree against the database of
[clearlydefined.io](https://clearlydefined.io).

> ClearlyDefined and our parent organization, the Open Source Initiative, are on a mission to help FOSS projects thrive by being, well, clearly defined. 

## Installation

    cargo install cargo-clearlydefined

## Usage

In the project you wan to check, execute the following command:

    cargo clearlydefined

This will fetch dependencies metadata, and print out a report of the dependencies.

## Getting help

To get some command line help, run:

    cargo clearlydefined --help

Which should print out:

~~~
cargo-clearlydefined 0.2.1

USAGE:
    cargo clearlydefined [OPTIONS]

OPTIONS:
    -i, --input <input>                     Override the location of the input file (`Cargo.lock`)
    -v, --verbose                           Verbose mode, repeat to increase verbosity
    -s, --score <score>                     The score required to pass the test [default: 80]
    -t, --score-type <score-type>           Which score to test [default: effective]  [possible values: Effective,
                                            Licensed]
    -f, --failed                            Show only failed dependencies
    -x, --exclude <exclude>...              List the dependencies to exclude completely
    -n, --ignore <ignore>...                List the dependencies to ignore when testing
    -o, --output-format <output-format>     Output format [default: text]  [possible values: Text, CSV, Markdown]
    -l, --link                              Add a link to clearly defined
    -q, --quiet                             Don't show any results
        --lax                               Lax parsing of SPDX expressions
        --approve-all                       Approve all licenses
        --approve-osi                       Pass if a dependency has at least one OSI approved license
    -L, --approve <approved-licenses>...    Pass if a dependency has at least one of the approved licenses (can be used
                                            multiple times)
    -h, --help                              Prints help information
    -V, --version                           Prints version information
~~~

## Setting the target score

The default target score is 80, but you can change that:

    cargo clearlydefined --score 50

It is also possible to lower the score to `0`, which effectively disables this test.

## Score type to test

Clearlydefined provides different types of scores. By default, the tool will check of the "effective", or "overall"
score.

You can choose the score to test using `-t`. Testing for the "licensed score" would require:

    cargo clearlydefined --score 50 -t licensed

## Adding a link

It is also possible to provide the argument `--link`, which will add a link to the definition page
at [clearlydefined.io](https://clearlydefined.io]). Not all output formats support this though.

    cargo clearlydefined --link

## Ignoring & Excluding

You can exclude dependencies completely from processing:

    cargo clearlydefined -x my-test

Or simply ignore it from the target score test:

    cargo clearlydefined -n my-test

For example:

~~~
$ cargo clearlydefined -n my-test
+------------------------------+---------+------------------------+---------+-------+
| Name                         | Version | Declared license       | License | Score |
+------------------------------+---------+------------------------+---------+-------+
| ansi_term                    | 0.12.1  | MIT                    | ❌      | ✅ 88 |
| atty                         | 0.2.14  | MIT                    | ❌      | ✅ 88 |
| colored_json                 | 2.1.0   | EPL-2.0                | ❌      | ✅ 87 |
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT     | ❌      | ❌ 52 |
| itoa                         | 0.4.6   | Apache-2.0 AND MIT     | ❌      | ✅ 87 |
| libc                         | 0.2.76  | Apache-2.0 AND MIT     | ❌      | ✅ 87 |
| my-test                      | 0.1.0   |                        | 🙈      | 🙈 0  |
| ryu                          | 1.0.5   | Apache-2.0 AND BSL-1.0 | ❌      | ✅ 80 |
| serde                        | 1.0.115 | Apache-2.0 AND MIT     | ❌      | ✅ 87 |
| serde_json                   | 1.0.57  | Apache-2.0 AND MIT     | ❌      | ✅ 87 |
| winapi                       | 0.3.9   | Apache-2.0 AND MIT     | ❌      | ✅ 87 |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0      | ❌      | ❌ 37 |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0      | ❌      | ❌ 37 |
+------------------------------+---------+------------------------+---------+-------+
~~~

## SPDX parsing

The tool will parse the SPDX expression coming back from clearlydefined. Unfortunately, some dependencies
have an invalid (according to the SPDX spec) expression.

You may use the `--lax` option, to switch to a more lax parser for SPDX expressions. However, in some cases
this still isn't enough. The only choice at the moment is, to ignore or exclude such dependencies.

## License testing

If a dependency has a valid SPDX license definition, it can be tested. If it doesnt', all checks based on the
license will fail for this dependency.

All license tests are disabled by default.

In most cases, the module you are checking, is not registered with clearly defined, and thus cannot pass the
license tests. If that is a problem, then you can exclude the module using the `-x` switch.

### Testing if the dependency has an OSI approved license

Using the option `--require-osi-approved` you require that each dependency has at least one
[OSI approved license](https://opensource.org/licenses).

### Check for provided list

You can also provide a list of accepted licenses, and the tool will check for those:

    cargo clearlydefined -L EPL-2.0 -L MIT

## Output format

The default output format is "text", but you have some other options as well:

### CSV

In order to get a comma separated output:

    cargo clearlydefined -o csv

Example output:   

~~~
$ cargo clearlydefined --approve-osi -o csv
Name,Version,Declared license,License,Score,Score check
ansi_term,0.12.1,MIT,+,88,+
atty,0.2.14,MIT,+,88,+
colored_json,2.1.0,EPL-2.0,+,87,+
hermit-abi,0.1.15,Apache-2.0 AND MIT,+,52,-
itoa,0.4.6,Apache-2.0 AND MIT,+,87,+
libc,0.2.76,Apache-2.0 AND MIT,+,87,+
my-test,0.1.0,,-,0,-
ryu,1.0.5,Apache-2.0 AND BSL-1.0,+,80,+
serde,1.0.115,Apache-2.0 AND MIT,+,87,+
serde_json,1.0.57,Apache-2.0 AND MIT,+,87,+
winapi,0.3.9,Apache-2.0 AND MIT,+,87,+
winapi-i686-pc-windows-gnu,0.4.0,MIT OR Apache-2.0,+,37,-
winapi-x86_64-pc-windows-gnu,0.4.0,MIT OR Apache-2.0,+,37,-
~~~

### Markdown

To get a nice markdown result, use:

    cargo clearlydefined -o markdown

This will create a markdown table, including a badge, that shows the outcome of the test, if you choose to
display all dependencies.

Example output:

| Name                         | Version | Declared license       | License | Score |
|------------------------------|---------|------------------------|---------|-------|
| ansi_term                    | 0.12.1  | MIT                    | ✅      | ✅ 88 |
| atty                         | 0.2.14  | MIT                    | ✅      | ✅ 88 |
| colored_json                 | 2.1.0   | EPL-2.0                | ✅      | ✅ 87 |
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT     | ✅      | ❌ 52 |
| itoa                         | 0.4.6   | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| libc                         | 0.2.76  | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| my-test                      | 0.1.0   |                        | ❌      | ❌ 0  |
| ryu                          | 1.0.5   | Apache-2.0 AND BSL-1.0 | ✅      | ✅ 80 |
| serde                        | 1.0.115 | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| serde_json                   | 1.0.57  | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| winapi                       | 0.3.9   | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0      | ✅      | ❌ 37 |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0      | ✅      | ❌ 37 |

Using the `--link` option:

| Name                         | Version | Declared license       | License | Score                                                                                                                                                                                |
|------------------------------|---------|------------------------|---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| ansi_term                    | 0.12.1  | MIT                    | ✅      | [![88](https://img.shields.io/badge/ansi__term_0.12.1-88-success)](https://clearlydefined.io/definitions/crate/cratesio/-/ansi_term/0.12.1)                                          |
| atty                         | 0.2.14  | MIT                    | ✅      | [![88](https://img.shields.io/badge/atty_0.2.14-88-success)](https://clearlydefined.io/definitions/crate/cratesio/-/atty/0.2.14)                                                     |
| colored_json                 | 2.1.0   | EPL-2.0                | ✅      | [![87](https://img.shields.io/badge/colored__json_2.1.0-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/colored_json/2.1.0)                                      |
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT     | ✅      | [![52](https://img.shields.io/badge/hermit--abi_0.1.15-52-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/hermit-abi/0.1.15)                                       |
| itoa                         | 0.4.6   | Apache-2.0 AND MIT     | ✅      | [![87](https://img.shields.io/badge/itoa_0.4.6-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/itoa/0.4.6)                                                       |
| libc                         | 0.2.76  | Apache-2.0 AND MIT     | ✅      | [![87](https://img.shields.io/badge/libc_0.2.76-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/libc/0.2.76)                                                     |
| my-test                      | 0.1.0   |                        | ❌      | [![0](https://img.shields.io/badge/my--test_0.1.0-0-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/my-test/0.1.0)                                                 |
| ryu                          | 1.0.5   | Apache-2.0 AND BSL-1.0 | ✅      | [![80](https://img.shields.io/badge/ryu_1.0.5-80-success)](https://clearlydefined.io/definitions/crate/cratesio/-/ryu/1.0.5)                                                         |
| serde                        | 1.0.115 | Apache-2.0 AND MIT     | ✅      | [![87](https://img.shields.io/badge/serde_1.0.115-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/serde/1.0.115)                                                 |
| serde_json                   | 1.0.57  | Apache-2.0 AND MIT     | ✅      | [![87](https://img.shields.io/badge/serde__json_1.0.57-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/serde_json/1.0.57)                                        |
| winapi                       | 0.3.9   | Apache-2.0 AND MIT     | ✅      | [![87](https://img.shields.io/badge/winapi_0.3.9-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/winapi/0.3.9)                                                   |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0      | ✅      | [![37](https://img.shields.io/badge/winapi--i686--pc--windows--gnu_0.4.0-37-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/winapi-i686-pc-windows-gnu/0.4.0)      |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0      | ✅      | [![37](https://img.shields.io/badge/winapi--x86__64--pc--windows--gnu_0.4.0-37-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/winapi-x86_64-pc-windows-gnu/0.4.0) |

### Text

And of course, there is plain text as well. The default:

~~~
$ cargo clearlydefined --approve-osi -o text
+------------------------------+---------+------------------------+---------+-------+
| Name                         | Version | Declared license       | License | Score |
+------------------------------+---------+------------------------+---------+-------+
| ansi_term                    | 0.12.1  | MIT                    | ✅      | ✅ 88 |
| atty                         | 0.2.14  | MIT                    | ✅      | ✅ 88 |
| colored_json                 | 2.1.0   | EPL-2.0                | ✅      | ✅ 87 |
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT     | ✅      | ❌ 52 |
| itoa                         | 0.4.6   | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| libc                         | 0.2.76  | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| my-test                      | 0.1.0   |                        | ❌      | ❌ 0  |
| ryu                          | 1.0.5   | Apache-2.0 AND BSL-1.0 | ✅      | ✅ 80 |
| serde                        | 1.0.115 | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| serde_json                   | 1.0.57  | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| winapi                       | 0.3.9   | Apache-2.0 AND MIT     | ✅      | ✅ 87 |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0      | ✅      | ❌ 37 |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0      | ✅      | ❌ 37 |
+------------------------------+---------+------------------------+---------+-------+
~~~
