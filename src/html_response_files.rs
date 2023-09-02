use crate::response_files::ResponseFiles;

pub struct HtmlResponseFiles {}

const NOT_FOUND_HTML_FILE: &[u8; 272] = include_bytes!("../pages/404.html");
const NOT_GET_REQUST_METHOD_HTML_FILE: &[u8; 243] = include_bytes!("../pages/not-get.html");
const NOT_GUESSED_MIME_TYPE: &[u8; 323] = include_bytes!("../pages/mime.html");

impl ResponseFiles for HtmlResponseFiles {
    fn not_found(&self) -> &[u8] {
        return NOT_FOUND_HTML_FILE;
    }
    fn not_get_request_method(&self) -> &[u8] {
        return NOT_GET_REQUST_METHOD_HTML_FILE;
    }
    fn not_guessed_mime_type(&self) -> &[u8] {
        return NOT_GUESSED_MIME_TYPE;
    }
}
