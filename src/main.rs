use cli_render::{object, view::View};

fn main() {
    let mut camera = View::new(
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(5.0, 3.0, 10.0),
    );
    // camera.resolution = (120, 30);
    // camera.center_pixel = Some('o');
    // camera.clear_pixel = '.';

    let mut object = object::Object::new();
    object.add_point(-2.0, -2.0, -2.0);
    object.add_point(-2.0, -2.0, 2.0);
    object.add_point(-2.0, 2.0, -2.0);
    object.add_point(-2.0, 2.0, 2.0);
    object.add_point(2.0, -2.0, -2.0);
    object.add_point(2.0, -2.0, 2.0);
    object.add_point(2.0, 2.0, -2.0);
    object.add_point(2.0, 2.0, 2.0);

    // object.add_point(0.0, 0.0, 0.0);

    camera.render(&object);
}
