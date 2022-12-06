# LightRay

## Introduction

LightRay is a physically based raytracer written in Rust.

The starting point for LightRay is Peter Shirley's excellent book series starting with [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).  The project will then be extend towards photorealistic rendering given previous experience with such renderers.

## 2) Output an Image

The LightRay 0.2 release corresponds with [Chapter 2 of Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage) which creates a simple gradient image and writes it to stdout in the PNM image format.  The output for 0.2 is the same gradient image created by the original source.

```bash
cargo run --release > current.pnm
display current.pnm
```
_(Using ImageMagick to display PNM files.)_

![gradients image](images/image-0.2-gradients.png).

There are a few differences between the LightRay implementation and the version presented in the book.

1) The nested for loop has been replaced with a more rusty iterator loop that clunkily recomputes i and j but transparently supports per-pixel parallel rendering by using into_par_iter() from Rust's "rayon" package.

2) The progress meter has been removed to simplify parallel rendering support.

3) The output of this version is a P6 binary PNM rather than a P3 text PNM.  P6 is also a simple header followed by W*H*3 bytes rather than text.  P6 is closer to what is created internally and is a more compact format.

4) A simplified Color struct from Chapter 3 has been implemented ahead of time along with the Color to u8 converter used by the PNM writer.

5) The f32 type was chosen over f64 primarily to enable future SSE optimization.  Use of f32 results in only a 5-10% faster render on the development machine.
