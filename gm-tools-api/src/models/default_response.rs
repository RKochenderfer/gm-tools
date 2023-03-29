use serde::Serialize;

#[derive(Serialize, Default)]
pub struct DefaultResponse {
    pub ok: bool,
    pub message: String,
}
