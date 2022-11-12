use pancurses::{Input, Window};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tiny_pseudo_3d_renderer::*;
use toml::Value;

fn main() {
    let mut window = pancurses::initscr();
    window.refresh();
    window.keypad(true);
    pancurses::noecho();
    let (world, mut camera) = parse_config(Path::new("config.toml"));
    loop {
        render_frame(&window, &world, &camera);
        match window.getch() {
            Some(Input::Character('w')) => {
                camera.translate(Vector2::forward().rotate(camera.rotation))
            }
            Some(Input::Character('s')) => {
                camera.translate(-Vector2::forward().rotate(camera.rotation))
            }
            Some(Input::Character('a')) => {
                camera.translate(-Vector2::right().rotate(camera.rotation))
            }
            Some(Input::Character('d')) => {
                camera.translate(Vector2::right().rotate(camera.rotation))
            }
            Some(Input::Character('q')) => camera.rotate(1.),
            Some(Input::Character('e')) => camera.rotate(-1.),
            Some(Input::KeyDC) => break,
            Some(Input::KeyResize) => {
                window.resize(0, 0);
            }
            Some(_) | None => (),
        }
    }
    pancurses::endwin();
}

fn fill_if<F: Fn(i32, i32) -> bool>(window: &Window, f: F, c: char) {
    for y in 0..window.get_max_y() {
        for x in 0..window.get_max_x() {
            window.mv(y, x);
            if f(x, y) {
                window.addch(c);
            }
        }
    }
    window.refresh();
}

fn render_frame(window: &Window, world: &World, camera: &Camera) {
    window.clear();
    let center_x = window.get_max_x() / 2;
    let center_y = window.get_max_y() / 2;

    for y in 0..window.get_max_y() {
        window.mv(y, 0);
        window.addch(format!("{}", y - center_y).chars().last().unwrap());
    }
    for x in 0..window.get_max_x() {
        window.mv(window.get_max_y() - 1, x);
        window.addch(format!("{}", x - center_x).chars().last().unwrap());
    }

    let world = camera.see(world.clone());
    macro_rules! render_wall {
        ($v: expr, $u: expr) => {
            let (zv, zu) = ($v.1 / 2., $u.1 / 2.);
            let a = (zu - zv) / ($u.0 - $v.0);
            let b = zv - a * $v.0;
            let min_x = $v.0.min($u.0);
            let max_x = $v.0.max($u.0);
            fill_if(window, |x, y| {
                let (x, y) = (x - center_x, y - center_y);
                let (x, y) = (x as f64, y as f64);
                min_x <= x && x <= max_x && -a * x - b <= y && y <= a * x + b
            }, 'O');
        }
    }
    let last = world
        .vertexes
        .iter()
        .reduce(|v, u| {
            render_wall!(v, u);
            u
        })
        .unwrap();
    let first = world.vertexes.first().unwrap();
    render_wall!(last, first);
}

fn parse_config(file_path: &Path) -> (World, Camera) {
    let s = File::open(file_path).map_or_else(
        |_| String::from(include_str!("default_config.toml")),
        |mut f| {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            buf
        },
    );
    let value = s.parse::<Value>().unwrap();
    fn as_vector2(value: &Value) -> Option<Vector2> {
        Some(Vector2(
            value.as_array()?.first()?.as_float()?,
            value.as_array()?.last()?.as_float()?,
        ))
    }

    let camera = Camera {
        position: as_vector2(&value["camera"]["position"]).unwrap(),
        rotation: value["camera"]["rotation"].as_float().unwrap(),
        y_fixpoint: value["camera"]["y_fixpoint"].as_float().unwrap(),
        z_fixpoint: value["camera"]["z_fixpoint"].as_float().unwrap(),
        vertex_height: value["camera"]["vertex_height"].as_float().unwrap(),
        translation_speed: value["camera"]["translation_speed"].as_float().unwrap(),
        rotation_speed: value["camera"]["rotation_speed"].as_float().unwrap(),
    };
    let world = World::new(
        value["world"]["vertexes"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(as_vector2)
            .map(Option::unwrap)
            .collect(),
    );

    (world, camera)
}
