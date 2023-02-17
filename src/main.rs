use image::ImageBuffer;
use nalgebra as na;

extern crate rayon;

mod sdf;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
type Scalar = f64;
type Point = na::Point3<Scalar>;
type Vector = na::Vector3<Scalar>;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = WIDTH;
fn main() {
    let origin = Point::new(0.0, 0.0, -3.5);

    let buff: Vec<u8> = (0..HEIGHT)
        .into_par_iter()
        .flat_map(|y| {
            (0..WIDTH).into_par_iter().flat_map(move |x| {
                let screen_point = Point::new(
                    2.0 * (x as Scalar) / (WIDTH as Scalar) - 1.0,
                    2.0 * (y as Scalar) / (HEIGHT as Scalar) - 1.0,
                    0.0,
                );
                let dir = (screen_point - origin) * 10000.0;
                let res = ray_cast(origin, dir);
                [res, res, res]
            })
        })
        .collect();

    let img: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::from_vec(WIDTH, HEIGHT, buff).unwrap();

    img.save("output.png").unwrap();
    println!("DONE");
}

fn ray_cast_towards(origin: Point, dir: Vector) -> (Scalar, Scalar) {
    let mut ds = 0.0;
    let dir_len = dir.magnitude();
    let mut min_closest = Scalar::MAX;
    for _ in 0..500 {
        //this is not a good idea later
        let closest = distance(origin + dir * ds).magnitude();
        min_closest = min_closest.min(closest);
        let step_size = closest;
        ds += step_size / dir_len;
        if step_size <= 0.005 || ds > 1.0 {
            return (ds.min(1.0), min_closest);
        }
    }
    panic!("to many iters");
}

fn ray_cast(origin: Point, dir: Vector) -> u8 {
    let _light = Vector::new(-2.0, -3.5, 2.0).normalize();
    let light_pos = Point::new(-1.0, 2.0, 5.0);
    let (param, _) = ray_cast_towards(origin, dir);
    if param >= 1.0 {
        0
    } else {
        let p = origin + dir * param;
        let normal = distance(p);
        let light_dir = light_pos - p;
        let (r, c) = ray_cast_towards(p + normal.normalize() * 0.01, light_dir);

        return (200.0 * r// * c.min(0.05) / 0.05
            * (p - light_pos)
                .normalize()
                .dot(&(-normal.normalize()))
                .max(0.0)
            + 54.0) as u8;
    }
}

fn distance(p: Point) -> Vector {
    max_vec(&[
        sphere(Point::new(2.0, 0.0, 10.0), 2.3, p),
        sphere(Point::new(0.0, 0.0, 9.3), 1.5, p),
        sphere(Point::new(1.0, -1.0, 8.0), 0.8, p),
        sphere(Point::new(1.0, 3.0, 8.0), 1.0, p),
        sphere(Point::new(0.0, 0.0, 20.0), 7.0, p),
    ])
}

fn sphere(origin: Point, r: Scalar, p: Point) -> Vector {
    let v = p - origin;
    let mag = v.magnitude();
    return v * (mag - r) / mag;
}

fn max_vec(vecs: &[Vector]) -> Vector {
    *vecs
        .into_iter()
        .max_by(|&v2, &v1| v1.magnitude_squared().total_cmp(&v2.magnitude_squared()))
        .unwrap()
}
