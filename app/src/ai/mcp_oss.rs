use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp_core::ui::Icon;
use warp_core::ui::appearance::Appearance;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::cloud_object::model::generic_string_model::{
    GenericStringModel, GenericStringObjectId, StringModel,
};
use crate::cloud_object::model::json_model::{JsonModel, JsonSerializer};
use crate::cloud_object::{
    GenericCloudObject, GenericStringObjectFormat, GenericStringObjectUniqueKey, JsonObjectType,
    Revision, ServerCloudObject, UniquePer,
};
use crate::drive::items::WarpDriveItem;
use crate::rmcp;
use crate::server::ids::SyncId;
use crate::server::sync_queue::QueueItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MCPProvider {
    Warp,
    Claude,
    Codex,
    Agents,
}

impl MCPProvider {
    pub fn display_name(&self) -> &str {
        match self {
            MCPProvider::Warp => "Warp",
            MCPProvider::Claude => "Claude",
            MCPProvider::Codex => "Codex",
            MCPProvider::Agents => "Other Agents",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            MCPProvider::Warp => Icon::Warp,
            MCPProvider::Claude => Icon::ClaudeLogo,
            MCPProvider::Codex => Icon::OpenAILogo,
            MCPProvider::Agents => Icon::Warp,
        }
    }

    pub fn home_config_path(&self) -> &'static Path {
        match self {
            MCPProvider::Warp => Path::new(".warp/.mcp.json"),
            MCPProvider::Claude => Path::new(".claude.json"),
            MCPProvider::Codex => Path::new(".codex/config.toml"),
            MCPProvider::Agents => Path::new(".agents/.mcp.json"),
        }
    }

    pub fn project_config_path(&self) -> &'static Path {
        match self {
            MCPProvider::Warp => Path::new(".warp/.mcp.json"),
            MCPProvider::Claude => Path::new(".mcp.json"),
            MCPProvider::Codex => Path::new(".codex/config.toml"),
            MCPProvider::Agents => Path::new(".agents/.mcp.json"),
        }
    }
}

pub(crate) fn home_config_file_path(provider: MCPProvider) -> Option<PathBuf> {
    match provider {
        MCPProvider::Warp => warp_core::paths::warp_home_mcp_config_file_path(),
        _ => dirs::home_dir().map(|home_dir| home_dir.join(provider.home_config_path())),
    }
}

