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

use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Opts {
    #[structopt(
    name = "clearlydefined",
    setting = AppSettings::UnifiedHelpMessage,
    setting = AppSettings::DeriveDisplayOrder,
    setting = AppSettings::DontCollapseArgsInUsage
    )]
    ClearlyDefined(Args),
}

use clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum OutputFormat {
        Text,
        CSV,
        Markdown,
    }
}

#[derive(StructOpt)]
pub struct Args {
    /// Override the location of the input file (`Cargo.lock`)
    #[structopt(short, long, parse(from_os_str))]
    pub input: Option<PathBuf>,
    /// Verbose mode, repeat to increase verbosity.
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,
    /// The score requires to pass the test.
    #[structopt(short, long, default_value = "80")]
    pub score: u64,
    /// Show all dependencies, failed or not.
    #[structopt(short = "a", long)]
    pub all: bool,
    /// List the dependencies to exclude completely.
    #[structopt(short = "x", long)]
    pub exclude: Vec<String>,
    /// List the dependencies to ignore when testing.
    #[structopt(short = "n", long)]
    pub ignore: Vec<String>,
    /// Output format
    #[structopt(short = "o", long, possible_values = &OutputFormat::variants(), case_insensitive = true, default_value="text")]
    pub output_format: OutputFormat,
    /// Add a link to clearly defined.
    #[structopt(short, long)]
    pub link: bool,
    /// Don't show any results.
    #[structopt(short, long)]
    pub quiet: bool,
}
