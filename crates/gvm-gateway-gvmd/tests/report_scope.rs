//! Regression coverage for endpoint-owned report isolation (issue #741).
use gvm_gateway_domain::{GatewayError, ReportPort, ResultQuery};
use gvm_gateway_gvmd::GvmdAdapter;
use gvm_mock_server::{GmpVersion, MockGmpServer, ServerMode};

const PATH_REPORT: &str = "aabbccdd-0000-4000-8000-000000000001";
const OTHER_REPORT: &str = "aabbccdd-0000-4000-8000-000000000002";

async fn adapter_with_result(report_id: &str) -> (GvmdAdapter, MockGmpServer) {
    let response = format!(
        r#"<get_results_response status="200" status_text="OK"><result id="11111111-1111-4111-8111-111111111111"><name>Scoped result</name><severity>8.0</severity><report id="{report_id}"><name>Report</name></report></result><result_count>1<filtered>1</filtered></result_count></get_results_response>"#
    );
    let server = MockGmpServer::builder()
        .mode(ServerMode::Fixture)
        .version(GmpVersion::V22_8)
        .override_response("get_results", &response)
        .unix_socket_auto()
        .build()
        .await
        .expect("start fixture server");
    let adapter = GvmdAdapter::unix_socket(server.socket_path().expect("socket"));
    adapter
        .connect_session("scope-session", "admin", "admin")
        .await
        .expect("authenticate");
    (adapter, server)
}

#[tokio::test]
async fn report_results_fail_closed_when_backend_returns_a_different_report() {
    // A misbehaving backend must not leak rows or return misleading filtered
    // counts, even when it responds with a syntactically successful payload.
    let (adapter, server) = adapter_with_result(OTHER_REPORT).await;
    let result = adapter
        .get_report_results(
            "scope-session",
            PATH_REPORT,
            &ResultQuery {
                page: 1,
                per_page: 25,
                ..Default::default()
            },
        )
        .await;
    assert!(
        matches!(result, Err(GatewayError::BackendUnavailable(detail))
        if detail == "backend returned results outside the requested report")
    );
    server.shutdown().await;
}

#[tokio::test]
async fn report_scope_accepts_equivalent_uuid_case_without_rewriting_payload() {
    // UUID spelling is not identity: an uppercase requested path remains the
    // same report while the server's original lowercase spelling is retained.
    let (adapter, server) = adapter_with_result(PATH_REPORT).await;
    let result = adapter
        .get_report_results(
            "scope-session",
            &PATH_REPORT.to_uppercase(),
            &ResultQuery {
                page: 1,
                per_page: 25,
                ..Default::default()
            },
        )
        .await
        .expect("same report");
    assert_eq!(result.data.len(), 1);
    assert_eq!(
        result.data[0].report.as_ref().expect("report").id,
        PATH_REPORT
    );
    server.shutdown().await;
}

#[tokio::test]
async fn protected_scope_applies_to_boolean_results_counts_and_saved_filters() {
    // Each report has high and low findings. The low OR arm must not escape
    // the path report, and pagination counts must be scoped before slicing.
    let server = MockGmpServer::builder()
        .mode(ServerMode::Stateful)
        .version(GmpVersion::V22_8)
        .seed(|store| {
            for report in [PATH_REPORT, OTHER_REPORT] {
                let report_id = uuid::Uuid::parse_str(report).unwrap();
                store.create(gvm_mock_server::Resource::with_id(
                    "report", "Report", report_id,
                ));
                for (index, severity) in [(0, "8.0"), (1, "5.0")] {
                    let mut result = gvm_mock_server::Resource::with_id(
                        "result",
                        "Finding",
                        uuid::Uuid::new_v5(&report_id, &[index]),
                    );
                    result.set_attr("report_id", report);
                    result.set_attr("severity", severity);
                    store.create(result);
                }
            }
            let mut filter = gvm_mock_server::Resource::with_id(
                "filter",
                "Saved alternative",
                uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap(),
            );
            filter.set_attr("term", "severity>7 OR severity<6");
            filter.set_attr("type", "result");
            store.create(filter);
        })
        .unix_socket_auto()
        .build()
        .await
        .unwrap();
    let adapter = GvmdAdapter::unix_socket(server.socket_path().unwrap());
    adapter
        .connect_session("scope-session", "admin", "admin")
        .await
        .unwrap();
    for saved in [false, true] {
        for page in [1, 2] {
            let query = ResultQuery {
                filter_string: (!saved).then(|| "severity>7 OR severity<6".to_string()),
                filter_id: saved.then(|| "123e4567-e89b-12d3-a456-426614174000".to_string()),
                page,
                per_page: 1,
            };
            let results = adapter
                .get_report_results("scope-session", PATH_REPORT, &query)
                .await
                .unwrap();
            assert_eq!(results.pagination.total, 2, "saved={saved}, page={page}");
            assert_eq!(results.data.len(), 1);
            assert_eq!(results.data[0].report.as_ref().unwrap().id, PATH_REPORT);
        }
    }
    server.shutdown().await;
}

#[tokio::test]
async fn invalid_report_filter_controls_fail_before_result_execution() {
    // Unsupported comparators and endpoint-owned controls are client errors,
    // not silently ignored predicates or backend result requests.
    let (adapter, server) = adapter_with_result(PATH_REPORT).await;
    for filter in [
        "severity>=7",
        "severity<=7",
        "severity==7",
        "report_id=x",
        "_and_report_id=x",
        "first=9",
        "rows=100",
    ] {
        server.clear_history();
        let result = adapter
            .get_report_results(
                "scope-session",
                PATH_REPORT,
                &ResultQuery {
                    filter_string: Some(filter.to_string()),
                    page: 1,
                    per_page: 25,
                    ..Default::default()
                },
            )
            .await;
        assert!(
            matches!(result, Err(GatewayError::InvalidInput(_))),
            "{filter}: {result:?}"
        );
        assert!(!server
            .command_history()
            .iter()
            .any(|record| record.command_name() == "get_results"));
    }
    server.shutdown().await;
}
