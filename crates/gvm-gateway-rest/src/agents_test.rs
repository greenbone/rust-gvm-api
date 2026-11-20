// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG

use super::*;

const RESOURCE_ID: &str = "550e8400-e29b-41d4-a716-446655440000";

#[test]
fn agent_query_decodes_filter_filter_id_and_pagination() {
    // Agent collection requests must preserve the shared list contract exactly
    // so REST-level filtering and paging reach the typed gvmd adapter intact.
    let query = parse_agent_query(
        "filter=status%3Dactive+and+name~demo&filterId=550e8400-e29b-41d4-a716-446655440000&page=2&perPage=50",
    )
    .expect("query should parse");

    assert_eq!(
        query.filter_string.as_deref(),
        Some("status=active and name~demo")
    );
    assert_eq!(query.filter_id.as_deref(), Some(RESOURCE_ID));
    assert_eq!(query.page, 2);
    assert_eq!(query.per_page, 50);
}

#[test]
fn agent_group_query_preserves_trash_flag() {
    // Trash listing is part of the public agent-group lifecycle and must not
    // be dropped when translating REST query strings into domain inputs.
    let query =
        parse_agent_group_query("trash=true&page=3&perPage=10").expect("query should parse");

    assert!(query.trash);
    assert_eq!(query.page, 3);
    assert_eq!(query.per_page, 10);
}

#[test]
fn support_bundle_query_rejects_zero_days() {
    // The REST boundary should reject nonsensical bundle windows before the
    // request reaches gvmd, keeping the contract precise and predictable.
    let error = parse_agent_support_bundle_query("days=0").expect_err("days=0 must fail");

    assert!(
        matches!(error, GatewayError::InvalidInput(message) if message.contains("greater than or equal to 1"))
    );
}

#[test]
fn support_bundle_query_preserves_encryption_tristate() {
    // The backend distinguishes an omitted encryption selector from explicit
    // false and true, so REST parsing must not apply a boolean default.
    let omitted = parse_agent_support_bundle_query("").expect("empty query should parse");
    let disabled =
        parse_agent_support_bundle_query("encryption=false").expect("false should parse");
    let enabled = parse_agent_support_bundle_query("encryption=true").expect("true should parse");

    assert_eq!(omitted.encryption, None);
    assert_eq!(disabled.encryption, Some(false));
    assert_eq!(enabled.encryption, Some(true));
}

#[test]
fn support_bundle_query_rejects_invalid_encryption() {
    // Invalid boolean text must fail at the public boundary instead of being
    // silently treated as omission or a backend default.
    let error = parse_agent_support_bundle_query("encryption=1")
        .expect_err("non-boolean encryption must fail");

    assert!(
        matches!(error, GatewayError::InvalidInput(message) if message.contains("encryption must be true or false"))
    );
}

#[test]
fn installer_instruction_query_defaults_language_and_requires_absolute_uri() {
    // Installer instructions must default language consistently, but they also
    // require an absolute origin URL because gvmd uses it to render the output.
    let parsed =
        parse_agent_installer_instruction_query("originUrl=https%3A%2F%2Fexample.com%2Fconsole")
            .expect("absolute origin URL should parse");
    assert_eq!(parsed.language, "en");
    assert_eq!(parsed.origin_url, "https://example.com/console");

    let error = parse_agent_installer_instruction_query("originUrl=%2Frelative%2Fpath")
        .expect_err("relative origin URL must fail");
    assert!(
        matches!(error, GatewayError::InvalidInput(message) if message.contains("absolute URI"))
    );
}

#[test]
fn create_agent_group_requires_trimmed_name_and_scheduler_cron_time() {
    // Agent-group creation should fail fast on empty required strings so the
    // API does not emit create commands that gvmd will reject later anyway.
    let error = CreateAgentGroupRequest {
        name: Some("  ".to_string()),
        scheduler_cron_time: Some(" ".to_string()),
        max_delay_seconds: None,
        comment: None,
        agent_ids: Vec::new(),
    }
    .validate_into()
    .expect_err("blank required fields must fail");

    assert!(
        matches!(error, GatewayError::InvalidInput(message) if message.contains("name is required"))
    );
}

#[test]
fn modify_agent_group_requires_scheduler_cron_time() {
    // The typed modify-agent-group builder requires a scheduler cron value, so
    // REST must enforce that contract instead of silently sending omissions.
    let error = ModifyAgentGroupRequest {
        scheduler_cron_time: None,
        max_delay_seconds: None,
        name: Some("renamed".to_string()),
        comment: None,
        agent_ids: None,
    }
    .validate_into()
    .expect_err("missing schedulerCronTime must fail");

    assert!(
        matches!(error, GatewayError::InvalidInput(message) if message.contains("schedulerCronTime is required"))
    );
}

