pub struct Certificate {
    pub cert: String,
    pub key: String,
}

impl Certificate {
    pub fn get_certificate(&self) -> rustls::Certificate {
        todo!()
    }
}

type Certificates = Vec<Certificate>;
