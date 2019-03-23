//! Jenkins Credentials

use crate::client_internals::{Name, Path};
use crate::Jenkins;
use failure::Error;
use serde::{Deserialize, Serialize};
mod credentials;
mod domain;

/// List of `Credential` associated to a domain in the `Jenkins` instance
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainCredentials {
    /// List of credentials
    #[serde(rename = "credentials")]
    pub credentials: Vec<credentials::CommonCredentials>,
}

impl Jenkins {
    /// Get `DomainCredentials`
    pub fn get_credentials<'a, D>(&self, domain: D) -> Result<DomainCredentials, Error>
    where
        D: Into<domain::DomainName<'a>>,
    {
        Ok(self
            .get(&Path::Credentials {
                domain: Name::Name(&domain.into().0),
            })?
            .json()?)
    }
}
