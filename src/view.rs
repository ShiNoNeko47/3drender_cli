use crate::object;

use ndarray::arr2;

#[derive(Debug)]
pub struct View {
    pub origin: object::Point,
    pub look: object::Point, // a point the camera is looking at
    pub tilt: f32,           // number between 0 and 2pi
    pub fov: f32,            // number between 0 and pi
    far: f32,
    near: f32,
    pub resolution: (usize, usize), // width and height, number of columns and lines in terminal
    pub clear_pixel: char,
    pub vert_pixel: char,
}

impl View {
    pub fn new(look: object::Point, origin: object::Point) -> Self {
        Self {
            origin,
            look,
            resolution: (60, 15), // cell height to width ratio in terminal emulators is usually 2:1
            tilt: 0.0,
            fov: 0.5 * std::f32::consts::PI,
            far: 100.0,
            near: 0.1,
            clear_pixel: ' ',
            vert_pixel: '*',
        }
    }

    pub fn render(&self, obj: &object::Object) {
        let mut verts = vec![];
        for point in obj.points() {
            verts.push(self.get_projection(point));
            // println!("{:?}", verts.last());
        }
        // print!("{:?}", verts);
        for i in 0..self.resolution.1 {
            let y: i32 = self.resolution.1 as i32 / 2 - i as i32;
            for j in 0..self.resolution.0 {
                let x: i32 = j as i32 - self.resolution.0 as i32 / 2;
                if verts.contains(&(x as f32, y as f32)) {
                    print!("{}", self.vert_pixel);
                    continue;
                }
                print!("{}", self.clear_pixel);
            }
            println!();
        }
    }

    fn get_projection(&self, point: &object::Point) -> (f32, f32) {
        let projection_matrix = arr2(&[
            [1.0 / (self.fov / 2.0).tan(), 0.0, 0.0, 0.0],
            [0.0, 1.0 / (self.fov / 2.0).tan(), 0.0, 0.0],
            [
                0.0,
                0.0,
                -1.0 * (self.far + self.near) / (self.far - self.near),
                -2.0 * self.far * self.near / (self.far - self.near),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ]);

        let projection = projection_matrix.dot(
            &(&point.to_matrix()
                + &arr2(&[[self.origin.x], [self.origin.y], [self.origin.z], [0.0]])),
        );

        let (x, y) = (
            (projection.get((0, 0)).unwrap() / projection.last().unwrap() * 10.0 * 2.0).round(),
            (projection.get((1, 0)).unwrap() / projection.last().unwrap() * 10.0).round(),
        );
        // println!("{:?}", (x, y));
        (x, y)
    }
}
