mod line;
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
    pub edge_pixel: char,
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
            edge_pixel: '-',
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

    pub fn render(&self, obj: &object::Object) -> Vec<Vec<u8>> {
        let mut render = vec![vec![self.clear_pixel as u8; self.resolution.0]; self.resolution.1];
        let mut verts = vec![];
        for point in obj.points() {
            if point.x != self.origin.x || point.y != self.origin.y || point.z != self.origin.z {
                verts.push(self.get_projection(point));
            }
        }

        let mut edges = vec![];
        for edge in obj.edges() {
            let edge = &mut line::get_points_between(verts[edge.0], verts[edge.1]);
            if let Some(edge) = edge {
                edges.append(edge);
            }
        }

        for vert in verts {
            if vert.is_none() {
                continue;
            }
            self.render_pixel(vert, &mut render, self.vert_pixel);
        }

        for edge in edges {
            self.render_pixel(Some(edge), &mut render, self.edge_pixel);
        }

        render
    }

    fn render_pixel(&self, coords: Option<(f32, f32)>, render: &mut Vec<Vec<u8>>, pixel: char) {
        let y = self.resolution.1 as f32 - (coords.unwrap().1 + self.resolution.1 as f32 / 2.0);
        let x = self.resolution.0 as f32 - (coords.unwrap().0 + self.resolution.0 as f32 / 2.0);
        if y >= 0.0 && x >= 0.0 && y < self.resolution.1 as f32 && x < self.resolution.0 as f32 {
            render[y as usize][x as usize] = pixel as u8;
        }
    }

    fn get_projection(&self, point: &Point3<f32>) -> Option<(f32, f32)> {
        let view = Isometry3::look_at_rh(&self.origin, &self.look, &self.up);

        let projection = Perspective3::new(0.5, self.fov, self.near, self.far);
        let model_view_projection = projection.into_inner() * view.to_homogeneous();
        let point_coords = model_view_projection * Point3::to_homogeneous(point);
        let multiplier = self.resolution.0.max(self.resolution.1);
        if point_coords.z < 0.0 {
            return None;
        }
        Some((
            (Point3::from_homogeneous(point_coords).unwrap().x * multiplier as f32 / 4.0).round(),
            (Point3::from_homogeneous(point_coords).unwrap().y * multiplier as f32 / 4.0).round(),
        ))
    }

    pub fn move_forward(
        &mut self,
        distance: f32,
    ) -> (nalgebra::Point3<f32>, nalgebra::Point3<f32>) {
        self.origin += self.forward * distance;
        self.look += self.forward * distance;
        self.update_vectors();
        (self.origin, self.look)
    }
}
