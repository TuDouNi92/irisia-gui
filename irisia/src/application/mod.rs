use std::sync::Arc;

use irisia_backend::{window_handle::WindowBuilder, WinitWindow};

use crate::{
    element::{Element, UpdateElement},
    event::{standard::window_event::WindowDestroyed, EventDispatcher},
    Result, UpdateWith,
};

mod backend;
pub(crate) mod content;
pub(crate) mod event_comp;
pub(crate) mod redraw_scheduler;

use backend::new_window;

pub use irisia_backend::window_handle::CloseHandle;

#[derive(Clone)]
pub struct Window {
    winit_window: Arc<WinitWindow>,
    close_handle: CloseHandle,
    event_dispatcher: EventDispatcher,
}

type EmptyUpdateOptions<'a, El> = UpdateElement<'a, El, (), ()>;

impl Window {
    pub async fn new<El>(title: impl Into<String>) -> Result<Self>
    where
        El: Element + for<'a> UpdateWith<EmptyUpdateOptions<'a, El>>,
    {
        let title = title.into();
        new_window::<El, _>(move |wb| wb.with_title(title)).await
    }

    pub async fn with_builder<El, F>(f: F) -> Result<Self>
    where
        El: Element + for<'a> UpdateWith<EmptyUpdateOptions<'a, El>>,
        F: FnOnce(WindowBuilder) -> WindowBuilder + Send + 'static,
    {
        new_window::<El, _>(f).await
    }

    pub fn winit_window(&self) -> &Arc<WinitWindow> {
        &self.winit_window
    }

    pub fn close_handle(&self) -> CloseHandle {
        self.close_handle
    }

    pub fn close(&self) {
        self.close_handle.close();
    }

    pub fn event_dispatcher(&self) -> &EventDispatcher {
        &self.event_dispatcher
    }

    pub async fn join(&self) {
        self.event_dispatcher.recv_sys::<WindowDestroyed>().await;
    }
}
