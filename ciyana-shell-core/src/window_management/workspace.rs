#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputId(pub u64);

/// Opaque group identifier, backend-specific
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkspaceGroupId(pub u64);

#[derive(Debug, Clone)]
pub enum WorkspaceIdentifier {
    Niri(u64),
    /// https://github.com/hyprwm/Hyprland/blob/529f72249c2cf4cefc824a612aeddf2d5f858f54/src/SharedDefs.hpp#L57
    /// As of time written.
    Hyprland(i64),
    /// From Sway IPC (Sway uses string names)
    Sway(String),
    /// If compositor does not provide identification
    /// mechanism
    None,
}

#[derive(Debug)]
pub struct WorkspaceGroup {
    pub id: WorkspaceGroupId,
    pub output_ids: Vec<OutputId>,
}

#[derive(Debug)]
pub struct WorkspaceState {
    pub is_active: bool,
    pub is_urgent: bool,
}

#[derive(Debug)]
pub struct Workspace {
    pub id: WorkspaceIdentifier,
    pub group_id: WorkspaceGroupId,
    pub name: String,
    pub state: WorkspaceState,
}

#[derive(Debug)]
pub enum WorkspaceEvent {
    Initialize(Vec<Workspace>, Vec<WorkspaceGroup>),
    WorkspaceGroupCreated(WorkspaceGroup),
    WorkspaceGroupDestroyed(WorkspaceGroupId),
    WorkspaceCreated(Workspace),
    WorkspaceDestroyed(WorkspaceIdentifier),
    WorkspaceStateChanged(WorkspaceIdentifier, WorkspaceState),
}

#[derive(Debug)]
pub enum WorkspaceActions {
    Focus(WorkspaceIdentifier),
}

#[derive(Debug)]
pub struct WorkspaceCapabilities {
    pub workspace_properties: CompositorWorkspaceProperties,
    pub workspace_support: CompositorWorkspaceSupport,
}

#[derive(Debug)]
pub struct CompositorWorkspaceProperties {
    pub per_output_workspaces: bool,
}

#[derive(Debug)]
pub enum CompositorWorkspaceSupport {
    WindowMapped,
    Indication,
    None,
}

pub trait WorkspaceHandler {
    fn capabilities() -> WorkspaceCapabilities;

    fn daemon(
        event_tx: kanal::AsyncSender<WorkspaceEvent>,
        action_rx: kanal::AsyncReceiver<WorkspaceActions>,
    ) -> impl std::future::Future<Output = ()> + Send + 'static;
}
