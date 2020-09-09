/*
 * Copyright (c) 2020 Red Hat Inc.
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0
 *
 * SPDX-License-Identifier: EPL-2.0
 */

use crate::args::{Args, OutputFormat};
use crate::data::{Dependency, Outcome};

use anyhow::Result;

use std::io;

use prettytable;
use prettytable::csv::Writer;
use prettytable::format::{self, FormatBuilder};
use prettytable::{Cell, Row, Table};

const ERR_PREFIX: &str = "ERR: ";
#[cfg(any(not(windows), not(feature = "win_crlf")))]
const NEWLINE: &str = "\n";
#[cfg(any(not(windows), not(feature = "win_crlf")))]
const ERR_PREFIX_NEWLINE: &str = "\nERR: ";
#[cfg(all(windows, feature = "win_crlf"))]
const NEWLINE: &str = "\r\n";
#[cfg(all(windows, feature = "win_crlf"))]
const ERR_PREFIX_NEWLINE: &str = "\r\nERR: ";

fn clearly_link(dep: &Dependency) -> String {
    format!(
        "https://clearlydefined.io/definitions/crate/cratesio/-/{name}/{version}",
        name = &dep.name,
        version = &dep.version
    )
}

fn shield_escape(input: &String) -> String {
    input.replace('-', "--").replace('_', "__")
}

fn shield_score(dep: &Dependency, score: &String) -> String {
    let passed = match dep.passed_score {
        Outcome::Pass => "success",
        Outcome::Fail => "critical",
        Outcome::Ignore => "inactive",
    };

    let name = shield_escape(&dep.name);
    let version = shield_escape(&dep.version.to_string());

    format!(
        "https://img.shields.io/badge/{name}_{version}-{score}-{passed}",
        name = name,
        version = version,
        score = score,
        passed = passed,
    )
}

fn markdown_image_link(img: &String, link: &String, alt: &String) -> String {
    format!(
        "[![{alt}]({img})]({link})",
        img = img,
        link = link,
        alt = alt
    )
}

fn format_error<E>(format: OutputFormat, err: E) -> String
where
    E: ToString,
{
    match format {
        OutputFormat::Text => {
            let s = format!("{}{}", ERR_PREFIX, err.to_string());
            s.replace(NEWLINE, ERR_PREFIX_NEWLINE)
        }
        OutputFormat::Markdown => format!("<b>ERR:</b> <i>{}</i>", err.to_string()),
        OutputFormat::CSV => format!("ERR: {}", err.to_string()),
    }
}

fn emoji(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Pass => "✅",
        Outcome::Fail => "❌",
        Outcome::Ignore => "🙈",
    }
}

fn csv(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Pass => "+",
        Outcome::Fail => "-",
        Outcome::Ignore => "",
    }
}

pub fn show(
    format: OutputFormat,
    args: &Args,
    show_score_check: bool,
    show_license_check: bool,
    dependencies: &Vec<Dependency>,
) -> Result<()> {
    let mut table = Table::new();

    let mut titles = vec![
        Cell::new("Name"),
        Cell::new("Version"),
        Cell::new("Declared license"),
    ];

    if show_license_check {
        titles.push(Cell::new("License"))
    }

    titles.push(Cell::new("Score"));

    match (&format, show_score_check) {
        (OutputFormat::CSV, true) => titles.push(Cell::new("Score check")),
        _ => {}
    }

    // set title

    table.set_titles(Row::new(titles));

    // start iterating

    let lax = args.lax;
    let link = args.link;

    for dep in dependencies {
        let (license_str, score) = dep
            .clearly_defined
            .as_ref()
            .map(|cd| {
                let score = format!("{}", cd.score(args.score_type));

                let l = cd
                    .declared_license
                    .as_ref()
                    .map(|l| l.expression(lax))
                    .transpose();

                match l {
                    Ok(Some(license)) => (license.to_string(), score),
                    Err(parse_err) => (format_error(format, parse_err), score),
                    _ => ("".into(), score),
                }
            })
            .unwrap_or(("".into(), "".into()));

        let score = match (&format, link, show_score_check, dep.passed_score) {
            (OutputFormat::Markdown, true, false, _) => {
                format!("[{}]({})", score, clearly_link(dep))
            }
            (OutputFormat::Markdown, true, true, _) => {
                markdown_image_link(&shield_score(dep, &score), &clearly_link(dep), &score)
            }
            (OutputFormat::Markdown, _, true, outcome) => format!("{} {}", emoji(outcome), score),
            (OutputFormat::Text, false, true, outcome) => format!("{} {}", emoji(outcome), score),
            (OutputFormat::Text, true, true, outcome) => {
                format!("{} {} ({})", emoji(outcome), score, clearly_link(dep))
            }

            // all other variant only show the score
            _ => score,
        };

        // default rows

        let mut row = vec![
            Cell::new(&dep.name),
            Cell::new(&dep.version.to_string()),
            Cell::new(&license_str),
        ];

        // license test column

        if show_license_check {
            match (&format, dep.passed_license) {
                (OutputFormat::CSV, outcome) => Some(csv(outcome)),
                (OutputFormat::Text, outcome) => Some(emoji(outcome)),
                (OutputFormat::Markdown, outcome) => Some(emoji(outcome)),
            }
            .map(|text| Cell::new(text))
            .map(|cell| row.push(cell));
        }

        // add score

        row.push(Cell::new(&score));

        // add test column

        match (&format, show_score_check, dep.passed_score) {
            (OutputFormat::CSV, true, outcome) => row.push(Cell::new(csv(outcome))),
            _ => {}
        }

        // add row

        table.add_row(Row::new(row));
    }

    // print result

    match format {
        OutputFormat::CSV => {
            table.to_csv_writer(Writer::from_writer(io::stdout()))?;
        }
        OutputFormat::Text => {
            table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        OutputFormat::Markdown => {
            let format = FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .padding(1, 1)
                .separator(
                    format::LinePosition::Title,
                    format::LineSeparator::new('-', '|', '|', '|'),
                )
                .build();

            table.set_format(format);
            table.printstd();
        }
    }

    Ok(())
}
