/// Opaque output identifier, backend-specific
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputId(pub u64);

/// Opaque group identifier, backend-specific
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkspaceGroupId(pub u64);

/// A workspace group bound to a set of outputs.
/// Niri: one group per output. Hyprland: one group across all outputs.
#[derive(Debug)]
pub struct WorkspaceGroup {
    pub id: WorkspaceGroupId,
    pub output_ids: Vec<OutputId>,
}

/// Independent state flags for a workspace.
#[derive(Debug)]
pub struct WorkspaceState {
    pub is_active: bool,
    pub is_urgent: bool,
}

/// A single workspace within a group
#[derive(Debug)]
pub struct Workspace {
    pub id: WorkspaceIdentifier,
    pub group_id: WorkspaceGroupId,
    pub name: String,
    pub state: WorkspaceState,
}

/// Window objects obtained from different sources uses
/// different format for stable identifiers
#[derive(Debug, Clone)]
pub enum WindowIdentifier {
    /// From Wayland ext_foreign_toplevel_list_v1
    ExtForeignToplevel(String),
    /// From Niri IPC
    Niri(u64),
    /// From Hyprland IPC
    Hyprland(i64),
    /// Wayland zwlr_foreign_toplevel_manager_v1 does not
    /// provide stable identifiers. Instead, their handle
    /// provides the binding. However, to keep crates
    /// decoupled, this variant stores a backend-generated
    /// u32 integer as a transparent identifier. The backend
    /// is responsible for mapping this id back to the
    /// corresponding toplevel handle when receiving an
    /// action
    WlrForeignToplevel(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum WorkspaceIdentifier {
    Niri(u64),
    /// https://github.com/hyprwm/Hyprland/blob/529f72249c2cf4cefc824a612aeddf2d5f858f54/src/SharedDefs.hpp#L57
    /// As of time written.
    Hyprland(i64),
    /// If compositor does not provide identification
    /// mechanism
    None,
}

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub app_id: String,
    pub identifier: WindowIdentifier,
    pub workspace: WorkspaceIdentifier,
}

/// Events sent back from the window management daemon
#[derive(Debug)]
pub enum WindowManagementEvents {
    Initialize(Vec<Window>),
    TitleChanged(WindowIdentifier, String),
    AppIdChanged(WindowIdentifier, String),
    NewWindow(Window),
    WindowClosed(WindowIdentifier),
    WindowMoved(WindowIdentifier, WorkspaceIdentifier),
    WorkspaceGroupCreated(WorkspaceGroup),
    WorkspaceGroupDestroyed(WorkspaceGroupId),
    WorkspaceCreated(Workspace),
    WorkspaceDestroyed(WorkspaceIdentifier),
    WorkspaceStateChanged(WorkspaceIdentifier, WorkspaceState),
}

/// Actions to send to the window management daemon
#[derive(Debug)]
pub enum WindowManagementActions {
    Focus(WindowIdentifier),
    Close(WindowIdentifier),
    FocusWorkspace(WorkspaceIdentifier),
    MoveWindowToWorkspace(WindowIdentifier, WorkspaceIdentifier),
}

pub struct WorkspaceManagementCapabilities {
    pub workspaces_supported: bool,
    pub workspace_creation: bool,
    pub window_workspace_assignment: bool,
    /// True if workspace groups are per-output (Niri-style),
    /// false if workspaces are shared across all outputs (Hyprland-style)
    pub per_output_workspaces: bool,
}

pub enum WorkspaceManagementSupport {
    Yes(WorkspaceManagementCapabilities),
    No,
}

pub struct WindowManagementCapabilities {
    pub workspaces_supported: WorkspaceManagementSupport,
    pub ordered_window: bool
}

pub trait WindowManagementBackend {
    fn capabilities() -> WindowManagementCapabilities;
    fn daemon(
        event_sender: kanal::AsyncSender<WindowManagementEvents>,
        action_receiver: kanal::AsyncReceiver<WindowManagementActions>,
    ) -> impl Future + Send + 'static;
}
