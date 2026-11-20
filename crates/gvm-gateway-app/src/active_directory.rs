// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Greenbone AG
//! Read-only directory discovery use case.
use crate::GatewayService;
use gvm_gateway_domain::{ActiveDirectorySearchInput, ActiveDirectorySearchResult, GatewayError};
impl GatewayService {
    /// Execute bounded discovery through the authenticated session queue.
    pub async fn search_active_directory_objects(
        &self,
        session_token: &str,
        input: ActiveDirectorySearchInput,
    ) -> Result<ActiveDirectorySearchResult, GatewayError> {
        input.validate()?;
        self.execute_with_resource(
            "active_directory.search",
            session_token,
            "search",
            "active_directory",
            None,
            |session| async move {
                self.credentials
                    .search_active_directory_objects(&session.token, input)
                    .await
            },
        )
        .await
    }
}
