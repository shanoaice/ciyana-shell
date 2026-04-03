use crate::window_management::window::{
    WindowActions, WindowCapabilities, WindowEvent, WindowHandler,
};
use crate::window_management::workspace::{
    WorkspaceActions, WorkspaceCapabilities, WorkspaceEvent, WorkspaceHandler,
};
use futures::stream::Stream;
use futures::StreamExt;

pub enum ShellEvent {
    Window(WindowEvent),
    Workspace(WorkspaceEvent),
}

pub enum ShellAction {
    Window(WindowActions),
    Workspace(WorkspaceActions),
}

pub struct CompositorShell<W: WindowHandler, WS: WorkspaceHandler> {
    window_event_tx: kanal::AsyncSender<WindowEvent>,
    window_event_rx: kanal::AsyncReceiver<WindowEvent>,
    window_action_tx: kanal::AsyncSender<WindowActions>,
    window_action_rx: kanal::AsyncReceiver<WindowActions>,
    workspace_event_tx: kanal::AsyncSender<WorkspaceEvent>,
    workspace_event_rx: kanal::AsyncReceiver<WorkspaceEvent>,
    workspace_action_tx: kanal::AsyncSender<WorkspaceActions>,
    workspace_action_rx: kanal::AsyncReceiver<WorkspaceActions>,
    _window_handler: std::marker::PhantomData<W>,
    _workspace_handler: std::marker::PhantomData<WS>,
}

impl<W: WindowHandler, WS: WorkspaceHandler> CompositorShell<W, WS> {
    pub fn new() -> Self {
        let (window_event_tx, window_event_rx) = kanal::unbounded_async();
        let (window_action_tx, window_action_rx) = kanal::unbounded_async();
        let (workspace_event_tx, workspace_event_rx) = kanal::unbounded_async();
        let (workspace_action_tx, workspace_action_rx) = kanal::unbounded_async();

        Self {
            window_event_tx,
            window_event_rx,
            window_action_tx,
            window_action_rx,
            workspace_event_tx,
            workspace_event_rx,
            workspace_action_tx,
            workspace_action_rx,
            _window_handler: std::marker::PhantomData,
            _workspace_handler: std::marker::PhantomData,
        }
    }

    pub fn run(&self) -> impl std::future::Future<Output = ()> + Send + 'static {
        let window_fut = W::daemon(
            self.window_event_tx.clone(),
            self.window_action_rx.clone(),
        );
        let workspace_fut = WS::daemon(
            self.workspace_event_tx.clone(),
            self.workspace_action_rx.clone(),
        );

        async move {
            futures::join!(window_fut, workspace_fut);
        }
    }

    pub fn event_stream(&mut self) -> impl Stream<Item = ShellEvent> + '_ {
        let window_stream = self.window_event_rx.stream().map(ShellEvent::Window);
        let workspace_stream = self.workspace_event_rx.stream().map(ShellEvent::Workspace);
        futures::stream::select(window_stream, workspace_stream)
    }

    pub fn send_action(&self, action: ShellAction) -> Result<(), kanal::SendError> {
        match action {
            ShellAction::Window(action) => {
                self.window_action_tx.as_sync().send(action)
            }
            ShellAction::Workspace(action) => {
                self.workspace_action_tx.as_sync().send(action)
            }
        }
    }

    pub fn window_capabilities() -> WindowCapabilities {
        W::capabilities()
    }

    pub fn workspace_capabilities() -> WorkspaceCapabilities {
        WS::capabilities()
    }
}

impl<W: WindowHandler, WS: WorkspaceHandler> Default for CompositorShell<W, WS> {
    fn default() -> Self {
        Self::new()
    }
}
