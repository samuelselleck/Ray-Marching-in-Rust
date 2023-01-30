use image::ImageBuffer;
use nalgebra as na;

extern crate rayon;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
type Scalar = f64;
type Point = na::Point3<Scalar>;
type Vector = na::Vector3<Scalar>;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = WIDTH;
fn main() {

    let origin = Point::new(0.0, 0.0, -3.5);

    let buff: Vec<u8> = (0..HEIGHT).into_par_iter().flat_map(|y| {
        (0..WIDTH).into_par_iter().flat_map(move |x| {
            let screen_point = Point::new(
                2.0 * (x as Scalar) / (WIDTH as Scalar) - 1.0,
                2.0 * (y as Scalar) / (HEIGHT as Scalar) - 1.0,
                0.0,
            );
            let dir = screen_point - origin;
            let res = ray_cast(origin, dir.normalize());
            [res, res, res]
        })
    }).collect();

    let img: ImageBuffer<image::Rgb<u8>, _> = ImageBuffer::from_vec(WIDTH, HEIGHT, buff).unwrap();

    img.save("output.png").unwrap();
    println!("DONE");
}

fn ray_cast(origin: Point, dir: Vector) -> u8 {
    let _light = Vector::new(-2.0, -3.5, 2.0).normalize();
    let light_pos = Point::new(-1.0, 2.0, 5.0);
    let mut p = origin;
    for _ in 0..50 {
        let closest = distance(p);
        let step = closest.magnitude();
        if step <= 0.05 {
            return (200.0 * (p - light_pos).normalize().dot(&(-closest / step)).max(0.0) + 54.0) as u8;
        }
        p = p + dir * step
    }
    0
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
    *vecs.into_iter().max_by(|&v2, &v1| v1.magnitude_squared().total_cmp(&v2.magnitude_squared())).unwrap()
}