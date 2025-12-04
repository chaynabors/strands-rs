#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ToolSpec {
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub input_schema: serde_json::Map<String, serde_json::Value>,
}

impl From<rmcp::model::Tool> for ToolSpec {
    fn from(tool: rmcp::model::Tool) -> Self {
        ToolSpec {
            name: tool.name.to_string(),
            display_name: tool.title.map(|t| t.to_string()),
            description: tool.description.map(|d| d.to_string()),
            input_schema: (*tool.input_schema).to_owned(),
        }
    }
}
