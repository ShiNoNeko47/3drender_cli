use ndarray::arr2;
#[derive(Debug)]
pub struct Object {
    points: Vec<Point>,
    edges: Vec<(Point, Point)>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            points: vec![],
            edges: vec![],
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32, z: f32) {
        self.points.push(Point::new(x, y, z));
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    pub fn to_matrix(
        &self,
    ) -> ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>> {
        arr2(&[[self.x], [self.y], [self.z], [self.w]])
    }
}
