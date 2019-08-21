extern crate rayon;
use rayon::prelude::*;

mod vec3d;
use vec3d::Vec3d;

const SPHERE_RADIUS: f64 = 1.5;
const NOISE_AMPLITUDE: f64 = 1.;

fn palette_fire(d: f64) -> Vec3d {
    let yellow = Vec3d::new(1.7, 1.3, 1.0); // note that the color is "hot", i.e. has components >1
    let orange = Vec3d::new(1.0, 0.6, 0.0);
    let red = Vec3d::new(1.0, 0.0, 0.0);
    let darkgray = Vec3d::new(0.2, 0.2, 0.2);
    let gray = Vec3d::new(0.4, 0.4, 0.4);

    let x = 0f64.max(1f64.min(d));
    if x < 0.25 {
        return vec3d::lerp(gray, darkgray, x * 4.);
    } else if x < 0.5 {
        return vec3d::lerp(darkgray, red, x * 4. - 1.);
    } else if x < 0.75 {
        return vec3d::lerp(red, orange, x * 4. - 2.);
    }
    vec3d::lerp(orange, yellow, x * 4. - 4.)
}

fn lerp(v0: f64, v1: f64, d: f64) -> f64 {
    v0 + (v1 - v0) * 0f64.max(1f64.min(d))
}

fn hash(n: f64) -> f64 {
    let x = n.sin() * 43758.5453;
    x - x.floor()
}

fn noise(x: &Vec3d) -> f64 {
    let p = Vec3d::new(x.x.floor(), x.y.floor(), x.z.floor());
    let mut f = Vec3d::new(x.x - p.x, x.y - p.y, x.z - p.z);
    f = f * (f.dot(Vec3d::new(3., 3., 3.) - f * 2.));
    let n = p.dot(Vec3d::new(1., 57., 113.));
    lerp(
        lerp(
            lerp(hash(n + 0.), hash(n + 1.), f.x),
            lerp(hash(n + 57.), hash(n + 58.), f.x),
            f.y,
        ),
        lerp(
            lerp(hash(n + 113.), hash(n + 114.), f.x),
            lerp(hash(n + 170.), hash(n + 171.), f.x),
            f.y,
        ),
        f.z,
    )
}

fn rotate(v: &Vec3d) -> Vec3d {
    Vec3d::new(
        Vec3d::new(0., 0.8, 0.6).dot(*v),
        Vec3d::new(-0.80, 0.36, -0.48).dot(*v),
        Vec3d::new(-0.60, -0.48, 0.64).dot(*v),
    )
}

fn fractal_brownian_motion(x: &Vec3d) -> f64 {
    let mut p = rotate(x);
    let mut f = 0.;
    f += 0.5000 * noise(&p);
    p = p * 2.32;
    f += 0.2500 * noise(&p);
    p = p * 3.03;
    f += 0.1250 * noise(&p);
    p = p * 2.61;
    f += 0.0625 * noise(&p);
    f / 0.9375
}

fn signed_distance(p: &Vec3d) -> f64 {
    let displacement = -fractal_brownian_motion(&(*p * 3.4)) * NOISE_AMPLITUDE;
    p.length() - (SPHERE_RADIUS + displacement)
}

fn sphere_trace(orig: Vec3d, dir: Vec3d, pos: &mut Vec3d) -> bool {
    if orig.dot(orig) - (orig.dot(dir)).powi(2) > SPHERE_RADIUS.powi(2) {
        return false;
    } // early discard

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
    const WIDTH: usize = 960;
    const HEIGHT: usize = 720;
    let fov = std::f64::consts::PI / 3.;
    let framebuffer = &mut vec![Vec3d::new(0., 0., 0.); WIDTH * HEIGHT];

    let w = WIDTH as f64;
    let h = HEIGHT as f64;

    framebuffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, frame)| {
            let id = (idx % WIDTH) as f64;
            let jd = (idx / WIDTH) as f64;
            let dir_x: f64 = (id + 0.5) - w / 2.;
            let dir_y: f64 = -(jd + 0.5) + h / 2.;
            let dir_z: f64 = -h / (2. * (fov / 2.).tan());
            let mut hit = Vec3d::new(0., 0., 0.);
            if sphere_trace(
                [0., 0., 3.].into(),
                Vec3d::new(dir_x, dir_y, dir_z).normalized(),
                &mut hit,
            ) {
                let noise_level = (SPHERE_RADIUS - hit.length()) / NOISE_AMPLITUDE;
                let light_dir = (Vec3d::new(10., 10., 10.) - hit).normalized();
                let light_intensity = 0.4f64.max(light_dir.dot(distance_field_normal(hit)));
                *frame = palette_fire((-0.25 + noise_level) * 2.) * light_intensity;
            } else {
                *frame = Vec3d::new(0.2, 0.7, 0.8);
            }
        });

    use std::io::prelude::Write;
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
