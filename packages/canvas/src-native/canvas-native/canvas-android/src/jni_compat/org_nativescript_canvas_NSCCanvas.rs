use std::ffi::c_void;

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jfloat, jint, jlong, jobject, JNI_FALSE, JNI_TRUE};
use jni::JNIEnv;
use ndk::native_window::NativeWindow;
use parking_lot::RwLock;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::{AlphaType, ColorType, ISize, ImageInfo, Rect, Surface};

use canvas_core::context_attributes::ContextAttributes;
use canvas_core::gl::GLContext;

#[allow(dead_code)]
pub(crate) struct AndroidGLContext {
    pub(crate) contextAttributes: ContextAttributes,
    pub(crate) gl_context: GLContext,
    android_window: Option<NativeWindow>,
}

#[no_mangle]
pub extern "system" fn nativeInitGL(
    mut env: JNIEnv,
    _: JClass,
    surface: jobject,
    alpha: jboolean,
    antialias: jboolean,
    depth: jboolean,
    fail_if_major_performance_caveat: jboolean,
    power_preference: JString,
    premultiplied_alpha: jboolean,
    preserve_drawing_buffer: jboolean,
    stencil: jboolean,
    desynchronized: jboolean,
    xr_compatible: jboolean,
    version: jint,
    is_canvas: jboolean,
) -> jlong {
    unsafe {
        let interface = env.get_native_interface();
        if let Some(window) = NativeWindow::from_surface(interface, surface) {
            if version == 2 && !GLContext::has_gl2support() {
                return 0;
            }
            let power_preference = match env.get_string(&power_preference) {
                Ok(value) => value.to_string_lossy().to_string(),
                Err(_) => "default".to_string(),
            };

            let mut attrs = ContextAttributes::new(
                alpha == JNI_TRUE,
                antialias == JNI_TRUE,
                depth == JNI_TRUE,
                fail_if_major_performance_caveat == JNI_TRUE,
                power_preference.as_ref(),
                premultiplied_alpha == JNI_TRUE,
                preserve_drawing_buffer == JNI_TRUE,
                stencil == JNI_TRUE,
                desynchronized == JNI_TRUE,
                xr_compatible == JNI_TRUE,
                is_canvas == JNI_TRUE,
            );
            if let Some(gl_context) = GLContext::create_window_context(
                &mut attrs,
                window.width(),
                window.height(),
                window.raw_window_handle(),
            ) {
                let context = AndroidGLContext {
                    android_window: Some(window),
                    gl_context,
                    contextAttributes: attrs,
                };
                drop(env);
                return Box::into_raw(Box::new(context)) as jlong;
            }
        }
    }
    0
}

#[no_mangle]
pub extern "system" fn nativeInitGLNoSurface(
    mut env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    alpha: jboolean,
    antialias: jboolean,
    depth: jboolean,
    fail_if_major_performance_caveat: jboolean,
    power_preference: JString,
    premultiplied_alpha: jboolean,
    preserve_drawing_buffer: jboolean,
    stencil: jboolean,
    desynchronized: jboolean,
    xr_compatible: jboolean,
    version: jint,
    is_canvas: jboolean,
) -> jlong {
    if version == 2 && !GLContext::has_gl2support() {
        return 0;
    }
    let power_preference = match env.get_string(&power_preference) {
        Ok(value) => value.to_string_lossy().to_string(),
        Err(_) => "default".to_string(),
    };

    let mut attrs = ContextAttributes::new(
        alpha == JNI_TRUE,
        antialias == JNI_TRUE,
        depth == JNI_TRUE,
        fail_if_major_performance_caveat == JNI_TRUE,
        power_preference.as_ref(),
        premultiplied_alpha == JNI_TRUE,
        preserve_drawing_buffer == JNI_TRUE,
        stencil == JNI_TRUE,
        desynchronized == JNI_TRUE,
        xr_compatible == JNI_TRUE,
        is_canvas == JNI_TRUE,
    );

    drop(env);
    if let Some(gl_context) = GLContext::create_pbuffer(&mut attrs, width, height) {
        return Box::into_raw(Box::new(AndroidGLContext {
            android_window: None,
            gl_context,
            contextAttributes: attrs,
        })) as jlong;
    }
    0
}

