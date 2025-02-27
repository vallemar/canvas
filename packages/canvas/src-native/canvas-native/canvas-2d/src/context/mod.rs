use log::log;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::os::raw::c_float;
use std::sync::Arc;

use skia_safe::gpu::gl::TextureInfo;
use skia_safe::{images, Color, Data, Image, Point, Surface};

use crate::context::drawing_text::typography::Font;
use compositing::composite_operation_type::CompositeOperationType;
use fill_and_stroke_styles::paint::Paint;
use filter_quality::FilterQuality;
use image_smoothing::ImageSmoothingQuality;
use line_styles::line_cap::LineCap;
use line_styles::line_join::LineJoin;
use paths::path::Path;
use text_styles::{
    text_align::TextAlign, text_baseline::TextBaseLine, text_direction::TextDirection,
};

pub mod drawing_images;
pub mod drawing_text;
pub mod fill_and_stroke_styles;
pub mod paths;
pub mod pixel_manipulation;
pub mod text_styles;

pub mod compositing;
pub mod drawing_paths;
pub mod drawing_rectangles;
pub mod filters;
pub mod gradients_and_patterns;

pub mod image_smoothing;
pub mod line_styles;
pub mod shadows;
pub mod state;

pub mod filter_quality;
pub mod matrix;
pub mod non_standard;
pub mod surface;
pub mod surface_gl;
pub mod text_decoder;
pub mod text_encoder;
pub mod transformations;

const VERTEX_SHADER: &str = "
    precision highp float;
    attribute vec4 aPosition;
    uniform mat4 uTextureMatrix;
    varying vec2 TexCoord;
    void main(){
    vec2 clipSpace = (1.0 - 2.0 * aPosition.xy);
    TexCoord = aPosition.xy;
    gl_Position = vec4(clipSpace, 0.0, 1.0);
    }
    ";

const FRAGMENT_SHADER: &str = "
    precision highp float;
    varying vec2 TexCoord;
    uniform sampler2D uSampler;
    void main(){
    gl_FragColor = texture2D(uSampler, TexCoord);
    }
    ";

#[derive(Copy, Clone, Debug)]
pub struct Device {
    pub width: c_float,
    pub height: c_float,
    pub density: c_float,
    pub non_gpu: bool,
    pub samples: usize,
    pub alpha: bool,
    pub ppi: c_float,
}

