use std::ffi::{CStr, CString};
use std::fmt::Formatter;
use std::io::Read;
use std::num::NonZeroU32;
use std::string::ToString;
use std::sync::Arc;
use std::time::{Instant, SystemTime};

use chrono::Timelike;
use glutin::api::cgl::config::Config;
use glutin::api::cgl::display::Display;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, GlContext, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use rand::Rng;
use raw_window_handle::{
    AppKitDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use canvas_2d::context::compositing::composite_operation_type::CompositeOperationType;
use canvas_2d::context::drawing_paths::fill_rule::FillRule;
use canvas_2d::context::fill_and_stroke_styles::paint::PaintStyle;
use canvas_2d::context::line_styles::line_cap::LineCap;
use canvas_2d::context::text_styles::text_align::TextAlign;
use canvas_2d::context::{Context, ContextWrapper};
use canvas_2d::to_data_url;
use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::{get_shader_info_log, GLContext};
use canvas_core::image_asset::ImageAsset;
use canvas_webgl::prelude::{WebGLResult, WebGLState, WebGLVersion};
use canvas_webgl::webgl::{
    canvas_native_webgl_attach_shader, canvas_native_webgl_bind_buffer,
    canvas_native_webgl_bind_texture, canvas_native_webgl_buffer_data,
    canvas_native_webgl_buffer_data_f32, canvas_native_webgl_clear,
    canvas_native_webgl_clear_color, canvas_native_webgl_compile_shader,
    canvas_native_webgl_create_buffer, canvas_native_webgl_create_program,
    canvas_native_webgl_create_shader, canvas_native_webgl_create_texture,
    canvas_native_webgl_delete_program, canvas_native_webgl_delete_shader,
    canvas_native_webgl_draw_arrays, canvas_native_webgl_enable_vertex_attrib_array,
    canvas_native_webgl_get_attrib_location, canvas_native_webgl_get_error,
    canvas_native_webgl_get_program_info_log, canvas_native_webgl_get_program_parameter,
    canvas_native_webgl_get_shader_info_log, canvas_native_webgl_get_shader_parameter,
    canvas_native_webgl_get_tex_parameter, canvas_native_webgl_get_uniform_location,
    canvas_native_webgl_link_program, canvas_native_webgl_pixel_storei,
    canvas_native_webgl_shader_source, canvas_native_webgl_tex_image2d,
    canvas_native_webgl_tex_image2d_asset, canvas_native_webgl_tex_parameteri,
    canvas_native_webgl_uniform2f, canvas_native_webgl_uniform4fv, canvas_native_webgl_use_program,
    canvas_native_webgl_vertex_attrib_pointer, canvas_native_webgl_viewport,
};
use canvas_webgl::webgl2::{
    canvas_native_webgl2_bind_vertex_array, canvas_native_webgl2_create_vertex_array,
};

// Vertex and fragment shaders

// Function to compile a shader
fn compile_shader(state: &mut WebGLState, source: &str, shader_type: u32) -> u32 {
    let shader = canvas_native_webgl_create_shader(shader_type, state);
    canvas_native_webgl_shader_source(shader, source, state);
    canvas_native_webgl_compile_shader(shader, state);

    match canvas_native_webgl_get_shader_parameter(shader, 0x8B81, state) {
        WebGLResult::Boolean(status) => {
            if !status {
                println!(
                    "Shader compilation failed: {:?}",
                    canvas_native_webgl_get_shader_info_log(shader, state)
                );
                canvas_native_webgl_delete_shader(shader, state);
                return 0;
            }
        }
        _ => {}
    }

    return shader;
}

// Function to link a program
fn link_program(state: &mut WebGLState, vertex_shader: u32, fragment_shader: u32) -> u32 {
    let program = canvas_native_webgl_create_program(state);
    canvas_native_webgl_attach_shader(program, vertex_shader, state);
    canvas_native_webgl_attach_shader(program, fragment_shader, state);
    canvas_native_webgl_link_program(program, state);

    match canvas_native_webgl_get_program_parameter(program, 0x8B82, state) {
        WebGLResult::Boolean(result) => {
            if !result {
                println!(
                    "Program linking failed: {:?}",
                    canvas_native_webgl_get_program_info_log(program, state)
                );
                return 0;
            }
        }
        _ => {}
    }

    return program;
}

// Function to set the rectangle vertices
fn set_rectangle(state: &mut WebGLState, x: f32, y: f32, width: f32, height: f32) {
    let vertices = [
        x,
        y,
        x + width,
        y,
        x,
        y + height,
        x,
        y + height,
        x + width,
        y,
        x + width,
        y + height,
    ];
    canvas_native_webgl_buffer_data_f32(0x8892, &vertices, 0x88E4, state);
}

fn test(state: &mut WebGLState, gl_state: &mut WebGLState, ctx_2d: &mut ContextWrapper) {
    // Compile shaders

    let fs = include_str!("./fs.txt");
    let vs = include_str!("./vs.txt");

    let fragment_shader = compile_shader(state, fs, 0x8B30);

    let vertex_shader = compile_shader(state, vs, 0x8B31);

    // Link shaders into a program
    let program = link_program(state, vertex_shader, fragment_shader);

    // Get attribute and uniform locations
    let position_attribute_location =
        canvas_native_webgl_get_attrib_location(program, "a_position", state);
    let color_uniform_location =
        canvas_native_webgl_get_uniform_location(program, "u_color", state);

    // Create a buffer and set the position data
    let position_buffer = canvas_native_webgl_create_buffer(state);
    canvas_native_webgl_bind_buffer(0x8892, position_buffer, state);

    set_rectangle(
        state,
        0.,
        0.,
        state.get_drawing_buffer_width() as f32,
        state.get_drawing_buffer_height() as f32,
    );

    canvas_native_webgl_clear_color(1.0, 1.0, 1.0, 1.0, state);
    canvas_native_webgl_clear(0x00004000, state);

    canvas_native_webgl_use_program(program, state);

    canvas_native_webgl_bind_buffer(0x8892, position_buffer, state);

    // Tell the attribute how to get data out of the position buffer
    let size = 2; // 2 components per iteration
    let ttype = 0x1406; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0; // start at the beginning of the buffer

    canvas_native_webgl_vertex_attrib_pointer(
        position_attribute_location as u32,
        size,
        ttype,
        normalize,
        stride,
        offset,
        state,
    );

    dbg!("webgl error {}", canvas_native_webgl_get_error(state));

    canvas_native_webgl_enable_vertex_attrib_array(position_attribute_location as u32, state);

    dbg!("webgl error {}", canvas_native_webgl_get_error(state));

    canvas_native_webgl_uniform4fv(color_uniform_location, &[1.0, 1.0, 1.0, 1.0], state);

    // Draw the rectangle
    let primitive_type = 0x0004;
    let offset_draw = 0;
    let count = 6;

    canvas_native_webgl_draw_arrays(primitive_type, offset_draw, count, state);

    let w = gl_state.drawing_buffer_width();
    let h = gl_state.drawing_buffer_height();

    // state.remove_if_current();
    gl_state.make_current();

    let mut ctx = ctx_2d.get_context_mut();

    ctx.set_font("30px Arial");

    let fill_style = PaintStyle::new_color_str("white").unwrap();
    ctx.set_fill_style(fill_style);
    ctx.rect(0., 0., w as f32, h as f32);
    ctx.fill(None);

    let fill_style = PaintStyle::new_color_str("red").unwrap();
    ctx.set_fill_style(fill_style);
    ctx.fill_text("Hello, WebGL!", 50., 50., None);

    // ctx.flush_and_sync_cpu();
    //
    // gl_state.swap_buffers();

    let (_, texture_id) = ctx.snapshot_with_texture_id();

    // let mut buf = ctx.read_pixels();
    //  drop(ctx);

    // dbg!(to_data_url(ctx_2d, "image/jpg", 100));

    // gl_state.remove_if_current();
    state.make_current();

    gl_state.init_transfer_surface(texture_id);

    gl_state.draw_tex_image_2d(
        0x0DE1, 0, w as u32, h as u32, 0x1908, 0x1908, false, texture_id,
    );

    // canvas_native_webgl_tex_image2d(
    //     0x0DE1,
    //     0,
    //     0x1908,
    //     w,
    //     h,
    //     0,
    //     0x1908,
    //     0x1401,
    //     buf.as_slice(),
    //     state,
    // );

    canvas_native_webgl_draw_arrays(primitive_type, offset_draw, count, state);
}

fn leaves(state: &mut WebGLState, gl_state: &mut WebGLState, ctx_2d: &mut ContextWrapper) {
    let drawingBufferWidth = state.get_drawing_buffer_width();
    let drawingBufferHeight = state.get_drawing_buffer_height();

    let vertexShader2d = "
    #version 330
in vec2 a_texCoord;
 in vec2 a_position;


uniform vec2 u_resolution;

out vec2 v_texCoord;

void main() {
    // convert the rectangle from pixels to 0.0 to 1.0
    vec2 zeroToOne = a_position / u_resolution;

    // convert from 0->1 to 0->2
    vec2 zeroToTwo = zeroToOne * 2.0;

    // convert from 0->2 to -1->+1 (clipspace)
    vec2 clipSpace = zeroToTwo - 1.0;

    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);

    // pass the texCoord to the fragment shader
    // The GPU will interpolate this value between points.
    v_texCoord = a_texCoord;
}";

    let fragmentShader2d = "
        #version 330
        precision mediump float;

// our texture
uniform sampler2D u_image;

// the texCoords passed in from the vertex shader.
in vec2 v_texCoord;

out vec4 FragColor;

void main() {
    FragColor = texture(u_image, v_texCoord);
}";

    fn set_rectangle(state: &mut WebGLState, x: f32, y: f32, width: f32, height: f32) {
        let x1 = x;
        let x2 = x + width;
        let y1 = y;
        let y2 = y + height;
        canvas_native_webgl_buffer_data_f32(
            gl_bindings::ARRAY_BUFFER,
            &[x1, y1, x2, y1, x1, y2, x1, y2, x2, y1, x2, y2],
            gl_bindings::STATIC_DRAW,
            state,
        );
    }

    // function main() {
    //     const asset = new global.ImageAsset();
    //     asset.fromUrl("https://webglfundamentals.org/webgl/resources/leaves.jpg")
    //         .then(image =>{
    //             render(asset);
    //         });
    // }

    // setup GLSL program
    let program = create_program_from_scripts(
        state,
        [
            (vertexShader2d, gl_bindings::VERTEX_SHADER),
            (fragmentShader2d, gl_bindings::FRAGMENT_SHADER),
        ],
    )
    .unwrap();

    // look up where the vertex data needs to go.
    let position_location = canvas_native_webgl_get_attrib_location(program, "a_position", state);
    let texcoord_location = canvas_native_webgl_get_attrib_location(program, "a_texCoord", state);

    // Create a buffer to put three 2d clip space points in
    let positionBuffer = canvas_native_webgl_create_buffer(state);

    // Bind it to ARRAY_BUFFER (think of it as ARRAY_BUFFER = positionBuffer)
    canvas_native_webgl_bind_buffer(gl_bindings::ARRAY_BUFFER, positionBuffer, state);

    let width;
    let height;

    {
        let ctx = ctx_2d.get_context();
        width = ctx.device().width;
        height = ctx.device().height;
    }

    // Set a rectangle the same size as the image.
    set_rectangle(state, 0., 0., width, height);

    // provide texture coordinates for the rectangle.
    let texcoordBuffer = canvas_native_webgl_create_buffer(state);
    canvas_native_webgl_bind_buffer(gl_bindings::ARRAY_BUFFER, texcoordBuffer, state);
    canvas_native_webgl_buffer_data_f32(
        gl_bindings::ARRAY_BUFFER,
        &[0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0],
        gl_bindings::STATIC_DRAW,
        state,
    );

    // Create a texture.
    let texture = canvas_native_webgl_create_texture(state);
    canvas_native_webgl_bind_texture(gl_bindings::TEXTURE_2D, texture, state);

    canvas_native_webgl_tex_parameteri(
        gl_bindings::TEXTURE_2D,
        gl_bindings::TEXTURE_WRAP_S,
        gl_bindings::CLAMP_TO_EDGE as i32,
        state,
    );
    canvas_native_webgl_tex_parameteri(
        gl_bindings::TEXTURE_2D,
        gl_bindings::TEXTURE_WRAP_T,
        gl_bindings::CLAMP_TO_EDGE as i32,
        state,
    );

    canvas_native_webgl_tex_parameteri(
        gl_bindings::TEXTURE_2D,
        gl_bindings::TEXTURE_MIN_FILTER,
        gl_bindings::NEAREST as i32,
        state,
    );
    canvas_native_webgl_tex_parameteri(
        gl_bindings::TEXTURE_2D,
        gl_bindings::TEXTURE_MAG_FILTER,
        gl_bindings::NEAREST as i32,
        state,
    );

    gl_state.make_current();

    let mut ctx = ctx_2d.get_context_mut();

    ctx.set_font("30px Arial");

    let fill_style = PaintStyle::new_color_str("white").unwrap();
    ctx.set_fill_style(fill_style);
    ctx.rect(0., 0., width, height);
    ctx.fill(None);

    let fill_style = PaintStyle::new_color_str("red").unwrap();
    ctx.set_fill_style(fill_style);
    ctx.fill_text("Hello, WebGL!", 50., 50., None);

    //  let d = ctx.read_pixels();

    let (_, texture_id) = ctx.snapshot_with_texture_id();

    state.make_current();

    // gl_state.init_transfer_surface(texture_id);

    gl_state.draw_tex_image_2d(
        gl_bindings::TEXTURE_2D,
        0,
        width as u32,
        height as u32,
        gl_bindings::RGBA as i32,
        gl_bindings::RGBA as i32,
        false,
        texture_id,
    );

    /*
    let mut asset = ImageAsset::new();
    let bytes = include_bytes!("./svh.jpeg");
    asset.load_from_bytes(bytes);
    */

    // Upload the image into the texture.

    // canvas_native_webgl_tex_image2d_asset(
    //     gl_bindings::TEXTURE_2D as i32,
    //     0,
    //     gl_bindings::RGBA as i32,
    //     gl_bindings::RGBA as i32,
    //     gl_bindings::UNSIGNED_BYTE as i32,
    //     &asset,
    //     state,
    // );

    // canvas_native_webgl_tex_image2d(
    //     gl_bindings::TEXTURE_2D as i32,
    //     0,
    //     gl_bindings::RGBA as i32,
    //     width as i32, height as i32,
    //     0,
    //     gl_bindings::RGBA as i32,
    //     gl_bindings::UNSIGNED_BYTE as i32,
    //     d.as_slice(),
    //     state,
    // );

    // lookup uniforms
    let resolutionLocation =
        canvas_native_webgl_get_uniform_location(program, "u_resolution", state);

    // webglUtils.resizeCanvasToDisplaySize(gl.canvas);

    // Tell WebGL how to convert from clip space to pixels
    canvas_native_webgl_viewport(0, 0, drawingBufferWidth, drawingBufferHeight, state);

    canvas_native_webgl_clear_color(0., 0., 0., 0., state);
    // Clear the canvas
    canvas_native_webgl_clear(gl_bindings::COLOR_BUFFER_BIT, state);

    // Tell it to use our program (pair of shaders)
    canvas_native_webgl_use_program(program, state);

    // opengl needs this one newer versions
    // https://stackoverflow.com/a/24644675
    let vaoId = canvas_native_webgl2_create_vertex_array(state);
    canvas_native_webgl2_bind_vertex_array(vaoId, state);

    // Turn on the position attribute
    canvas_native_webgl_enable_vertex_attrib_array(position_location as u32, state);

    // Bind the position buffer.
    canvas_native_webgl_bind_buffer(gl_bindings::ARRAY_BUFFER, positionBuffer, state);

    // Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    let size = 2; // 2 components per iteration
    let ttype = gl_bindings::FLOAT; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0; // start at the beginning of the buffer

    canvas_native_webgl_vertex_attrib_pointer(
        position_location as u32,
        size,
        ttype,
        normalize,
        stride,
        offset,
        state,
    );

    // Turn on the teccord attribute
    canvas_native_webgl_enable_vertex_attrib_array(texcoord_location as u32, state);

    // Bind the position buffer.
    canvas_native_webgl_bind_buffer(gl_bindings::ARRAY_BUFFER, texcoordBuffer, state);

    // Tell the position attribute how to get data out of positionBuffer (ARRAY_BUFFER)
    let size = 2; // 2 components per iteration
    let ttype = gl_bindings::FLOAT; // the data is 32bit floats
    let normalize = false; // don't normalize the data
    let stride = 0; // 0 = move forward size * sizeof(type) each iteration to get the next position
    let offset = 0; // start at the beginning of the buffer
    canvas_native_webgl_vertex_attrib_pointer(
        texcoord_location as u32,
        size,
        ttype,
        normalize,
        stride,
        offset,
        state,
    );

    // set the resolution
    canvas_native_webgl_uniform2f(
        resolutionLocation,
        drawingBufferWidth as f32,
        drawingBufferHeight as f32,
        state,
    );

    // Draw the rectangle.
    let primitive_type = gl_bindings::TRIANGLES;
    let offset = 0;
    let count = 6;

    canvas_native_webgl_draw_arrays(primitive_type, offset, count, state);
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window_builder = WindowBuilder::new();
    let mut asset = ImageAsset::new();

    let window = window_builder
        .with_title("CanvasNative Demo")
        .build(&event_loop)
        .unwrap();

    let raw_window_handle = window.raw_window_handle();

    let mut attrs = ContextAttributes::default();
    attrs.set_antialias(false);

    let size = window.inner_size();

    let ocontext = GLContext::create_offscreen_context_with_event_loop(
        &mut attrs,
        size.width as i32,
        size.height as i32,
        &event_loop,
    );

    let owebgl = ocontext.unwrap();

    let context = GLContext::create_shared_window_context(
        &mut attrs,
        size.width as i32,
        size.height as i32,
        raw_window_handle,
        &owebgl,
    );

    let webgl = context.unwrap();

    let mut gl_state = WebGLState::new_with_context(webgl, WebGLVersion::V2);

    // let value = match canvas_webgl::webgl::canvas_native_webgl_get_parameter(36006, &mut gl_state) {
    //     WebGLResult::U32(value) => value as i32,
    //     WebGLResult::I32(value) => value,
    //     _ => 0,
    // };

    // let mut done = false;

    // let mut ctx_2d = ContextWrapper::new(Context::new_gl(
    //     size.width as f32,
    //     size.height as f32,
    //     1.,
    //     value,
    //     0,
    //     true,
    //     value,
    //     0.,
    //     canvas_2d::context::text_styles::text_direction::TextDirection::LTR,
    // ));

    // {
    //     let mut ctx = ctx_2d.get_context_mut();
    //     ctx.scale(3., 3.);
    //     ctx.translate(size.width as f32 * 0.1, size.height as f32 * 0.1);
    // }

    /* let h = (*ctx_2d.get_context().device()).height;
    let total = (*ctx_2d.get_context().device()).width / 2.;

    let mut colors = Vec::with_capacity(total as usize);
    let mut dots = vec![h; total as usize];
    let mut dots_vel = vec![10f32; total as usize];


    let particle_count = 1000;
    let mut particles = Vec::<Particle>::with_capacity(particle_count as usize);

    */

    /*

        let sun = reqwest::blocking::get("https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_sun.png")
            .map(|mut v| {
                let mut asset = ImageAsset::new();
                let mut buf = Vec::new();
                let _ = v.read_to_end(&mut buf);
                asset.load_from_bytes(buf.as_slice());
                asset
            })
            .ok();

        let moon = reqwest::blocking::get("https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_moon.png")
            .map(|mut v| {
                let mut asset = ImageAsset::new();
                let mut buf = Vec::new();
                let _ = v.read_to_end(&mut buf);
                asset.load_from_bytes(buf.as_slice());
                asset
            })
            .ok();

        let earth = reqwest::blocking::get("https://github.com/mdn/content/raw/main/files/en-us/web/api/canvas_api/tutorial/basic_animations/canvas_earth.png")
            .map(|mut v| {
                let mut asset = ImageAsset::new();
                let mut buf = Vec::new();
                let _ = v.read_to_end(&mut buf);
                asset.load_from_bytes(buf.as_slice());
                asset
            })
            .ok();

        let fill = PaintStyle::new_color_str("rgba(0, 0, 0, 0.4)").unwrap();
        let stroke = PaintStyle::new_color_str("rgba(0, 153, 255, 0.4)").unwrap();

    */

    //  let mut ro = RainbowOctopus::new();

    //  textures(&mut gl_state);

    //  triangle(&mut gl_state);

    let mut done = false;

    let mut gl_state_other = WebGLState::new_with_context(owebgl, WebGLVersion::V2);

    gl_state_other.make_current();

    let value =
        match canvas_webgl::webgl::canvas_native_webgl_get_parameter(36006, &mut gl_state_other) {
            WebGLResult::U32(value) => value as i32,
            WebGLResult::I32(value) => value,
            _ => 0,
        };

    let w = gl_state_other.drawing_buffer_width();
    let h = gl_state_other.drawing_buffer_height();

    let mut ctx_2d = ContextWrapper::new(Context::new_gl(
        w as f32,
        h as f32,
        1.,
        value,
        0,
        true,
        0,
        0.,
        canvas_2d::context::text_styles::text_direction::TextDirection::LTR,
    ));

    event_loop.run(move |event, target| {
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(resized) => {
                        canvas_webgl::webgl::canvas_native_webgl_viewport(
                            0,
                            0,
                            resized.width as i32,
                            resized.height as i32,
                            &gl_state,
                        );

                        // window.request_redraw();
                        //
                        // ctx_2d.get_context_mut().flush();
                        //
                        // gl_state.swap_buffers();
                    }
                    WindowEvent::CloseRequested => {
                        //control_flow.set_exit();
                    }
                    WindowEvent::RedrawRequested => {
                        if !done {
                            window.request_redraw();

                            canvas_native_webgl_clear_color(1., 1., 1., 1., &mut gl_state);

                            canvas_native_webgl_clear(16384, &mut gl_state);

                            // test(&mut gl_state, &mut gl_state_other, &mut ctx_2d);

                            let mut ctx = ctx_2d.get_context_mut();

                            ctx.fill_text(" ", 0., 0., None);

                            //  leaves(&mut gl_state, &mut gl_state_other, &mut ctx_2d);

                            done = true;

                            gl_state.swap_buffers();
                        }

                        //  rainbow_octopus(&mut ctx_2d, &mut ro);

                        // if let (Some(sun), Some(moon), Some(earth)) = (sun.as_ref(), moon.as_ref(), earth.as_ref()) {
                        //     solar(&ctx_2d, earth, moon, sun, fill.clone(), stroke.clone())
                        // }

                        // {
                        //     clock(&mut ctx_2d);
                        // }

                        //swarm(&mut ctx_2d, &mut particles, particle_count);

                        // colorRain(&mut ctx_2d, &mut colors, &mut dots, &mut dots_vel);

                        /*

                        if let Some(color) = PaintStyle::new_color_str("red") {
                            {
                                let mut ctx = ctx_2d.get_context_mut();
                                //  colorRain(&mut ctx_2d, &mut colors, &mut dots, &mut dots_vel);
                                let bg = PaintStyle::new_color_str("white").unwrap();
                                let black = PaintStyle::new_color_str("black").unwrap();
                                ctx.set_fill_style(bg);
                                let device = *ctx.device();
                                ctx.rect(0., 0., device.width, device.height);
                                ctx.fill(None);
                                ctx.set_fill_style(black);

                                // Create a red line in position 150
                                ctx.set_stroke_style(color);
                                ctx.move_to(150., 20.);
                                ctx.line_to(150., 170.);
                                ctx.stroke(None);

                                ctx.set_font("15px Arial");

                                // Show the different textAlign values
                                ctx.set_text_align(TextAlign::START);
                                ctx.fill_text("textAlign = start", 150., 60., None);
                                ctx.set_text_align(TextAlign::END);
                                ctx.fill_text("textAlign = end", 150., 80., None);
                                ctx.set_text_align(TextAlign::LEFT);
                                ctx.fill_text("textAlign = left", 150., 100., None);
                                ctx.set_text_align(TextAlign::CENTER);
                                ctx.fill_text("textAlign = center", 150., 120., None);
                                ctx.set_text_align(TextAlign::RIGHT);
                                ctx.fill_text("textAlign = right", 150., 140., None);

                                ctx.flush();
                            }

                            //  println!("{}", canvas_2d::to_data_url(&mut ctx_2d, "image/jpg", 100))
                        }

                        */
                        //
                        // ctx_2d.fill_rect_xywh(0., 0., 300., 300.);
                        //  ctx_2d.get_context_mut().flush();
                        //ctx_2d.flush();

                        //   canvas_webgl::webgl::canvas_native_webgl_clear_color(1.0, 1.0, 0.0, 1.0, &mut gl_state);
                        //
                        // let start = Instant::now();
                        // canvas_webgl::webgl::canvas_native_webgl_clear(canvas_webgl::webgl::COLOR_BUFFER_BIT, &mut gl_state);
                        // let end = Instant::now() - start;
                        //
                        // println!("clear {:?}", end.as_millis());
                        //  canvas_webgl::webgl::canvas_native_webgl_draw_arrays(canvas_webgl::webgl::TRIANGLES, 0, 3, &mut gl_state);

                        //  canvas_webgl::webgl::canvas_native_webgl_draw_arrays(canvas_webgl::webgl::POINTS, 0, 1, &mut gl_state);

                        //  gl_state.swap_buffers();
                    }
                    _ => {}
                }
            }
            Event::Resumed => {}
            _ => {}
        }
    });

    /*

    #[cfg(target_os = "macos")]
        let display = unsafe {
        Display::new(
            RawDisplayHandle::AppKit(
                AppKitDisplayHandle::empty()
            )
        )
    };

    if let Ok(display) = display {
        let tmpl = ConfigTemplateBuilder::new()
            .with_transparency(true)
            .with_stencil_size(8)
            .with_depth_size(16)
            .with_alpha_size(8)
            .build();

        let config = unsafe {
            display.find_configs(tmpl)
                .unwrap()
                .reduce(|acc, config| {
                    if config.supports_transparency().unwrap_or_default()
                        && config.alpha_size() == 8u8
                        && config.stencil_size() == 8u8
                        && config.depth_size() == 16u8 {
                        return config;
                    }
                    acc
                }).unwrap()
        };

        let attrs = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(raw_window_handle));

        let mut ctx = unsafe { display.create_context(&config, &attrs).unwrap() };

        let size = window.inner_size();

        let one = NonZeroU32::new(1).unwrap();
        let surface_attrs = SurfaceAttributesBuilder::<WindowSurface>::new()
            .build(raw_window_handle, NonZeroU32::new(size.width).unwrap_or(one), NonZeroU32::new(size.height).unwrap_or(one));

        let mut surface = unsafe { display.create_window_surface(&config, &surface_attrs).unwrap() };


        let current_context = ctx.make_current(&surface).unwrap();


        event_loop.run(|event, target, control_flow| {
            control_flow.set_wait();
            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::Resized(resized) => {}
                        WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {}
                Event::MainEventsCleared => {}
                Event::RedrawRequested(_) => {}
                Event::RedrawEventsCleared => {}
                Event::LoopDestroyed => {}
            }
        });
    }

    */
}

