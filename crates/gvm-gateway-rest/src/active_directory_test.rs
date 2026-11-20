// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG
use super::*;
use serde_json::{json, Value};
const ID: &str = "123e4567-e89b-12d3-a456-426614174000";
fn parse(value: Value) -> Result<gvm_gateway_domain::ActiveDirectorySearchInput, ()> {
    let input: ActiveDirectorySearchInput = serde_json::from_value(value).map_err(|_| ())?;
    let input: gvm_gateway_domain::ActiveDirectorySearchInput = input.into();
    input.validate().map_err(|_| ())?;
    Ok(input)
}
#[test]
fn omitted_options_preserve_backend_defaults_and_explicit_false_survives() {
    // Backend defaults must not become guessed downstream values.
    let input = parse(json!({"credentialId":ID})).unwrap();
    assert!(input.scope.is_none() && input.max_results.is_none() && input.object_types.is_none());
    assert!(input.targets.is_none() && input.tasks.is_none() && input.details.is_none());
    assert!(
        input.include_disabled_computers.is_none()
            && input.base_dn.is_none()
            && input.group_dn.is_none()
    );
    let input = parse(json!({"credentialId":ID,"targets":false,"tasks":false,"details":false,"includeDisabledComputers":false})).unwrap();
    assert_eq!(input.targets, Some(false));
    assert_eq!(input.tasks, Some(false));
    assert_eq!(input.details, Some(false));
    assert_eq!(input.include_disabled_computers, Some(false));
}
#[test]
fn bounded_selectors_reject_bad_types_nulls_and_combinations() {
    // Fail before any backend dispatch, without echoing sensitive selectors.
    let mut bad = vec![
        json!({}),
        json!({"credentialId":"not-a-uuid"}),
        json!({"credentialId":ID,"unknown":"secret"}),
    ];
    for (field, values) in [
        ("scope", vec![json!("BASE"), json!(true)]),
        (
            "maxResults",
            vec![json!(0), json!(1001), json!(-1), json!(1.5)],
        ),
        ("baseDn", vec![json!("")]),
        ("groupDn", vec![json!("")]),
        ("objectTypes", vec![json!([]), json!(["unknown"])]),
        ("targets", vec![json!("true"), json!(1)]),
        ("tasks", vec![json!(true)]),
    ] {
        for value in values {
            bad.push(json!({"credentialId":ID,field:value}));
        }
    }
    for field in [
        "scope",
        "maxResults",
        "includeDisabledComputers",
        "targets",
        "tasks",
        "details",
        "baseDn",
        "groupDn",
        "objectTypes",
    ] {
        bad.push(json!({"credentialId":ID,field:null}));
    }
    for value in bad {
        assert!(parse(value).is_err());
    }
}
#[test]
fn every_selector_and_scope_preserves_sensitive_dns_verbatim() {
    // DN containment/case and deduplication remain upstream responsibilities.
    for scope in ["base", "onelevel", "subtree"] {
        let input=parse(json!({"credentialId":ID,"scope":scope,"maxResults":1000,"includeDisabledComputers":true,"targets":true,"tasks":true,"details":true,"baseDn":"OU=R&D,DC=Example","groupDn":"CN=<team>,DC=Example","objectTypes":["ou","container","computer","group","ou"]})).unwrap();
        assert_eq!(input.base_dn.as_deref(), Some("OU=R&D,DC=Example"));
        assert_eq!(input.object_types.as_ref().unwrap().len(), 5);
        assert!(!format!("{input:?}").contains("Example"));
    }
}
#[test]
fn response_absence_empty_and_nested_expansions_survive_json_projection() {
    // Preserve optional envelopes/counts/arrays instead of inventing empty successes.
    for value in [
        json!({}),
        json!({"objects":{"truncated":true,"counts":{"ou":0,"container":0,"group":0,"computer":{"total":0}},"items":[]}}),
        json!({"objects":{"truncated":false,"counts":{"ou":0,"container":0,"group":0,"computer":{"total":1,"disabled":0}},"items":[{"objectType":"computer","name":"sensitive","distinguishedName":"CN=x","parentDn":"","description":"","disabled":false,"servicePrincipalNames":[],"targets":[{"id":ID,"name":"target","tasks":[]},{"id":ID,"name":"other","tasks":[{"id":ID,"name":"task"}]}]}]}}),
    ] {
        let domain: gvm_gateway_domain::ActiveDirectorySearchResult =
            serde_json::from_value(value.clone()).unwrap();
        let response = ActiveDirectorySearchResult::from(domain);
        assert_eq!(serde_json::to_value(&response).unwrap(), value);
        assert!(!format!("{response:?}").contains("CN=x"));
    }
}
#[test]
fn generated_schema_exposes_the_bounded_search_contract() {
    // Generated schemas and route registration must be shipped with the runtime.
    let api = crate::router::build_openapi();
    let op = &api["paths"]["/active-directory/object-searches"]["post"];
    assert_eq!(op["operationId"], "searchActiveDirectoryObjects");
    for status in ["200", "400", "401", "403", "404", "501", "502", "504"] {
        assert!(op["responses"].get(status).is_some());
    }
    // Cache semantics must be machine-readable, not only mentioned in prose.
    assert_eq!(
        op["responses"]["200"]["headers"]["Cache-Control"]["schema"],
        json!({"type": "string", "const": "no-store"})
    );
    let input = &api["components"]["schemas"]["ActiveDirectorySearchInput"];
    assert_eq!(input["properties"]["maxResults"]["maximum"], 1000);
}

