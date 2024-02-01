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

use anyhow::anyhow;
use clap::{ArgAction, ValueEnum};
use spdx::LicenseId;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Text,
    CSV,
    Markdown,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ScoreType {
    Effective,
    Licensed,
}

#[derive(Debug, clap::Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Cli {
    Clearlydefined(Args),
}

#[derive(Debug, clap::Args)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Override the location of the input file
    #[arg(short, long, default_value = "Cargo.lock")]
    pub input: PathBuf,
    /// Verbose mode, repeat to increase verbosity.
    #[arg(short, long, action(ArgAction::Count))]
    pub verbose: u8,
    /// Don't show any results, conflicts with 'verbose'
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,
    /// The score required to pass the test.
    #[arg(short, long, default_value_t = 80)]
    pub score: u64,
    /// Which score to test.
    #[arg(short = 't', long, value_enum, default_value_t = ScoreType::Effective)]
    pub score_type: ScoreType,
    /// Show only failed dependencies.
    #[arg(short = 'f', long)]
    pub failed: bool,
    /// List the dependencies to exclude completely.
    #[arg(short = 'x', long)]
    pub exclude: Vec<String>,
    /// List the dependencies to ignore when testing.
    #[arg(short = 'n', long)]
    pub ignore: Vec<String>,
    /// Output format
    #[arg(short = 'o', long, value_enum, default_value_t = OutputFormat::Text)]
    pub output_format: OutputFormat,
    /// Add a link to clearly defined.
    #[arg(short, long)]
    pub link: bool,
    /// Lax parsing of SPDX expressions.
    #[arg(long)]
    pub lax: bool,
    /// Approve all licenses
    #[arg(long = "approve-all")]
    pub approve_all: bool,
    /// Pass if a dependency has at least one OSI approved license.
    #[arg(long = "approve-osi")]
    pub approve_osi: bool,
    /// Pass if a dependency has at least one of the approved licenses (can be used multiple times).
    #[arg(short = 'L', long = "approve")]
    pub approved_licenses: Vec<LicenseName>,
}

#[derive(Debug, Clone)]
pub struct LicenseName(pub(crate) LicenseId);

impl FromStr for LicenseName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match spdx::license_id(s) {
            Some(id) => Ok(LicenseName(id)),
            None => Err(anyhow!("Unknown license: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
