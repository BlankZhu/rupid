// Domain holds a domain name with SANs
pub struct Domain {
    pub main: String,      // default subject name
    pub sans: Vec<String>, // subject alternative names
}

impl Domain {
    // to_str_array convert a domain into an array of Strings
    pub fn to_str_array(&self) -> Vec<String> {
        let mut ret = Vec::<String>::new();

        if self.main.is_empty() {
            ret.push(self.main.clone());
        }
        ret.append(&mut self.sans.clone());

        ret
    }

    pub fn set(&mut self, domains: &Vec<String>) {
        if !domains.is_empty() {
            self.main = domains[0].clone();
            self.sans.clear();
            self.sans.extend_from_slice(&domains[1..]);
        }
    }

    pub fn match_domain(domain: &String, cert_domain: &String) -> bool {
        if domain == cert_domain {
            return true;
        }

        let mut cert_domain_copy = cert_domain.clone();
        while cert_domain_copy.len() > 0 {
            if cert_domain_copy.chars().last().unwrap() == '.' {
                cert_domain_copy.remove(cert_domain_copy.len() - 1);
            }
        }

        let mut labels: Vec<String> = domain.split(".").map(|s| s.to_string()).collect();
        for i in 0..labels.len() {
            let label = labels.get_mut(i).unwrap();
            *label = "*".to_string();
            let candidate = labels.join(".");
            if candidate == cert_domain_copy {
                return true;
            }
        }
        
        false
    }

    pub fn canonical_domain(domain: &String) -> String {
        domain.trim().to_ascii_lowercase().to_string()
    } 
}
