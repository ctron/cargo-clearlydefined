# Cargo dependency checker for [clearlydefined.io](clearlydefined.io)

This is small extension to `cargo`, to check your dependency tree against the database of
[clearlydefined.io](clearlydefined.io).

> ClearlyDefined and our parent organization, the Open Source Initiative, are on a mission to help FOSS projects thrive by being, well, clearly defined. 

## Installation

    cargo install cargo-clearlydefined

## Usage

In the project you wan to check, execute the following command:

    cargo clearlydefined

This will fetch dependencies metadata, and print out a report of the dependencies that have a score below 80.

## Getting help

To get some command line help, run:

    cargo clearlydefined --help

Which should print out:

~~~
cargo-clearlydefined 0.1.0

USAGE:
    cargo clearlydefined [OPTIONS]

OPTIONS:
    -i, --input <input>                    Override the location of the input file (`Cargo.lock`)
    -v, --verbose                          Verbose mode, repeat to increase verbosity
    -s, --score <score>                    The score requires to pass the test [default: 80]
    -a, --all                              Show all dependencies, failed or not
    -x, --exclude <exclude>...             List the dependencies to ignore when testing
    -o, --output-format <output-format>    Output format [default: text]  [possible values: Text, CSV, Markdown]
    -l, --link                             Add a link to clearly defined
    -q, --quiet                            Don't show any results
    -h, --help                             Prints help information
    -V, --version                          Prints version information
~~~

## Setting the target score

You can set the target score:

    cargo clearlydefined --score 50

It is also possible to lower the score to `0`.

## Showing all dependencies

By default, only the "failed" dependencies are shown. You can however get a report of all
dependencies:

    cargo clearlydefined --all

## Ignoring & Excluding

You can exclude dependencies completly from processing:

    cargo clearlydefined -x wasi

Or simply ignore it from the target score test:

    cargo clearlydefined -n wasi

## Output format

The default output format is "text", but you have some other options as well:

### CSV

In order to get a comma separated output:

    cargo clearlydefined -o csv

If you choose to show all dependencies, an additional column will be added, that contains the
result of the test. 

#### Example, failures only

~~~
Name,Version,Declared license,Score
hermit-abi,0.1.15,Apache-2.0 AND MIT,52
my-test,0.1.0,,0
winapi-i686-pc-windows-gnu,0.4.0,MIT OR Apache-2.0,37
winapi-x86_64-pc-windows-gnu,0.4.0,MIT OR Apache-2.0,37
~~~

#### Example, all

~~~
Name,Version,Declared license,Score,Check
ansi_term,0.12.1,MIT,88,+
atty,0.2.14,MIT,88,+
colored_json,2.1.0,EPL-2.0,87,+
hermit-abi,0.1.15,Apache-2.0 AND MIT,52,-
itoa,0.4.6,Apache-2.0 AND MIT,87,+
libc,0.2.76,Apache-2.0 AND MIT,87,+
my-test,0.1.0,,0,-
ryu,1.0.5,Apache-2.0 AND BSL-1.0,80,+
serde,1.0.115,Apache-2.0 AND MIT,87,+
serde_json,1.0.57,Apache-2.0 AND MIT,87,+
winapi,0.3.9,Apache-2.0 AND MIT,87,+
winapi-i686-pc-windows-gnu,0.4.0,MIT OR Apache-2.0,37,-
winapi-x86_64-pc-windows-gnu,0.4.0,MIT OR Apache-2.0,37,-
~~~

### Markdown

To get a nice markdown result, use:

    cargo clearlydefined -o markdown

This will create a markdown table, including a badge, that shows the outcome of the test, if you choose to
display all dependencies.

It is also possible to provide the argument `--link`, which will always add a link to clearlydefined.io
in the score column.

#### Example, failures only

Using the `--link` option.

