use log::{log, Level};
use skia_safe::gpu::gl::Interface;
use skia_safe::{gpu, surfaces, Color, ColorType, PixelGeometry, Surface};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, Device, State};

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

#[cfg(feature = "gl")]
impl Context {
    pub fn new_gl(
        width: f32,
        height: f32,
        density: f32,
        buffer_id: i32,
        samples: i32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
    ) -> Self {
        let device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples: samples as usize,
            alpha,
            ppi,
        };
        let interface = Interface::new_native();
        let mut ctx = gpu::DirectContext::new_gl(interface, None).unwrap();

        ctx.reset(None);

        let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);
        if alpha {
            frame_buffer.format = GR_GL_RGBA8;
        } else {
            frame_buffer.format = GR_GL_RGB565;
        }

        let target = skia_safe::gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            Some(samples as usize),
            0,
            frame_buffer,
        );
        let surface_props = skia_safe::SurfaceProps::new(
            skia_safe::SurfacePropsFlags::default(),
            PixelGeometry::Unknown,
        );
        let mut color_type = ColorType::RGBA8888;
        if !alpha {
            color_type = ColorType::RGB565;
        }
        let surface_holder = gpu::surfaces::wrap_backend_render_target(
            &mut ctx,
            &target,
            gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            Some(&surface_props),
        );

        let mut surface = surface_holder.unwrap();

        let mut state = State::from_device(device, direction);

        if density > 1. {
            surface.canvas().scale((density, density));
        } else {
            state.use_device_scale = false;
            state.did_use_device_scale = false;
        };

        Context {
            surface,
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            device,
        }
    }

    pub fn resize_gl(
        context: &mut Context,
        width: f32,
        height: f32,
        density: f32,
        buffer_id: i32,
        samples: i32,
        alpha: bool,
        ppi: f32,
    ) {
        let interface = Interface::new_native();
        let ctx = gpu::DirectContext::new_gl(interface, None);
        if ctx.is_none() {
            return;
        }
        let mut ctx = ctx.unwrap();
        ctx.reset(None);
        let device = Device {
            width,
            height,
            density,
            non_gpu: false,
            samples: samples as usize,
            alpha,
            ppi,
        };
        let mut frame_buffer = gpu::gl::FramebufferInfo::from_fboid(buffer_id as u32);

        if alpha {
            frame_buffer.format = GR_GL_RGBA8;
        } else {
            frame_buffer.format = GR_GL_RGB565;
        }

        let target = gpu::BackendRenderTarget::new_gl(
            (width as i32, height as i32),
            Some(samples as usize),
            0,
            frame_buffer,
        );
        let surface_props = skia_safe::SurfaceProps::new(
            skia_safe::SurfacePropsFlags::default(),
            PixelGeometry::Unknown,
        );
        let mut color_type = ColorType::RGBA8888;

        if !alpha {
            color_type = ColorType::RGB565;
        }

        if let Some(mut surface) = gpu::surfaces::wrap_backend_render_target(
            &mut ctx,
            &target,
            gpu::SurfaceOrigin::BottomLeft,
            color_type,
            None,
            Some(&surface_props),
        ) {
            if density > 1. {
                surface.canvas().scale((density, density));
            } else {
                context.state.use_device_scale = false;
                context.state.did_use_device_scale = false;
            };

            context.surface = surface;
            context.device = device;
            context.path = Path::default();
            context.reset_state();
        }
    }
}
