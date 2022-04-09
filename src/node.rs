#[derive(Debug, Default)]
pub(crate) struct Node {
    children: Vec<Node>,
    height: usize,
    leaf: bool,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    bbox: BBox,
}

impl Node {
    fn dist_bbox(&mut self, k: usize, p: usize, node: Option<&Node>) {
        self.min_x = f32::INFINITY;
        self.min_y = f32::INFINITY;
        self.max_x = f32::NEG_INFINITY;
        self.max_y = f32::NEG_INFINITY;

        match node {
            None => {
                for i in k..p {
                    self.extend_from_children(i);
                }
            }
            Some(n) => {
                n.children
                    .iter()
                    .skip(k)
                    .take(p)
                    .for_each(|n| self.extend(n));
            }
        }
    }

    fn extend_from_children(&mut self, i: usize) {
        let &Node {
            min_x,
            min_y,
            max_x,
            max_y,
            ..
        } = unsafe {
            self.children.get_unchecked(i);
        };

        self.min_x = self.min_x.min(min_x);
        self.min_y = self.min_y.min(min_y);

        self.max_x = self.max_x.max(max_x);
        self.max_y = self.max_y.max(max_y);
    }

    fn extend(&mut self, node: &Node) {
        self.min_x = self.min_x.min(node.min_x);
        self.min_y = self.min_y.min(node.min_y);

        self.max_x = self.max_x.max(node.max_x);
        self.max_y = self.max_y.max(node.max_y);
    }

    fn calc_bounds(&mut self) {
        self.dist_bbox(0, self.children.len(), None);
    }

    fn copy_bbox(&mut self, b: BBox) {
        // bounds == box
        self.min_x = b.min_x;
        self.min_y = b.min_y;
        self.max_x = b.max_x;
        self.max_y = b.max_y;
        self.bbox = b;
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct BBox {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}
