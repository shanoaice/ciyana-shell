use super::workspace::WorkspaceIdentifier;

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
    /// From Sway IPC
    SwayIpc(u64),
}

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub app_id: String,
    pub identifier: WindowIdentifier,
    pub workspace: WorkspaceIdentifier,
}

#[derive(Debug)]
pub enum WindowEvent {
    Initialize(Vec<Window>),
    TitleChanged(WindowIdentifier, String),
    AppIdChanged(WindowIdentifier, String),
    NewWindow(Window),
    WindowClosed(WindowIdentifier),
    WindowMoved(WindowIdentifier, WorkspaceIdentifier),
}

#[derive(Debug)]
pub enum WindowActions {
    Focus(WindowIdentifier),
    Close(WindowIdentifier),
    MoveToWorkspace(WindowIdentifier, WorkspaceIdentifier),
}

#[derive(Debug)]
pub struct WindowCapabilities {
    pub ordered_window: bool,
    pub can_move_windows: bool,
}

pub trait WindowHandler {
    fn capabilities() -> WindowCapabilities;

    fn daemon(
        event_tx: kanal::AsyncSender<WindowEvent>,
        action_rx: kanal::AsyncReceiver<WindowActions>,
    ) -> impl std::future::Future<Output = ()> + Send + 'static;
}
