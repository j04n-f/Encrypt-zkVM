use super::{Export, Import};

use winterfell::{
    math::fields::f128::BaseElement, ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

#[derive(Clone)]
pub struct FheUInt8 {
    ciphertext: Vec<BaseElement>,
}

impl FheUInt8 {
    pub fn new(ciphertext: &[BaseElement]) -> FheUInt8 {
        FheUInt8 {
            ciphertext: ciphertext.to_vec(),
        }
    }

    pub fn ciphertext(&self) -> &[BaseElement] {
        &self.ciphertext
    }
}

impl Serializable for FheUInt8 {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_usize(self.ciphertext.len());
        for value in self.ciphertext.iter() {
            target.write(value);
        }
    }
}

impl Deserializable for FheUInt8 {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let ct_len = source.read_usize()?;

        let mut ciphertext = Vec::new();

        for _ in 0..ct_len {
            ciphertext.push(BaseElement::read_from(source)?);
        }

        Ok(FheUInt8 { ciphertext })
    }
}

impl Export for FheUInt8 {}

impl Import for FheUInt8 {}

impl std::fmt::Debug for FheUInt8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.ciphertext)?;

        Ok(())
    }
}
