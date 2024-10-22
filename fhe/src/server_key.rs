use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::ops::Mul;
use winterfell::math::{FieldElement, StarkField};
use winterfell::{
    math::fields::f128::BaseElement, ByteReader, ByteWriter, Deserializable, DeserializationError, Serializable,
};

use super::integer::{FheElement, FheUInt8};
use super::parameters::LweParameters;
use super::{Export, Import};

#[derive(Clone)]
pub struct ServerKey {
    key: Vec<BaseElement>,
    parameters: LweParameters,
}

impl ServerKey {
    pub fn new(parameters: LweParameters) -> ServerKey {
        let mut rng = rand::thread_rng();
        ServerKey {
            key: (0..parameters.k)
                .map(|_| BaseElement::from(rng.gen_range(0..2) as u8))
                .collect(),
            parameters,
        }
    }

    fn generate_mask(&self) -> Vec<BaseElement> {
        let mut rng = rand::thread_rng();
        (0..self.parameters.k)
            .map(|_| BaseElement::try_from(rng.gen_range(0..=u128::MAX)).unwrap())
            .collect()
    }

    fn generate_trivial_mask<E: FieldElement>(&self) -> Vec<E> {
        (0..self.parameters.k).map(|_| E::ZERO).collect()
    }

    pub fn encrypt(&self, value: u8) -> FheUInt8 {
        let mut ciphertext = self.generate_mask();
        let normal = Normal::new(0.0, self.parameters.std).unwrap();
        let noise: f64 = normal.sample(&mut rand::thread_rng()) as f64;
        let scaled_noise = BaseElement::try_from(noise.abs().round() as u128).unwrap();

        let mut body = BaseElement::ZERO;
        let k = self.parameters.k;
        for (ct, key) in ciphertext.iter().take(k).zip(self.key.iter().take(k)) {
            body += ct.mul(*key);
        }
        let val = BaseElement::from(value);
        body += BaseElement::from(self.parameters.delta).mul(val);
        if noise > 0.0 {
            body += scaled_noise;
        } else {
            body -= scaled_noise;
        }
        ciphertext.push(body);

        FheUInt8::new(&ciphertext)
    }

    pub fn decrypt(&self, value: &FheUInt8) -> u8 {
        let ciphertext = value.ciphertext().to_vec();
        let mut applied_mask = BaseElement::ZERO;

        for (i, ct) in ciphertext.iter().enumerate().take(self.parameters.k) {
            applied_mask += *ct * self.key[i];
        }

        let decrypted_message = ciphertext[self.parameters.k] - applied_mask;
        let log2_delta = (self.parameters.delta as f64).log2() as u128;
        let round_bit = (decrypted_message.as_int() >> (log2_delta - 1)) & 1;
        ((decrypted_message.as_int() >> log2_delta) + round_bit) as u8
    }

    pub fn encrypt_trivial<E: FieldElement + From<BaseElement>>(&self, message: &E) -> FheElement<E> {
        let mut ciphertext = self.generate_trivial_mask();
        let body = E::from(self.parameters.delta).mul(*message);
        ciphertext.push(body);
        FheElement::new(&ciphertext)
    }

    pub fn lwe_size(&self) -> usize {
        self.parameters.k + 1
    }

    pub fn add<E: FieldElement + From<BaseElement>>(
        &self,
        value0: &FheElement<E>,
        value1: &FheElement<E>,
    ) -> FheElement<E> {
        let ciphertext = value0
            .ciphertext()
            .iter()
            .zip(value1.ciphertext().iter())
            .take(self.lwe_size())
            .map(|(&ct_value0, &ct_value1)| ct_value0 + ct_value1)
            .collect::<Vec<E>>();
        FheElement::new(&ciphertext)
    }

    pub fn scalar_add<E: FieldElement + From<BaseElement>>(&self, scalar: &E, value: &FheElement<E>) -> FheElement<E> {
        let trivial_scalar = self.encrypt_trivial(scalar);
        let ciphertext = value
            .ciphertext()
            .iter()
            .zip(trivial_scalar.ciphertext().iter())
            .take(self.lwe_size())
            .map(|(&ct_value, &ct_trivial)| ct_value + ct_trivial)
            .collect::<Vec<E>>();
        FheElement::new(&ciphertext)
    }

    pub fn scalar_mul<E: FieldElement + Clone>(&self, &scalar: &E, value: &FheElement<E>) -> FheElement<E> {
        let ciphertext = value
            .ciphertext()
            .iter()
            .take(self.lwe_size())
            .map(|&ct_value| ct_value * scalar)
            .collect::<Vec<E>>();
        FheElement::new(&ciphertext)
    }

    pub fn key(&self) -> &[BaseElement] {
        &self.key
    }
}

impl Serializable for ServerKey {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.parameters.write_into(target);

        target.write_usize(self.key.len());
        for value in self.key.iter() {
            target.write(value);
        }
    }
}

impl Deserializable for ServerKey {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let parameters = LweParameters::read_from(source)?;
        let key_len = source.read_usize()?;

        let mut key = Vec::new();

        for _ in 0..key_len {
            key.push(BaseElement::read_from(source)?);
        }

        Ok(ServerKey { key, parameters })
    }
}

impl Export for ServerKey {}

impl Import for ServerKey {}

impl std::fmt::Debug for ServerKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.key)?;

        Ok(())
    }
}