const VERTEX_SHADER_SRC: &str = "
  #version 330
  precision highp float;
  attribute vec2 position;
  void main() {
       gl_Position = vec4(position, 0.0, 1.0);
       gl_PointSize = 128.0;
   }
";

const FRAGMENT_SHADER_SRC: &str = "
#version 330
precision mediump float;
out vec4 fragColor;
void main() {
        vec2 fragmentPosition = 2.0*gl_PointCoord - 1.0;
        float distance = length(fragmentPosition);
        float distanceSqrd = distance * distance;
        fragColor = vec4(
            0.2/distanceSqrd,
            0.1/distanceSqrd,
            0.0, 1.0 );
    }
";

fn textures(state: &mut WebGLState) {
    let drawing_buffer_width = state.drawing_buffer_width();
    let drawing_buffer_height = state.get_drawing_buffer_height();

    canvas_webgl::webgl::canvas_native_webgl_viewport(
        0,
        0,
        drawing_buffer_width,
        drawing_buffer_height,
        state,
    );
    canvas_webgl::webgl::canvas_native_webgl_clear_color(1.0, 1.0, 1.0, 1.0, state);
    canvas_webgl::webgl::canvas_native_webgl_clear(canvas_webgl::webgl::COLOR_BUFFER_BIT, state);

    let mut buffer = 0;
    let mut program = 0;

    let initialize_attributes = |buffer: &mut u32, state: &mut WebGLState| {
        canvas_webgl::webgl::canvas_native_webgl_enable_vertex_attrib_array(0, state);
        {
            *buffer = canvas_webgl::webgl::canvas_native_webgl_create_buffer(state);
        }
        canvas_webgl::webgl::canvas_native_webgl_bind_buffer(
            canvas_webgl::webgl::ARRAY_BUFFER,
            *buffer,
            state,
        );
        canvas_webgl::webgl::canvas_native_webgl_buffer_data(
            canvas_webgl::webgl::ARRAY_BUFFER,
            &[0u8, 0u8],
            canvas_webgl::webgl::STATIC_DRAW,
            state,
        );
        canvas_webgl::webgl::canvas_native_webgl_vertex_attrib_pointer(
            0,
            2,
            canvas_webgl::webgl::FLOAT,
            false,
            0,
            0,
            state,
        );
    };

    let cleanup = |buffer: u32, program: u32, state: &mut WebGLState| {
        canvas_webgl::webgl::canvas_native_webgl_use_program(0, state);
        if buffer > 0 {
            canvas_webgl::webgl::canvas_native_webgl_delete_buffer(buffer, state);
        }

        if program > 0 {
            canvas_webgl::webgl::canvas_native_webgl_delete_program(program, state);
        }
    };

    let vertex_shader = canvas_webgl::webgl::canvas_native_webgl_create_shader(
        canvas_webgl::webgl::VERTEX_SHADER,
        state,
    );
    canvas_webgl::webgl::canvas_native_webgl_shader_source(vertex_shader, VERTEX_SHADER_SRC, state);
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(vertex_shader, state);

    let fragment_shader = canvas_webgl::webgl::canvas_native_webgl_create_shader(
        canvas_webgl::webgl::FRAGMENT_SHADER,
        state,
    );

    canvas_webgl::webgl::canvas_native_webgl_shader_source(
        fragment_shader,
        FRAGMENT_SHADER_SRC,
        state,
    );
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(fragment_shader, state);

    let vs_log = canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(vertex_shader, state);
    let fs_log =
        canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(fragment_shader, state);

    let vs_src = canvas_webgl::webgl::canvas_native_webgl_get_shader_source(vertex_shader, state);
    let fs_src = canvas_webgl::webgl::canvas_native_webgl_get_shader_source(fragment_shader, state);

    println!("{vs_src}");
    println!("{fs_src}");

    if !vs_log.is_empty() {
        println!("vs log {vs_log}");
    }

    if !fs_log.is_empty() {
        println!("fs log {fs_log}");
    }

    program = canvas_webgl::webgl::canvas_native_webgl_create_program(state);

    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, vertex_shader, state);
    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, fragment_shader, state);
    canvas_webgl::webgl::canvas_native_webgl_link_program(program, state);
    canvas_webgl::webgl::canvas_native_webgl_detach_shader(program, vertex_shader, state);
    canvas_webgl::webgl::canvas_native_webgl_detach_shader(program, fragment_shader, state);
    match canvas_webgl::webgl::canvas_native_webgl_get_program_parameter(
        program,
        canvas_webgl::webgl::LINK_STATUS,
        state,
    ) {
        WebGLResult::Boolean(result) => {
            if !result {
                let link_err_log =
                    canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(program, state);
                cleanup(buffer, program, state);
                println!("Shader program did not link successfully. Error log: {link_err_log}");
                return;
            }
        }
        _ => {}
    }
    initialize_attributes(&mut buffer, state);

    canvas_webgl::webgl::canvas_native_webgl_use_program(program, state);
    // canvas_webgl::webgl::canvas_native_webgl_draw_arrays(canvas_webgl::webgl::POINTS, 0, 1, state);
    //   cleanup(buffer, program, state);
}

