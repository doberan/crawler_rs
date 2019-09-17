//! # crawler library
//! 
//! ## getting started.
//! 

/// crawler client struct
pub struct Client {
    /// domain. ex) https://sample.com/
    pub domain: String,
}

/// client impl
impl Client {
    /// new method.
    /// return Client
    pub fn new(domain: String) ->  Self {
        Client {
            domain: domain.clone(),
        }
    }
}