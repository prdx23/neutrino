

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let mut msg = "Panic occurred".to_string();

        if let Some(location) = panic_info.location() {
            msg.push_str(format!(" in file '{}' at line {}",
                location.file(),
                location.line(),
            ).as_str());
        }

        if let Some(e) = panic_info.payload().downcast_ref::<&str>() {
            msg.push_str(format!(": {}", e).as_str());
        }

        unsafe {
            crate::console_error_raw(msg.as_ptr(), msg.len());
        }
    }));
}


pub fn console_log(text: &str) {
    unsafe {
        crate::console_log_raw(text.as_ptr(), text.len());
    }
}

pub fn console_error(text: &str) {
    unsafe {
        crate::console_error_raw(text.as_ptr(), text.len());
    }
}


#[macro_export]
macro_rules! add_shader {
    ($name:expr) => {
        {
            let name = stringify!($name);
            let vert = include_str!(
                concat!("shaders/", stringify!($name), ".vert")
            );
            let frag = include_str!(
                concat!("shaders/", stringify!($name), ".frag")
            );
            unsafe {
                crate::add_shader(
                    name.as_ptr(), name.len(),
                    vert.as_ptr(), vert.len(),
                    frag.as_ptr(), frag.len(),
                );
            }
        }
    }
}


#[macro_export]
macro_rules! add_buffer {
    (float, $name:expr, $size:expr, $normalize:expr) => {
        {
            let name = stringify!($name);
            let data = include_str!(
                concat!("data/", stringify!($name), ".json")
            );
            unsafe {
                crate::add_buffer_float(
                    name.as_ptr(), name.len(),
                    data.as_ptr(), data.len(),
                    $size, $normalize
                );
            }
        }
    };
    (bytes, $name:expr, $size:expr, $normalize:expr) => {
        {
            let name = stringify!($name);
            let data = include_str!(
                concat!("data/", stringify!($name), ".json")
            );
            unsafe {
                crate::add_buffer_bytes(
                    name.as_ptr(), name.len(),
                    data.as_ptr(), data.len(),
                    $size, $normalize
                );
            }
        }
    }
}
