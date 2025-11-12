use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;

pub fn generate_refresh_token() -> String {
    let mut buf = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut buf);
    general_purpose::URL_SAFE_NO_PAD.encode(&buf)
}
