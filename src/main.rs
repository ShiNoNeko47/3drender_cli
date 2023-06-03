use std::f32::consts::PI;

use cli_render::{object, render::View};

fn main() {
    let mut camera = View::new(
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(0.0, 0.0, 5.0),
    );
    camera.resolution = (150, 45);
    // camera.fov = PI;

    // camera.center_pixel = Some('o');
    // camera.clear_pixel = '.';

    let object = object::Object::from_obj_file("src/data/monkey.obj");
    // let object = object::presets::cube(4.0, Point3::new(0.0, 0.0, 0.0));

    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::nodelay(ncurses::stdscr(), true);
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
        ncurses::napms(16);
        key = ncurses::getch();
        if key == -1 {
            continue;
        }
        ncurses::clear();
        // ncurses::addstr(&format!("{}", key));
        let mut info = format!("Key pressed: {}", key);
        match key {
            113 => {
                //q
                ncurses::endwin();
                break;
            }
            119 => {
                //w
                camera.move_forward(1.0);
            }
            115 => {
                //s
                camera.move_forward(-1.0);
            }
            100 => {
                //d
                camera.move_right(1.0);
            }
            97 => {
                //a
                camera.move_right(-1.0);
            }
            32 => {
                //<space>
                camera.move_up(1.0);
            }
            0 => {
                //<C-space>
                camera.move_up(-1.0);
            }
            1 => {
                //<C-a>
                camera.look_right(-PI / 16.0);
            }
            4 => {
                //<C-d>
                camera.look_right(PI / 16.0);
            }
            23 => {
                //<C-w>
                camera.look_up(-PI / 16.0);
            }
            19 => {
                //<C-s>
                camera.look_up(PI / 16.0);
            }
            65 => {
                //A
                camera.look_right_around(PI / 16.0);
            }
            68 => {
                //D
                camera.look_right_around(-PI / 16.0);
            }
            87 => {
                //W
                camera.look_up_around(-PI / 16.0);
            }
            83 => {
                //S
                camera.look_up_around(PI / 16.0);
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
