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

## Output format

The default output format is "text", but you have some other options as well:

### CSV

In order to get a comma separated output:

    cargo clearlydefined -o csv

If you choose to show all dependencies, an additional column will be added, that contains the
result of the test. 

### Markdown

To get a nice markdown result, use:

    cargo clearlydefined -o markdown

This will create a markdown table, including a badge, that shows the outcome of the test, if you choose to
display all dependencies.

It is also possible to provide the argument `--link`, which will always add a link to clearlydefined.io
in the score column.

## Text

And of course, there is plain text as well. The default.

## Exclude

You can exclude dependencies from the 