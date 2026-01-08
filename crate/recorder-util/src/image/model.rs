use std::collections::VecDeque;

use tao::dpi::PhysicalPosition;

use crate::{to_point, Rectangle};

pub struct RecordQueue {
    queue: VecDeque<PhysicalPosition<f64>>,
    store: Vec<Rectangle>,
}

impl RecordQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            store: Vec::new(),
        }
    }
    pub fn push(&mut self, value: PhysicalPosition<f64>) {
        if self.queue.len() == 2 {
            self.queue.pop_back();
        }
        self.queue.push_back(value);
    }

    pub fn pop(&mut self) {
        self.queue.pop_back();
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    ///保存当前操作，结束截图返回true
    pub fn save(&mut self) -> bool {
        if self.queue.len() == 2 {
            let rect = Rectangle::new(
                to_point(self.queue.pop_front().unwrap()),
                to_point(self.queue.pop_front().unwrap()),
            );
            self.store.push(rect);
        }
        self.store.len() == 2
    }
    pub fn get_store(&self) -> Option<(Rectangle, Rectangle)> {
        if self.store.len() < 2 {
            None
        } else {
            Some((
                self.store.get(0).unwrap().clone(),
                self.store.get(1).unwrap().clone(),
            ))
        }
    }
}
