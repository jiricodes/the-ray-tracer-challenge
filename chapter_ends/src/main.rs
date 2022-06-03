#![allow(dead_code)]
use rtlib::prelude::*;

fn main() {
    // Plane material
    let mut plane_material = Material::default();
    plane_material.color = Color::rgb(1.0, 0.9, 0.9);
    plane_material.specular = 0.0;

    // Floor sphere
    let floor = shapes::Plane::new_boxed(None, Some(plane_material.clone()));

    // Left Wall
    let left_wall = shapes::Plane::new_boxed(
        Some(
            Mat4::translation(0.0, 0.0, 5.0)
                * Mat4::rotation_y(-PI / 4.0)
                * Mat4::rotation_x(PI / 2.0),
        ),
        Some(plane_material.clone()),
    );

    // Right wall
    let right_wall = shapes::Plane::new_boxed(
        Some(
            Mat4::translation(0.0, 0.0, 5.0)
                * Mat4::rotation_y(PI / 4.0)
                * Mat4::rotation_x(PI / 2.0),
        ),
        Some(plane_material),
    );

    // Large sphere
    let mut material = Material::default();
    material.color = Color::rgb(0.1, 1.0, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    let transform = Mat4::translation(-0.5, 1.0, 0.5);
    let large_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

    // medium sphere
    let transform = Mat4::translation(1.5, 0.5, -0.5) * Mat4::scaling(0.5, 0.5, 0.5);
    let mut material = Material::default();
    material.color = Color::rgb(0.5, 1.0, 0.1);
    material.diffuse = 0.4;
    material.shininess = 50.0;
    let medium_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

    // small sphere
    let transform = Mat4::translation(-1.5, 0.33, -0.75) * Mat4::scaling(0.33, 0.33, 0.33);
    let mut material = Material::default();
    material.color = Color::rgb(1.0, 0.8, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    let small_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

    // small sphere in shadows
    let transform = Mat4::translation(-1.0, 0.15, -0.6) * Mat4::scaling(0.15, 0.15, 0.15);
    let mut material = Material::default();
    material.color = Color::rgb(1.0, 0.3, 0.1);
    let shadow_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

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
    w.add_object(shadow_sphere);
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
