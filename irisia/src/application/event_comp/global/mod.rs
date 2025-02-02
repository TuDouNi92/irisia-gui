use irisia_backend::{
    winit::event::{ElementState, MouseButton, Touch, TouchPhase},
    StaticWindowEvent,
};

use crate::{
    application::content::GlobalContent,
    event::{
        standard::{PointerDown, PointerEntered, PointerMove, PointerOut, PointerUp},
        EventDispatcher,
    },
    primitive::{Pixel, Point},
};

use self::new_event::{NewPointerEvent, PointerStateChange};

pub(crate) mod focusing;
pub(crate) mod new_event;

pub(crate) struct GlobalEventMgr {
    last_cursor_position: Option<Point>,
    pointer_state: PointerState,
}

#[derive(Clone, Copy)]
pub(crate) enum PointerState {
    Pressing,
    Release,
    OutOfViewport,
}

impl GlobalEventMgr {
    pub fn new() -> Self {
        GlobalEventMgr {
            last_cursor_position: None,
            pointer_state: PointerState::OutOfViewport,
        }
    }

    #[must_use]
    pub fn emit_event<'a>(
        &'a mut self,
        event: StaticWindowEvent,
        gc: &'a GlobalContent,
    ) -> Option<NewPointerEvent<'a>> {
        match cursor_behavior(&event, self.pointer_state, self.last_cursor_position) {
            Some((new_position, new_pointer_state)) => {
                let npe = NewPointerEvent::new(event, self, gc, new_position, new_pointer_state);
                emit_physical_pointer_event(
                    &gc.global_ed,
                    new_position,
                    npe.cursor_delta,
                    npe.pointer_state_change,
                );
                Some(npe)
            }
            None => {
                gc.global_ed.emit_sys(event);
                None
            }
        }
    }
}

fn emit_physical_pointer_event(
    ed: &EventDispatcher,
    position: Option<Point>,
    delta: Option<(Pixel, Pixel)>,
    new_pointer_state: PointerStateChange,
) {
    match (new_pointer_state, position) {
        (PointerStateChange::EnterViewport, None) => ed.emit_sys(PointerEntered),
        (PointerStateChange::Press, Some(position)) => ed.emit_sys(PointerDown {
            is_current: false,
            position,
        }),
        (PointerStateChange::Unchange, Some(position)) => ed.emit_sys(PointerMove {
            is_current: false,
            delta: delta.unwrap(),
            position,
        }),
        (PointerStateChange::Release, Some(position)) => ed.emit_sys(PointerUp {
            is_current: false,
            position,
        }),
        (PointerStateChange::LeaveViewport, None) => ed.emit_sys(PointerOut),
        _ => unreachable!("unexpected new-pointer-state and optioned position combination"),
    }
}

fn cursor_behavior(
    event: &StaticWindowEvent,
    old_state: PointerState,
    old_position: Option<Point>,
) -> Option<(Option<Point>, PointerState)> {
    let mut new_pointer_state = old_state;
    let mut new_position: Option<Point> = match &event {
        StaticWindowEvent::Touch(touch) => Some(touch.location.into()),
        _ => old_position,
    };

    match event {
        StaticWindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        }
        | StaticWindowEvent::Touch(Touch {
            phase: TouchPhase::Started,
            ..
        }) => {
            new_pointer_state = PointerState::Pressing;
        }

        StaticWindowEvent::CursorMoved { position, .. } => {
            new_position = Some(Point::from(*position))
        }

        StaticWindowEvent::Touch(Touch {
            phase: TouchPhase::Moved,
            ..
        }) => {}

        StaticWindowEvent::MouseInput {
            state: ElementState::Released,
            button: MouseButton::Left,
            ..
        }
        | StaticWindowEvent::Touch(Touch {
            phase: TouchPhase::Ended,
            ..
        }) => {
            new_pointer_state = PointerState::Release;
        }

        StaticWindowEvent::CursorLeft { .. }
        | StaticWindowEvent::Touch(Touch {
            phase: TouchPhase::Cancelled,
            ..
        }) => {
            new_pointer_state = PointerState::OutOfViewport;
        }

        _ => return None,
    }

    Some((new_position, new_pointer_state))
}
