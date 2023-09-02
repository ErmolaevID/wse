pub trait ResponseFiles {
    fn not_found(&self) -> &[u8];
    fn not_get_request_method(&self) -> &[u8];
    fn not_guessed_mime_type(&self) -> &[u8];
}
