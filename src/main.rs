use std::{collections::HashSet, ffi::c_double};
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: c_double,
    pub y: c_double,
}

impl Point {
    pub fn new(x: c_double, y: c_double) -> Self {
        Self { x: (x), y: (y) }
    }
}

pub fn distance(p1: &Point, p2: &Point) -> c_double {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Box {
    pub left: c_double,
    pub top: c_double,
    pub right: c_double,
    pub bottom: c_double,
}

impl Box {
    pub fn new(left: c_double, top: c_double, right: c_double, bottom: c_double) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
    // pub fn new(p1: &Point, p2: &Point) -> Self {
    //     Self {
    //         left: p1.x.min(p2.x),
    //         top: p1.y.max(p2.y),
    //         right: p1.x.max(p2.x),
    //         bottom: p1.y.min(p2.y),
    //     }
    // }
}

fn main() {}
