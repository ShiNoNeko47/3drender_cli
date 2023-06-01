use cli_render::{object, render::View};
use nalgebra::Point3;

fn main() {
    let mut camera = View::new(
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(4.0, 3.0, 6.0),
    );
    camera.resolution = (150, 45);
    // camera.fov = PI;

    // camera.center_pixel = Some('o');
    // camera.clear_pixel = '.';

    // let object = object::Object::from_obj_file("src/data/icosphere.obj");
    let object = object::presets::cube(4.0, Point3::new(0.0, 0.0, 0.0));

    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::refresh();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut key;

    let mut render = camera.render(&object);
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
        let mut render = camera.render(&object);
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
