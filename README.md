# Golem

`golem` is an opinionated mostly-safe graphics API

 When possible, `golem` should make simple things safe (bind objects before acting on them, or
 check if they're bound for objects that are expensive to bind.) However, when not possible or
 convenient (bounds checking the indices in an element buffer, for example), `golem` provides
 unsafe APIs with well-defined safety conditions.

 A minimal example to display a triangle:

 ```rust
 # use golem::*;
 # use golem::Dimension::*;
 # fn func(ctx: &Context) -> Result<(), GolemError> {
 let vertices = [
     // Position         Color
     -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
     0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
     0.0, 0.5,           0.0, 0.0, 1.0, 1.0
    ];
 let indices = [0, 1, 2];

 let mut shader = ShaderProgram::new(
     ctx,
     ShaderDescription {
         vertex_input: &[
             Attribute::new("vert_position", AttributeType::Vector(D2)),
             Attribute::new("vert_color", AttributeType::Vector(D4)),
         ],
         fragment_input: &[Attribute::new("frag_color", AttributeType::Vector(D4))],
         uniforms: &[],
         vertex_shader: r#" void main() {
         gl_Position = vec4(vert_position, 0, 1);
         frag_color = vert_color;
     }"#,
         fragment_shader: r#" void main() {
         gl_FragColor = frag_color;
     }"#,
     },
 )?;

 let mut vb = VertexBuffer::new(ctx)?;
 let mut eb = ElementBuffer::new(ctx)?;
 vb.set_data(&vertices);
 eb.set_data(&indices);
 shader.bind();

 ctx.clear();
 unsafe {
     shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
 }
 # Ok(()) }
 ```

The core type of `golem` is the `Context`, which is constructed from the `glow::Context`.
 From the `Context`, `ShaderProgram`s are created, which take in data from `Buffer`s. Once
 the data is uploaded to the GPU via `Buffer::set_data`, it can be drawn via `ShaderProgram::draw`.

 ## Initializing

 The user is respnsible for windowing and providing a valid glow Context to create a
 `Context`. You can try out the [`blinds`](https://crates.io/crates/blinds) crate, which works
 well with `golem`, but using `winit` directly or other windowing solutions like `sdl2` are also
 options.

 ## OpenGL Versions
 It currently is implemented via glow, and it targets OpenGL 3.2 on desktop and WebGL 1 (so it
 should run on a wide range of hardware.) GL 3.2 is selected for maximum desktop availability,
 ! and WebGL 1 is available on 97% of clients to WebGL's 75% (taken from caniuse.com at time of
 writing.)
