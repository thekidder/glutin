use libc;
use {Event, WindowBuilder};

mod ffi;

pub struct Window {
    context: ffi::EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
}

pub struct MonitorID;

pub fn get_available_monitors() -> Vec<MonitorID> {
    vec![MonitorID]
}

pub fn get_primary_monitor() -> MonitorID {
    MonitorID
}

impl MonitorID {
    pub fn get_name(&self) -> Option<String> {
        Some("Canvas".to_string())
    }

    pub fn get_dimensions(&self) -> (uint, uint) {
        unimplemented!()
    }
}

impl Window {
    pub fn new(builder: WindowBuilder) -> Result<Window, String> {
        // getting the default values of attributes
        let mut attributes = unsafe {
            use std::mem;
            let mut attributes: ffi::EmscriptenWebGLContextAttributes = mem::uninitialized();
            ffi::emscripten_webgl_init_context_attributes(&mut attributes);
            attributes
        };

        // setting the attributes
        match builder.gl_version {
            Some((major, minor)) => {
                attributes.majorVersion = major as libc::c_int;
                attributes.minorVersion = minor as libc::c_int;
            },
            None => ()
        };

        // creating the context
        let context = unsafe {
            use std::{mem, ptr};
            let context = ffi::emscripten_webgl_create_context(ptr::null(), &attributes);
            if context <= 0 {
                return Err(format!("Error while calling emscripten_webgl_create_context: {}",
                    error_to_str(mem::transmute(context))));
            }
            context
        };

        // TODO: emscripten_set_webglcontextrestored_callback

        Ok(Window {
            context: context
        })
    }

    pub fn is_closed(&self) -> bool {
        use std::ptr;
        unsafe { ffi::emscripten_is_webgl_context_lost(ptr::null()) != 0 }
    }

    pub fn set_title(&self, _title: &str) {
    }

    pub fn get_position(&self) -> Option<(int, int)> {
        Some((0, 0))
    }

    pub fn set_position(&self, _: int, _: int) {
    }

    pub fn get_inner_size(&self) -> Option<(uint, uint)> {
        unsafe {
            use std::{mem, ptr};
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();

            if ffi::emscripten_get_element_css_size(ptr::null(), &mut width, &mut height)
                != ffi::EMSCRIPTEN_RESULT_SUCCESS
            {
                None
            } else {
                Some((width as uint, height as uint))
            }
        }
    }

    pub fn get_outer_size(&self) -> Option<(uint, uint)> {
        self.get_inner_size()
    }

    pub fn set_inner_size(&self, width: uint, height: uint) {
        unsafe {
            use std::ptr;
            ffi::emscripten_set_element_css_size(ptr::null(), width as libc::c_double, height
                as libc::c_double);
        }
    }

    pub fn poll_events(&self) -> Vec<Event> {
        unimplemented!()
    }

    pub fn wait_events(&self) -> Vec<Event> {
        unimplemented!()
    }

    pub unsafe fn make_current(&self) {
        // TOOD: check if == EMSCRIPTEN_RESULT
        ffi::emscripten_webgl_make_context_current(self.context);
    }

    pub fn get_proc_address(&self, addr: &str) -> *const () {
        addr.with_c_str(|s| {
            unsafe { ffi::emscripten_GetProcAddress(s) as *const () }
        })
    }

    pub fn swap_buffers(&self) {
        // there is no need to swap buffers in webgl
        // the browser avoids drawing our buffer as long as we continue to execute code
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::emscripten_exit_fullscreen();
            ffi::emscripten_webgl_destroy_context(self.context);
        }
    }
}

fn error_to_str(code: ffi::EMSCRIPTEN_RESULT) -> &'static str {
    match code {
        ffi::EMSCRIPTEN_RESULT_SUCCESS | ffi::EMSCRIPTEN_RESULT_DEFERRED
            => "Internal error in the library (success detected as failure)",

        ffi::EMSCRIPTEN_RESULT_NOT_SUPPORTED => "Not supported",
        ffi::EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED => "Failed not deferred",
        ffi::EMSCRIPTEN_RESULT_INVALID_TARGET => "Invalid target",
        ffi::EMSCRIPTEN_RESULT_UNKNOWN_TARGET => "Unknown target",
        ffi::EMSCRIPTEN_RESULT_INVALID_PARAM => "Invalid parameter",
        ffi::EMSCRIPTEN_RESULT_FAILED => "Failed",
        ffi::EMSCRIPTEN_RESULT_NO_DATA => "No data",

        _ => "Undocumented error"
    }
}
