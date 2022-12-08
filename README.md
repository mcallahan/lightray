# LightRay

## Introduction

LightRay is a physically based raytracer written in Rust.

The starting point for LightRay is Peter Shirley's excellent book series starting with [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).  The project will then be extend towards photorealistic rendering given previous experience with such renderers.

## 2) Output an Image

The LightRay 0.2.0 release corresponds with [Chapter 2 of Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#outputanimage) which creates a simple gradient image and writes it to stdout in the PNM image format.  The output for 0.2.0 is the same gradient image created by the original source.

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

## 3) Points, Vectors, and Colors

The LightRay 0.3.0 release adds support for Vector3, Point3 and Color structs.  These are implemented similarly to [Chapter 3](https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class) except using Rust traits to support operations.  Implementing these for Chapter 3 does not change the generated image in any way.

Unlike the reference book the Vector3, Point3, and Color structs are distinct types.  There are several reasons for this.

1) Distinct types means that the operations on vectors and points can be limited to those that make sense.  For example `P + V -> P`, `P + P -> undefined`, `V * V -> undefined`.  Getting these correct means catching math mistakes at compile time.

2) The Neg and Sub traits on Color are not physically based(outside of quantum effects) and trying to use them generally means there is a computation error.  They are not present in this implementation to prevent that from happening, although they could be added back for artistic reasons.

3) For physically based rendering the Color struct should be samples in the frequency domain and not RGB which is a perceptual domain.  These are close but using RGB can lead to subtle blending issues.  This is not yet implemented but is anticipated for a far future release.

There are some other design differences as well.

1) The operation traits do not support borrow versions at this time and the structs support Copy.  Testing showed that #[inline] and Copy were about the same speed as explicitly borrowing everywhere while making the math operations less clunky.

2) Casts of Color/Point3/Vector3 to/from slices is not yet supported.  Realistically these operations will ultimately be needed in some way to import triangle meshes efficiently.
