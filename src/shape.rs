// network of various kinds of edges -> flattened edges
// insert self intersections
// find cyclic shapes
// 

use std::simd::f32x2;

pub enum Edge {
    Straight([f32x2; 2]),
    Curve { main: [f32; 2], control: [f32; 2] }
}

pub enum Object {
    Edges(Vec<Edge>),
    Polygons
}