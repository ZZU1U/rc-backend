use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct OurJwtPayload {
    pub sub: String,
    pub exp: usize,
}

impl OurJwtPayload {
    pub fn new(sub: String) -> Self {
        // expires by default in 60 minutes from now
        let exp = SystemTime::now()
            .checked_add(Duration::from_secs(60 * 60))
            .expect("valid timestamp")
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("valid duration")
            .as_secs() as usize;

        OurJwtPayload { sub, exp }
    }
}
