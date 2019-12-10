use blinds::traits::*;
use blinds::*;
use golem::{Context, GolemError};
use golem::program::{Attribute, ShaderDescription, Uniform, UniformType};
use golem::objects::{ColorFormat, UniformValue};

async fn app(window: Window, ctx: glow::Context, mut events: EventStream) -> Result<(), GolemError> {
    let ctx = Context::from_glow(ctx);

    let image = [
        // R, G, B
        255, 255, 255,
        0, 255, 0,
        255, 0, 0,
        255, 255, 255,
        0, 0, 255
    ];

    let texture = ctx.new_texture(Some(&image), 2, 2, ColorFormat::RGB)?;

    let vertices = [
        // Position         UV
        -0.5, -0.5,         0.0, 0.0,
        0.5, -0.5,          1.0, 0.0,
        0.5, 0.5,           1.0, 1.0,
        -0.5, 0.5,          0.0, 1.0,
    ];
    let indices = [
        0, 1, 2,
        2, 3, 0,
    ];

    let mut shader = ctx.new_shader(ShaderDescription {
        vertex_input: &[
            Attribute::Vector(2, "vert_position"),
            Attribute::Vector(2, "vert_uv"),
        ],
        fragment_input: &[ Attribute::Vector(2, "frag_uv") ],
        uniforms: &[ Uniform::new("image", UniformType::Sampler(2)) ],
        vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_uv = vert_uv;
        }"#,
        fragment_shader:
        r#" void main() {
            gl_FragColor = texture(image, frag_uv);
        }"#
    })?;

    let mut vb = ctx.new_vertex_buffer()?;
    let mut eb = ctx.new_element_buffer()?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind(&vb);
    shader.set_uniform("image", UniformValue::Int(0))?;

    ctx.bind_texture(Some(&texture), 0);

    ctx.clear();
    ctx.draw(&eb, 0..indices.len())?;
    window.present();

    while let Some(_) = events.next().await {
    }

    Ok(())
}

fn main() {
    blinds::run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
