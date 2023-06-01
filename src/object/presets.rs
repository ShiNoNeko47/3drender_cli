use nalgebra::Point3;

use super::Object;

pub fn cube(size: f32, center: Point3<f32>) -> Object {
    let mut cube = Object::new();
    cube.add_point(
        center.x - size / 2.0,
        center.y - size / 2.0,
        center.z - size / 2.0,
    ); //0
    cube.add_point(
        center.x - size / 2.0,
        center.y - size / 2.0,
        center.z + size / 2.0,
    ); //1
    cube.add_point(
        center.x - size / 2.0,
        center.y + size / 2.0,
        center.z - size / 2.0,
    ); //2
    cube.add_point(
        center.x - size / 2.0,
        center.y + size / 2.0,
        center.z + size / 2.0,
    ); //3
    cube.add_point(
        center.x + size / 2.0,
        center.y - size / 2.0,
        center.z - size / 2.0,
    ); //4
    cube.add_point(
        center.x + size / 2.0,
        center.y - size / 2.0,
        center.z + size / 2.0,
    ); //5
    cube.add_point(
        center.x + size / 2.0,
        center.y + size / 2.0,
        center.z - size / 2.0,
    ); //6
    cube.add_point(
        center.x + size / 2.0,
        center.y + size / 2.0,
        center.z + size / 2.0,
    ); //7
    cube.add_edge(0, 1);
    cube.add_edge(0, 2);
    cube.add_edge(0, 4);
    cube.add_edge(5, 4);
    cube.add_edge(5, 1);
    cube.add_edge(5, 7);
    cube.add_edge(2, 3);
    cube.add_edge(2, 6);
    cube.add_edge(4, 6);
    cube.add_edge(7, 6);
    cube.add_edge(7, 3);
    cube.add_edge(1, 3);

    cube
}
