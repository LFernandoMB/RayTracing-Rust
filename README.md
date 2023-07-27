# RayTracing-Rust
 Challenge to transpose the c++ code to rust

## References
[_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

## Description
Transcript of the ``Ray Tracing in One Weekend`` challenge, which was originally built in c++. This document provides a code translation for Rust.

## What is Ray Tracing?
Ray Tracing, technology that uses “ray tracing” to create better lighting effects, isn't quite a novelty created just now to make games more realistic. The technique has existed for years in cinema, but it still couldn't be applied, in real-time, in games due to technical limitations until then.

In short, Ray Tracing applied in games (and with RT Cores) uses artificial intelligence to create, in real time, more realistic lighting and shading effects. This is also true when generating more physically believable reflections, refractions and general lighting. The idea is to make the sensation of looking at the TV (or monitor) as close as possible to the experience of looking through a window, in the real world.

Part of this high processing is thanks to the new Turing architecture, which brings performance six times greater than its predecessor: Pascal – both belonging to Nvidia.

Turing promises to support games in 4K and 60 frames per second, even in the most advanced games. In addition, it allows shaders to focus processing on physical areas with high detail, increasing overall performance.

Now, more specifically, Ray Tracing comes to replace (over time or until something more advanced appears) the technique known as Rasterization, used to generate those “realistic” graphics that we all love, but which are actually nothing more than a 2D projection onto the screen of a 3D environment.

Rasterization is a quick process and generates images in a simpler way. For anyone who's been playing for a while, it's easy to remember the days when the first 3D titles were very boxy and with rudimentary lighting techniques. Sometimes just switching between shades of colors to give an idea of shading and depth.
`https://tecnoblog.net/responde/o-que-e-ray-tracing/`

## Step-by-Step Transcription Results
First Images Generated

![image](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/17b47425-47bc-49cc-8c15-f7c20a098c34)
![first_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/fabc8638-3e6b-45f9-a418-9aed7b047333)

Second Image Generated 

![second_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/186a97d4-4f92-4dfe-84db-11a2be4bebb8)

Some learning outcomes and validation of concepts learned initially

![image](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/3d8917f0-6a39-4a7e-a395-5173e362a0d4)

Third Image Generated - A blue-to-white gradient depending on the ray Y coordinate

![third_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/e32e77e1-6b7f-4788-ac9e-a54644f056df)

Fourth Image Generated - A simple red sphere

![fourth_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/35ad77c4-82b0-45d3-acb0-194cc4f86161)

Fifth Image Generated - A sphere colored according to its normal vectors

![fifth_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/426f0396-e540-41a7-8105-497df50787b6)

Sixth Image Generated - Render of normals-colored sphere with ground

![sixth_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/c7c301af-b402-49f2-88f3-8ccfea9b9cb9)

Seventh Image Generated - Improving the resolution of the sphere with 'Antialiasing' and resizing for better visualization

![resolution](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/be466e7b-d400-41e8-8f72-9df8d471fd74)

Eighth Image Generated - First render of a diffuse sphere

![darkGama](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/37cedf8a-7fe6-4a88-8698-fa4934cd6190)

Ninth Image Generated - Just change de color of sphere to test concepts 

![dark](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/b64a422f-5501-4617-9a62-0107a593dc62)

Tenth Image Generated - Diffuse sphere, with gamma correction

![darkGamaWithoutAcne](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/271d3107-4124-41a5-bc73-f359a002ef3f)

Eleventh Image Generated - Correct rendering of Lambertian spheres

![sphereLambertiana](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/6bf58230-24d6-48d4-b530-7d084b3d4454)

Twelfth Image Generated - 

![iron](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/5656ee8c-f2ae-4039-ad67-708aaa3c0cc4)

Thirteenth Image Generated - 

![ironFuzzed](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/952d9eca-5929-467f-bca9-e8a915515048)

Fourteenth Image Generated - Just a sphere to test concepts 

![eighth_img](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/5f827436-f42d-4315-9464-a3c05f92cb34)

Fifteenth Image Generated - Glass sphere that always refracts

![vidro](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/22c0cdbf-fcf9-4819-af44-3a10f2ac06a0)

Sixteenth Image Generated - Glass sphere that sometimes refracts

![dieletrico](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/9a2e7ec6-2eb3-47e5-aa39-fba01eb1cdc5)

Seventeenth Image Generated - A hollow glass sphere

![esferoOca](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/002b429c-f46d-47ae-92f4-c2d2182d4fdd)

Eighteenth Image Generated - A wide-angle view

![twoballs](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/e38a7f6f-22c6-4b16-9236-ade1625146da)

Nineteenth Image Generated -  A Distant view

![pontoVista](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/be0b4468-0e9c-4314-8151-22160fd835df)

Twentieth Image Generated - Zooming in

![pontoVista20](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/b21c2e19-b919-4469-b68b-241cfe82e685)

Twenty-First Image Generated - Spheres with depth-of-field

![desfoqueDeCampo](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/1291b9cc-0f1f-44bd-9507-9d80a46bdb42)

Final Images Generated - Wrong Parameter on the first image

![image](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/bb631de6-05d3-4220-b347-cc50d92dbb8e)

Final Images Generated - With Parallelism (With the Rayon library)

![final_paralel](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/306ecf73-b128-4a41-af37-306db7a3baf3)

Final Image Generated - Size: 1200x800
![final_img_1600x800](https://github.com/LFernandoMB/RayTracing-Rust/assets/91624923/dd010c83-7b79-48fa-baa5-8821cd37e2bc)
