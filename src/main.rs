use cli_render::{object, render::View};

fn main() {
    let mut camera = View::new(
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(4.0, 3.0, 6.0),
    );
    camera.resolution = (150, 45);
    // camera.fov = PI;

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

    let icosphere = object::Object::from_obj_file("src/data/icosphere.obj");

    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::refresh();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut key;

    let mut render = camera.render(&icosphere);
    render.reverse();
    for _ in 0..render.len() {
        ncurses::addstr(&format!(
            "{}\n",
            String::from_utf8(render.pop().unwrap()).unwrap()
        ));
    }

    loop {
        key = ncurses::getch();
        ncurses::clear();
        // ncurses::addstr(&format!("{}", key));
        let mut info = format!("Key pressed: {}", key);
        match key {
            113 => {
                ncurses::endwin();
                break;
            }
            119 => {
                camera.move_forward(1.0);
            }
            115 => {
                camera.move_forward(-1.0);
            }
            100 => {
                camera.move_right(1.0);
            }
            97 => {
                camera.move_right(-1.0);
            }
            43 => {
                camera.zoom += 0.1;
                info = format!("Zoom: {}", camera.zoom);
            }
            45 => {
                if camera.zoom > 0.2 {
                    camera.zoom -= 0.1;
                }
                info = format!("Zoom: {}", camera.zoom);
            }
            _ => {}
        }
        let mut render = camera.render(&icosphere);
        render.reverse();
        for _ in 0..render.len() {
            ncurses::addstr(&format!(
                "{}\n",
                String::from_utf8(render.pop().unwrap()).unwrap()
            ));
        }
        ncurses::addstr(&info);
    }
}
