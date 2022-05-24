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
fn main() {
    let height: u32 = 800;
    let mut canvas = Canvas::new(1000, height);
    let mut projectile = Projectile {
        pos: Vec4::new_point(0.0, 10.0, 0.0),
        vel: Vec4::new_vec(304.19, 0.0, 0.0) / 1000.0, // glock 19
    };
    let env = Environment::default();
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
