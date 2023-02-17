use std::fs::File;
use std::io::Write;

use ray_tracer_challenge::{Canvas, Color, Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn main() {
    let mut p = Projectile {
        position: Point(0.0, 1.0, 0.0),
        velocity: 5.0 * Vector::new(2.0, 1.0, 0.0).normalize(),
    };
    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };
    let mut canvas = Canvas::new(200, 110);
    let c = Color {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
    };

    let mut num_of_ticks = 0;
    while p.position.1 > 0.0 {
        canvas.write_pixel(
            (p.position.0) as usize,
            (canvas.height as f64 - p.position.1) as usize,
            c,
        );
        p = tick(&e, p);
        num_of_ticks += 1;
        println!("{:?}", p.position);
        println!("{}", num_of_ticks);
    }

    let buffer = canvas.to_ppm();
    let mut file = File::create("canvas.ppm").unwrap();
    write!(file, "{}", buffer).unwrap();
}

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}
