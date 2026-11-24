// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG
//! Typed live directory search boundary. No local LDAP or GMP wire processing.
use super::super::*;
use gvm_gateway_domain as domain;
use gvm_gmp::commands::active_directory::{
    ActiveDirectoryObjectType as Kind, ActiveDirectorySearchScope as Scope,
    GetActiveDirectoryObjectsRequest,
};
use gvm_gmp::responses::active_directory as response;

pub(super) async fn search(
    adapter: &GvmdAdapter,
    token: &str,
    input: domain::ActiveDirectorySearchInput,
) -> Result<domain::ActiveDirectorySearchResult, GatewayError> {
    input.validate()?;
    let request = request_from_input(input)?;
    let client = adapter.session_client(token)?;
    let mut client = client.lock().await?;
    if client.command_support("get_active_directory_objects") == CommandSupport::RequiresDiscovery {
        client.discover_commands().await.map_err(map_search_error)?;
    }
    let result = client
        .get_active_directory_objects(request)
        .await
        .map_err(map_search_error)?;
    Ok(result_from_gmp(result))
}
fn request_from_input(
    v: domain::ActiveDirectorySearchInput,
) -> Result<GetActiveDirectoryObjectsRequest, GatewayError> {
    let mut r = GetActiveDirectoryObjectsRequest::new(parse_entity_id(&v.credential_id)?);
    r.scope = v.scope.map(|v| match v {
        domain::ActiveDirectorySearchScope::Base => Scope::Base,
        domain::ActiveDirectorySearchScope::Onelevel => Scope::OneLevel,
        domain::ActiveDirectorySearchScope::Subtree => Scope::Subtree,
    });
    r.max_results = v.max_results;
    r.include_disabled_computers = v.include_disabled_computers;
    r.targets = v.targets;
    r.tasks = v.tasks;
    r.details = v.details;
    r.base_dn = v.base_dn;
    r.group_dn = v.group_dn;
    r.object_types = v.object_types.map(|v| {
        v.into_iter()
            .map(|v| match v {
                domain::ActiveDirectoryObjectType::Ou => Kind::Ou,
                domain::ActiveDirectoryObjectType::Container => Kind::Container,
                domain::ActiveDirectoryObjectType::Computer => Kind::Computer,
                domain::ActiveDirectoryObjectType::Group => Kind::Group,
            })
            .collect()
    });
    Ok(r)
}
fn result_from_gmp(
    v: response::GetActiveDirectoryObjectsResponse,
) -> domain::ActiveDirectorySearchResult {
    domain::ActiveDirectorySearchResult {
        objects: v.objects.map(|v| domain::ActiveDirectoryObjects {
            truncated: v.truncated,
            counts: domain::ActiveDirectoryObjectCounts {
                ou: v.counts.ou,
                container: v.counts.container,
                group: v.counts.group,
                computer: domain::ActiveDirectoryComputerCounts {
                    total: v.counts.computer.total,
                    disabled: v.counts.computer.disabled,
                },
            },
            items: v
                .items
                .into_iter()
                .map(|v| domain::ActiveDirectoryObject {
                    object_type: match v.object_type {
                        Kind::Ou => domain::ActiveDirectoryObjectType::Ou,
                        Kind::Container => domain::ActiveDirectoryObjectType::Container,
                        Kind::Computer => domain::ActiveDirectoryObjectType::Computer,
                        Kind::Group => domain::ActiveDirectoryObjectType::Group,
                    },
                    name: v.name,
                    distinguished_name: v.distinguished_name,
                    parent_dn: v.parent_dn,
                    description: v.description,
                    dns_hostname: v.dns_hostname,
                    operating_system: v.operating_system,
                    operating_system_version: v.operating_system_version,
                    service_principal_names: v.service_principal_names,
                    disabled: v.disabled,
                    targets: v.targets.map(|v| {
                        v.into_iter()
                            .map(|v| domain::ActiveDirectoryTarget {
                                id: v.id.to_string(),
                                name: v.name,
                                tasks: v.tasks.map(|v| {
                                    v.into_iter()
                                        .map(|v| domain::ActiveDirectoryTask {
                                            id: v.id.to_string(),
                                            name: v.name,
                                            config: v.config.map(|v| {
                                                domain::ActiveDirectoryConfig {
                                                    id: v.id.to_string(),
                                                    name: v.name,
                                                }
                                            }),
                                        })
                                        .collect()
                                }),
                            })
                            .collect()
                    }),
                })
                .collect(),
        }),
    }
}
pub(super) fn map_search_error(error: GvmError) -> GatewayError {
    // Verification and search use the same sanitized AD failure categories.
    super::credentials::map_ad_error(error)
}

#[cfg(test)]
#[path = "active_directory_test.rs"]
mod tests;
