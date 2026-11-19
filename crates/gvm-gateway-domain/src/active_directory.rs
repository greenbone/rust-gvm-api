// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG

//! Sensitive read-only directory search models.
use serde::{Deserialize, Serialize};
/// Supported selector.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActiveDirectorySearchScope {
    /// Supported value.
    Base,
    /// Supported value.
    Onelevel,
    /// Supported value.
    Subtree,
}
/// Supported selector.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ActiveDirectoryObjectType {
    /// Supported value.
    Ou,
    /// Supported value.
    Container,
    /// Supported value.
    Computer,
    /// Supported value.
    Group,
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectorySearchResult {
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<ActiveDirectoryObjects>,
}
impl std::fmt::Debug for ActiveDirectorySearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectorySearchResult(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryObjects {
    /// Observed value; optional values preserve absence.
    pub truncated: bool,
    /// Observed value; optional values preserve absence.
    pub counts: ActiveDirectoryObjectCounts,
    /// Observed value; optional values preserve absence.
    pub items: Vec<ActiveDirectoryObject>,
}
impl std::fmt::Debug for ActiveDirectoryObjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryObjects(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryObjectCounts {
    /// Observed value; optional values preserve absence.
    pub ou: u32,
    /// Observed value; optional values preserve absence.
    pub container: u32,
    /// Observed value; optional values preserve absence.
    pub group: u32,
    /// Observed value; optional values preserve absence.
    pub computer: ActiveDirectoryComputerCounts,
}
impl std::fmt::Debug for ActiveDirectoryObjectCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryObjectCounts(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryComputerCounts {
    /// Observed value; optional values preserve absence.
    pub total: u32,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<u32>,
}
impl std::fmt::Debug for ActiveDirectoryComputerCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryComputerCounts(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryObject {
    /// Observed value; optional values preserve absence.
    pub object_type: ActiveDirectoryObjectType,
    /// Observed value; optional values preserve absence.
    pub name: String,
    /// Observed value; optional values preserve absence.
    pub distinguished_name: String,
    /// Observed value; optional values preserve absence.
    pub parent_dn: String,
    /// Observed value; optional values preserve absence.
    pub description: String,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dns_hostname: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_principal_names: Option<Vec<String>>,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<ActiveDirectoryTarget>>,
}
impl std::fmt::Debug for ActiveDirectoryObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryObject(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryTarget {
    /// Observed value; optional values preserve absence.
    pub id: String,
    /// Observed value; optional values preserve absence.
    pub name: String,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<ActiveDirectoryTask>>,
}
impl std::fmt::Debug for ActiveDirectoryTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryTarget(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryTask {
    /// Observed value; optional values preserve absence.
    pub id: String,
    /// Observed value; optional values preserve absence.
    pub name: String,
    /// Observed value; optional values preserve absence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ActiveDirectoryConfig>,
}
impl std::fmt::Debug for ActiveDirectoryTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryTask(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDirectoryConfig {
    /// Observed value; optional values preserve absence.
    pub id: String,
    /// Observed value; optional values preserve absence.
    pub name: String,
}
impl std::fmt::Debug for ActiveDirectoryConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectoryConfig(<redacted>)")
    }
}
/// Sensitive authorized search data.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActiveDirectorySearchInput {
    /// Observed value; optional values preserve absence.
    pub credential_id: String,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub scope: Option<ActiveDirectorySearchScope>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub max_results: Option<u32>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub include_disabled_computers: Option<bool>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub targets: Option<bool>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub tasks: Option<bool>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub details: Option<bool>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub base_dn: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub group_dn: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub object_types: Option<Vec<ActiveDirectoryObjectType>>,
}
impl std::fmt::Debug for ActiveDirectorySearchInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ActiveDirectorySearchInput(<redacted>)")
    }
}
fn present<'de, D, T>(d: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(d).map(Some)
}
impl ActiveDirectorySearchInput {
    /// Reject invalid selectors before dispatch, without interpreting DNs.
    pub fn validate(&self) -> Result<(), crate::GatewayError> {
        if uuid::Uuid::parse_str(&self.credential_id).is_err()
            || self.max_results.is_some_and(|v| !(1..=1000).contains(&v))
            || self.tasks == Some(true) && self.targets != Some(true)
            || [&self.base_dn, &self.group_dn]
                .into_iter()
                .any(|v| v.as_ref().is_some_and(String::is_empty))
            || self.object_types.as_ref().is_some_and(Vec::is_empty)
        {
            return Err(crate::GatewayError::InvalidInput(
                "invalid Active Directory search selectors".into(),
            ));
        }
        Ok(())
    }
}
