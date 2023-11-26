use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CheckOutput {
    pub title: String,
    pub summary: String,
    pub text: String,
}

impl CheckOutput {
    pub fn to_value(self) -> serde_json::Value {
        serde_json::to_value(self).expect("valid check run output")
    }
}
