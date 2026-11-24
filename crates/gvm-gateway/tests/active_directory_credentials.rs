// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG
//! HTTP contract fixtures only: no LDAP bind or live qualification.
use async_trait::async_trait;
use gvm_gateway_app::{GatewayPorts, GatewayService};
use gvm_gateway_domain::*;
use gvm_gateway_gvmd::StaticGvmdAdapter;
use gvm_gateway_rest::router::build_router;
use reqwest::{Client, StatusCode};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
struct AdFixturePort(Arc<Mutex<Vec<VerifyActiveDirectoryInput>>>, Option<u16>);
#[async_trait]
impl CredentialPort for AdFixturePort {
    async fn verify_active_directory(
        &self,
        _: &str,
        input: VerifyActiveDirectoryInput,
    ) -> Result<(), GatewayError> {
        self.0.lock().unwrap().push(input);
        // Typed AD adapter errors have already removed upstream diagnostics.
        match self.1 {
            Some(409) => Err(GatewayError::Conflict(
                "Active Directory operation failed".into(),
            )),
            Some(500) => Err(GatewayError::Internal(
                "Active Directory operation failed".into(),
            )),
            Some(504) => Err(GatewayError::GatewayTimeout(
                "Active Directory operation failed".into(),
            )),
            _ => Ok(()),
        }
    }

    async fn list_credential_stores(&self, _: &str) -> Result<Vec<CredentialStore>, GatewayError> {
        Err(GatewayError::Internal("unexpected fixture call".into()))
    }

    async fn get_credential_store(
        &self,
        _: &str,
        _: &str,
    ) -> Result<CredentialStore, GatewayError> {
        Err(GatewayError::Internal("unexpected fixture call".into()))
    }

    async fn modify_credential_store(
        &self,
        _: &str,
        _: &str,
        _: ModifyCredentialStoreInput,
    ) -> Result<CredentialStore, GatewayError> {
        Err(GatewayError::Internal("unexpected fixture call".into()))
    }

    async fn verify_credential_store(&self, _: &str, _: &str) -> Result<(), GatewayError> {
        Err(GatewayError::Internal("unexpected fixture call".into()))
    }

    async fn list_credentials(
        &self,
        _: &str,
        query: &CredentialQuery,
    ) -> Result<CredentialPage, GatewayError> {
        Ok(CredentialPage {
            data: vec![],
            pagination: Pagination {
                page: query.page,
                per_page: query.per_page,
                total: 0,
                total_pages: 0,
            },
        })
    }

    async fn create_credential(
        &self,
        _: &str,
        _: CreateCredentialInput,
    ) -> Result<String, GatewayError> {
        Err(GatewayError::Internal(
            "not implemented in test credential port".to_string(),
        ))
    }

    async fn get_credential(&self, _: &str, id: &str) -> Result<Credential, GatewayError> {
        Err(GatewayError::NotFound(format!("credential {id} not found")))
    }

    async fn modify_credential(
        &self,
        _: &str,
        id: &str,
        _: ModifyCredentialInput,
    ) -> Result<Credential, GatewayError> {
        Err(GatewayError::NotFound(format!("credential {id} not found")))
    }

    async fn delete_credential(&self, _: &str, id: &str, _: bool) -> Result<(), GatewayError> {
        Err(GatewayError::NotFound(format!("credential {id} not found")))
    }
}
async fn spawn_credential_server(
    credential_port: Arc<dyn CredentialPort>,
) -> (std::net::SocketAddr, String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let sessions = Arc::new(SessionManager::default());
    let token = sessions.create("admin").unwrap().token;
    let adapter = Arc::new(StaticGvmdAdapter::ready("22.7"));
    let service = GatewayService::new(
        GatewayPorts {
            system: adapter.clone(),
            alerts: adapter.clone(),
            schedules: adapter.clone(),
            credentials: credential_port,
            port_lists: adapter.clone(),
            feeds: adapter.clone(),
            identity: adapter.clone(),
            targets: adapter.clone(),
            tasks: adapter.clone(),
            auth: adapter.clone(),
            reports: adapter.clone(),
            results: adapter.clone(),
            scan_configs: adapter.clone(),
            scanners: adapter.clone(),
            agents: adapter.clone(),
            supporting_resources: adapter,
        },
        sessions,
    );
    let app = build_router(service);
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    (addr, token, handle)
}

