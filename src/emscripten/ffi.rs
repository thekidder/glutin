#![allow(dead_code)]
#![allow(non_snake_case_functions)]
#![allow(non_camel_case_types)]

use libc;

pub type EM_BOOL = libc::c_int;
pub type EM_UTF8 = libc::c_char;
pub type EMSCRIPTEN_WEBGL_CONTEXT_HANDLE = libc::c_int;
pub type EMSCRIPTEN_RESULT = libc::c_int;

pub type em_webgl_context_callback = extern fn(libc::c_int, *const libc::c_void, *mut libc::c_void)
    -> EM_BOOL;

#[repr(C)]
pub struct EmscriptenWebGLContextAttributes {
    alpha: EM_BOOL,
    depth: EM_BOOL,
    stencil: EM_BOOL,
    antialias: EM_BOOL,
    premultipliedAlpha: EM_BOOL,
    preserveDrawingBuffer: EM_BOOL,
    preferLowPowerToHighPerformance: EM_BOOL,
    failIfMajorPerformanceCaveat: EM_BOOL,
    majorVersion: libc::c_int,
    minorVersion: libc::c_int,
    enableExtensionsByDefault: EM_BOOL,
}

// values for EMSCRIPTEN_RESULT
pub static EMSCRIPTEN_RESULT_SUCCESS: libc::c_int = 0;
pub static EMSCRIPTEN_RESULT_DEFERRED: libc::c_int = 1;
pub static EMSCRIPTEN_RESULT_NOT_SUPPORTED: libc::c_int = -1;
pub static EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED: libc::c_int = -2;
pub static EMSCRIPTEN_RESULT_INVALID_TARGET: libc::c_int = -3;
pub static EMSCRIPTEN_RESULT_UNKNOWN_TARGET: libc::c_int = -4;
pub static EMSCRIPTEN_RESULT_INVALID_PARAM: libc::c_int = -5;
pub static EMSCRIPTEN_RESULT_FAILED: libc::c_int = -6;
pub static EMSCRIPTEN_RESULT_NO_DATA: libc::c_int = -7;

extern {
    pub fn emscripten_webgl_init_context_attributes(attributes: *mut EmscriptenWebGLContextAttributes);
    pub fn emscripten_webgl_create_context(target: *const libc::c_char,
        attributes: *const EmscriptenWebGLContextAttributes);

    pub fn emscripten_webgl_make_context_current(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE)
    -> EMSCRIPTEN_RESULT;

    pub fn emscripten_webgl_get_current_context() -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE;

    pub fn emscripten_webgl_destroy_context(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE)
        -> EMSCRIPTEN_RESULT;

    pub fn emscripten_webgl_enable_extension(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
        extension: *const libc::c_char) -> EM_BOOL;

    pub fn emscripten_set_webglcontextlost_callback(target: *const libc::c_char,
        userData: *mut libc::c_void, useCapture: EM_BOOL, callback: em_webgl_context_callback)
        -> EMSCRIPTEN_RESULT;
    pub fn emscripten_set_webglcontextrestored_callback(target: *const libc::c_char,
        userData: *mut libc::c_void, useCapture: EM_BOOL, callback: em_webgl_context_callback)
        -> EMSCRIPTEN_RESULT;

    pub fn emscripten_is_webgl_context_lost(target: *const libc::c_char) -> EM_BOOL;
}
