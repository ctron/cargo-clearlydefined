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
mod report;

use std::env;
use std::path::{Path, PathBuf};

use anyhow::Error;
use cargo_lock::lockfile::Lockfile;
use futures::{stream, TryStreamExt};
use log::LevelFilter;
use structopt::StructOpt;

use crate::args::Opts;
use crate::report::Dependency;
use simplelog::{Config, TermLogger, TerminalMode};
use tokio::stream::StreamExt;

use anyhow::anyhow;

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
async fn main() -> Result<(), Error> {
    let Opts::ClearlyDefined(args) = Opts::from_args();

    TermLogger::init(
        verbosity(args.verbose),
        Config::default(),
        TerminalMode::Stderr,
    )?;

    let cwd = env::current_dir()?;
    let input = args
        .input
        .unwrap_or_else(|| default_dir().unwrap_or(cwd).join(Path::new("Cargo.lock")));

    log::info!("Loading from: {}", &input.to_str().unwrap_or_default());

    let lockfile = Lockfile::load(&input)?;

    log::info!("Loaded {} dependencies", lockfile.packages.len());

    let exclude = args.exclude;

    let deps = lockfile
        .packages
        .iter()
        .filter(|&dep| !exclude.contains(&dep.name.to_string()))
        .map(|p| {
            Ok(Dependency {
                name: p.name.to_string(),
                version: p.version.clone(),
                clearly_defined: None,
                passed: false,
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

    let ignore = args.ignore;
    let required_score = args.score;

    deps = deps
        .iter()
        .filter(|dep| !ignore.contains(&dep.name))
        .map(|dep| {
            let mut dep = dep.clone();

            let score = dep.clearly_defined.as_ref().map(|cd| cd.score).unwrap_or(0);

            dep.passed = ignore.contains(&dep.name) || score >= required_score;

            dep
        })
        .collect();

    // now sort it

    if !args.quiet {
        deps.sort();

        if !args.all {
            let failed_deps: Vec<_> = deps
                .iter()
                .filter(|dep| !dep.passed)
                .map(|dep| dep.clone())
                .collect();

            log::info!(
                "{} dependencies are below the required score of {}",
                failed_deps.len(),
                required_score
            );

            report::show(args.output_format, args.link, false, &failed_deps)?;
        } else {
            report::show(args.output_format, args.link, args.all, &deps)?;
        }
    }

    let failed = deps.iter().filter(|&d| !d.passed).count();
    match failed {
        0 => Ok(()),
        _ => Err(anyhow!("{} dependencies failed the score test", failed)),
    }
}
