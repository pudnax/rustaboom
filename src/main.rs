mod vec3d;
use vec3d::Vec3d;

const SPHERE_RADIUS: f64 = 1.5;

fn signed_distance(p: &Vec3d) -> f64 {
    p.length() - SPHERE_RADIUS
}

fn sphere_trace(orig: Vec3d, dir: Vec3d, pos: &mut Vec3d) -> bool {
    *pos = orig;
    for _i in 0..128 {
        let d = signed_distance(pos);
        if d < 0. {
            return true;
        }
        *pos += dir * (d * 0.1).max(0.01);
    }
    false
}

fn distance_field_normal(pos: Vec3d) -> Vec3d {
    let eps = 0.1;
    let d = signed_distance(&pos);
    let nx = signed_distance(&(pos + Vec3d::new(eps, 0., 0.))) - d;
    let ny = signed_distance(&(pos + Vec3d::new(0., eps, 0.))) - d;
    let nz = signed_distance(&(pos + Vec3d::new(0., 0., eps))) - d;
    Vec3d::new(nx, ny, nz).normalized()
}

fn main() {
    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    let fov = std::f64::consts::PI / 3.;
    let framebuffer = &mut [Vec3d::new(0., 0., 0.); WIDTH * HEIGHT];

    let w = WIDTH as f64;
    let h = HEIGHT as f64;

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let id = i as f64;
            let jd = j as f64;
            let dir_x: f64 = (id + 0.5) - w / 2.;
            let dir_y: f64 = -(jd + 0.5) + h / 2.;
            let dir_z: f64 = -h / (2. * (fov / 2.).tan());
            let mut hit = Vec3d::new(0., 0., 0.);
            if sphere_trace(
                [0., 0., 3.].into(),
                Vec3d::new(dir_x, dir_y, dir_z).normalized(),
                &mut hit,
            ) {
                let light_dir = (Vec3d::new(10., 10., 10.) - hit).normalized();
                let light_intensity = 0.4f64.max(light_dir.dot(distance_field_normal(hit)));
                framebuffer[i + j * WIDTH] = Vec3d::new(1., 1., 1.) * light_intensity;
            } else {
                framebuffer[i + j * WIDTH] = Vec3d::new(0.2, 0.7, 0.8);
            }
        }
    }

    use std::io::prelude::*;
    let mut file = std::io::BufWriter::new(std::fs::File::create("out_r.ppm").unwrap());
    file.write_all(&format!("P6\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
        .unwrap();
    for frame in framebuffer.iter() {
        for j in 0..3 {
            let pixel = (255. * frame[j]) as u8;
            file.write_all(&[pixel]).unwrap();
        }
    }
}
