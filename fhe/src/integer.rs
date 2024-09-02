use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FheUInt8 {
    ciphertext: Vec<u128>,
}

impl FheUInt8 {
    pub fn new(ciphertext: &[u128]) -> FheUInt8 {
        FheUInt8 {
            ciphertext: ciphertext.to_vec(),
        }
    }

    pub fn ciphertext(&self) -> Vec<u128> {
        self.ciphertext.clone()
    }
}

impl std::fmt::Debug for FheUInt8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.ciphertext)?;

        Ok(())
    }
}
