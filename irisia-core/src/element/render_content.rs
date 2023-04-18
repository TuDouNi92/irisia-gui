use std::time::Duration;

use irisia_backend::{skia_safe::Canvas, window_handle::close_handle::CloseHandle, WinitWindow};

use crate::{
    application::elem_table::{self, focus::SharedFocusing},
    event::EventDispatcher,
    primary::Region,
    CacheBox,
};

pub(crate) struct BareContent<'a> {
    pub canvas: &'a mut Canvas,
    pub window: &'a WinitWindow,
    pub delta_time: Duration,
    pub window_event_dispatcher: &'a EventDispatcher,
    pub close_handle: CloseHandle,
    pub elem_table_builder: elem_table::Builder<'a>,
    pub focusing: &'a SharedFocusing,
}

impl BareContent<'_> {
    pub fn downgrade_lifetime(&mut self) -> BareContent {
        BareContent {
            canvas: self.canvas,
            window: self.window,
            delta_time: self.delta_time,
            window_event_dispatcher: self.window_event_dispatcher,
            close_handle: self.close_handle,
            elem_table_builder: self.elem_table_builder.downgrade_lifetime(),
            focusing: self.focusing,
        }
    }
}

pub struct RenderContent<'a> {
    pub(crate) bare: BareContent<'a>,
    pub(crate) cache_box_for_children: &'a mut CacheBox,
    pub(crate) elem_table_index: usize,
}

impl RenderContent<'_> {
    pub fn canvas_ref(&self) -> &Canvas {
        self.bare.canvas
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        self.bare.canvas
    }

    pub fn window(&self) -> &WinitWindow {
        self.bare.window
    }

    pub fn delta_time(&self) -> Duration {
        self.bare.delta_time
    }

    pub fn set_interact_region(&mut self, region: Region) {
        self.bare
            .elem_table_builder
            .set_interact_region_for(self.elem_table_index, Some(region));
    }

    pub fn clear_interact_region(&mut self) {
        self.bare
            .elem_table_builder
            .set_interact_region_for(self.elem_table_index, None);
    }

    pub(crate) fn downgrade_lifetime(&mut self) -> RenderContent {
        RenderContent {
            bare: self.bare.downgrade_lifetime(),
            cache_box_for_children: self.cache_box_for_children,
            elem_table_index: self.elem_table_index,
        }
    }
}