fn triangle(state: &mut WebGLState) {
    let vertex: [f32; 9] = [-1., 1., 0., -1., -1., 0., 1., -1., 0.];

    let vs_source = "
    #version 400
	in vec3 aPos;
	void main(){
	gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0f);
	}
	";

    let fs_source = "
    #version 400
	precision mediump float;
	out vec4 fragColor;
	void main(){
	fragColor = vec4(1.0f,0.0f,0.0f,1.0f);
	}
    ";

    let program = canvas_webgl::webgl::canvas_native_webgl_create_program(state);
    let vs = canvas_webgl::webgl::canvas_native_webgl_create_shader(
        canvas_webgl::webgl::VERTEX_SHADER,
        state,
    );
    canvas_webgl::webgl::canvas_native_webgl_shader_source(vs, vs_source, state);

    let fs = canvas_webgl::webgl::canvas_native_webgl_create_shader(
        canvas_webgl::webgl::FRAGMENT_SHADER,
        state,
    );
    canvas_webgl::webgl::canvas_native_webgl_shader_source(fs, fs_source, state);

    canvas_webgl::webgl::canvas_native_webgl_compile_shader(vs, state);
    canvas_webgl::webgl::canvas_native_webgl_compile_shader(fs, state);

    let vs_log = canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(vs, state);
    let fs_log = canvas_webgl::webgl::canvas_native_webgl_get_shader_info_log(fs, state);

    if !vs_log.is_empty() {
        println!("vs log {vs_log}");
    }

    if !fs_log.is_empty() {
        println!("fs log {fs_log}");
    }

    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, vs, state);
    canvas_webgl::webgl::canvas_native_webgl_attach_shader(program, fs, state);

    canvas_webgl::webgl::canvas_native_webgl_link_program(program, state);

    let program_log = canvas_webgl::webgl::canvas_native_webgl_get_program_info_log(program, state);

    if !program_log.is_empty() {
        println!("program log {program_log}");
    }

    let error = canvas_webgl::webgl::canvas_native_webgl_get_error(state);

    let attr = canvas_webgl::webgl::canvas_native_webgl_get_attrib_location(program, "aPos", state);

    let vbo = canvas_webgl::webgl::canvas_native_webgl_create_buffer(state);

    canvas_webgl::webgl::canvas_native_webgl_bind_buffer(
        canvas_webgl::webgl::ARRAY_BUFFER,
        vbo,
        state,
    );

    let slice: &[u8] = unsafe {
        std::slice::from_raw_parts_mut(std::mem::transmute(vertex.as_ptr()), vertex.len() * 4)
    };
    canvas_webgl::webgl::canvas_native_webgl_buffer_data(
        canvas_webgl::webgl::ARRAY_BUFFER,
        slice,
        canvas_webgl::webgl::STATIC_DRAW,
        state,
    );

    // canvas_webgl::webgl::canvas_native_webgl_buffer_data_f32(
    //     canvas_webgl::webgl::ARRAY_BUFFER,
    //     &vertex,
    //     canvas_webgl::webgl::STATIC_DRAW,
    //      state,
    // );

    canvas_webgl::webgl::canvas_native_webgl_vertex_attrib_pointer(
        attr as _,
        3,
        canvas_webgl::webgl::FLOAT,
        false,
        3 * 4,
        0,
        state,
    );
    canvas_webgl::webgl::canvas_native_webgl_enable_vertex_attrib_array(0, state);
    canvas_webgl::webgl::canvas_native_webgl_use_program(program, state);

    //	canvas_webgl::webgl::canvas_native_webgl_DrawArrays(canvas_webgl::webgl::TRIANGLES, 0, 3)
    //canvas_webgl::webgl::canvas_native_webgl_draw_arrays(canvas_webgl::webgl::TRIANGLES, 0, 3, state);
}

