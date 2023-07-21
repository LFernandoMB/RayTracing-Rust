// Título (série) : “Ray Tracing em uma série de fim de semana”
// Título (livro) : “Ray Tracing em um fim de semana”
// Autor : Peter Shirley
// Editores : Steve Hollasch, Trevor David Black
// Versão/Edição : v3.2.3
// Data : 2020-12-07
// URL (série) : https://raytracing.github.io/
// URL (livro) : https://raytracing.github.io/books/RayTracingInOneWeekend.html

// Transposing challenge code from c++ to Rust

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let g = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

// To generate the image in ppm document: cargo run > first_img.ppm 