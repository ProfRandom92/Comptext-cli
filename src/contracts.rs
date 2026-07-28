use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputItem {
    pub kind: String,
    pub value: String,
    pub digest: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputItem {
    pub kind: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentSpec {
    pub contract_name: String,
    pub schema_version: String,
    pub agent_spec_id: String,
    pub intent: String,
    pub goal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputItem>>,
    pub pipeline: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracts: Option<Vec<String>>,
    pub outputs: Vec<OutputItem>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeResolution {
    pub contract_name: String,
    pub schema_version: String,
    pub binary: String,
    pub args: Vec<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapabilityDecision {
    pub contract_name: String,
    pub schema_version: String,
    pub tool_name: String,
    pub policy: String,
    pub decision: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToolEvent {
    pub contract_name: String,
    pub schema_version: String,
    pub run_id: String,
    pub sequence: usize,
    pub event_type: String,
    pub tool_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_payload: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashItem {
    pub key: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvidenceEvent {
    pub contract_name: String,
    pub schema_version: String,
    pub event_id: String,
    pub run_id: String,
    pub sequence: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_event_hash: Option<String>,
    pub timestamp: String,
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<HashItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<HashItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub content_hash: String,
    pub event_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionContract {
    pub contract_name: String,
    pub schema_version: String,
    pub run_id: String,
    pub status: String,
    pub reason: String,
    pub final_sequence: usize,
    pub deterministic_root_hash: String,
    pub execution_chain_hash: String,
    pub replayable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplayEventItem {
    pub sequence: usize,
    pub content_hash: String,
    pub event_hash: String,
    pub event_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplayManifest {
    pub contract_name: String,
    pub schema_version: String,
    pub run_id: String,
    pub agent_spec_id: String,
    pub deterministic_root_hash: String,
    pub execution_chain_hash: String,
    pub event_count: usize,
    pub final_sequence: usize,
    pub events: Vec<ReplayEventItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorEnvelope {
    pub contract_name: String,
    pub schema_version: String,
    pub error_code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[allow(dead_code)]
pub fn jcs_serialize(val: &serde_json::Value) -> String {
    serde_jcs::to_string(val).unwrap_or_default()
}

pub fn jcs_hash(val: &serde_json::Value) -> String {
    let serialized = serde_jcs::to_vec(val).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(&serialized);
    format!("{:x}", hasher.finalize())
}

#[allow(clippy::too_many_arguments)]
pub fn compute_content_hash(
    contract_name: &str,
    schema_version: &str,
    sequence: usize,
    event_type: &str,
    action: &Option<String>,
    inputs: &Option<Vec<HashItem>>,
    outputs: &Option<Vec<HashItem>>,
    metadata: &Option<serde_json::Value>,
) -> String {
    let mut preimage = serde_json::Map::new();
    preimage.insert(
        "contract_name".to_string(),
        serde_json::Value::String(contract_name.to_string()),
    );
    preimage.insert(
        "schema_version".to_string(),
        serde_json::Value::String(schema_version.to_string()),
    );
    preimage.insert(
        "sequence".to_string(),
        serde_json::Value::Number(serde_json::Number::from(sequence)),
    );
    preimage.insert(
        "event_type".to_string(),
        serde_json::Value::String(event_type.to_string()),
    );
    if let Some(ref act) = action {
        preimage.insert("action".to_string(), serde_json::Value::String(act.clone()));
    }
    if let Some(ref inp) = inputs {
        preimage.insert("inputs".to_string(), serde_json::to_value(inp).unwrap());
    }
    if let Some(ref out) = outputs {
        preimage.insert("outputs".to_string(), serde_json::to_value(out).unwrap());
    }
    if let Some(ref meta) = metadata {
        preimage.insert("metadata".to_string(), meta.clone());
    }

    let val = serde_json::Value::Object(preimage);
    jcs_hash(&val)
}

pub fn compute_event_hash(
    content_hash: &str,
    parent_event_hash: &Option<String>,
    run_id: &str,
    event_id: &str,
    timestamp: &str,
) -> String {
    let mut preimage = serde_json::Map::new();
    preimage.insert(
        "content_hash".to_string(),
        serde_json::Value::String(content_hash.to_string()),
    );
    if let Some(ref parent) = parent_event_hash {
        preimage.insert(
            "parent_event_hash".to_string(),
            serde_json::Value::String(parent.clone()),
        );
    }
    preimage.insert(
        "run_id".to_string(),
        serde_json::Value::String(run_id.to_string()),
    );
    preimage.insert(
        "event_id".to_string(),
        serde_json::Value::String(event_id.to_string()),
    );
    preimage.insert(
        "timestamp".to_string(),
        serde_json::Value::String(timestamp.to_string()),
    );

    let val = serde_json::Value::Object(preimage);
    jcs_hash(&val)
}