#[derive(Debug)]
struct RainbowOctopus {
    r: f32,
    beta: f32,
    gamma: f32,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    hue: f32,
    t: f32,
}

impl RainbowOctopus {
    pub fn new() -> Self {
        Self {
            r: 0.0,
            beta: 0.0,
            gamma: 0.0,
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0,
            hue: 0.0,
            t: 0.0,
        }
    }
}

fn rainbow_octopus(ctx: &ContextWrapper, ro: &mut RainbowOctopus) {
    let device = *ctx.get_context_mut().device();
    let w = device.width;
    let h = device.height;

    let pi = std::f32::consts::PI;

    /* spiral vars */
    /* https://twitter.com/jackrugile/status/420459385505079296 */
    let n = 8f32; /* shades */
    let m = 4f32; /* shade repetitions */
    let p = 32f32; /* dots on each branch */

    let mut t_step: f32 = 1.0 / 60.0;

    let mut ctx = ctx.get_context_mut();
    ctx.clear_rect(0., 0., w, h);

    for i in 0..(n * m) as i32 {
        ro.beta = i as f32 * 2. * pi / (n * m);
        ro.x0 = 0.;

        /* Begin the path up here */
        ctx.begin_path();
        ro.hue = (i as f32 * 360. / n);
        ctx.translate(w / 2., h / 2.);
        ctx.rotate(ro.t / 3.);
        /* only need to set the fillstyle once up here now */

        ctx.set_fill_style(
            PaintStyle::new_color_str(&format!("hsl({},100%,65%)", ro.hue)).unwrap(),
        );
        for j in 0..p as i32 {
            let j = j as f32;
            let p = p as f32;
            let t = ro.t as f32;
            ro.gamma = j * 2. * pi / p;
            ro.r = f32::max(1., f32::powf(2. * (j * (p - j)), 0.43) - 10.);

            ro.x0 += 3.4 * ro.r;
            ro.y0 = ro.x0 * f32::sin(ro.gamma + 2. * ro.t + ro.x0 / 99.) / 5.;

            /* change of coordinates */
            ro.x1 = ro.x0 * f32::cos(ro.beta) - ro.y0 * f32::sin(ro.beta);
            ro.y1 = ro.x0 * f32::sin(ro.beta) + ro.y0 * f32::cos(ro.beta);
            /* move it to the position of the arc */
            /* (remove this for a cool effect) */
            ctx.move_to(ro.x1, ro.y1);
            /* setup the arc path here */
            ctx.arc(ro.x1, ro.y1, ro.r, 0., 2. * pi, false);
        }
        /* close the 1 path that now is a combination of all the arcs */
        ctx.close_path();
        /* fill the whole path only once now */
        ctx.fill(None);
        /*
         * reason for moving the fill out of the inner loop:
         * see https://twitter.com/loktar00/status/420369245378076672
         * (thanks!)
         */

        ctx.rotate(-ro.t as f32 / 3.);
        ctx.translate(-w / 2., -h / 2.);
    }

    ro.t += t_step as f32;
}

