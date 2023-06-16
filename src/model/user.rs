use serde::{Deserialize, Serialize};

use crate::EmailConfig;

#[derive(Serialize, Deserialize, Default)]
pub struct ActiveCode {
    pub email: String,
    pub code: String,
    pub email_cfg: EmailConfig,
}
