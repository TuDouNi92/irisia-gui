use crate::{
    event::EventEmitter,
    primary::{Point, Region},
    Event,
};

struct Item {
    event_emitter: EventEmitter,
    interact_region: Option<Region>,
    parent: Option<usize>,
}

pub(crate) struct ElemTable {
    emitters: Vec<Item>,
    builder_stack: Vec<usize>,
}

impl ElemTable {
    pub fn new() -> Self {
        ElemTable {
            emitters: Vec::new(),
            builder_stack: Vec::new(),
        }
    }

    pub fn builder(&mut self) -> Builder {
        self.emitters.clear();
        let builder = Builder {
            emitters: &mut self.emitters,
            builder_stack: &mut self.builder_stack,
        };
        builder
    }

    pub async fn emit<E>(&self, point: Point, event: &E)
    where
        E: Event + Clone,
    {
        let mut selected = None;

        for (
            index,
            Item {
                interact_region, ..
            },
        ) in self.emitters.iter().enumerate().rev()
        {
            if let Some(re) = interact_region {
                if point.abs_ge(re.0) && point.abs_le(re.1) {
                    selected = Some(index);
                    break;
                }
            }
        }

        while let Some(index) = selected {
            let item = &self.emitters[index];
            item.event_emitter.emit(event).await;
            selected = item.parent;
        }
    }
}

pub(crate) struct Builder<'a> {
    emitters: &'a mut Vec<Item>,
    builder_stack: &'a mut Vec<usize>,
}

impl Builder<'_> {
    pub fn downgrade_lifetime(&mut self) -> Builder {
        Builder {
            emitters: self.emitters,
            builder_stack: self.builder_stack,
        }
    }

    pub fn push(&mut self, event_emitter: EventEmitter) -> usize {
        self.emitters.push(Item {
            event_emitter,
            interact_region: None,
            parent: self.builder_stack.last().copied(),
        });

        let index = self.emitters.len() - 1;
        self.builder_stack.push(index);
        index
    }

    pub fn set_interact_region_for(&mut self, index: usize, r: Region) {
        self.emitters[index].interact_region = Some(r);
    }

    pub fn finish(&mut self) {
        assert!(self.builder_stack.pop().is_some());
    }
}