fn colorRain(
    ctx: &mut ContextWrapper,
    colors: &mut Vec<f32>,
    dots: &mut Vec<f32>,
    dots_vel: &mut Vec<f32>,
) {
    //initial
    let device = *ctx.get_context().device();
    let mut ctx = ctx.get_context_mut();
    let w = device.width;
    let h = device.height;

    //parameters
    let total = w / 2.;

    let accelleration: f32 = 0.05;
    //afterinitial calculations
    let size = w / total;

    let occupation = w / total;
    let repaint_color = PaintStyle::black();

    let portion = 360f32 / total;
    if colors.is_empty() {
        for i in 0..total as i32 {
            let i = i as usize;
            colors.push(portion * i as f32);
        }
    }

    let mut animate = || {
        ctx.set_fill_style(repaint_color);
        ctx.fill_rect_xywh(0., 0., w as f32, h as f32);

        for i in 0..total as i32 {
            let i = i as usize;
            let current_y = dots[i] - 1.;
            let vel = dots_vel[i];

            dots[i] += vel + accelleration;

            dots_vel[i] += accelleration;

            ctx.set_fill_style(
                PaintStyle::new_color_str(&format!("hsl({}, 80%, 50%)", colors[i])).unwrap(),
            );
            ctx.fill_rect_xywh(
                (occupation * i as f32),
                current_y as f32,
                size as f32,
                dots_vel[i] + 1 as f32,
            );

            let mut rng = rand::thread_rng();
            let random: f32 = rng.gen_range(0f32..1f32);
            if dots[i] > h && random < 0.01 {
                dots[i] = 0.;
                dots_vel[i] = 0.;
            }
        }
    };

    animate()
}

