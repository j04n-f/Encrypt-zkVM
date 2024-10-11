use winterfell::{
    math::{fields::f128::BaseElement, FieldElement},
    ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

use super::{Export, Import};

pub type FheUInt8 = FheElement<BaseElement>;

#[derive(Clone, PartialEq)]
pub struct FheElement<E>
where
    E: FieldElement,
{
    ciphertext: Vec<E>,
}

impl<E: FieldElement> FheElement<E> {
    pub fn new(ciphertext: &[E]) -> FheElement<E> {
        FheElement {
            ciphertext: ciphertext.to_vec(),
        }
    }

    pub fn ciphertext(&self) -> &[E] {
        &self.ciphertext
    }
}

impl<E: FieldElement> Serializable for FheElement<E> {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        target.write_usize(self.ciphertext.len());
        for value in self.ciphertext.iter() {
            target.write(value);
        }
    }
}

impl<E: FieldElement> Deserializable for FheElement<E> {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let ct_len = source.read_usize()?;

        let mut ciphertext = Vec::new();

        for _ in 0..ct_len {
            ciphertext.push(E::read_from(source)?);
        }

        Ok(FheElement { ciphertext })
    }
}

impl<E: FieldElement> Export for FheElement<E> {}

impl<E: FieldElement> Import for FheElement<E> {}

impl<E: FieldElement> std::fmt::Debug for FheElement<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.ciphertext)?;

        Ok(())
    }
}
