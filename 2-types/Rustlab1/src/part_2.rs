use serde::{Deserialize, Serialize};

/// JSON-RPC request id can be a number, string, or null
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum RequestId {
    Num(i64),
    Str(String),
    Null,
}

/// params can be an array, object, or absent
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Params {
    Array(Vec<serde_json::Value>),
    Object(serde_json::Map<String, serde_json::Value>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Request {
    pub jsonrpc: String,
    pub method: String,

    #[serde(default)]
    pub params: Option<Params>,

    pub id: RequestId,
}

pub fn json_to_toml(json_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request: Request = serde_json::from_str(json_data)?;
    let toml_data = toml::to_string_pretty(&request)?;
    Ok(toml_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_jsonrpc_request() {
        let json = r#"{
    "jsonrpc": "2.0",
    "method": "getUser",
    "params": {"id": 42},
    "id": 1
}"#;


        let req: Request = serde_json::from_str(json).unwrap();
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "getUser");
        assert_eq!(req.id, RequestId::Num(1));
        assert!(req.params.is_some());
    }
}
