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

use anyhow::{anyhow, Result};

use crate::args::ScoreType;
use semver::Version;
use spdx::{Expression, LicenseId, LicenseItem, ParseMode};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Pass,
    Fail,
    Ignore,
}

impl From<bool> for Outcome {
    fn from(b: bool) -> Self {
        match b {
            true => Outcome::Pass,
            false => Outcome::Fail,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: Version,

    pub clearly_defined: Option<ClearlyDefined>,

    pub passed_license: Outcome,
    pub passed_score: Outcome,
}

#[derive(Debug, Clone)]
pub struct ClearlyDefined {
    pub declared_license: Option<License>,
    effective_score: u64,
    licensed_score: u64,
}

impl ClearlyDefined {
    pub fn new(
        declared_license: Option<License>,
        effective_score: u64,
        licensed_score: u64,
    ) -> Self {
        ClearlyDefined {
            declared_license,
            effective_score,
            licensed_score,
        }
    }

    pub fn score(&self, score_type: ScoreType) -> u64 {
        match score_type {
            ScoreType::Effective => self.effective_score,
            ScoreType::Licensed => self.licensed_score,
        }
    }
}

#[derive(Debug, Clone)]
pub struct License {
    pub raw: String,
}

impl License {
    pub fn new(expression: String) -> Result<Self> {
        Ok(License { raw: expression })
    }

    pub fn expression(&self, lax: bool) -> Result<Expression> {
        let mode = match lax {
            true => ParseMode::LAX,
            false => ParseMode::STRICT,
        };

        match Expression::parse_mode(&self.raw, mode) {
            Ok(e) => Ok(e),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
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

pub trait LicenseCheck {
    fn check(&self, expression: &Expression) -> Result<()>;
}

/// Check if the dependency has an OSI approved license
pub struct OsiApproved;

impl LicenseCheck for OsiApproved {
    fn check(&self, expression: &Expression) -> Result<()> {
        let r = expression.evaluate(|r| match r.license {
            LicenseItem::Spdx { id, .. } => id.is_osi_approved(),
            _ => false,
        });

        match r {
            true => Ok(()),
            false => Err(anyhow!("{} is not OSI approved", expression)),
        }
    }
}

/// Check if the dependency has any of the approved licenses
pub struct ApprovedLicenses {
    pub licenses: Vec<LicenseId>,
}

impl LicenseCheck for ApprovedLicenses {
    fn check(&self, expression: &Expression) -> Result<()> {
        let r = expression.evaluate(|r| match r.license {
            LicenseItem::Spdx { id, .. } => self.licenses.contains(&id),
            _ => false,
        });

        match r {
            true => Ok(()),
            false => Err(anyhow!("{} is not OSI approved", expression)),
        }
    }
}

impl Dependency {
    /// Check if the dependency passed all tests.
    pub fn passed(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (self.passed_score, self.passed_license) {
            (Outcome::Fail, _) => false,
            (_, Outcome::Fail) => false,
            _ => true,
        }
    }

    /// run the license test.
    pub fn test_license(
        &self,
        lax: bool,
        checks: &[Box<dyn LicenseCheck>],
    ) -> Result<(), Vec<anyhow::Error>> {
        let license = match &self.clearly_defined {
            Some(ClearlyDefined {
                declared_license: Some(license),
                ..
            }) => license,
            _ => Err(vec![anyhow!("Missing license information")])?,
        };

        let expression = license.expression(lax).map_err(|e| vec![e])?;

        let errors: Vec<_> = checks
            .iter()
            .map(|check| check.check(&expression))
            .flat_map(|r| match r {
                Ok(_) => None,
                Err(e) => e.into(),
            })
            .collect();

        match errors.len() {
            0 => Ok(()),
            _ => Err(errors),
        }
    }
}
