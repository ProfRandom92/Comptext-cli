use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RuntimeResolution {
    pub contract_name: String,
    pub schema_version: String,
    pub binary: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CapabilityDecision {
    pub contract_name: String,
    pub schema_version: String,
    pub tool_name: String,
    pub policy: String,
    pub decision: String,
}

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
    pub evidence_root_hash: String,
    pub replayable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplayEventItem {
    pub sequence: usize,
    pub event_hash: String,
    pub event_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplayManifest {
    pub contract_name: String,
    pub schema_version: String,
    pub run_id: String,
    pub agent_spec_id: String,
    pub evidence_root_hash: String,
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

pub fn jcs_serialize(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::Object(map) => {
            let mut parts = Vec::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for k in keys {
                let v = map.get(k).unwrap();
                parts.push(format!("\"{}\":{}", k, jcs_serialize(v)));
            }
            format!("{{{}}}", parts.join(","))
        }
        serde_json::Value::Array(arr) => {
            let parts: Vec<String> = arr.iter().map(jcs_serialize).collect();
            format!("[{}]", parts.join(","))
        }
        serde_json::Value::String(s) => {
            serde_json::to_string(s).unwrap_or_default()
        }
        serde_json::Value::Number(n) => {
            n.to_string()
        }
        serde_json::Value::Bool(b) => {
            b.to_string()
        }
        serde_json::Value::Null => {
            "null".to_string()
        }
    }
}

pub fn jcs_hash(val: &serde_json::Value) -> String {
    let serialized = jcs_serialize(val);
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    format!("{:x}", hasher.finalize())
}
