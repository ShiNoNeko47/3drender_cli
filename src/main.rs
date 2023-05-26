use cli_render::{
    object::{self, Point},
    view::View,
};

fn main() {
    let mut camera = View::new(Point::new(0.0, 0.0, 0.0), Point::new(-4.0, -4.0, -12.0));
    // camera.print_projection_plane();

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