fn native_create_2d_context(
    context: jlong,
    width: jint,
    height: jint,
    alpha: jboolean,
    density: jfloat,
    samples: jint,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    if context == 0 {
        return 0;
    }

    let context = context as *mut AndroidGLContext;
    let context = unsafe { &mut *context };

    context.gl_context.make_current();
    let mut frame_buffers = [0];
    unsafe {
        gl_bindings::GetIntegerv(gl_bindings::FRAMEBUFFER_BINDING, frame_buffers.as_mut_ptr())
    };

    let ctx_2d = canvas_c::CanvasRenderingContext2D::new(
        canvas_2d::context::ContextWrapper::new(canvas_2d::context::Context::new_gl(
            width as f32,
            height as f32,
            density,
            frame_buffers[0],
            samples,
            alpha == JNI_TRUE,
            font_color,
            ppi,
            canvas_2d::context::text_styles::text_direction::TextDirection::from(direction as u32),
        )),
        context.gl_context.clone(),
        alpha == JNI_TRUE,
    );

    //ctx_2d.get_context_mut().flush();
    //context.gl_context.swap_buffers();

    Box::into_raw(Box::new(ctx_2d)) as jlong
}

#[no_mangle]
pub extern "system" fn nativeCreate2DContext(
    context: jlong,
    width: jint,
    height: jint,
    alpha: jboolean,
    density: jfloat,
    samples: jint,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    native_create_2d_context(
        context, width, height, alpha, density, samples, font_color, ppi, direction,
    )
}

#[no_mangle]
pub extern "system" fn nativeCreate2DContextNormal(
    _env: JNIEnv,
    _: JClass,
    context: jlong,
    width: jint,
    height: jint,
    alpha: jboolean,
    density: jfloat,
    samples: jint,
    font_color: jint,
    ppi: jfloat,
    direction: jint,
) -> jlong {
    native_create_2d_context(
        context, width, height, alpha, density, samples, font_color, ppi, direction,
    )
}

#[no_mangle]
pub extern "system" fn nativeUpdateGLSurface(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut AndroidGLContext;
    let context = unsafe { &mut *context };
    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            context.gl_context.set_window_surface(
                &mut context.contextAttributes,
                window.width(),
                window.height(),
                window.raw_window_handle(),
            );
            context.android_window = Some(window);
            drop(env);
        }
    }
}

#[no_mangle]
pub extern "system" fn nativeUpdate2DSurface(
    env: JNIEnv,
    _: JClass,
    surface: jobject,
    context: jlong,
) {
    if context == 0 {
        return;
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();

    unsafe {
        if let Some(window) = NativeWindow::from_surface(env.get_native_interface(), surface) {
            context.resize(window.width() as f32, window.height() as f32)
        }
        drop(env);
    }
}

fn native_update_2d_surface_no_surface(width: jint, height: jint, context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();

    context.resize(width as f32, height as f32)
}

#[no_mangle]
pub extern "system" fn nativeUpdate2DSurfaceNoSurface(width: jint, height: jint, context: jlong) {
    native_update_2d_surface_no_surface(width, height, context)
}

#[no_mangle]
pub extern "system" fn nativeUpdate2DSurfaceNoSurfaceNormal(
    _env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    context: jlong,
) {
    native_update_2d_surface_no_surface(width, height, context)
}

fn native_update_gl_no_surface(width: jint, height: jint, context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut AndroidGLContext;
    let context = unsafe { &mut *context };
    context
        .gl_context
        .resize_pbuffer(&mut context.contextAttributes, width, height);
}

#[no_mangle]
pub extern "system" fn nativeUpdateGLNoSurface(width: jint, height: jint, context: jlong) {
    native_update_gl_no_surface(width, height, context)
}

#[no_mangle]
pub extern "system" fn nativeUpdateGLNoSurfaceNormal(
    _env: JNIEnv,
    _: JClass,
    width: jint,
    height: jint,
    context: jlong,
) {
    native_update_gl_no_surface(width, height, context)
}

#[no_mangle]
pub extern "system" fn nativeReleaseGL(context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut AndroidGLContext;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "system" fn nativeReleaseGLNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }
    let context = context as *mut AndroidGLContext;
    let _ = unsafe { Box::from_raw(context) };
}

