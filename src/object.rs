pub mod presets;
use std::{fs::File, io::Read};

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

    pub fn from_obj_file(path: &str) -> Self {
        let file = File::open(path);
        let mut object = Object::new();
        match file {
            Ok(file) => {
                let mut reader = std::io::BufReader::new(file);
                let mut contents = String::new();
                reader.read_to_string(&mut contents).unwrap();

                let _ = contents
                    .split("\n")
                    .map(|line| {
                        if line.starts_with("v ") {
                            let vert = line
                                .split_at(2)
                                .1
                                .split(" ")
                                .map(|x| x.parse::<f32>().unwrap())
                                .collect::<Vec<f32>>();
                            object
                                .points
                                .push(nalgebra::geometry::Point3::new(vert[0], vert[1], vert[2]));
                        } else if line.starts_with("l ") {
                            let edge = line
                                .split_at(2)
                                .1
                                .split(" ")
                                .map(|x| x.parse::<usize>().unwrap())
                                .collect::<Vec<usize>>();
                            object.edges.push((edge[0] - 1, edge[1] - 1));
                        }
                    })
                    .collect::<Vec<_>>();
            }
            _ => {
                println!("File not found");
            }
        };

        object
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