impl Device {
    pub fn new_non_gpu(width: c_float, height: c_float, density: c_float, ppi: c_float) -> Self {
        Self {
            width,
            height,
            density,
            non_gpu: true,
            samples: 0,
            alpha: false,
            ppi,
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub(crate) direction: TextDirection,
    pub(crate) paint: Paint,
    pub(crate) font: String,
    pub(crate) font_style: Font,
    pub(crate) text_align: TextAlign,
    pub(crate) text_baseline: TextBaseLine,
    pub(crate) shadow_color: Color,
    pub(crate) shadow_offset: Point,
    pub(crate) shadow_blur: f32,
    pub(crate) image_smoothing_enabled: bool,
    pub(crate) image_smoothing_quality: ImageSmoothingQuality,
    pub(crate) line_width: c_float,
    pub(crate) line_cap: LineCap,
    pub(crate) line_join: LineJoin,
    pub(crate) miter_limit: f32,
    pub(crate) line_dash_list: Vec<f32>,
    pub(crate) line_dash_offset: f32,
    pub(crate) filter: String,
    pub(crate) global_alpha: f32,
    pub(crate) global_composite_operation: CompositeOperationType,
    pub(crate) use_device_scale: bool,
    pub(crate) did_use_device_scale: bool,
    pub(crate) word_spacing_value: String,
    pub(crate) word_spacing: f32,
    pub(crate) letter_spacing_value: String,
    pub(crate) letter_spacing: f32,
}

impl State {
    pub(crate) fn image_filter_quality(&self) -> FilterQuality {
        if self.image_smoothing_enabled {
            self.image_smoothing_quality.into()
        } else {
            FilterQuality::None
        }
    }
    pub fn from_device(device: Device, direction: TextDirection) -> Self {
        let font = "10px sans-serif".to_owned();
        let mut paint = Paint::default();
        paint
            .stroke_paint_mut()
            .set_stroke_width(1.)
            .set_stroke_miter(10.);
        Self {
            direction,
            paint,
            font,
            font_style: Font::default(),
            text_align: TextAlign::default(),
            text_baseline: TextBaseLine::default(),
            shadow_color: Color::TRANSPARENT,
            shadow_offset: (0.0, 0.0).into(),
            shadow_blur: 0.0,
            image_smoothing_enabled: false,
            image_smoothing_quality: ImageSmoothingQuality::default(),
            line_width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            miter_limit: 10.0,
            line_dash_list: Default::default(),
            line_dash_offset: 0.0,
            filter: "none".into(),
            global_alpha: 1.0,
            global_composite_operation: CompositeOperationType::default(),
            use_device_scale: true,
            did_use_device_scale: true,
            word_spacing_value: "0px".to_string(),
            word_spacing: 0.,
            letter_spacing_value: "0px".to_string(),
            letter_spacing: 0.,
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pub(crate) surface: Surface,
    pub(crate) path: Path,
    pub(crate) state: State,
    pub(crate) state_stack: Vec<State>,
    pub(crate) device: Device,
    pub(crate) font_color: Color,
}

pub struct ContextWrapper {
    inner: Arc<RwLock<Context>>,
}

unsafe impl Send for ContextWrapper {}

unsafe impl Sync for ContextWrapper {}

impl ContextWrapper {
    pub fn new(context: Context) -> ContextWrapper {
        Self {
            inner: Arc::new(RwLock::new(context)),
        }
    }

    pub fn get_context(&self) -> MappedRwLockReadGuard<Context> {
        RwLockReadGuard::map(self.inner.read(), |v| v)
    }

    pub fn get_context_mut(&self) -> MappedRwLockWriteGuard<Context> {
        RwLockWriteGuard::map(self.inner.write(), |v| v)
    }

    pub fn into_box(self) -> Box<ContextWrapper> {
        Box::new(self)
    }

    pub fn into_raw(self) -> *mut ContextWrapper {
        Box::into_raw(self.into_box())
    }

    pub fn get_inner(&self) -> &Arc<RwLock<Context>> {
        &self.inner
    }

    pub fn from_inner(inner: Arc<RwLock<Context>>) -> ContextWrapper {
        Self { inner }
    }

    #[cfg(feature = "gl")]
    pub fn resize_gl(&mut self, width: f32, height: f32) {
        let mut context = self.get_context_mut();
        let device = context.device;

        let ctx = &mut *context;

        let mut fb = [0];

        unsafe {
            gl_bindings::ClearColor(0., 0., 0., 0.);
            gl_bindings::Clear(gl_bindings::COLOR_BUFFER_BIT);
            gl_bindings::Viewport(0, 0, width as i32, height as i32);
            gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, fb.as_mut_ptr());
        }

        Context::resize_gl(
            ctx,
            width,
            height,
            device.density,
            fb[0],
            device.samples as i32,
            device.alpha,
            device.ppi,
        );
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        let mut context = self.get_context_mut();
        let device = context.device;

        let ctx = &mut *context;

        Context::resize(ctx, width, height, device.density, device.alpha, device.ppi);
    }
}

impl Clone for ContextWrapper {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner = Arc::clone(&source.inner)
    }
}

impl Context {
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn reset_state(&mut self) {
        let direction = self.state.direction;
        self.state = State::from_device(self.device, direction);
    }

    pub fn clear_canvas(&mut self) {
        self.surface.canvas().clear(Color::TRANSPARENT);
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
    }

    pub fn flush(&mut self) {
        if let Some(mut context) = self.surface.direct_context() {
            context.flush(None);
        }
    }

    pub fn flush_and_sync_cpu(&mut self) {
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
    }

    pub fn reset(&mut self) {
        if let Some(mut context) = self.surface.direct_context() {
            context.reset(None);
        }
    }

    pub fn snapshot_to_raster_data(&mut self) -> Vec<u8> {
        let ss = self.surface.image_snapshot();

        let info = ss.image_info();
        let row_bytes = info.min_row_bytes();
        let mut buf = vec![0_u8; info.compute_byte_size(row_bytes)];

        match ss.make_raster_image(
            &mut self.surface.direct_context(),
            skia_safe::image::CachingHint::Disallow,
        ) {
            Some(image) => {
                let mut info = skia_safe::ImageInfo::new(
                    info.dimensions(),
                    skia_safe::ColorType::RGBA8888,
                    skia_safe::AlphaType::Unpremul,
                    None,
                );

                let _read = image.read_pixels(
                    &mut info,
                    buf.as_mut_slice(),
                    row_bytes,
                    skia_safe::IPoint::new(0, 0),
                    skia_safe::image::CachingHint::Disallow,
                );
            }
            _ => {}
        }

        buf
    }

    pub fn snapshot_to_raster_image(&mut self) -> Option<Image> {
        let ss = self.surface.image_snapshot();
        ss.make_raster_image(
            &mut self.surface.direct_context(),
            skia_safe::image::CachingHint::Allow,
        )
    }

    pub fn to_raster_image(&mut self, image: Image) -> Option<Image> {
        image.make_raster_image(
            &mut self.surface.direct_context(),
            skia_safe::image::CachingHint::Allow,
        )
    }

    pub fn to_raster_pixels(
        &mut self,
        image: Image,
        is_alpha: bool,
        premultiply: bool,
    ) -> Option<Vec<u8>> {
        if let Some(image) = image.make_raster_image(
            &mut self.surface.direct_context(),
            skia_safe::image::CachingHint::Disallow,
        ) {
            let info = image.image_info();

            let mut info = skia_safe::ImageInfo::new(
                info.dimensions(),
                if is_alpha {
                    skia_safe::ColorType::RGBA8888
                } else {
                    skia_safe::ColorType::RGB565
                },
                if premultiply {
                    skia_safe::AlphaType::Premul
                } else {
                    skia_safe::AlphaType::Unpremul
                },
                None,
            );

            let mut buf = vec![0_u8; info.compute_byte_size(info.min_row_bytes())];

            image.read_pixels(
                &info,
                &mut buf,
                info.min_row_bytes(),
                skia_safe::IPoint::new(0, 0),
                skia_safe::image::CachingHint::Disallow,
            );
            return Some(buf);
        }

        None
    }

    pub fn snapshot_with_texture_id(&mut self) -> (Image, u32) {
        let ss = self.surface.image_snapshot();
        if let Some((texture, _)) =
            skia_safe::gpu::images::get_backend_texture_from_image(&ss, false)
        {
            return (
                ss,
                texture
                    .gl_texture_info()
                    .map(|info| info.id)
                    .unwrap_or_default(),
            );
        }
        (ss, 0)
    }

    pub fn read_pixels(&mut self) -> Vec<u8> {
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
        let info = self.surface.image_info();

        let mut info = skia_safe::ImageInfo::new(
            info.dimensions(),
            info.color_type(),
            info.alpha_type(),
            info.color_space(),
        );

        let mut buf = vec![0_u8; info.compute_byte_size(info.min_row_bytes())];

        self.surface.read_pixels(
            &info,
            &mut buf,
            info.min_row_bytes(),
            skia_safe::IPoint::new(0, 0),
        );
        buf
    }

    pub fn read_pixels_to_data(&mut self) -> Option<Vec<u8>> {
        let buf = self.read_pixels();
        image::load_from_memory(buf.as_slice())
            .map(|image| image.into_bytes())
            .ok()
    }

    pub fn read_pixels_into_bitmap(&mut self) -> skia_safe::Bitmap {
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
        let info = self.surface.image_info();
        let mut bm = skia_safe::Bitmap::new();
        bm.alloc_pixels_flags(&info);
        self.surface
            .read_pixels_to_bitmap(&bm, skia_safe::IPoint::new(0, 0));
        bm
    }

    pub fn read_pixels_into_image(&mut self) -> Option<Image> {
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
        let info = self.surface.image_info();
        let mut bm = skia_safe::Bitmap::new();
        bm.alloc_pixels_flags(&info);
        self.surface
            .read_pixels_to_bitmap(&bm, skia_safe::IPoint::new(0, 0));
        images::raster_from_bitmap(&bm)
    }

    pub fn image_snapshot(&mut self) -> Image {
        self.surface.image_snapshot()
    }

    pub fn image_snapshot_to_non_texture_image(&mut self) -> Option<Image> {
        let mut context = self.surface.direct_context();
        self.surface
            .image_snapshot()
            .make_non_texture_image(&mut context)
    }

    pub fn read_pixels_to_encoded_data(&mut self) -> Option<Data> {
        let image = self.surface.image_snapshot();
        let mut ctx = self.surface.direct_context();
        image.encode(&mut ctx, skia_safe::EncodedImageFormat::PNG, 100)
    }

    pub fn snapshot(&mut self) -> Image {
        if let Some(mut context) = self.surface.direct_context() {
            context.flush_submit_and_sync_cpu();
        }
        self.surface.image_snapshot()
    }

    pub fn draw_on_surface(&mut self, surface: &mut Surface) {
        let src_surface = &mut self.surface;
        src_surface.draw(
            surface.canvas(),
            Point::new(0., 0.),
            FilterQuality::High,
            None,
        )
    }
}
