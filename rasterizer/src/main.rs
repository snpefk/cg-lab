use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        let start_horizontal = Point { x: 100, y: 100 };
        let end_horizontal = Point { x: 300, y: 220 };

        let start_vertical = Point { x: 50, y: 50 };
        let end_vertical = Point { x: 60, y: 150 };

        draw_line(&mut d, start_horizontal, end_horizontal, Color::RED);
        draw_line(&mut d, start_vertical, end_vertical, Color::BLUE);
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn draw_line(d: &mut RaylibDrawHandle, p0: Point, p1: Point, color: Color) {
    let mut x0 = p0.x;
    let mut y0 = p0.y;

    let mut x1 = p1.x;
    let mut y1 = p1.y;

    if (x1 - x0).abs() > (y1 - y0).abs() {
        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }

        let ys = interpolate(x0, y0, x1, y1);
        for x in x0..x1 {
            let _y = ys[(x - x0) as usize]; // I promise
            d.draw_pixel(x, _y as i32, color);
        }
    } else {
        if y0 > y1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }
        let xs = interpolate(y0, x0, y1, x1);
        for y in y0..y1 {
            let _x = xs[(y - y0) as usize]; // I promise x2
            d.draw_pixel(_x as i32, y, color);
        }
    }
}

fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0 as f32];
    }

    let mut values = Vec::new();
    let a = (d1 - d0) as f32 / (i1 - i0) as f32;
    let mut d = d0 as f32;

    for _ in i0..i1 {
        values.push(d);
        d = d + a;
    }
    values
}
