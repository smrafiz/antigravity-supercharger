use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HookInput {
    pub conversation_id: Option<String>,
    pub workspace_paths: Option<Vec<String>>,
    pub transcript_path: Option<String>,
    pub artifact_directory_path: Option<String>,
    pub model_name: Option<String>,
    pub last_user_input: Option<String>,
    pub hook_event_name: Option<String>,
    pub cwd: Option<String>,
    pub tool_call: Option<ToolCall>,
    pub tool_response: Option<serde_json::Value>,
    pub tool_result: Option<serde_json::Value>,
    pub tool_output: Option<serde_json::Value>,
    pub step_idx: Option<u64>,
    pub error: Option<String>,
    pub execution_num: Option<u64>,
    pub termination_reason: Option<String>,
    pub continuation_active: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    pub id: Option<String>,
    pub name: String,
    pub args: Option<HashMap<String, serde_json::Value>>,
}

impl ToolCall {
    pub fn get_str_arg(&self, key: &str) -> Option<String> {
        self.args.as_ref()?.get(key).and_then(|v| v.as_str().map(|s| s.to_string()))
    }
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreToolDecision {
    pub decision: String, // "allow" | "deny" | "ask" | "force_ask"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overrides: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<HashMap<String, serde_json::Value>>,
}

impl PreToolDecision {
    pub fn allow() -> Self {
        Self {
            decision: "allow".to_string(),
            reason: None,
            permission_overrides: None,
            overwrite: None,
        }
    }

    pub fn allow_with_overrides(overrides: Vec<String>) -> Self {
        Self {
            decision: "allow".to_string(),
            reason: None,
            permission_overrides: Some(overrides),
            overwrite: None,
        }
    }

    pub fn deny(reason: impl Into<String>) -> Self {
        Self {
            decision: "deny".to_string(),
            reason: Some(reason.into()),
            permission_overrides: None,
            overwrite: None,
        }
    }

    pub fn ask(reason: impl Into<String>) -> Self {
        Self {
            decision: "ask".to_string(),
            reason: Some(reason.into()),
            permission_overrides: None,
            overwrite: None,
        }
    }
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopDecision {
    pub decision: String, // "continue" | "stop"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl StopDecision {
    pub fn stop() -> Self {
        Self {
            decision: "stop".to_string(),
            reason: None,
        }
    }

    #[allow(dead_code)]
    pub fn continue_loop(reason: impl Into<String>) -> Self {
        Self {
            decision: "continue".to_string(),
            reason: Some(reason.into()),
        }
    }
}
