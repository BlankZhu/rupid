use std::{fs, io, io::Read, path::{self, Path}};

const CERTIFICATE_HEADER: &str = "-----BEGIN CERTIFICATE-----\n";

pub struct Certificate {
    pub cert: String,
    pub key: String,
}

impl Certificate {
    pub fn read_certificate(&self) -> Result<rustls::Certificate, io::Error> {
        let cert_file = fs::File::open(&self.cert)?;
        let mut reader = io::BufReader::new(cert_file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(rustls::Certificate(data))
    }

    pub fn read_private_key(&self) -> Result<rustls::PrivateKey, std::io::Error> {
        let key_file = fs::File::open(&self.key)?;
        let mut reader = io::BufReader::new(key_file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        Ok(rustls::PrivateKey(data))
    }

    pub fn get_truncated_certificate_name(&self) -> String {
        let cert_name = self.cert.as_str();
        let mut ret = self.cert.clone();

        if !is_path(cert_name) && 
            cert_name.starts_with(CERTIFICATE_HEADER) && 
            cert_name.len() > CERTIFICATE_HEADER.len() + 50    
        {
            ret = cert_name.trim_start_matches(CERTIFICATE_HEADER)[..50].to_string();
        }
        ret
    }
}

fn is_path(s: &str) -> bool {
    Path::new(s).exists()
}