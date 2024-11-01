use fhe::{FheUInt8, ServerKey};

#[derive(Clone, Debug)]
pub struct ProgramInputs<'a> {
    public: &'a [u8],
    secret: &'a [FheUInt8],
    server_key: &'a ServerKey,
}

impl<'a> ProgramInputs<'a> {
    pub fn new(public: &'a [u8], secret: &'a [FheUInt8], server_key: &'a ServerKey) -> Self {
        ProgramInputs {
            public,
            secret,
            server_key,
        }
    }

    pub fn public(&self) -> &[u8] {
        self.public
    }

    pub fn secret(&self) -> &[FheUInt8] {
        self.secret
    }

    pub fn server_key(&self) -> &ServerKey {
        self.server_key
    }
}
