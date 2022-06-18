#![allow(dead_code)]
use rtlib::prelude::*;

fn main() {
    // Plane material
    let mut plane_material = Material::default();
    plane_material.color = Color::rgb(1.0, 0.9, 0.9);
    plane_material.specular = 0.0;

    // Floor sphere
    plane_material.pattern = Some(patterns::CheckersPattern::default_boxed());
    let floor = shapes::Plane::new_boxed(None, Some(plane_material.clone()));

    // Left Wall
    let left_wall = shapes::Plane::new_boxed(
        Some(
            Mat4::translation(0.0, 0.0, 5.0)
                * Mat4::rotation_y(-PI / 4.0)
                * Mat4::rotation_x(PI / 2.0),
        ),
        Some(Material {
            color: Color::WHITE,
            reflectivness: 1.0,
            ..Default::default()
        }),
    );

    // Right wall
    let right_wall = shapes::Plane::new_boxed(
        Some(
            Mat4::translation(0.0, 0.0, 5.0)
                * Mat4::rotation_y(PI / 4.0)
                * Mat4::rotation_x(PI / 2.0),
        ),
        Some(Material {
            color: Color::rgb8(17, 17, 17),
            reflectivness: 0.15,
            ..Default::default()
        }),
    );

    // Large sphere
    let mut material = Material::default();
    material.color = Color::rgb8(117, 57, 147);
    material.diffuse = 0.7;
    material.specular = 0.9;
    material.transparency = 0.35;
    material.refractive_index = 3.0;
    material.pattern = None;
    material.diffuse = 0.5;
    material.specular = 1.0;
    material.shininess = 300.0;
    // material.color = Color::rgb(0.05, 0.0, 0.05);
    let transform = Mat4::translation(0.0, 1.0, 0.0) * Mat4::scaling(0.75, 0.75, 0.75);
    let large_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

    // // medium sphere
    let transform = Mat4::scaling(2000.0, 2000.0, 2000.0);
    let mut material = Material::default();
    material.color = Color::rgb8(0, 9, 76);
    material.diffuse = 0.4;
    material.shininess = 50.0;
    // material.reflectivness = 1.0;
    let medium_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

    // small CUBE
    let transform = Mat4::translation(-0.0, 0.66, 0.0) * Mat4::scaling(1.33, 1.33, 1.33);
    let mut material = Material::GLASS;
    material.color = Color::rgb(0.01, 0.01, 0.1);
    let small_cube = shapes::Cube::new_boxed(Some(transform), Some(material));

    // cylinder
    let cylinder = shapes::Cylinder::new_boxed(None, Some(Material::GLASS), Some((2.0, 3.0)), true);
    // // small sphere in shadows
    // let transform = Mat4::translation(-1.0, 0.15, -0.6) * Mat4::scaling(0.15, 0.15, 0.15);
    // let mut material = Material::default();
    // material.color = Color::rgb(1.0, 0.3, 0.1);
    // material.reflectivness = 1.0;
    // let shadow_sphere = shapes::Sphere::new_boxed(Some(transform), Some(material));

    // Light
    let light = PointLight::new(Vec4::point(-10.0, 10.0, -10.0), Color::WHITE);
    // // let light2 = PointLight::new(Vec4::point(10.0, 10.0, -10.0), Color::rgb(0.5, 0.5, 0.5));

    // world
    let mut w = World::new();
    w.add_object(floor);
    w.add_object(left_wall);
    w.add_object(right_wall);
    w.add_object(large_sphere);
    // w.add_object(medium_sphere);
    w.add_object(small_cube);
    // w.add_object(shadow_sphere);
    w.add_object(cylinder);
    w.add_light(light);
    // w.add_light(light2);

    // Camera
    let mut camera = Camera::new(1920, 1080, PI / 3.0);
    camera.view_transform(
        &Vec4::point(1.0, 5.0, -6.0),
        &Vec4::point(0.0, 1.0, 0.0),
        &Vec4::VEC_Y_ONE,
    );

    // Render
    let canvas = render(&camera, &w, &RenderSettings::default());

    println!("{}", canvas.into_ppm_string());
}
