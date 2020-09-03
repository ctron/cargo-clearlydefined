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

use crate::args::OutputFormat;
use anyhow::Result;
use prettytable;
use prettytable::csv::Writer;
use prettytable::format::{self, FormatBuilder};
use prettytable::{Cell, Row, Table};
use semver::Version;
use std::cmp::Ordering;
use std::io;

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Version,

    pub clearly_defined: Option<ClearlyDefined>,
    pub passed: bool,
}

#[derive(Debug, Clone)]
pub struct ClearlyDefined {
    pub declared_license: Option<String>,
    pub score: u64,
}

impl Eq for Dependency {}

impl Ord for Dependency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name
            .cmp(&other.name)
            .then_with(|| self.version.cmp(&other.version))
    }
}

impl PartialEq for Dependency {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name) && self.version.eq(&other.version)
    }
}

impl PartialOrd for Dependency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn shield(dep: &Dependency, score: &String) -> String {
    let passed = match dep.passed {
        true => "success",
        false => "critical",
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

pub fn show(
    format: OutputFormat,
    link: bool,
    show_passed: bool,
    dependencies: &Vec<Dependency>,
) -> Result<()> {
    let mut table = Table::new();

    let mut titles = vec![
        Cell::new("Name"),
        Cell::new("Version"),
        Cell::new("Declared license"),
        Cell::new("Score"),
    ];

    match (&format, show_passed) {
        (OutputFormat::CSV, true) => titles.push(Cell::new("Check")),
        _ => {}
    }

    table.set_titles(Row::new(titles));

    for dep in dependencies {
        let (license, score) = dep
            .clearly_defined
            .as_ref()
            .map(|cd| {
                (
                    cd.declared_license
                        .as_ref()
                        .map(|s| s.clone())
                        .unwrap_or_default(),
                    format!("{}", cd.score),
                )
            })
            .unwrap_or(("".into(), "".into()));

        let score = match (&format, link, show_passed, dep.passed) {
            (OutputFormat::Markdown, true, false, _) => {
                format!("[{}]({})", score, clearly_link(dep),)
            }

            (OutputFormat::Markdown, true, true, _) => {
                markdown_image_link(&shield(dep, &score), &clearly_link(dep), &score)
            }

            (OutputFormat::Text, _, true, true) => format!("{} ✅", score),
            (OutputFormat::Text, _, true, false) => format!("{} ❌", score),
            _ => score,
        };

        let mut row = vec![
            Cell::new(&dep.name),
            Cell::new(&dep.version.to_string()),
            Cell::new(&license),
            Cell::new(&score),
        ];

        match (&format, show_passed, dep.passed) {
            (OutputFormat::CSV, true, true) => row.push(Cell::new("+")),
            (OutputFormat::CSV, true, false) => row.push(Cell::new("-")),
            _ => {}
        }

        table.add_row(Row::new(row));
    }

    match format {
        OutputFormat::CSV => {
            table.to_csv_writer(Writer::from_writer(io::stdout()))?;
        }
        OutputFormat::Text => {
            table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
            println!("{} dependencies processed", dependencies.len());
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
