// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG
use super::*;
const FULL: &str = r#"<get_active_directory_objects_response status="200" status_text="OK">
<objects truncated="1"><count><ou>1</ou><container>1</container><group>1</group><computer><total>2</total><disabled>1</disabled></computer></count>
<object type="ou"><name>R&amp;D</name><distinguished_name>OU=R&amp;D,DC=example</distinguished_name><parent_dn>DC=example</parent_dn><description>Research &lt;team&gt;</description></object>
<object type="container"><name>Computers</name><distinguished_name>CN=Computers,DC=example</distinguished_name><parent_dn>DC=example</parent_dn><description/></object>
<object type="group"><name>Scan group</name><distinguished_name>CN=Scan group,DC=example</distinguished_name><parent_dn/><description/></object>
<object type="computer"><name>PC1</name><distinguished_name>CN=PC1,OU=R&amp;D,DC=example</distinguished_name><parent_dn>OU=R&amp;D,DC=example</parent_dn><description>Desktop</description><dns_hostname>pc1.example</dns_hostname><operating_system>Windows</operating_system><operating_system_version>10.0</operating_system_version><service_principal_names><service_principal_name>HOST/pc1</service_principal_name><service_principal_name>HOST/pc1.example</service_principal_name></service_principal_names><disabled>1</disabled><targets><target id="target-1"><name>PC1 &amp; friends</name><tasks><task id="task-1"><name>Daily</name><config id="config-1"><name>Full &amp; fast</name></config></task><task id="task-2"><name>Hidden config</name></task></tasks></target><target id="target-2"><name>No tasks</name><tasks/></target></targets></object>
<object type="computer"><name>PC2</name><distinguished_name>CN=PC2,DC=example</distinguished_name><parent_dn>DC=example</parent_dn><description/><service_principal_names/><disabled>0</disabled><targets/></object>
</objects></get_active_directory_objects_response>"#;

fn parsed(xml: &str) -> domain::ActiveDirectorySearchResult {
    result_from_gmp(
        response::GetActiveDirectoryObjectsResponse::from_response(&gvm_protocol::Response::from(
            xml,
        ))
        .unwrap(),
    )
}
#[test]
fn typed_projection_preserves_every_kind_counts_and_reference_visibility() {
    // Exercise the gateway mapping using upstream typed parsing, not local XML logic.
    let objects = parsed(FULL).objects.unwrap();
    assert!(objects.truncated);
    assert_eq!(
        (
            objects.counts.ou,
            objects.counts.container,
            objects.counts.group,
            objects.counts.computer.total,
            objects.counts.computer.disabled
        ),
        (1, 1, 1, 2, Some(1))
    );
    let kinds: Vec<_> = objects.items.iter().map(|v| v.object_type).collect();
    assert_eq!(
        kinds,
        vec![
            domain::ActiveDirectoryObjectType::Ou,
            domain::ActiveDirectoryObjectType::Container,
            domain::ActiveDirectoryObjectType::Group,
            domain::ActiveDirectoryObjectType::Computer,
            domain::ActiveDirectoryObjectType::Computer
        ]
    );
    assert_eq!(objects.items[0].distinguished_name, "OU=R&D,DC=example");
    assert_eq!(objects.items[0].description, "Research <team>");
    assert!(objects.items[0].targets.is_none());
    let pc = &objects.items[3];
    assert_eq!(pc.dns_hostname.as_deref(), Some("pc1.example"));
    assert_eq!(pc.operating_system.as_deref(), Some("Windows"));
    assert_eq!(pc.operating_system_version.as_deref(), Some("10.0"));
    assert_eq!(
        pc.service_principal_names.as_ref().unwrap(),
        &["HOST/pc1", "HOST/pc1.example"]
    );
    assert_eq!(pc.disabled, Some(true));
    let targets = pc.targets.as_ref().unwrap();
    let tasks = targets[0].tasks.as_ref().unwrap();
    assert_eq!(tasks[0].config.as_ref().unwrap().name, "Full & fast");
    assert!(tasks[1].config.is_none());
    assert_eq!(targets[1].tasks, Some(vec![]));
    let pc = &objects.items[4];
    assert_eq!(pc.disabled, Some(false));
    assert_eq!(pc.service_principal_names, Some(vec![]));
    assert_eq!(pc.targets, Some(vec![]));
    assert!(!format!("{objects:?}").contains("example"));
}
#[test]
fn absent_and_empty_truncated_subsets_do_not_conflate() {
    // Missing envelope and unavailable disabled count are not defaulted to zero.
    assert!(
        parsed("<get_active_directory_objects_response status=\"200\" status_text=\"OK\"/>")
            .objects
            .is_none()
    );
    for truncated in [false, true] {
        let xml = format!(
            "<get_active_directory_objects_response status=\"200\" status_text=\"OK\"><objects truncated=\"{}\"><count><ou>0</ou><container>0</container><group>0</group><computer><total>0</total></computer></count></objects></get_active_directory_objects_response>",
            u8::from(truncated)
        );
        let objects = parsed(&xml).objects.unwrap();
        assert_eq!(objects.truncated, truncated);
        assert!(objects.items.is_empty());
        assert_eq!(objects.counts.computer.disabled, None);
    }
}
#[test]
fn upstream_request_defaults_are_retained_and_only_read_command_is_selected() {
    // No create/modify request exists on the discovery execution path.
    use gvm_gmp::GmpRequestCodec;
    let input = domain::ActiveDirectorySearchInput {
        credential_id: "123e4567-e89b-12d3-a456-426614174000".into(),
        scope: None,
        max_results: None,
        include_disabled_computers: None,
        targets: None,
        tasks: None,
        details: None,
        base_dn: None,
        group_dn: None,
        object_types: None,
    };
    let request = request_from_input(input).unwrap();
    assert!(
        request.scope.is_none()
            && request.max_results.is_none()
            && request.include_disabled_computers.is_none()
            && request.targets.is_none()
            && request.tasks.is_none()
            && request.details.is_none()
            && request.object_types.is_none()
    );
    assert_eq!(
        request.command(),
        Some(gvm_gmp::GmpCommand::new("get_active_directory_objects"))
    );
}
#[test]
fn denial_discovery_policy_and_tls_errors_remain_sanitized_failures() {
    // A status-400 permission error is not caller syntax; policy/TLS failure is not empty success.
    assert!(matches!(
        map_search_error(GvmError::Server {
            status: 400,
            message: "Permission denied".into()
        }),
        GatewayError::Forbidden(_)
    ));
    assert!(matches!(
        map_search_error(GvmError::CommandNotAdvertised {
            command: "get_active_directory_objects".into()
        }),
        GatewayError::NotImplemented(_)
    ));
    assert!(matches!(
        map_search_error(GvmError::Timeout(std::time::Duration::from_secs(1))),
        GatewayError::GatewayTimeout(_)
    ));
    assert!(matches!(
        map_search_error(GvmError::Server {
            status: 404,
            message: "hidden DN".into()
        }),
        GatewayError::NotFound(_)
    ));
    for message in [
        "LDAP bind failed for CN=secret",
        "TLS certificate secret",
        "Destination not allowed: secret",
    ] {
        let error = map_search_error(GvmError::Server {
            status: 400,
            message: message.into(),
        });
        assert!(matches!(error, GatewayError::BackendUnavailable(_)));
        assert!(!format!("{error:?}").contains("secret"));
    }
}

