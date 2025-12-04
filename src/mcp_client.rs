use rmcp::{RoleClient, service::DynService};

pub struct McpClient {
    name: String,
    version: String,
    service: Box<dyn DynService<RoleClient>>,
}

impl std::fmt::Debug for McpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("McpClient")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("service", &"<DynService>")
            .finish()
    }
}