use std::sync::Arc;
fn static_gateway_service() -> GatewayService {
    let adapter = Arc::new(gvm_gateway_gvmd::StaticGvmdAdapter::ready("22.7"));
    let system: Arc<dyn gvm_gateway_domain::SystemPort> = adapter.clone();
    let alerts: Arc<dyn gvm_gateway_domain::AlertPort> = adapter.clone();
    let schedules: Arc<dyn gvm_gateway_domain::SchedulePort> = adapter.clone();
    let credentials: Arc<dyn gvm_gateway_domain::CredentialPort> = adapter.clone();
    let port_lists: Arc<dyn gvm_gateway_domain::PortListPort> = adapter.clone();
    let feeds: Arc<dyn gvm_gateway_domain::FeedPort> = adapter.clone();
    let identity: Arc<dyn gvm_gateway_domain::IdentityPort> = adapter.clone();
    let targets: Arc<dyn gvm_gateway_domain::TargetPort> = adapter.clone();
    let tasks: Arc<dyn gvm_gateway_domain::TaskPort> = adapter.clone();
    let auth: Arc<dyn gvm_gateway_domain::AuthPort> = adapter.clone();
    let reports: Arc<dyn gvm_gateway_domain::ReportPort> = adapter.clone();
    let results: Arc<dyn gvm_gateway_domain::ResultPort> = adapter.clone();
    let scan_configs: Arc<dyn gvm_gateway_domain::ScanConfigPort> = adapter.clone();
    let scanners: Arc<dyn gvm_gateway_domain::ScannerPort> = adapter.clone();
    let agents: Arc<dyn gvm_gateway_domain::AgentPort> = adapter.clone();
    let supporting_resources: Arc<dyn gvm_gateway_domain::SupportingResourcePort> = adapter;

    GatewayService::new(
        gvm_gateway_app::GatewayPorts {
            system,
            alerts,
            schedules,
            credentials,
            port_lists,
            feeds,
            identity,
            targets,
            tasks,
            auth,
            reports,
            results,
            scan_configs,
            scanners,
            agents,
            supporting_resources,
        },
        Arc::new(gvm_gateway_domain::SessionManager::default()),
    )
}

#[tokio::test]
async fn malformed_and_unsupported_search_responses_are_no_store_and_sanitized() {
    // Handler failures must neither echo sensitive bodies nor manufacture results.
    use axum::http::{StatusCode, Uri};
    let service = static_gateway_service();
    let response = search_active_directory_objects(
        State(service),
        HeaderMap::new(),
        OriginalUri(Uri::from_static("/api/v1/active-directory/object-searches")),
        Bytes::from_static(br#"{"credentialId":"secret DN"}"#),
    )
    .await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(response.headers()[CACHE_CONTROL], "no-store");
    assert!(response.headers().get("location").is_none());
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!String::from_utf8_lossy(&body).contains("secret DN"));
    let service = static_gateway_service();
    let response = search_active_directory_objects(
        State(service),
        HeaderMap::new(),
        OriginalUri(Uri::from_static("/api/v1/active-directory/object-searches")),
        Bytes::from(format!("{{\"credentialId\":\"{ID}\"}}")),
    )
    .await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.headers()[CACHE_CONTROL], "no-store");
}