#[test]
fn ad_backend_internal_conflict_and_disabled_statuses_are_not_empty_success() {
    // Verification and search must share stable sanitized category mappings.
    assert!(matches!(
        map_search_error(GvmError::Server {
            status: 500,
            message: "LDAP secret".into()
        }),
        GatewayError::Internal(_)
    ));
    assert!(matches!(
        map_search_error(GvmError::Server {
            status: 409,
            message: "LDAP secret".into()
        }),
        GatewayError::Conflict(_)
    ));
    assert!(matches!(
        map_search_error(GvmError::Server {
            status: 503,
            message: "Command disabled".into()
        }),
        GatewayError::NotImplemented(_)
    ));
}

#[tokio::test]
async fn built_in_mock_is_not_live_ad_and_search_never_creates_targets_or_tasks() {
    // Command history proves the controller has no write fallback. The upstream
    // built-in mock deliberately cannot assert LDAP/TLS interoperability.
    use gvm_mock_server::{GmpVersion, MockGmpServer, ServerMode};
    let server = MockGmpServer::builder()
        .mode(ServerMode::Stateful)
        .version(GmpVersion::V22_8)
        .unix_socket_auto()
        .build()
        .await
        .unwrap();
    let adapter = GvmdAdapter::unix_socket(server.socket_path().unwrap());
    adapter
        .connect_session("ad-search-test", "admin", "admin")
        .await
        .unwrap();
    server.clear_history();
    let input = domain::ActiveDirectorySearchInput {
        credential_id: "123e4567-e89b-12d3-a456-426614174000".into(),
        scope: None,
        max_results: None,
        include_disabled_computers: None,
        targets: Some(true),
        tasks: Some(true),
        details: None,
        base_dn: None,
        group_dn: None,
        object_types: None,
    };
    let result = adapter
        .search_active_directory_objects("ad-search-test", input)
        .await;
    assert!(
        result.is_err(),
        "built-in mock must never fake LDAP discovery success"
    );
    let history = server.command_history();
    assert!(!history.is_empty());
    assert!(history.iter().all(|record| matches!(
        record.command_name(),
        "help" | "get_active_directory_objects"
    )));
    server.shutdown().await;
}
