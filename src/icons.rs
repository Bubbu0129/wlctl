/// Small helper for choosing icon vs ASCII alternatives at render time.

pub fn maybe<'a>(ascii: bool, icon: &'a str, ascii_alt: &'a str) -> String {
    if ascii {
        ascii_alt.to_string()
    } else {
        icon.to_string()
    }
}
