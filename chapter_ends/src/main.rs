#![allow(dead_code)]
use rtlib::prelude::*;

fn main() {
    // Plane material
    let mut plane_material = Material::default();
    plane_material.color = Color::rgb(1.0, 0.9, 0.9);
    plane_material.specular = 0.0;

    // Floor sphere
    let mut floor = Sphere::new();
    floor.transform = Mat4::scaling(10.0, 0.05, 10.0);
    floor.material = plane_material;

    // Left Wall
    let mut left_wall = Sphere::new();
    left_wall.transform = Mat4::translation(0.0, 0.0, 5.0)
        * Mat4::rotation_y(-PI / 4.0)
        * Mat4::rotation_x(PI / 2.0)
        * Mat4::scaling(10.0, 1.0, 10.0);
    left_wall.material = plane_material;

    // Right wall
    let mut right_wall = Sphere::new();
    right_wall.transform = Mat4::translation(0.0, 0.0, 5.0)
        * Mat4::rotation_y(PI / 4.0)
        * Mat4::rotation_x(PI / 2.0)
        * Mat4::scaling(10.0, 1.0, 10.0);
    right_wall.material = plane_material;

    // Large sphere
    let mut large_sphere = Sphere::new();
    large_sphere.transform = Mat4::translation(-0.5, 1.0, 0.5);
    large_sphere.material.color = Color::rgb(0.1, 1.0, 0.5);
    large_sphere.material.diffuse = 0.7;
    large_sphere.material.specular = 0.3;

    // medium sphere
    let mut medium_sphere = Sphere::new();
    medium_sphere.transform = Mat4::translation(1.5, 0.5, -0.5) * Mat4::scaling(0.5, 0.5, 0.5);
    medium_sphere.material.color = Color::rgb(0.5, 1.0, 0.1);
    medium_sphere.material.diffuse = 0.4;
    // medium_sphere.material.specular = 0.3;
    medium_sphere.material.shininess = 50.0;

    // small sphere
    let mut small_sphere = Sphere::new();
    small_sphere.transform = Mat4::translation(-1.5, 0.33, -0.75) * Mat4::scaling(0.33, 0.33, 0.33);
    small_sphere.material.color = Color::rgb(1.0, 0.8, 0.1);
    small_sphere.material.diffuse = 0.7;
    small_sphere.material.specular = 0.3;

    // Light
    let light = PointLight::new(Vec4::new_point(-10.0, 10.0, -10.0), Color::WHITE);

    // world
    let mut w = World::new();
    w.add_object(floor);
    w.add_object(left_wall);
    w.add_object(right_wall);
    w.add_object(large_sphere);
    w.add_object(medium_sphere);
    w.add_object(small_sphere);
    w.add_light(light);

    // Camera
    let mut camera = Camera::new(1920, 1080, PI / 3.0);
    camera.view_transform(
        &Vec4::new_point(0.0, 1.5, -5.0),
        &Vec4::new_point(0.0, 1.0, 0.0),
        &Vec4::VEC_Y_ONE,
    );

    // Render
    let canvas = render(&camera, &w);

    println!("{}", canvas.into_ppm_string());
}
