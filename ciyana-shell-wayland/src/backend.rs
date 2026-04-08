use ciyana_shell_core::window_management::window::{
    WindowActions, WindowCapabilities, WindowEvent, WindowHandler,
};

pub struct ExtWaylandWindowHandler {}

impl WindowHandler for ExtWaylandWindowHandler {
    fn capabilities() -> WindowCapabilities {
        WindowCapabilities {
            ordered_window: false,
            can_move_windows: false,
        }
    }

    async fn daemon(
        _event_tx: kanal::AsyncSender<WindowEvent>,
        _action_rx: kanal::AsyncReceiver<WindowActions>,
    ) {
        todo!()
    }
}
