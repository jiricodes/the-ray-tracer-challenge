#![allow(dead_code)]
use rtlib::prelude::*;

struct Projectile {
    pub pos: Vec4,
    pub vel: Vec4,
}

impl Projectile {
    pub fn update(&mut self) {
        self.pos = self.pos + self.vel;
    }
}

struct Environment {
    grav: Vec4,
    wind: Vec4,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            grav: Vec4::new_vec(0.0, -9.807, 0.0) / 1000.0,
            wind: Vec4::new_vec(-2.0, 0.0, 0.0) / 1000.0,
        }
    }
}

fn tick_system(env: &Environment, projectile: &mut Projectile) {
    projectile.update();
    projectile.vel = projectile.vel + env.grav + env.wind;
}

/// Projectile
fn ch01() {
    let height: u32 = 500;
    let mut canvas = Canvas::new(900, height);

    // projectile settings
    let vel = Vec4::new_vec(1.0, 1.8, 0.0).normalize() * 11.25;
    let mut projectile = Projectile {
        pos: Vec4::new_point(0.0, 1.0, 0.0),
        vel: vel,
    };
    let env = Environment {
        grav: Vec4::new_vec(0.0, -0.1, 0.0),
        wind: Vec4::new_vec(-0.01, 0.0, 0.0),
    };
    let color = Color::RED;
    while projectile.pos.y > 0.0 {
        let _ = canvas.put_pixel(
            projectile.pos.x as u32,
            height - projectile.pos.y as u32,
            color,
        );
        tick_system(&env, &mut projectile);
    }
    println!("{}", canvas.into_ppm_string());
}

#[derive(Debug)]
struct Clock {
    points: Vec<Vec4>,
}

impl Clock {
    fn new(origin: Vec4) -> Self {
        let mut points = Vec::new();
        let mut cp = Mat4::translation(0.0, 1.0, 0.0) * origin;
        let r = Mat4::rotation_z(-PI / 6.0);
        for _ in 0..12 {
            points.push(cp);
            cp = &r * cp;
        }
        Self { points }
    }

    fn scale(&mut self, s: f32) {
        let scale = Mat4::scaling(s, s, s);
        for p in self.points.iter_mut() {
            let p1 = *p;
            *p = &scale * p1;
        }
    }
}

/// Clock
fn ch04() {
    let height = 200;
    let mut canvas = Canvas::new(200, height);
    let color = Color::WHITE;

    let origin = Vec4::new_point(0.0, 0.0, 0.0);

    let mut clock = Clock::new(origin);
    clock.scale(65.0);

    for p in clock.points.iter() {
        let x = (p.x + 100.0).round() as u32;
        let y = height - (p.y + 100.0).round() as u32;
        // dbg!((x, y));
        let _ = canvas.put_pixel(x, y, color);
    }
    println!("{}", canvas.into_ppm_string());
}

fn ch05() {
    let height = 200;
    let width = 200;
    let mut canvas = Canvas::new(width, height);
    let color = Color::WHITE;

    let mut s = Sphere::new();
    s.transform = Mat4::scaling(40.0, 20.0, 50.0) * Mat4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let direction = Vec4::new_vec(0.0, 0.0, 1.0);
    for y in 0..height {
        for x in 0..width {
            let r = Ray::new(
                &Vec4::new_point(
                    x as f32 - (width as f32 / 2.0),
                    y as f32 - (height as f32 / 2.0),
                    -1000.0,
                ),
                &direction,
            );
            let mut ixs = s.intersect(&r);
            ixs.sort();
            let hit = ixs.hit();
            if hit.is_some() {
                let _ = canvas.put_pixel(x, y, color);
            }
        }
    }
    println!("{}", canvas.into_ppm_string());
}

fn main() {
    ch05();
}
