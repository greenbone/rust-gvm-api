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