#[tokio::test]
async fn static_backend_does_not_emulate_directory_success() {
    // The shipped static backend must advertise unsupported rather than fake an LDAP search.
    use axum::http::{header::AUTHORIZATION, StatusCode, Uri};
    let service = static_gateway_service();
    let session = service.create_session("admin", "admin").await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", session.token)).unwrap(),
    );
    let response = search_active_directory_objects(
        State(service),
        headers,
        OriginalUri(Uri::from_static("/api/v1/active-directory/object-searches")),
        Bytes::from(format!("{{\"credentialId\":\"{ID}\"}}")),
    )
    .await;
    assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    assert_eq!(response.headers()[CACHE_CONTROL], "no-store");
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert!(!String::from_utf8_lossy(&body).contains("objects"));
}

#[test]
fn generated_and_curated_directory_shapes_have_identical_fields_and_optionality() {
    // Keep split curated models in lockstep with the handler-generated response
    // tree, including optional envelope/count/array distinctions.
    use std::collections::BTreeSet;
    let generated = crate::router::build_openapi();
    let curated: Value =
        serde_yaml::from_str(include_str!("../../../spec/rest-api/active-directory.yaml")).unwrap();
    let gs = &generated["components"]["schemas"];
    let cs = &curated["components"]["schemas"];
    for name in [
        "ActiveDirectorySearchInput",
        "ActiveDirectorySearchResult",
        "ActiveDirectoryObjects",
        "ActiveDirectoryObjectCounts",
        "ActiveDirectoryComputerCounts",
        "ActiveDirectoryObject",
        "ActiveDirectoryTarget",
        "ActiveDirectoryTask",
        "ActiveDirectoryConfig",
    ] {
        let keys =
            |v: &Value| -> BTreeSet<String> { v.as_object().unwrap().keys().cloned().collect() };
        assert_eq!(
            keys(&gs[name]["properties"]),
            keys(&cs[name]["properties"]),
            "fields: {name}"
        );
        let required = |v: &Value| -> BTreeSet<String> {
            v.as_array()
                .into_iter()
                .flatten()
                .map(|v| v.as_str().unwrap().to_string())
                .collect()
        };
        assert_eq!(
            required(&gs[name]["required"]),
            required(&cs[name]["required"]),
            "optionality: {name}"
        );
    }
    fn enum_values(schema: &Value, schemas: &Value) -> BTreeSet<String> {
        if let Some(name) = schema["$ref"]
            .as_str()
            .and_then(|v| v.strip_prefix("#/components/schemas/"))
        {
            return enum_values(&schemas[name], schemas);
        }
        let mut values = BTreeSet::new();
        if let Some(value) = schema["const"].as_str() {
            values.insert(value.to_string());
        }
        for value in schema["enum"].as_array().into_iter().flatten() {
            values.insert(value.as_str().unwrap().to_string());
        }
        for key in ["oneOf", "anyOf", "allOf"] {
            for branch in schema[key].as_array().into_iter().flatten() {
                values.extend(enum_values(branch, schemas));
            }
        }
        values
    }
    for (name, owner, field) in [
        (
            "ActiveDirectorySearchScope",
            "ActiveDirectorySearchInput",
            "scope",
        ),
        (
            "ActiveDirectoryObjectType",
            "ActiveDirectoryObject",
            "objectType",
        ),
    ] {
        // Schemars preserves variant documentation with const/oneOf rather
        // than enum arrays; compare the exact closed set through either form.
        let schema = &gs[owner]["properties"][field];
        assert_eq!(
            enum_values(schema, gs),
            enum_values(&cs[name], cs),
            "enum: {name}; generated: {schema}"
        );
    }

    assert_eq!(
        gs["ActiveDirectorySearchInput"]["properties"]["credentialId"]["format"],
        "uuid"
    );
}
