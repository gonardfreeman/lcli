use std::ffi::OsString;

pub fn convert_vec_str(arr_os_str: Vec<OsString>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(arr_os_str.len());
    for s in &arr_os_str {
        result.push(convert_os_to_str(s));
    }

    result
}

pub fn convert_os_to_str(os_str: &OsString) -> String {
    os_str.to_string_lossy().into_owned()
}
