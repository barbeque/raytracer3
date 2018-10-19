extern crate cgmath;
use cgmath::Vector3;

fn main() {
    let nx : i32 = 200;
    let ny : i32 = 100;

    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {

            let col = Vector3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
