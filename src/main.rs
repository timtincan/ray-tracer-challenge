use ray_tracer_challenge::{Point, Vector};

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
        velocity: Vector::new(2.0, 1.0, 0.0),
    };
    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut num_of_ticks = 0;
    while p.position.1 > 0.0 {
        p = tick(&e, p);
        num_of_ticks += 1;
        println!("{:?}", p.position);
        println!("{}", num_of_ticks);
    }
}

fn tick(environment: &Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}