struct Particle {
    context: ContextWrapper,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
}

impl std::fmt::Display for Particle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {})",
            self.x, self.y, self.vx, self.vy, self.radius
        )
    }
}

fn math_random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0f32..1f32)
}

impl Particle {
    pub fn new(context: &ContextWrapper, width: f32, height: f32) -> Self {
        Self {
            context: context.clone(),
            x: math_random() * width,
            y: math_random() * height,
            vx: -1. + math_random() * 2.,
            vy: -1. + math_random() * 2.,
            radius: 4.,
        }
    }
    pub fn draw(&self) {
        let mut context = self.context.get_context_mut();
        context.set_fill_style(PaintStyle::white());
        context.begin_path();

        context.arc(
            self.x,
            self.y,
            self.radius,
            0.,
            std::f32::consts::PI * 2.,
            false,
        );

        // Fill the color to the arc that we just created
        context.fill(None);
    }
}

fn distance(ctx: &ContextWrapper, chunk: &mut [Particle]) {
    let p1 = &chunk[0];
    let p2 = &chunk[1];
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;

    let dist = f32::sqrt(dx * dx + dy * dy);
    let mut ctx = ctx.get_context_mut();

    if dist <= MIN_DIST {
        ctx.begin_path();
        let alpha = (1.2 - dist / MIN_DIST);
        ctx.set_stroke_style(PaintStyle::new_color_rgba(
            255,
            255,
            255,
            (alpha * 255.) as u8,
        ));
        ctx.move_to(p1.x, p1.y);
        ctx.line_to(p2.x, p2.y);
        ctx.stroke(None);
        ctx.close_path();

        let ax = dx / 2000.;
        let ay = dy / 2000.;
        {
            let mut p1 = &mut chunk[0];
            p1.vx -= ax;
            p1.vy -= ay;
        }

        {
            let mut p2 = &mut chunk[1];
            p2.vx += ax;
            p2.vy += ay;
        }
    }
}

