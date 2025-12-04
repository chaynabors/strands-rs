use rmcp::ServiceExt;
use rmcp::service::RunningService;
use rmcp::transport::StreamableHttpClientTransport;
use rmcp::{RoleClient, transport::TokioChildProcess};
use tokio::process::Command;

use crate::error::Result;
use crate::tool::ToolSpec;

#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error(transparent)]
    InitializeError(Box<rmcp::service::ClientInitializeError>),
    #[error(transparent)]
    ServiceError(Box<rmcp::service::ServiceError>),
}

impl From<rmcp::service::ClientInitializeError> for McpError {
    fn from(err: rmcp::service::ClientInitializeError) -> Self {
        Self::InitializeError(Box::new(err))
    }
}

impl From<rmcp::service::ServiceError> for McpError {
    fn from(err: rmcp::service::ServiceError) -> Self {
        Self::ServiceError(Box::new(err))
    }
}

pub enum TransportArgs {
    Stdio {
        command: String,
        args: Vec<String>,
    },
    StreamableHttp {
        url: String,
        api_key: Option<String>,
    },
}

pub struct McpClientArgs {
    pub name: String,
    pub version: String,
    pub transport: TransportArgs,
}

pub struct McpClient {
    name: String,
    version: String,
    service: RunningService<RoleClient, ()>,
    tool_specs: Vec<ToolSpec>,
}

impl McpClient {
    pub async fn new(args: McpClientArgs) -> Result<Self> {
        let service = match args.transport {
            TransportArgs::Stdio { command, args } => {
                let mut command = Command::new(command);
                command.args(args);
                ().serve(TokioChildProcess::new(command)?)
                    .await
                    .map_err(McpError::from)?
            }
            TransportArgs::StreamableHttp { url, api_key } => {
                ().serve(StreamableHttpClientTransport::from_uri(url))
                    .await
                    .map_err(McpError::from)?
            }
        };

        let tool_specs = service
            .list_all_tools()
            .await
            .map_err(McpError::from)?
            .into_iter()
            .map(ToolSpec::from)
            .collect();

        Ok(Self {
            name: args.name,
            version: args.version,
            service,
            tool_specs,
        })
    }

    pub fn tool_specs(&self) -> &[ToolSpec] {
        &self.tool_specs
    }
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
