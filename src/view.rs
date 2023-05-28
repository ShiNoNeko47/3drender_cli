use std::f32::consts::PI;

use crate::object;

use nalgebra::{Isometry3, Matrix3x1, Perspective3, Point3, Vector3};

#[derive(Debug)]
pub struct View {
    origin: Point3<f32>,
    look: Point3<f32>, // a point the camera is looking at
    pub fov: f32,      // number between 0 and pi
    far: f32,
    near: f32,
    forward: Matrix3x1<f32>,
    right: Matrix3x1<f32>,
    up: Matrix3x1<f32>,
    pub resolution: (usize, usize), // width and height, number of columns and lines in terminal
    pub clear_pixel: char,
    pub vert_pixel: char,
    pub center_pixel: Option<char>,
    pub border: [char; 6],
}

impl View {
    pub fn new(look: Point3<f32>, origin: Point3<f32>) -> Self {
        let forward = (look - origin).normalize();
        let right = forward.cross(&Vector3::y()).normalize();
        let up = right.cross(&forward).normalize();
        Self {
            origin,
            look,
            fov: PI / 2.0,
            forward,
            right,
            up,
            resolution: (60, 15), // cell height to width ratio in terminal emulators is usually 2:1
            far: 200.0,
            near: 0.1,
            clear_pixel: ' ',
            vert_pixel: '*',
            center_pixel: None,
            border: ['│', '─', '└', '┘', '┌', '┐'],
        }
    }

    pub fn set_look(&mut self, look: Point3<f32>) {
        self.look = look;
        self.update_vectors();
    }

    pub fn set_origin(&mut self, origin: Point3<f32>) {
        self.origin = origin;
        self.update_vectors();
    }

    fn update_vectors(&mut self) {
        self.forward = (self.look - self.origin).normalize();
        self.right = self.forward.cross(&Vector3::y()).normalize();
        self.up = self.right.cross(&self.forward).normalize();
    }

    pub fn render(&self, obj: &object::Object) {
        let mut verts = vec![];
        for point in obj.points() {
            if point.x != self.origin.x || point.y != self.origin.y || point.z != self.origin.z {
                verts.push(self.get_projection(point));
            }
        }
        // println!("{:?}", verts);
        for i in 0..self.resolution.1 {
            let y: i32 = self.resolution.1 as i32 / 2 - i as i32;

            if i == 0 {
                self.draw_border_top();
            }

            for j in 0..self.resolution.0 {
                let x: i32 = j as i32 - self.resolution.0 as i32 / 2;
                if y == 0 && x == 0 {
                    if let Some(center_pixel) = self.center_pixel {
                        print!("{}", center_pixel);
                        continue;
                    }
                }
                if verts.contains(&(x as f32, y as f32)) {
                    print!("{}", self.vert_pixel);
                    continue;
                }
                print!("{}", self.clear_pixel);
            }

            if i == self.resolution.1 - 1 {
                self.draw_border_bottom();
                continue;
            }
            print!("{}\n{}", self.border[0], self.border[0]);
        }
    }

    fn draw_border_bottom(&self) {
        print!("{}\n{}", self.border[0], self.border[2]);
        for _ in 0..self.resolution.0 {
            print!("{}", self.border[1]);
        }
        print!("{}", self.border[3]);
    }

    fn draw_border_top(&self) {
        print!("{}", self.border[4]);
        for _ in 0..self.resolution.0 {
            print!("{}", self.border[1]);
        }
        print!("{}\n{}", self.border[5], self.border[0]);
    }

    fn get_projection(&self, point: &Point3<f32>) -> (f32, f32) {
        let view = Isometry3::look_at_rh(&self.origin, &self.look, &self.up);

        let projection = Perspective3::new(0.5, self.fov, self.near, self.far);
        let model_view_projection = projection.into_inner() * view.to_homogeneous();
        let point_coords = model_view_projection * Point3::to_homogeneous(point);
        let multiplier = self.resolution.0.max(self.resolution.1);
        (
            (Point3::from_homogeneous(point_coords).unwrap().x * multiplier as f32 / 4.0).round(),
            (Point3::from_homogeneous(point_coords).unwrap().y * multiplier as f32 / 4.0).round(),
        )
    }
}
