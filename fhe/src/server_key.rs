use std::num::Wrapping;

use rand::Rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

use super::{Export, Import};

use super::FheUInt8;
use super::LweParameters;

#[derive(Serialize, Deserialize, Clone)]
pub struct ServerKey {
    key: Vec<u32>,
    parameters: LweParameters,
}

impl ServerKey {
    pub fn new(parameters: LweParameters) -> ServerKey {
        let mut rng = rand::thread_rng();
        ServerKey {
            key: (0..parameters.k).map(|_| rng.gen_range(0..2)).collect(),
            parameters,
        }
    }

    fn generate_mask(&self) -> Vec<Wrapping<u128>> {
        let mut rng = rand::thread_rng();
        (0..self.parameters.k)
            .map(|_| Wrapping(rng.gen_range(0..=u128::MAX)))
            .collect()
    }

    fn generate_trivial_mask(&self) -> Vec<Wrapping<u128>> {
        (0..self.parameters.k).map(|_| Wrapping(0u128)).collect()
    }

    pub fn encrypt(&self, value: u8) -> FheUInt8 {
        let mut ciphertext = self.generate_mask();
        let normal = Normal::new(0.0, self.parameters.std).unwrap();
        let noise: f64 = normal.sample(&mut rand::thread_rng()) as f64;
        let scaled_noise = Wrapping(noise.abs().round() as u128);
        let secret_key = self
            .key
            .iter()
            .map(|value| Wrapping(*value as u128))
            .collect::<Vec<Wrapping<u128>>>();

        let mut body = Wrapping(0u128);
        for i in 0..self.parameters.k {
            body += ciphertext[i] * secret_key[i];
        }
        body += Wrapping((self.parameters.delta as u128) * (value as u128));
        if noise > 0.0 {
            body += scaled_noise;
        } else {
            body -= scaled_noise;
        }
        ciphertext.push(body);

        FheUInt8::new(&ciphertext.iter().map(|value| value.0).collect::<Vec<u128>>())
    }

    pub fn decrypt(&self, value: &FheUInt8) -> u8 {
        let ciphertext = value
            .ciphertext()
            .iter()
            .map(|value| Wrapping(*value))
            .collect::<Vec<Wrapping<u128>>>();
        let mut applied_mask = Wrapping(0u128);
        let secret_key = self
            .key
            .iter()
            .map(|value| Wrapping(*value as u128))
            .collect::<Vec<Wrapping<u128>>>();

        for i in 0..self.parameters.k {
            applied_mask += ciphertext[i] * secret_key[i];
        }

        let decrypted_message = ciphertext[self.parameters.k] - applied_mask;
        let log2_delta = (self.parameters.delta as f64).log2() as u128;
        let round_bit = (decrypted_message.0 >> (log2_delta - 1)) & 1;
        ((decrypted_message.0 >> log2_delta) + round_bit) as u8
    }

    pub fn encrypt_trivial(&self, message: &u8) -> FheUInt8 {
        let mut ciphertext = self.generate_trivial_mask();
        let body = Wrapping(self.parameters.delta as u128) * Wrapping(*message as u128);
        ciphertext.push(body);
        FheUInt8::new(&ciphertext.iter().map(|value| value.0).collect::<Vec<u128>>())
    }

    pub fn lwe_size(&self) -> usize {
        self.parameters.k + 1
    }

    pub fn scalar_add(&self, scalar: &u8, value: &FheUInt8) -> FheUInt8 {
        let trivial_ct = self.encrypt_trivial(scalar).ciphertext();
        let mut ciphertext = value.ciphertext();
        for i in 0..self.lwe_size() {
            ciphertext[i] = ciphertext[i].wrapping_add(trivial_ct[i]);
        }
        FheUInt8::new(&ciphertext)
    }

    pub fn scalar_mul(&self, scalar: &u8, value: &FheUInt8) -> FheUInt8 {
        let ciphertext = value.ciphertext();
        FheUInt8::new(
            &ciphertext
                .iter()
                .take(self.lwe_size())
                .map(|value| value.wrapping_mul(*scalar as u128))
                .collect::<Vec<u128>>(),
        )
    }

    pub fn key(&self) -> Vec<u32> {
        self.key.clone()
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