fn update(ctx: &ContextWrapper, width: f32, height: f32, particles: &mut [Particle]) {
    for chunk in particles.chunks_mut(2).into_iter() {
        let mut p = chunk.get_mut(0).unwrap();
        p.x += p.vx;
        p.y += p.vy;

        if p.x + p.radius > width {
            p.x = p.radius;
        } else if p.x - p.radius < 0. {
            p.x = width - p.radius;
        }

        if p.y + p.radius > height {
            p.y = p.radius;
        } else if p.y - p.radius < 0. {
            p.y = height - p.radius;
        }

        distance(ctx, chunk);
    }
}

fn draw(ctx: &ContextWrapper, width: f32, height: f32, particles: &mut [Particle]) {
    paint_canvas(ctx, width, height);

    for particle in particles.into_iter() {
        particle.draw();
    }
    update(ctx, width, height, particles);
}

fn paint_canvas(ctx: &ContextWrapper, width: f32, height: f32) {
    let mut ctx = ctx.get_context_mut();

    ctx.set_fill_style(PaintStyle::black());

    ctx.fill_rect_xywh(0., 0., width, height);
}

const MIN_DIST: f32 = 50f32;

fn swarm(ctx: &mut ContextWrapper, particles: &mut Vec<Particle>, particle_count: i32) {
    let mut init = || {
        let mut W = 0.;
        let mut H = 0.;

        {
            let device = *ctx.get_context().device();

            W = device.width;
            H = device.height;
        }

        let dist = 0.;

        if particles.len() != particle_count as usize {
            for _ in 0..particle_count {
                particles.push(Particle::new(ctx, W, H))
            }
        }

        //  let mut p = Particle::new(ctx, W, H);

        let mut animloop = || {
            draw(ctx, W, H, particles.as_mut_slice());
        };

        animloop();
    };

    init();
}

