use crate as cream;
use crate::primary::Point;
use crate::Event;

use super::EventDispatcher;

pub mod window_event;

/// Declares the element won't be used by the origin structure anymore,
/// but may not dropped immediately due to strong references of `Arc`.
/// This event will be emitted only
/// once, when received, runtimes of this element should handle quiting.
#[derive(Event, Clone, Copy)]
pub struct ElementAbondoned;

#[derive(Event, Clone)]
pub struct EventDispatcherCreated(pub EventDispatcher);

#[derive(Event, Clone)]
pub struct PointerDown {
    pub is_current: bool,
    pub position: Point,
}

#[derive(Event, Clone)]
pub struct PointerUp {
    pub is_current: bool,
    pub position: Point,
}

#[derive(Event, Clone)]
pub struct PointerMove {
    pub is_current: bool,
    pub position: Point,
}

#[derive(Event, Clone)]
pub struct PointerEnter;

#[derive(Event, Clone)]
pub struct PointerLeft;

#[derive(Event, Clone)]
pub struct Click {
    pub is_current: bool,
}