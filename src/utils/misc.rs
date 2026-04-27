use std::ffi::OsString;

pub fn convert_os_to_str(os_str: &OsString) -> String {
    os_str.to_string_lossy().into_owned()
}