fn clock(ctx: &mut ContextWrapper) {
    let pi = std::f32::consts::PI;
    let now = chrono::offset::Local::now();
    let mut ctx = ctx.get_context_mut();
    ctx.save();
    ctx.clear_rect(0., 0., 150., 150.);
    ctx.translate(75., 75.);
    ctx.scale(0.4, 0.4);
    ctx.rotate(-pi / 2.);
    ctx.set_stroke_style(PaintStyle::black());
    ctx.set_fill_style(PaintStyle::white());
    ctx.set_line_width(8.);
    ctx.set_line_cap(LineCap::CapRound);

    // Hour marks
    ctx.save();
    for i in 0..12 {
        ctx.begin_path();
        ctx.rotate(pi / 6.);
        ctx.move_to(100., 0.);
        ctx.line_to(120., 0.);
        ctx.stroke(None);
    }

    ctx.restore();

    // Minute marks
    ctx.save();
    ctx.set_line_width(5.);
    for i in 0..60 {
        if i % 5 != 0 {
            ctx.begin_path();
            ctx.move_to(117., 0.);
            ctx.line_to(120., 0.);
            ctx.stroke(None);
        }
        ctx.rotate(pi / 30.);
    }
    ctx.restore();

    let sec = now.second();
    let min = now.minute();
    let mut hr = now.hour();
    if hr >= 12 {
        hr = hr - 12;
    };

    ctx.set_fill_style(PaintStyle::black());

    // write Hours
    ctx.save();
    ctx.rotate(hr as f32 * (pi / 6.) + (pi / 360.) * min as f32 + (pi / 21600.) * sec as f32);
    ctx.set_line_width(14.);
    ctx.begin_path();
    ctx.move_to(-20., 0.);
    ctx.line_to(80., 0.);
    ctx.stroke(None);
    ctx.restore();

    // write Minutes
    ctx.save();
    ctx.rotate((pi / 30.) * min as f32 + (pi / 1800.) * sec as f32);
    ctx.set_line_width(10.);
    ctx.begin_path();
    ctx.move_to(-28., 0.);
    ctx.line_to(112., 0.);
    ctx.stroke(None);
    ctx.restore();

    // Write seconds
    ctx.save();
    ctx.rotate((sec as f32 * pi) / 30.);

    ctx.set_stroke_style(PaintStyle::new_color_str("#D40000").unwrap());
    ctx.set_fill_style(PaintStyle::new_color_str("#D40000").unwrap());

    ctx.set_line_width(6.);
    ctx.begin_path();
    ctx.move_to(-30., 0.);
    ctx.line_to(83., 0.);
    ctx.stroke(None);
    ctx.begin_path();
    ctx.arc(0., 0., 10., 0., pi * 2., true);
    ctx.fill(None);
    ctx.begin_path();
    ctx.arc(95., 0., 10., 0., pi * 2., true);
    ctx.stroke(None);
    ctx.set_fill_style(PaintStyle::transparent());
    ctx.arc(0., 0., 3., 0., pi * 2., true);
    ctx.fill(None);
    ctx.restore();

    ctx.begin_path();
    ctx.set_line_width(14.);
    ctx.set_stroke_style(PaintStyle::new_color_str("#325FA2").unwrap());
    ctx.arc(0., 0., 142., 0., pi * 2., true);
    ctx.stroke(None);

    ctx.restore();
}

fn solar(
    ctx: &ContextWrapper,
    earth: &ImageAsset,
    moon: &ImageAsset,
    sun: &ImageAsset,
    fill: PaintStyle,
    stroke: PaintStyle,
) {
    let mut ctx = ctx.get_context_mut();

    let device = *ctx.device();
    ctx.set_global_composite_operation(CompositeOperationType::DestinationOver);
    ctx.clear_rect(0., 0., device.width, device.height); // clear canvas
    ctx.set_fill_style(fill);
    ctx.set_stroke_style(stroke);
    ctx.save();
    ctx.translate(150., 150.);

    let pi = std::f32::consts::PI;

    // Earth
    let time = chrono::offset::Local::now();
    let seconds = time.second();
    let milliseconds = seconds * 1000;
    ctx.rotate(((2. * pi) / 60.) * seconds as f32 + ((2. * pi) / 60000.) * milliseconds as f32);
    ctx.translate(105., 0.);
    ctx.fill_rect_xywh(0., -12., 40., 24.); // Shadow
    ctx.draw_image_asset_dx_dy(earth, -12., -12.);

    // Moon
    ctx.save();
    ctx.rotate(((2. * pi) / 6.) * seconds as f32 + ((2. * pi) / 6000.) * milliseconds as f32);
    ctx.translate(0., 28.5);
    ctx.draw_image_asset_dx_dy(moon, -3.5, -3.5);
    ctx.restore();
    ctx.restore();
    ctx.begin_path();
    ctx.arc(150., 150., 105., 0., pi * 2., false); // Earth orbit
    ctx.stroke(None);

    ctx.draw_image_asset_dx_dy_dw_dh(sun, 0., 0., 300., 300.);
}

fn create_program_from_scripts(
    state: &mut WebGLState,
    shader_sources: [(&str, u32); 2],
) -> Option<u32> {
    // setup GLSL programs

    let mut shaders = vec![];

    for (source, shader_type) in shader_sources {
        unsafe {
            let shader = canvas_native_webgl_create_shader(shader_type, state);

            canvas_native_webgl_shader_source(shader, source, state);
            canvas_native_webgl_compile_shader(shader, state);

            let compiled = canvas_native_webgl_get_shader_parameter(
                shader,
                gl_bindings::COMPILE_STATUS,
                state,
            );

            match compiled {
                WebGLResult::Boolean(compiled) => {
                    if !compiled {
                        // Something went wrong during compilation; get the error
                        let last_error = get_shader_info_log(shader);
                        println!("*** Error compiling shader '{}': {:?}", shader, last_error);
                        canvas_native_webgl_delete_shader(shader, state);
                        return None;
                    }

                    shaders.push(shader);
                }
                _ => {}
            }
        }
    }

    let program = canvas_native_webgl_create_program(state);

    for shader in shaders {
        canvas_native_webgl_attach_shader(program, shader, state);
    }

    canvas_native_webgl_link_program(program, state);

    // Check the link status
    let linked =
        canvas_native_webgl_get_program_parameter(program, gl_bindings::LINK_STATUS, state);

    match linked {
        WebGLResult::Boolean(linked) => {
            if !linked {
                let last_error = canvas_native_webgl_get_program_info_log(program, state);
                println!("Error in program linking: {last_error}");
                canvas_native_webgl_delete_program(program, state);
                return None;
            }
        }
        _ => {}
    }

    Some(program)
}
