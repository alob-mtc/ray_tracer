mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: Vec3, redius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - redius * redius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / 2.0 * a;
    }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = Vec3::unit_vector(&(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let w = 200;
    let h = 100;
    let max_value = 255;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // construct a ppm file for image data
    println!("P3\n{} {}\n{}", w, h, max_value);
    for j in (0..h).rev() {
        for i in 0..w {
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;

            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
