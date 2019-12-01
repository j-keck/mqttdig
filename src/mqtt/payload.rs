use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content="content")]
pub enum Payload {
    Json(String),
    Numeric(f64),
    Text(String),
    Binary(Vec<u8>),
}

impl Payload {
    pub fn new(vec: Vec<u8>) -> Payload {
        // try to parse it as a string
        if let Ok(s) = String::from_utf8(vec.clone()) {
            // try to parse as numeric
            if let Some(num) = &s.parse().ok() {
                return Payload::Numeric(*num);
            }

            // try to parse it as json
            if let Some(json) = serde_json::from_str::<serde_json::Value>(&s)
                .ok()
                .filter(|json| json.is_object() || json.is_array())
                .and_then(|json| serde_json::to_string_pretty(&json).ok())
            {
                return Payload::Json(json);
            }

            Payload::Text(s)
        } else {
            Payload::Binary(vec)
        }
    }
}