| Name                         | Version | Declared license   | Score                                                                                           |
|------------------------------|---------|--------------------|-------------------------------------------------------------------------------------------------|
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT | [52](https://clearlydefined.io/definitions/crate/cratesio/-/hermit-abi/0.1.15)                  |
| my-test                      | 0.1.0   |                    | [0](https://clearlydefined.io/definitions/crate/cratesio/-/my-test/0.1.0)                       |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0  | [37](https://clearlydefined.io/definitions/crate/cratesio/-/winapi-i686-pc-windows-gnu/0.4.0)   |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0  | [37](https://clearlydefined.io/definitions/crate/cratesio/-/winapi-x86_64-pc-windows-gnu/0.4.0) |

#### Example, all

Using the `--link` option.

| Name                         | Version | Declared license       | Score                                                                                                                                                                                |
|------------------------------|---------|------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| ansi_term                    | 0.12.1  | MIT                    | [![88](https://img.shields.io/badge/ansi__term_0.12.1-88-success)](https://clearlydefined.io/definitions/crate/cratesio/-/ansi_term/0.12.1)                                          |
| atty                         | 0.2.14  | MIT                    | [![88](https://img.shields.io/badge/atty_0.2.14-88-success)](https://clearlydefined.io/definitions/crate/cratesio/-/atty/0.2.14)                                                     |
| colored_json                 | 2.1.0   | EPL-2.0                | [![87](https://img.shields.io/badge/colored__json_2.1.0-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/colored_json/2.1.0)                                      |
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT     | [![52](https://img.shields.io/badge/hermit--abi_0.1.15-52-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/hermit-abi/0.1.15)                                       |
| itoa                         | 0.4.6   | Apache-2.0 AND MIT     | [![87](https://img.shields.io/badge/itoa_0.4.6-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/itoa/0.4.6)                                                       |
| libc                         | 0.2.76  | Apache-2.0 AND MIT     | [![87](https://img.shields.io/badge/libc_0.2.76-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/libc/0.2.76)                                                     |
| my-test                      | 0.1.0   |                        | [![0](https://img.shields.io/badge/my--test_0.1.0-0-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/my-test/0.1.0)                                                 |
| ryu                          | 1.0.5   | Apache-2.0 AND BSL-1.0 | [![80](https://img.shields.io/badge/ryu_1.0.5-80-success)](https://clearlydefined.io/definitions/crate/cratesio/-/ryu/1.0.5)                                                         |
| serde                        | 1.0.115 | Apache-2.0 AND MIT     | [![87](https://img.shields.io/badge/serde_1.0.115-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/serde/1.0.115)                                                 |
| serde_json                   | 1.0.57  | Apache-2.0 AND MIT     | [![87](https://img.shields.io/badge/serde__json_1.0.57-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/serde_json/1.0.57)                                        |
| winapi                       | 0.3.9   | Apache-2.0 AND MIT     | [![87](https://img.shields.io/badge/winapi_0.3.9-87-success)](https://clearlydefined.io/definitions/crate/cratesio/-/winapi/0.3.9)                                                   |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0      | [![37](https://img.shields.io/badge/winapi--i686--pc--windows--gnu_0.4.0-37-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/winapi-i686-pc-windows-gnu/0.4.0)      |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0      | [![37](https://img.shields.io/badge/winapi--x86__64--pc--windows--gnu_0.4.0-37-critical)](https://clearlydefined.io/definitions/crate/cratesio/-/winapi-x86_64-pc-windows-gnu/0.4.0) |

### Text

And of course, there is plain text as well. The default.

#### Example, failures only

~~~
+------------------------------+---------+--------------------+-------+
| Name                         | Version | Declared license   | Score |
+------------------------------+---------+--------------------+-------+
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT | 52    |
| my-test                      | 0.1.0   |                    | 0     |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0  | 37    |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0  | 37    |
+------------------------------+---------+--------------------+-------+
~~~

#### Example, all

~~~
+------------------------------+---------+------------------------+-------+
| Name                         | Version | Declared license       | Score |
+------------------------------+---------+------------------------+-------+
| ansi_term                    | 0.12.1  | MIT                    | 88 ✅ |
| atty                         | 0.2.14  | MIT                    | 88 ✅ |
| colored_json                 | 2.1.0   | EPL-2.0                | 87 ✅ |
| hermit-abi                   | 0.1.15  | Apache-2.0 AND MIT     | 52 ❌ |
| itoa                         | 0.4.6   | Apache-2.0 AND MIT     | 87 ✅ |
| libc                         | 0.2.76  | Apache-2.0 AND MIT     | 87 ✅ |
| my-test                      | 0.1.0   |                        | 0 ❌  |
| ryu                          | 1.0.5   | Apache-2.0 AND BSL-1.0 | 80 ✅ |
| serde                        | 1.0.115 | Apache-2.0 AND MIT     | 87 ✅ |
| serde_json                   | 1.0.57  | Apache-2.0 AND MIT     | 87 ✅ |
| winapi                       | 0.3.9   | Apache-2.0 AND MIT     | 87 ✅ |
| winapi-i686-pc-windows-gnu   | 0.4.0   | MIT OR Apache-2.0      | 37 ❌ |
| winapi-x86_64-pc-windows-gnu | 0.4.0   | MIT OR Apache-2.0      | 37 ❌ |
+------------------------------+---------+------------------------+-------+
~~~
