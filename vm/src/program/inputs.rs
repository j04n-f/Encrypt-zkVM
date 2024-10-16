use fhe::{FheUInt8, ServerKey};

#[derive(Clone, Debug)]
pub struct ProgramInputs {
    public: Vec<u8>,
    secret: Vec<FheUInt8>,
    server_key: ServerKey,
}

impl ProgramInputs {
    /// Returns `ProgramInputs` initialized with the provided public and secret inputs.
    pub fn new(public: &[u8], secret: &[FheUInt8], server_key: &ServerKey) -> ProgramInputs {
        ProgramInputs {
            public: public.to_vec(),
            secret: secret.to_vec(),
            server_key: server_key.clone(),
        }
    }

    pub fn get_public(&self) -> &[u8] {
        &self.public
    }

    pub fn get_secret(&self) -> &[FheUInt8] {
        &self.secret
    }

    pub fn get_server_key(&self) -> ServerKey {
        self.server_key.clone()
    }
}