#[test]
fn modify_agent_group_validates_agent_selection_update_semantics() {
    // Omission must preserve the current selection, a non-empty list remains a
    // supported replacement, and an explicit empty list must fail before gvmd.
    let omitted = ModifyAgentGroupRequest {
        scheduler_cron_time: Some("0 */15 * * *".to_string()),
        max_delay_seconds: None,
        name: None,
        comment: None,
        agent_ids: None,
    }
    .validate_into()
    .expect("omitted agentIds should preserve the current selection");
    assert_eq!(omitted.agent_ids, None);

    let replacement = Uuid::parse_str(RESOURCE_ID).expect("valid agent UUID");
    let replaced = ModifyAgentGroupRequest {
        scheduler_cron_time: Some("0 */15 * * *".to_string()),
        max_delay_seconds: None,
        name: None,
        comment: None,
        agent_ids: Some(vec![replacement]),
    }
    .validate_into()
    .expect("a non-empty agentIds list should remain supported");
    assert_eq!(replaced.agent_ids, Some(vec![RESOURCE_ID.to_string()]));

    let clear = ModifyAgentGroupRequest {
        scheduler_cron_time: Some("0 */15 * * *".to_string()),
        max_delay_seconds: None,
        name: None,
        comment: None,
        agent_ids: Some(vec![]),
    };
    assert!(matches!(
        clear.validate_into(),
        Err(GatewayError::InvalidInput(detail))
            if detail.contains("agentIds must contain at least one entry")
                && detail.contains("rust-gvm issue #812")
    ));
}

#[test]
fn attachment_filename_sanitizes_path_and_control_characters() {
    // Support-bundle downloads expose a backend-derived filename; sanitizing it
    // here prevents header injection and path-like values from leaking through.
    assert_eq!(
        safe_attachment_filename("../bundle\r\nname.zip"),
        "_bundle__name.zip"
    );
    assert_eq!(safe_attachment_filename("..."), "agent-support-bundle.bin");
}

#[test]
fn schedules_accept_legacy_and_structured_and_publish_only_structured() {
    // Tech Preview migration deliberately breaks response type, while preserving legacy writes.
    let request: AgentScriptExecutorConfigRequest = serde_json::from_value(serde_json::json!({
        "schedulerCronTime": ["0 */6 * * *", {"cron":"1 * * * *", "maxDelaySeconds":0},
            {"cron":"2 * * * *", "maxDelaySeconds":2147483647}]
    }))
    .unwrap();
    let domain = request.validate_into().unwrap();
    assert_eq!(domain.scheduler_cron_time[0].max_delay_seconds, None);
    assert_eq!(domain.scheduler_cron_time[1].max_delay_seconds, Some(0));
    let response = serde_json::to_value(AgentScriptExecutorConfigResponse::from(domain)).unwrap();
    assert_eq!(
        response["schedulerCronTime"][0],
        serde_json::json!({"cron":"0 */6 * * *"})
    );
    assert_eq!(
        response["schedulerCronTime"][2]["maxDelaySeconds"],
        2147483647
    );
}

#[test]
fn scheduler_delay_rejects_invalid_ranges_and_types_before_dispatch() {
    // Null, signed, overflowing and fractional delays violate the non-nullable contract.
    for value in [
        serde_json::Value::Null,
        serde_json::json!(-1),
        serde_json::json!(2147483648u64),
        serde_json::json!(u32::MAX),
        serde_json::json!(1.5),
    ] {
        let parsed =
            serde_json::from_value::<AgentScriptExecutorConfigRequest>(serde_json::json!({
                "schedulerCronTime":[{"cron":"* * * * *", "maxDelaySeconds":value}]
            }));
        assert!(parsed
            .map(|request| request.validate_into().is_err())
            .unwrap_or(true));
    }
}

#[test]
fn inventory_token_cache_policy_covers_success_and_failure() {
    // Confidential issuance and error responses must not be cached.
    for status in [
        axum::http::StatusCode::OK,
        axum::http::StatusCode::FORBIDDEN,
    ] {
        let response = inventory_token_no_store(status.into_response());
        assert_eq!(response.headers()[header::CACHE_CONTROL], "no-store");
    }
}

