use crate::config::RupidConfig;

pub struct RupidEngine {
    conf: RupidConfig,
}

impl RupidEngine {
    pub fn new(conf: &RupidConfig) -> RupidEngine {
        RupidEngine {
            conf: conf.clone(),
        }
    }
}