// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG

//! Sensitive read-only directory search models.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/// Supported selector.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
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
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ActiveDirectorySearchInput {
    /// Observed value; optional values preserve absence.
    #[schemars(with = "uuid::Uuid")]
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
    #[schemars(range(min = 1, max = 1000))]
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
    #[schemars(length(min = 1))]
    pub base_dn: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(length(min = 1))]
    pub group_dn: Option<String>,
    /// Observed value; optional values preserve absence.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    #[schemars(length(min = 1))]
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
impl From<gvm_gateway_domain::ActiveDirectorySearchResult> for ActiveDirectorySearchResult {
    fn from(v: gvm_gateway_domain::ActiveDirectorySearchResult) -> Self {
        Self {
            objects: v.objects.map(Into::into),
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryObjects> for ActiveDirectoryObjects {
    fn from(v: gvm_gateway_domain::ActiveDirectoryObjects) -> Self {
        Self {
            truncated: v.truncated,
            counts: v.counts.into(),
            items: v.items.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryObjectCounts> for ActiveDirectoryObjectCounts {
    fn from(v: gvm_gateway_domain::ActiveDirectoryObjectCounts) -> Self {
        Self {
            ou: v.ou,
            container: v.container,
            group: v.group,
            computer: v.computer.into(),
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryComputerCounts> for ActiveDirectoryComputerCounts {
    fn from(v: gvm_gateway_domain::ActiveDirectoryComputerCounts) -> Self {
        Self {
            total: v.total,
            disabled: v.disabled,
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryObject> for ActiveDirectoryObject {
    fn from(v: gvm_gateway_domain::ActiveDirectoryObject) -> Self {
        Self {
            object_type: v.object_type.into(),
            name: v.name,
            distinguished_name: v.distinguished_name,
            parent_dn: v.parent_dn,
            description: v.description,
            dns_hostname: v.dns_hostname,
            operating_system: v.operating_system,
            operating_system_version: v.operating_system_version,
            service_principal_names: v.service_principal_names,
            disabled: v.disabled,
            targets: v.targets.map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryTarget> for ActiveDirectoryTarget {
    fn from(v: gvm_gateway_domain::ActiveDirectoryTarget) -> Self {
        Self {
            id: v.id,
            name: v.name,
            tasks: v.tasks.map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryTask> for ActiveDirectoryTask {
    fn from(v: gvm_gateway_domain::ActiveDirectoryTask) -> Self {
        Self {
            id: v.id,
            name: v.name,
            config: v.config.map(Into::into),
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryConfig> for ActiveDirectoryConfig {
    fn from(v: gvm_gateway_domain::ActiveDirectoryConfig) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}
impl From<ActiveDirectorySearchInput> for gvm_gateway_domain::ActiveDirectorySearchInput {
    fn from(v: ActiveDirectorySearchInput) -> Self {
        Self {
            credential_id: v.credential_id,
            scope: v.scope.map(Into::into),
            max_results: v.max_results,
            include_disabled_computers: v.include_disabled_computers,
            targets: v.targets,
            tasks: v.tasks,
            details: v.details,
            base_dn: v.base_dn,
            group_dn: v.group_dn,
            object_types: v
                .object_types
                .map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}
impl From<ActiveDirectorySearchScope> for gvm_gateway_domain::ActiveDirectorySearchScope {
    fn from(v: ActiveDirectorySearchScope) -> Self {
        match v {
            ActiveDirectorySearchScope::Base => Self::Base,
            ActiveDirectorySearchScope::Onelevel => Self::Onelevel,
            ActiveDirectorySearchScope::Subtree => Self::Subtree,
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectorySearchScope> for ActiveDirectorySearchScope {
    fn from(v: gvm_gateway_domain::ActiveDirectorySearchScope) -> Self {
        match v {
            gvm_gateway_domain::ActiveDirectorySearchScope::Base => Self::Base,
            gvm_gateway_domain::ActiveDirectorySearchScope::Onelevel => Self::Onelevel,
            gvm_gateway_domain::ActiveDirectorySearchScope::Subtree => Self::Subtree,
        }
    }
}
impl From<ActiveDirectoryObjectType> for gvm_gateway_domain::ActiveDirectoryObjectType {
    fn from(v: ActiveDirectoryObjectType) -> Self {
        match v {
            ActiveDirectoryObjectType::Ou => Self::Ou,
            ActiveDirectoryObjectType::Container => Self::Container,
            ActiveDirectoryObjectType::Computer => Self::Computer,
            ActiveDirectoryObjectType::Group => Self::Group,
        }
    }
}
impl From<gvm_gateway_domain::ActiveDirectoryObjectType> for ActiveDirectoryObjectType {
    fn from(v: gvm_gateway_domain::ActiveDirectoryObjectType) -> Self {
        match v {
            gvm_gateway_domain::ActiveDirectoryObjectType::Ou => Self::Ou,
            gvm_gateway_domain::ActiveDirectoryObjectType::Container => Self::Container,
            gvm_gateway_domain::ActiveDirectoryObjectType::Computer => Self::Computer,
            gvm_gateway_domain::ActiveDirectoryObjectType::Group => Self::Group,
        }
    }
}
use crate::{
    handler::{gateway_error, ok_json},
    openapi::{ok_json as ok_schema, problem_response},
    router::bearer_token,
};
use aide::transform::TransformOperation;
use axum::{
    body::Bytes,
    extract::{OriginalUri, State},
    http::{header::CACHE_CONTROL, HeaderMap, HeaderValue},
    response::Response,
    Json,
};
use gvm_gateway_app::GatewayService;
use gvm_gateway_domain::GatewayError;
/// Bounded read-only discovery; no selectors appear in the URL.
pub async fn search_active_directory_objects(
    State(service): State<GatewayService>,
    headers: HeaderMap,
    uri: OriginalUri,
    body: Bytes,
) -> Response {
    let result = async {
        let input: ActiveDirectorySearchInput = serde_json::from_slice(&body).map_err(|_| {
            GatewayError::InvalidInput("invalid Active Directory search body".into())
        })?;
        let input: gvm_gateway_domain::ActiveDirectorySearchInput = input.into();
        input.validate()?;
        let session = bearer_token(&headers)?;
        service
            .search_active_directory_objects(&session, input)
            .await
    }
    .await;
    let mut response = match result {
        Ok(value) => ok_json(ActiveDirectorySearchResult::from(value)),
        Err(error) => gateway_error(error, uri.path().into()),
    };
    response
        .headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}
pub(crate) fn search_active_directory_objects_docs(
    op: TransformOperation<'_>,
) -> TransformOperation<'_> {
    let op = op.id("searchActiveDirectoryObjects").tag("Credentials").summary("Search live Active Directory objects")
        .description("Read-only bounded discovery using a saved credential. Counts describe the fetched subset; no targets or tasks are created. Sensitive responses use Cache-Control: no-store.")
        .security_requirement("bearerAuth").input::<Json<ActiveDirectorySearchInput>>()
        .response_with::<200, Json<ActiveDirectorySearchResult>, _>(ok_schema("Fetched directory subset"));
    let op = problem_response::<400>(op, "Invalid search selectors");
    let op = problem_response::<401>(op, "Authentication required");
    let op = problem_response::<403>(op, "Permission denied");
    let op = problem_response::<404>(op, "Credential not found");
    let op = problem_response::<409>(op, "Resource conflict");
    let op = problem_response::<500>(op, "Backend operation failed");
    let op = problem_response::<501>(op, "Backend capability unavailable");
    let op = problem_response::<502>(op, "Directory operation failed");
    problem_response::<504>(op, "Backend timeout")
}
#[cfg(test)]
#[path = "active_directory_test.rs"]
mod tests;
