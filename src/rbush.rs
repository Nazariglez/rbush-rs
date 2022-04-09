use crate::node::Node;

pub struct RBush {
    max_entries: f32,
    min_entries: f32,
    data: Node,
}

impl RBush {
    pub fn new(max_entries: f32) -> Self {
        Self {
            max_entries: max_entries.max(4.0),
            min_entries: (max_entries * 0.4).ceil().max(2.0),
            data: Node::default(),
        }
    }

    pub fn load(&mut self, data: &[BBox]) {
        // todo
    }
}
