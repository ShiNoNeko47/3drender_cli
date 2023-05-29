use cli_render::{object, view::View};

fn main() {
    let mut camera = View::new(
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(4.0, 3.0, 6.0),
    );
    camera.border[0] = ' ';
    camera.border[1] = ' ';
    // camera.resolution = (300, 75);

    // camera.center_pixel = Some('o');
    // camera.clear_pixel = '.';

    let mut object = object::Object::new();
    object.add_point(-2.0, -2.0, -2.0); //0
    object.add_point(-2.0, -2.0, 2.0); //1
    object.add_point(-2.0, 2.0, -2.0); //2
    object.add_point(-2.0, 2.0, 2.0); //3
    object.add_point(2.0, -2.0, -2.0); //4
    object.add_point(2.0, -2.0, 2.0); //5
    object.add_point(2.0, 2.0, -2.0); //6
    object.add_point(2.0, 2.0, 2.0); //7

    object.add_edge(0, 1);
    object.add_edge(0, 2);
    object.add_edge(0, 4);
    object.add_edge(5, 4);
    object.add_edge(5, 1);
    object.add_edge(5, 7);
    object.add_edge(2, 3);
    object.add_edge(2, 6);
    object.add_edge(4, 6);
    object.add_edge(7, 6);
    object.add_edge(7, 3);
    object.add_edge(1, 3);

    // object.add_point(0.0, 0.0, 0.0);

    camera.render(&object);
}
