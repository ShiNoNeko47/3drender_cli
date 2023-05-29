#[derive(Debug)]
pub struct Object {
    points: Vec<nalgebra::geometry::Point3<f32>>,
    edges: Vec<(usize, usize)>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            points: vec![],
            edges: vec![],
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32) {
        self.points.push(nalgebra::geometry::Point3::new(x, y, z));
    }

    pub fn points(&self) -> &Vec<nalgebra::geometry::Point3<f32>> {
        &self.points
    }

    pub fn add_edge(&mut self, a: usize, b: usize) {
        self.edges.push((a, b));
    }

    pub fn edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
}
