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

use crate::data::{ClearlyDefined, Dependency, License};

use anyhow::Result;
use reqwest::Client;

pub async fn lookup_clearlydefined(
    client: &Client,
    mut dependency: Dependency,
) -> Result<Dependency> {
    let url = format!(
        "https://api.clearlydefined.io/definitions/crate/cratesio/-/{}/{}",
        dependency.name, dependency.version
    );

    let def: serde_json::Value = client.get(&url).send().await?.json().await?;

    let license = def["licensed"]["declared"]
        .as_str()
        .map(|s| License::new(s.into()))
        .transpose()?;

    dependency.clearly_defined = Some(ClearlyDefined {
        declared_license: license,
        score: def["scores"]["effective"].as_u64().unwrap_or(0),
    });

    Ok(dependency.clone())
}