#[tokio::test]
async fn ad_saved_and_inline_verification_are_status_only_and_do_not_expose_secrets() {
    // The accepting test port models a typed status-only result, never a real bind.
    let calls = Arc::new(Mutex::new(Vec::new()));
    let (addr, token, handle) =
        spawn_credential_server(Arc::new(AdFixturePort(calls.clone(), None))).await;
    let client = Client::new();
    let saved = client.post(format!("http://{addr}/gvm/public/v1/credentials/123e4567-e89b-12d3-a456-426614174000/actions/verify-active-directory")).bearer_auth(&token).send().await.unwrap();
    assert_eq!(saved.status(), StatusCode::NO_CONTENT);
    assert_eq!(saved.headers()["cache-control"], "no-store");
    assert!(!saved.headers().contains_key("location"));
    assert!(saved.bytes().await.unwrap().is_empty());
    let inline = client.post(format!("http://{addr}/gvm/public/v1/active-directory/credential-verifications")).bearer_auth(&token).json(&serde_json::json!({"activeDirectory":{"host":"fixture.invalid","bindDn":"CN=bind<&>","baseDn":"DC=fixture","bindPassword":"http-unique-secret","tlsMode":"starttls"}})).send().await.unwrap();
    assert_eq!(inline.status(), StatusCode::NO_CONTENT);
    assert_eq!(inline.headers()["cache-control"], "no-store");
    assert!(inline.bytes().await.unwrap().is_empty());
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 2);
    assert!(matches!(calls[0], VerifyActiveDirectoryInput::Saved(_)));
    let VerifyActiveDirectoryInput::Inline(fields) = &calls[1] else {
        panic!("inline form")
    };
    assert_eq!(fields.port, None);
    assert_eq!(fields.bind_dn.as_deref(), Some("CN=bind<&>"));
    assert!(!format!("{calls:?}").contains("http-unique-secret"));
    handle.abort();
}
#[tokio::test]
async fn ad_invalid_combinations_fail_before_dispatch_without_payload_echo() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let (addr, token, handle) =
        spawn_credential_server(Arc::new(AdFixturePort(calls.clone(), None))).await;
    for (path, body) in [
        (
            "/credentials/123e4567-e89b-12d3-a456-426614174000/actions/verify-active-directory",
            serde_json::json!({"bindPassword":"http-unique-secret"}),
        ),
        (
            "/active-directory/credential-verifications",
            serde_json::json!({"credentialId":"123e4567-e89b-12d3-a456-426614174000","activeDirectory":{"bindPassword":"http-unique-secret"}}),
        ),
        (
            "/active-directory/credential-verifications",
            serde_json::json!({"activeDirectory":{"host":"fixture.invalid","bindDn":"CN=bind","baseDn":"DC=fixture","bindPassword":"http-unique-secret","tlsMode":"ldap"}}),
        ),
    ] {
        let response = Client::new()
            .post(format!("http://{addr}/gvm/public/v1{path}"))
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(response.headers()["cache-control"], "no-store");
        assert!(!response
            .text()
            .await
            .unwrap()
            .contains("http-unique-secret"));
    }
    assert!(calls.lock().unwrap().is_empty());
    handle.abort();
}

#[tokio::test]
async fn ad_verification_generated_and_curated_errors_cover_runtime_categories() {
    // Both published operations must advertise every typed AD failure, not only success.
    let (addr, _, handle) = spawn_credential_server(Arc::new(AdFixturePort(
        Arc::new(Mutex::new(Vec::new())),
        None,
    )))
    .await;
    let generated: serde_json::Value = Client::new()
        .get(format!("http://{addr}/gvm/public/v1/openapi.json"))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    handle.abort();
    let curated: serde_json::Value =
        serde_yaml::from_str(include_str!("../../../spec/rest-api/credentials.yaml")).unwrap();
    for path in [
        "/credentials/{id}/actions/verify-active-directory",
        "/active-directory/credential-verifications",
    ] {
        let actual = &generated["paths"][path]["post"]["responses"];
        let expected = &curated["paths"][path]["post"]["responses"];
        let statuses = [
            "204", "400", "401", "403", "404", "409", "500", "501", "502", "504",
        ];
        assert_eq!(actual.as_object().unwrap().len(), statuses.len(), "{path}");
        assert_eq!(
            expected.as_object().unwrap().len(),
            statuses.len(),
            "{path}"
        );
        for status in statuses {
            assert!(actual.get(status).is_some(), "generated {path}: {status}");
            assert!(expected.get(status).is_some(), "curated {path}: {status}");
            if status != "204" {
                assert!(actual[status]["content"]["application/problem+json"].is_object());
            }
        }
    }
}

#[tokio::test]
async fn ad_saved_and_inline_errors_preserve_status_without_sensitive_payloads() {
    // Exercise the registered HTTP routes for conflict/internal/timeout typed failures.
    for (status, code) in [
        (409, "conflict"),
        (500, "internal_server_error"),
        (504, "gateway_timeout"),
    ] {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let (addr, token, handle) =
            spawn_credential_server(Arc::new(AdFixturePort(calls.clone(), Some(status)))).await;
        for path in [
            "/credentials/123e4567-e89b-12d3-a456-426614174000/actions/verify-active-directory",
            "/active-directory/credential-verifications",
        ] {
            let request = Client::new()
                .post(format!("http://{addr}/gvm/public/v1{path}"))
                .bearer_auth(&token);
            let request = if path.starts_with("/active-directory/") {
                request.json(&serde_json::json!({"activeDirectory":{"host":"private-host.invalid","bindDn":"CN=private-bind","baseDn":"DC=private-base","bindPassword":"http-unique-secret","tlsMode":"starttls"}}))
            } else {
                request
            };
            let response = request.send().await.unwrap();
            assert_eq!(response.status().as_u16(), status);
            assert_eq!(response.headers()["cache-control"], "no-store");
            assert_eq!(
                response.headers()["content-type"],
                "application/problem+json"
            );
            assert!(!response.headers().contains_key("location"));
            let text = response.text().await.unwrap();
            for sensitive in [
                "http-unique-secret",
                "private-host.invalid",
                "CN=private-bind",
                "DC=private-base",
                &token,
            ] {
                assert!(!text.contains(sensitive));
            }
            let problem: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(problem["status"], status);
            assert_eq!(problem["code"], code);
            assert_eq!(problem["instance"], format!("/gvm/public/v1{path}"));
        }
        assert_eq!(calls.lock().unwrap().len(), 2);
        handle.abort();
    }
}
