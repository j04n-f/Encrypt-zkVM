use winterfell::{ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable};
#[derive(Clone)]
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

impl Serializable for LweParameters {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_u32(self.plaintext_modulus);
        target.write_u32(self.ciphertext_modulus);
        target.write_u32(self.delta);
        target.write_usize(self.k);
        target.write(self.std.to_le_bytes());
    }
}

impl Deserializable for LweParameters {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let plaintext_modulus = source.read_u32()?;
        let ciphertext_modulus = source.read_u32()?;
        let delta = source.read_u32()?;
        let k = source.read_usize()?;
        let std_bytes = source.read_array::<8>()?;
        let std = f64::from_le_bytes(std_bytes);

        Ok(LweParameters {
            plaintext_modulus,
            ciphertext_modulus,
            delta,
            k,
            std,
        })
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
