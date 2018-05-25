#![allow(dead_code)]

use std::cmp::{max, min};

/// A simple rectangle used in the computation of the layout and to give widgets an hint about the
/// area they are supposed to render to.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Default for Rect {
    fn default() -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn left(&self) -> usize {
        self.x
    }

    pub fn right(&self) -> usize {
        self.x + self.width
    }

    pub fn top(&self) -> usize {
        self.y
    }

    pub fn bottom(&self) -> usize {
        self.y + self.height
    }

    pub fn inner(&self, margin: usize) -> Rect {
        if self.width < 2 * margin || self.height < 2 * margin {
            Rect::default()
        } else {
            Rect {
                x: self.x + margin,
                y: self.y + margin,
                width: self.width - 2 * margin,
                height: self.height - 2 * margin,
            }
        }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let x1 = min(self.x, other.x);
        let y1 = min(self.y, other.y);
        let x2 = max(self.x + self.width, other.x + other.width);
        let y2 = max(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersection(&self, other: &Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.x + self.width, other.x + other.width);
        let y2 = min(self.y + self.height, other.y + other.height);
        Rect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
}
