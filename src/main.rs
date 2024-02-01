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

mod args;
mod cd;
mod data;
mod report;

use crate::{
    args::Cli,
    data::{ApprovedLicenses, Dependency, LicenseCheck, OsiApproved, Outcome},
};
use anyhow::{Error, Result};
use cargo_lock::Lockfile;
use clap::Parser;
use futures::{stream, StreamExt, TryStreamExt};
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn default_dir() -> Option<PathBuf> {
    env::var_os("CARGO_MANIFEST_DIR").map(|s| PathBuf::from(&s))
}

fn verbosity(num: u8) -> LevelFilter {
    match num {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Off,
    }
}

#[tokio::main]
async fn main() -> Result<ExitCode, Error> {
    let Cli::Clearlydefined(args) = Cli::parse();

    TermLogger::init(
        verbosity(args.verbose),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    let input = match default_dir() {
        Some(dir) => dir,
        None => env::current_dir()?,
    }
    .join(&args.input);

    log::info!("Loading from: {}", &input.to_str().unwrap_or_default());

    let lockfile = Lockfile::load(&input)?;

    log::info!("Loaded {} dependencies", lockfile.packages.len());

    let exclude = &args.exclude;

    let deps = lockfile
        .packages
        .iter()
        .filter(|&dep| !exclude.contains(&dep.name.to_string()))
        .map(|p| {
            Ok(Dependency {
                name: p.name.to_string(),
                version: p.version.clone(),
                clearly_defined: None,
                passed_license: Outcome::Ignore,
                passed_score: Outcome::Ignore,
            })
        })
        .collect::<Vec<_>>();

    let client = reqwest::Client::builder().build()?;
    let mut deps = stream::iter(deps)
        .and_then(|d| cd::lookup_clearlydefined(&client, d))
        .map(|s| {
            match &s {
                Ok(dep) => {
                    log::info!("Processed: {}/{}", dep.name, dep.version);
                }
                _ => {}
            }

            s
        })
        .try_collect::<Vec<_>>()
        .await?;

    log::info!("Processed all dependencies");

    let ignore = &args.ignore;
    let required_score = args.score;

    let mut checks: Vec<Box<dyn LicenseCheck>> = Vec::new();
    if args.approve_osi {
        checks.push(Box::new(OsiApproved));
    }
    if !args.approved_licenses.is_empty() {
        checks.push(Box::new(ApprovedLicenses {
            licenses: args.approved_licenses.iter().map(|name| name.0).collect(),
        }))
    }

    let has_license_checks = !checks.is_empty();
    let approve_all = args.approve_all;
    let has_score_check = args.score > 0;

    deps = deps
        .iter()
        .map(|dep| {
            let mut dep = dep.clone();

            let score = dep
                .clearly_defined
                .as_ref()
                .map(|cd| cd.score(args.score_type))
                .unwrap_or(0);

            if !ignore.contains(&dep.name) {
                // check score
                dep.passed_score = (score >= required_score).into();
                // check license
                if !has_license_checks {
                    dep.passed_license = Outcome::Fail;
                } else if approve_all {
                    dep.passed_license = Outcome::Pass;
                } else {
                    dep.passed_license = dep.test_license(args.lax, &checks).is_ok().into();
                }
            }

            dep
        })
        .collect();

    if !&args.quiet {
        // now sort it
        deps.sort();

        if args.failed {
            let failed_deps: Vec<_> = deps.iter().filter(|dep| !dep.passed()).cloned().collect();

            log::info!(
                "{} dependencies are below the required score of {}",
                failed_deps.len(),
                required_score
            );

            report::show(
                args.output_format,
                &args,
                has_score_check,
                !approve_all,
                &failed_deps,
            )?;
        } else {
            report::show(
                args.output_format,
                &args,
                has_score_check,
                !approve_all,
                &deps,
            )?;
        }
    }

    if !args.quiet && !has_license_checks {
        eprintln!("You have no license checks. Try --approve-osi, --approve-all, or provide a manual selection using e.g. --approve <spdx-license>");
    }

    let failed = deps.iter().filter(|&d| !d.passed()).count();
    match failed {
        0 => Ok(ExitCode::SUCCESS),
        _ => {
            log::error!(
                "{} dependencies out of {} failed at least one of the tests",
                failed,
                deps.len()
            );
            Ok(ExitCode::FAILURE)
        }
    }
}