#[no_mangle]
pub extern "system" fn nativeGetGLPointer(gl_context: jlong) -> jlong {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut AndroidGLContext;
    let gl_context = unsafe { &*gl_context };
    gl_context.gl_context.as_raw_inner() as jlong
}

#[no_mangle]
pub extern "system" fn nativeGetGLPointerNormal(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
) -> jlong {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut AndroidGLContext;
    let gl_context = unsafe { &*gl_context };
    gl_context.gl_context.as_raw_inner() as jlong
}

#[no_mangle]
pub extern "system" fn nativeMakeGLCurrent(gl_context: jlong) -> jboolean {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut AndroidGLContext;
    let gl_context = unsafe { &*gl_context };
    if gl_context.gl_context.make_current() {
        return JNI_TRUE;
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeMakeGLCurrentNormal(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
) -> jboolean {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *mut AndroidGLContext;
    let gl_context = unsafe { &*gl_context };
    if gl_context.gl_context.make_current() {
        return JNI_TRUE;
    }
    JNI_FALSE
}

#[no_mangle]
pub extern "system" fn nativeGLPointerRefCount(gl_context: jlong) -> jlong {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    if gl_context.is_null() {
        return 0;
    }

    let ctx = GLContext::from_raw_inner(gl_context);
    let count = ctx.get_strong_count();
    let _ = ctx.as_raw_inner();

    count as i64
}

#[no_mangle]
pub extern "system" fn nativeGLPointerRefCountNormal(
    _env: JNIEnv,
    _: JClass,
    gl_context: jlong,
) -> jlong {
    if gl_context == 0 {
        return 0;
    }
    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    if gl_context.is_null() {
        return 0;
    }

    let ctx = GLContext::from_raw_inner(gl_context);
    let count = ctx.get_strong_count();
    let _ = ctx.as_raw_inner();

    count as i64
}

#[no_mangle]
pub extern "system" fn nativeReleaseGLPointer(gl_context: jlong) {
    if gl_context == 0 {
        return;
    }
    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    if gl_context.is_null() {
        return;
    }

    let ctx = GLContext::from_raw_inner(gl_context);

    if ctx.get_strong_count() <= 1 {
        ctx.increment_strong_count();
    }
}

#[no_mangle]
pub extern "system" fn nativeReleaseGLPointerNormal(_env: JNIEnv, _: JClass, gl_context: jlong) {
    if gl_context == 0 {
        return;
    }
    let gl_context = gl_context as *const RwLock<canvas_core::gl::GLContextInner>;
    if gl_context.is_null() {
        return;
    }

    let ctx = GLContext::from_raw_inner(gl_context);

    if ctx.get_strong_count() <= 1 {
        ctx.increment_strong_count();
    }
}

#[no_mangle]
pub extern "system" fn nativeContext2DTest(context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DTestNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();
        ctx.set_fill_style_with_color("red");
        ctx.fill_rect_xywh(0., 0., 300., 300.);
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DPathTest(context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();

        // Create path
        let mut region = canvas_2d::context::paths::path::Path::default();
        region.move_to(30f32, 90f32);
        region.line_to(110f32, 20f32);
        region.line_to(240f32, 130f32);
        region.line_to(60f32, 130f32);
        region.line_to(190f32, 20f32);
        region.line_to(270f32, 90f32);
        region.close_path();

        // Fill path
        ctx.set_fill_style_with_color("green");
        ctx.fill_rule(
            Some(&mut region),
            canvas_2d::context::drawing_paths::fill_rule::FillRule::EvenOdd,
        );
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeContext2DPathTestNormal(_env: JNIEnv, _: JClass, context: jlong) {
    if context == 0 {
        return;
    }

    let context = context as *mut canvas_c::CanvasRenderingContext2D;
    let context = unsafe { &mut *context };

    context.make_current();
    {
        let mut ctx = context.get_context_mut();

        // Create path
        let mut region = canvas_2d::context::paths::path::Path::default();
        region.move_to(30f32, 90f32);
        region.line_to(110f32, 20f32);
        region.line_to(240f32, 130f32);
        region.line_to(60f32, 130f32);
        region.line_to(190f32, 20f32);
        region.line_to(270f32, 90f32);
        region.close_path();

        // Fill path
        ctx.set_fill_style_with_color("green");
        ctx.fill_rule(
            Some(&mut region),
            canvas_2d::context::drawing_paths::fill_rule::FillRule::EvenOdd,
        );
    }
    context.render();
}

#[no_mangle]
pub extern "system" fn nativeWriteCurrentGLContextToBitmap(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    if context == 0 {
        return;
    }

    let context = context as *mut AndroidGLContext;
    let context = unsafe { &mut *context };

    unsafe {
        crate::utils::image::bitmap_handler(
            &env,
            bitmap,
            Box::new(move |cb| {
                if let Some((image_data, info)) = cb {
                    context.gl_context.make_current();
                    let mut buf = vec![0u8; (info.width() * info.height() * 4) as usize];
                    gl_bindings::Flush();
                    gl_bindings::ReadPixels(
                        0,
                        0,
                        info.width() as i32,
                        info.height() as i32,
                        gl_bindings::RGBA as std::os::raw::c_uint,
                        gl_bindings::UNSIGNED_BYTE as std::os::raw::c_uint,
                        buf.as_mut_ptr() as *mut c_void,
                    );
                    image_data.copy_from_slice(buf.as_slice());
                }
            }),
        )
    }

    drop(env);
}

#[no_mangle]
pub extern "system" fn nativeCustomWithBitmapFlush(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    bitmap: JObject,
) {
    unsafe {
        if context == 0 {
            return;
        }
        crate::utils::image::bitmap_handler(
            &env,
            bitmap,
            Box::new(move |cb| {
                if let Some((image_data, image_info)) = cb {
                    let mut ct = ColorType::RGBA8888;

                    if image_info.format() == ndk::bitmap::BitmapFormat::RGB_565 {
                        ct = ColorType::RGB565;
                    } else if image_info.format() == ndk::bitmap::BitmapFormat::RGBA_4444 {
                        ct = ColorType::ARGB4444;
                    }
                    let info = ImageInfo::new(
                        ISize::new(image_info.width() as i32, image_info.height() as i32),
                        ct,
                        AlphaType::Premul,
                        None,
                    );
                    let context = context as *mut canvas_c::CanvasRenderingContext2D;
                    let context = unsafe { &mut *context };
                    let mut context = context.get_context_mut();

                    let mut surface =
                        Surface::new_raster_direct(&info, image_data, None, None).unwrap();
                    let canvas = surface.canvas();
                    let mut paint = skia_safe::Paint::default();
                    paint.set_anti_alias(true);
                    paint.set_style(skia_safe::PaintStyle::Fill);
                    paint.set_blend_mode(skia_safe::BlendMode::Clear);
                    canvas.draw_rect(
                        Rect::from_xywh(
                            0f32,
                            0f32,
                            image_info.width() as f32,
                            image_info.height() as f32,
                        ),
                        &paint,
                    );
                    context.draw_on_surface(&mut surface);
                }
            }),
        );

        drop(env);
    }
}
