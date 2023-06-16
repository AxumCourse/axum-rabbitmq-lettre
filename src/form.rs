use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct ActiveForm {
    pub code: String,
    pub email: String,
}