pub fn mcp_provider_from_file_path(file_path: &Path) -> Option<MCPProvider> {
    for provider in [
        MCPProvider::Warp,
        MCPProvider::Claude,
        MCPProvider::Codex,
        MCPProvider::Agents,
    ] {
        if home_config_file_path(provider)
            .as_ref()
            .is_some_and(|home_config_path| file_path == home_config_path)
        {
            return Some(provider);
        }
    }

    let mut best: Option<(MCPProvider, usize)> = None;
    for provider in [
        MCPProvider::Warp,
        MCPProvider::Claude,
        MCPProvider::Codex,
        MCPProvider::Agents,
    ] {
        let config_path = provider.project_config_path();
        if file_path.ends_with(config_path) {
            let len = config_path.as_os_str().len();
            if best.is_none_or(|(_, best_len)| len > best_len) {
                best = Some((provider, len));
            }
        }
    }
    best.map(|(provider, _)| provider)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JSONMCPServer {
    #[serde(flatten)]
    pub transport_type: JSONTransportType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONTransportType {
    CLIServer {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
        #[serde(default)]
        working_directory: Option<String>,
    },
    SSEServer {
        #[serde(alias = "serverUrl")]
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MCPServer {
    pub transport_type: TransportType,
    pub name: String,
    #[serde(default)]
    pub uuid: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCPServerState {
    NotRunning,
    Starting,
    Authenticating,
    Running,
    ShuttingDown,
    FailedToStart,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportType {
    CLIServer(CLIServer),
    ServerSentEvents(ServerSentEvents),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CLIServer {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    pub cwd_parameter: Option<String>,
    pub static_env_vars: Vec<StaticEnvVar>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaticEnvVar {
    pub name: String,
    #[serde(skip_serializing, default)]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaticHeader {
    pub name: String,
    #[serde(skip_serializing, default)]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerSentEvents {
    pub url: String,
    #[serde(default)]
    pub headers: Vec<StaticHeader>,
}

pub type CloudMCPServer = GenericCloudObject<GenericStringObjectId, CloudMCPServerModel>;
pub type CloudMCPServerModel = GenericStringModel<MCPServer, JsonSerializer>;

impl StringModel for MCPServer {
    type CloudObjectType = CloudMCPServer;

    fn model_type_name(&self) -> &'static str {
        "MCP server"
    }

    fn should_enforce_revisions() -> bool {
        true
    }

    fn model_format() -> GenericStringObjectFormat {
        GenericStringObjectFormat::Json(JsonObjectType::MCPServer)
    }

    fn should_show_activity_toasts() -> bool {
        true
    }

    fn warn_if_unsaved_at_quit() -> bool {
        true
    }

    fn display_name(&self) -> String {
        self.name.clone()
    }

    fn update_object_queue_item(
        &self,
        revision_ts: Option<Revision>,
        object: &Self::CloudObjectType,
    ) -> QueueItem {
        QueueItem::UpdateMCPServer {
            model: object.model().clone().into(),
            id: object.id,
            revision: revision_ts.or_else(|| object.metadata.revision.clone()),
        }
    }

    fn new_from_server_update(&self, server_cloud_object: &ServerCloudObject) -> Option<Self> {
        if let ServerCloudObject::MCPServer(server_mcp_server) = server_cloud_object {
            return Some(server_mcp_server.model.clone().string_model);
        }
        None
    }

    fn uniqueness_key(&self) -> Option<GenericStringObjectUniqueKey> {
        None
    }

    fn renders_in_warp_drive(&self) -> bool {
        false
    }

    fn to_warp_drive_item(
        &self,
        _id: SyncId,
        _appearance: &Appearance,
        _mcp_server: &CloudMCPServer,
    ) -> Option<Box<dyn WarpDriveItem>> {
        None
    }
}

impl JsonModel for MCPServer {
    fn json_object_type() -> JsonObjectType {
        JsonObjectType::MCPServer
    }
}

#[derive(Debug, Clone)]
pub enum Author {
    CurrentUser,
    OtherUser { name: String },
    Unknown,
}

#[derive(Debug, Clone)]
pub enum MCPServerUpdate {
    CloudTemplate {
        publisher: Author,
        new_version_ts: i64,
        json_template: JsonTemplate,
    },
    Gallery {
        name: String,
        new_version: i32,
        json_template: JsonTemplate,
    },
}

pub mod templatable {
    use super::*;

    const UNIQUENESS_KEY_PREFIX: &str = "templatable_mcp_server";

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Hash)]
    pub struct JsonTemplate {
        pub json: String,
        pub variables: Vec<TemplateVariable>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
    pub struct TemplateVariable {
        pub key: String,
        #[serde(default)]
        pub allowed_values: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GalleryData {
        pub gallery_item_id: Uuid,
        pub version: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    pub struct TemplatableMCPServer {
        pub uuid: Uuid,
        pub name: String,
        pub description: Option<String>,
        pub template: JsonTemplate,
        #[serde(default)]
        pub version: i64,
        pub gallery_data: Option<GalleryData>,
    }

    #[derive(Debug)]
    pub enum FromStoredJsonError {
        NoServersFound,
        TooManyServersFound,
        ParseError(serde_json::Error),
    }

    impl TemplatableMCPServer {
        pub fn find_template_map(
            config: serde_json::Value,
        ) -> serde_json::Result<HashMap<String, serde_json::Value>> {
            for pointer in ["/mcp/servers", "/servers", "/mcpServers", "/mcp_servers"] {
                if let Some(value) = config.pointer(pointer) {
                    if let Ok(servers) =
                        serde_json::from_value::<HashMap<String, serde_json::Value>>(value.clone())
                    {
                        return Ok(servers);
                    }
                }
            }
            serde_json::from_value::<HashMap<String, serde_json::Value>>(config)
        }

        pub fn find_template_map_strict(
            config: &serde_json::Value,
        ) -> HashMap<String, serde_json::Value> {
            for pointer in ["/mcp/servers", "/servers", "/mcpServers", "/mcp_servers"] {
                if let Some(value) = config.pointer(pointer) {
                    if let Ok(servers) =
                        serde_json::from_value::<HashMap<String, serde_json::Value>>(value.clone())
                    {
                        return servers;
                    }
                }
            }
            HashMap::new()
        }

        pub fn to_user_json(&self) -> String {
            serde_json::to_string_pretty(
                &serde_json::from_str::<serde_json::Value>(&self.template.json).unwrap_or_default(),
            )
            .unwrap_or_default()
        }

        pub fn from_stored_json(
            json: &str,
            uuid: Uuid,
        ) -> Result<TemplatableMCPServer, FromStoredJsonError> {
            let templates = Self::from_user_json(json).map_err(FromStoredJsonError::ParseError)?;
            match templates.as_slice() {
                [] => Err(FromStoredJsonError::NoServersFound),
                [template] => {
                    let mut template = template.clone();
                    template.uuid = uuid;
                    Ok(template)
                }
                _ => Err(FromStoredJsonError::TooManyServersFound),
            }
        }

        pub fn from_user_json(json: &str) -> serde_json::Result<Vec<TemplatableMCPServer>> {
            let json = json.trim();
            let json = if json.starts_with('{') {
                json.to_owned()
            } else {
                format!("{{{json}}}")
            };
            let config: serde_json::Value = serde_json::from_str(&json)?;
            let templates = Self::find_template_map(config)?;
            Ok(templates
                .iter()
                .map(|(name, json)| {
                    let normalized_json =
                        serde_json::Value::Object(serde_json::Map::from_iter([(
                            name.to_owned(),
                            json.clone(),
                        )]))
                        .to_string();
                    TemplatableMCPServer {
                        uuid: Uuid::new_v4(),
                        name: name.to_owned(),
                        description: json
                            .get("description")
                            .and_then(|value| value.as_str().map(ToOwned::to_owned)),
                        template: JsonTemplate {
                            json: normalized_json,
                            variables: Vec::new(),
                        },
                        version: 0,
                        gallery_data: None,
                    }
                })
                .collect())
        }
    }

    pub type CloudTemplatableMCPServer =
        GenericCloudObject<GenericStringObjectId, CloudTemplatableMCPServerModel>;
    pub type CloudTemplatableMCPServerModel =
        GenericStringModel<TemplatableMCPServer, JsonSerializer>;

    impl StringModel for TemplatableMCPServer {
        type CloudObjectType = CloudTemplatableMCPServer;

        fn model_type_name(&self) -> &'static str {
            "MCP server"
        }

        fn should_enforce_revisions() -> bool {
            true
        }

        fn model_format() -> GenericStringObjectFormat {
            GenericStringObjectFormat::Json(JsonObjectType::TemplatableMCPServer)
        }

        fn should_show_activity_toasts() -> bool {
            true
        }

        fn warn_if_unsaved_at_quit() -> bool {
            true
        }

        fn display_name(&self) -> String {
            self.name.clone()
        }

        fn update_object_queue_item(
            &self,
            revision_ts: Option<Revision>,
            object: &Self::CloudObjectType,
        ) -> QueueItem {
            QueueItem::UpdateTemplatableMCPServer {
                model: object.model().clone().into(),
                id: object.id,
                revision: revision_ts.or_else(|| object.metadata.revision.clone()),
            }
        }

        fn new_from_server_update(&self, server_cloud_object: &ServerCloudObject) -> Option<Self> {
            if let ServerCloudObject::TemplatableMCPServer(server_templatable_mcp_server) =
                server_cloud_object
            {
                return Some(server_templatable_mcp_server.model.clone().string_model);
            }
            None
        }

        fn uniqueness_key(&self) -> Option<GenericStringObjectUniqueKey> {
            Some(GenericStringObjectUniqueKey {
                key: format!("{UNIQUENESS_KEY_PREFIX}_{}", self.uuid),
                unique_per: UniquePer::User,
            })
        }
    }

    impl JsonModel for TemplatableMCPServer {
        fn json_object_type() -> JsonObjectType {
            JsonObjectType::TemplatableMCPServer
        }
    }
}

pub use templatable::{JsonTemplate, TemplatableMCPServer, TemplateVariable};

pub mod templatable_installation {
    use super::*;
    use crate::warp_managed_secrets::ManagedSecretValue;

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub enum VariableType {
        Text,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct VariableValue {
        pub variable_type: VariableType,
        pub value: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct TemplatableMCPServerInstallation {
        uuid: Uuid,
        templatable_mcp_server: TemplatableMCPServer,
        variable_values: HashMap<String, VariableValue>,
    }

    impl TemplatableMCPServerInstallation {
        pub fn new(
            uuid: Uuid,
            templatable_mcp_server: TemplatableMCPServer,
            variable_values: HashMap<String, VariableValue>,
        ) -> Self {
            Self {
                uuid,
                templatable_mcp_server,
                variable_values,
            }
        }

        pub fn hash(&self) -> Option<u64> {
            Some(0)
        }

        pub fn uuid(&self) -> Uuid {
            self.uuid
        }

        pub fn templatable_mcp_server(&self) -> &TemplatableMCPServer {
            &self.templatable_mcp_server
        }

        pub fn template_uuid(&self) -> Uuid {
            self.templatable_mcp_server.uuid
        }

        pub fn template_json(&self) -> &str {
            &self.templatable_mcp_server.template.json
        }

        pub fn template_variables(&self) -> &Vec<TemplateVariable> {
            &self.templatable_mcp_server.template.variables
        }

        pub fn variable_values(&self) -> &HashMap<String, VariableValue> {
            &self.variable_values
        }

        pub fn apply_secrets(&mut self, _secrets: &HashMap<String, ManagedSecretValue>) {}

        pub fn gallery_uuid(&self) -> Option<Uuid> {
            self.templatable_mcp_server
                .gallery_data
                .as_ref()
                .map(|gallery| gallery.gallery_item_id)
        }

        pub fn gallery_version(&self) -> Option<i32> {
            self.templatable_mcp_server
                .gallery_data
                .as_ref()
                .map(|gallery| gallery.version)
        }
    }
}

pub use templatable_installation::{TemplatableMCPServerInstallation, VariableType, VariableValue};

pub mod parsing {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct ParsedTemplatableMCPServerResult {
        pub templatable_mcp_server: TemplatableMCPServer,
        pub templatable_mcp_server_installation: Option<TemplatableMCPServerInstallation>,
    }

    impl ParsedTemplatableMCPServerResult {
        pub fn from_user_json(_json: &str) -> serde_json::Result<Vec<Self>> {
            Ok(Vec::new())
        }
    }

    pub fn prettify_json(json: &str) -> String {
        json.to_owned()
    }

    pub fn resolve_json(_installation: &TemplatableMCPServerInstallation) -> String {
        String::new()
    }
}

pub use parsing::ParsedTemplatableMCPServerResult;

pub mod gallery {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct MCPTemplateVariable {
        pub key: String,
        pub allowed_values: Option<Vec<String>>,
    }

    #[derive(Clone, Debug)]
    pub struct MCPJsonTemplate {
        pub json: String,
        pub variables: Vec<MCPTemplateVariable>,
    }

    #[derive(Clone, Debug)]
    pub struct MCPGalleryTemplate {
        pub description: String,
        pub gallery_item_id: String,
        pub instructions_in_markdown: Option<String>,
        pub json_template: MCPJsonTemplate,
        pub template: String,
        pub title: String,
        pub version: i32,
    }

    #[derive(Clone, Debug)]
    pub struct GalleryMCPServer {
        uuid: Uuid,
        title: String,
        description: String,
        version: i32,
        instructions_in_markdown: Option<String>,
        json_template: JsonTemplate,
    }

    impl GalleryMCPServer {
        pub fn uuid(&self) -> Uuid {
            self.uuid
        }
        pub fn title(&self) -> String {
            self.title.clone()
        }
        pub fn description(&self) -> String {
            self.description.clone()
        }
        pub fn version(&self) -> i32 {
            self.version
        }
        pub fn json_template(&self) -> &JsonTemplate {
            &self.json_template
        }
        pub fn instructions_in_markdown(&self) -> Option<&String> {
            self.instructions_in_markdown.as_ref()
        }
    }

    impl TryFrom<GalleryMCPServer> for TemplatableMCPServer {
        type Error = String;

        fn try_from(gallery_server: GalleryMCPServer) -> Result<Self, Self::Error> {
            Ok(TemplatableMCPServer {
                uuid: Uuid::new_v4(),
                name: gallery_server.title,
                description: Some(gallery_server.description),
                template: gallery_server.json_template,
                version: 0,
                gallery_data: Some(templatable::GalleryData {
                    gallery_item_id: gallery_server.uuid,
                    version: gallery_server.version,
                }),
            })
        }
    }

    #[derive(Default)]
    pub struct MCPGalleryManager;

    impl MCPGalleryManager {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
        pub fn get_gallery(&self) -> Vec<GalleryMCPServer> {
            Vec::new()
        }
        pub fn get_gallery_item(&self, _gallery_uuid: Uuid) -> Option<&GalleryMCPServer> {
            None
        }
        pub fn get_templatable_mcp_server(
            &self,
            _gallery_uuid: Uuid,
        ) -> Option<&TemplatableMCPServer> {
            None
        }
    }

    pub enum MCPGalleryManagerEvent {
        ItemsRefreshed,
    }

    impl Entity for MCPGalleryManager {
        type Event = MCPGalleryManagerEvent;
    }

    impl SingletonEntity for MCPGalleryManager {}
}

pub use gallery::{MCPGalleryManager, MCPGalleryTemplate};

pub mod templatable_manager {
    use super::*;
    use std::collections::HashSet;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum McpIntegration {
        Figma,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum FigmaMcpStatus {
        NotInstalled,
        Installed,
        Enabling,
        Running,
    }

    pub struct TemplatableMCPServerInfo {
        name: String,
        resources: Vec<rmcp::model::Resource>,
        tools: Vec<rmcp::model::Tool>,
        installation_id: Uuid,
        description: Option<String>,
    }

    impl TemplatableMCPServerInfo {
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn resources(&self) -> &Vec<rmcp::model::Resource> {
            &self.resources
        }
        pub fn tools(&self) -> &Vec<rmcp::model::Tool> {
            &self.tools
        }
        pub fn installation_id(&self) -> Uuid {
            self.installation_id
        }
        pub fn description(&self) -> Option<&str> {
            self.description.as_deref()
        }
    }

    #[derive(Default)]
    pub struct TemplatableMCPServerManager;

    impl TemplatableMCPServerManager {
        pub fn new(
            _locally_installed_servers: HashMap<Uuid, TemplatableMCPServerInstallation>,
            _running_server_uuids: Vec<Uuid>,
            _running_legacy_servers: &[Uuid],
            _ctx: &mut ModelContext<Self>,
        ) -> Self {
            Self
        }

        pub fn get_installed_templatable_servers(
            &self,
        ) -> &HashMap<Uuid, TemplatableMCPServerInstallation> {
            static EMPTY: std::sync::LazyLock<HashMap<Uuid, TemplatableMCPServerInstallation>> =
                std::sync::LazyLock::new(HashMap::new);
            &EMPTY
        }

        pub fn get_installed_server(
            &self,
            _installation_uuid: &Uuid,
        ) -> Option<&TemplatableMCPServerInstallation> {
            None
        }

        pub fn get_figma_mcp_status(&self) -> FigmaMcpStatus {
            FigmaMcpStatus::NotInstalled
        }

        pub fn get_template_uuid(&self, _installation_uuid: Uuid) -> Option<Uuid> {
            None
        }

        pub fn get_server_state(&self, _installation_uuid: Uuid) -> Option<MCPServerState> {
            None
        }

        pub fn get_server_error_message(&self, _installation_uuid: Uuid) -> Option<&str> {
            None
        }

        pub fn resources(&self) -> impl Iterator<Item = &rmcp::model::Resource> {
            std::iter::empty()
        }

        pub fn tools(&self) -> impl Iterator<Item = &rmcp::model::Tool> {
            std::iter::empty()
        }

        pub fn tool_input_schema(
            &self,
            _installation_id: Option<Uuid>,
            _tool_name: &str,
        ) -> Option<std::sync::Arc<rmcp::model::JsonObject>> {
            None
        }

        pub fn server_with_tool_name(
            &self,
            _tool_name: String,
        ) -> Option<crate::ai::mcp::reconnecting_peer::ReconnectingPeer> {
            None
        }

        pub fn server_with_resource(
            &self,
            _resource: &rmcp::model::Resource,
        ) -> Option<crate::ai::mcp::reconnecting_peer::ReconnectingPeer> {
            None
        }

        pub fn server_from_tool(&self, _tool: String) -> Option<&Uuid> {
            None
        }

        pub fn server_from_resource(&self, _name: &str, _uri: Option<&str>) -> Option<&Uuid> {
            None
        }

        pub fn get_active_templatable_servers(&self) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
            HashMap::new()
        }

        pub fn get_active_file_based_servers(
            &self,
            _cwd: &Path,
            _app: &AppContext,
        ) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
            HashMap::new()
        }

        pub fn get_active_cli_spawned_servers(&self) -> HashMap<Uuid, &TemplatableMCPServerInfo> {
            HashMap::new()
        }

        pub fn is_mcp_server_running(&self, _integration: McpIntegration) -> bool {
            false
        }

        pub fn get_templatable_mcp_server(&self, _uuid: Uuid) -> Option<&TemplatableMCPServer> {
            None
        }

        pub fn get_all_cloud_synced_mcp_servers(_ctx: &AppContext) -> HashMap<Uuid, String> {
            HashMap::new()
        }

        pub fn get_mcp_name(_uuid: &Uuid, _app: &AppContext) -> Option<String> {
            None
        }

        pub fn install_figma_from_gallery(&mut self, _ctx: &mut ModelContext<Self>) {}

        pub fn enable_figma_mcp(&mut self, _ctx: &mut ModelContext<Self>) {}

        pub fn handle_oauth_callback(&mut self, _url: &url::Url) -> anyhow::Result<()> {
            anyhow::bail!("MCP OAuth callbacks are unavailable in OSS builds")
        }

        pub fn extract_server_info<T: Eq + std::hash::Hash>(
            &self,
            _template_fn: fn(&TemplatableMCPServer) -> Option<T>,
            _installation_fn: fn(&TemplatableMCPServerInstallation) -> Option<T>,
            _app: &AppContext,
        ) -> HashSet<T> {
            HashSet::new()
        }
    }

    #[derive(Debug)]
    pub enum TemplatableMCPServerManagerEvent {
        StateChanged { uuid: Uuid, state: MCPServerState },
        ServerInstallationAdded(Uuid),
        ServerInstallationDeleted(Uuid),
        TemplatableMCPServersUpdated,
        LegacyServerConverted,
    }

    impl Entity for TemplatableMCPServerManager {
        type Event = TemplatableMCPServerManagerEvent;
    }

    impl SingletonEntity for TemplatableMCPServerManager {}
}

pub use templatable_manager::{McpIntegration, TemplatableMCPServerManager};

#[derive(Default)]
pub struct FileBasedMCPManager;

impl FileBasedMCPManager {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn get_servers_for_working_directory(
        &self,
        _cwd: &Path,
        _app: &AppContext,
    ) -> Vec<&TemplatableMCPServerInstallation> {
        Vec::new()
    }

    pub fn file_based_servers(&self) -> Vec<&TemplatableMCPServerInstallation> {
        Vec::new()
    }

    pub fn get_installation_by_uuid(
        &self,
        _uuid: Uuid,
    ) -> Option<&TemplatableMCPServerInstallation> {
        None
    }

    pub fn directory_paths_for_installation_and_provider(
        &self,
        _uuid: Uuid,
        _provider: MCPProvider,
    ) -> Vec<PathBuf> {
        Vec::new()
    }
}

impl Entity for FileBasedMCPManager {
    type Event = ();
}

impl SingletonEntity for FileBasedMCPManager {}

pub struct FileMCPWatcher;

impl FileMCPWatcher {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        Self
    }
}

impl Entity for FileMCPWatcher {
    type Event = ();
}

impl SingletonEntity for FileMCPWatcher {}

pub mod logs {}
pub mod manager {}
pub mod reconnecting_peer {
    use super::*;

    #[derive(Clone)]
    pub struct ReconnectingPeer;

    impl ReconnectingPeer {
        pub async fn call_tool(
            &self,
            _params: rmcp::model::CallToolRequestParam,
        ) -> Result<rmcp::model::CallToolResult, String> {
            Err("MCP runtime is unavailable in OSS builds".to_owned())
        }

        pub async fn read_resource(
            &self,
            _params: rmcp::model::ReadResourceRequestParam,
        ) -> Result<rmcp::model::ReadResourceResult, String> {
            Err("MCP runtime is unavailable in OSS builds".to_owned())
        }
    }
}