#[test]
fn group_delay_omission_zero_and_bounds_match_the_schedule_contract() {
    // Optional is not nullable: only omission and integers in the GMP range are valid.
    for delay in [
        None,
        Some(serde_json::json!(0)),
        Some(serde_json::json!(2147483647)),
    ] {
        let mut body = serde_json::json!({"name":"group","schedulerCronTime":"* * * * *"});
        if let Some(delay) = delay {
            body["maxDelaySeconds"] = delay;
        }
        assert!(
            serde_json::from_value::<CreateAgentGroupRequest>(body.clone())
                .unwrap()
                .validate_into()
                .is_ok()
        );
        assert!(serde_json::from_value::<ModifyAgentGroupRequest>(body)
            .unwrap()
            .validate_into()
            .is_ok());
    }
    for delay in [
        serde_json::Value::Null,
        serde_json::json!(-1),
        serde_json::json!(2147483648u64),
        serde_json::json!(u32::MAX),
        serde_json::json!(1.5),
    ] {
        let body = serde_json::json!({"name":"group","schedulerCronTime":"* * * * *","maxDelaySeconds":delay});
        assert!(
            serde_json::from_value::<CreateAgentGroupRequest>(body.clone())
                .map(|request| request.validate_into().is_err())
                .unwrap_or(true)
        );
        assert!(serde_json::from_value::<ModifyAgentGroupRequest>(body)
            .map(|request| request.validate_into().is_err())
            .unwrap_or(true));
    }
}

#[test]
fn agent_os_metadata_is_shared_by_detail_and_list_projection() {
    // Both endpoints use the same projection and preserve open read-only fields exactly.
    let domain: gvm_gateway_domain::Agent = serde_json::from_value(serde_json::json!({
        "id":RESOURCE_ID,"name":"Agent","writable":true,"inUse":false,
        "operatingSystem":"Future OS","architecture":"future64","osCpe":"cpe:/o:future","osStatus":"new-status"
    })).unwrap();
    let detail = serde_json::to_value(AgentResponse::from(domain.clone())).unwrap();
    let list = serde_json::to_value(AgentListResponse::from(gvm_gateway_domain::AgentPage {
        data: vec![domain],
        pagination: gvm_gateway_domain::Pagination {
            page: 1,
            per_page: 25,
            total: 1,
            total_pages: 1,
        },
    }))
    .unwrap();
    assert_eq!(list["data"][0], detail);
    assert_eq!(detail["osStatus"], "new-status");
}

#[test]
fn response_delay_observations_validate_without_write_range_truncation() {
    // A typed backend observation is not a write: project and serialize full
    // agent/group responses, then validate them against both public contracts.
    let generated = crate::router::build_openapi();
    let spec_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spec/rest-api");
    let mut curated: serde_json::Value =
        serde_yaml::from_str(&std::fs::read_to_string(spec_dir.join("emerging.yaml")).unwrap())
            .unwrap();
    let common: serde_json::Value =
        serde_yaml::from_str(&std::fs::read_to_string(spec_dir.join("common.yaml")).unwrap())
            .unwrap();
    for (name, schema) in common["components"]["schemas"].as_object().unwrap() {
        curated["components"]["schemas"]
            .as_object_mut()
            .unwrap()
            .entry(name.clone())
            .or_insert_with(|| schema.clone());
    }
    // Bundle the shared resource references used by agent/group response objects.
    let curated: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&curated).unwrap().replace(
            "./common.yaml#/components/schemas/",
            "#/components/schemas/",
        ))
        .unwrap();
    for delay in [0, 2147483648u32, u32::MAX] {
        let agent: gvm_gateway_domain::Agent = serde_json::from_value(serde_json::json!({
            "id": RESOURCE_ID, "name": "observed", "writable": true, "inUse": false,
            "config": {"agentScriptExecutor": {"schedulerCronTime": [
                {"cron": "* * * * *", "maxDelaySeconds": delay}
            ]}}
        }))
        .unwrap();
        let group: gvm_gateway_domain::AgentGroup = serde_json::from_value(serde_json::json!({
            "id": RESOURCE_ID, "name": "observed", "writable": true, "inUse": false,
            "schedulerCronTime": "* * * * *", "maxDelaySeconds": delay
        }))
        .unwrap();
        let agent = serde_json::to_value(AgentResponse::from(agent)).unwrap();
        let group = serde_json::to_value(AgentGroupResponse::from(group)).unwrap();
        assert_eq!(
            agent["config"]["agentScriptExecutor"]["schedulerCronTime"][0]["maxDelaySeconds"],
            delay
        );
        assert_eq!(group["maxDelaySeconds"], delay);
        for document in [&generated, &curated] {
            for (name, body) in [("Agent", &agent), ("AgentGroup", &group)] {
                let schema = serde_json::json!({
                    "$ref": format!("#/components/schemas/{name}"),
                    "components": document["components"]
                });
                let validator = jsonschema::validator_for(&schema).unwrap();
                assert!(
                    validator.is_valid(body),
                    "{name}: {:?}",
                    validator.iter_errors(body).collect::<Vec<_>>()
                );
            }
        }
    }
}
