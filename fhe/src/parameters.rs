use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LweParameters {
    pub plaintext_modulus: u32,
    pub ciphertext_modulus: u32,
    pub delta: u32,
    pub k: usize,
    pub std: f64,
}

impl LweParameters {
    pub fn new(plaintext_modulus: u32, ciphertext_modulus: u32, k: usize, std: f64) -> Self {
        LweParameters {
            plaintext_modulus,
            ciphertext_modulus,
            delta: (ciphertext_modulus / plaintext_modulus),
            k,
            std,
        }
    }
}

impl std::fmt::Debug for LweParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Plaintext Modulus {}", self.plaintext_modulus)?;
        writeln!(f, "Ciphertext Modulus {}", self.ciphertext_modulus)?;
        writeln!(f, "Delta {}", self.delta)?;
        writeln!(f, "K {}", self.k)?;
        writeln!(f, "Standard Deviation {}", self.std)?;

        Ok(())
    }
}
