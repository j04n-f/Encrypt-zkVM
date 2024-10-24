use crypto::rescue::Hash;
use fhe::{FheUInt8, ServerKey};
use winterfell::{
    math::fields::f128::BaseElement, ByteReader, ByteWriter, Deserializable, DeserializationError, Proof, Serializable,
};

#[derive(Clone)]
pub struct InputData {
    public_inputs: Vec<u8>,
    secret_inputs: Vec<FheUInt8>,
    server_key: ServerKey,
}

impl InputData {
    pub fn new(public_inputs: &[u8], secret_inputs: &[FheUInt8], server_key: &ServerKey) -> InputData {
        InputData {
            public_inputs: public_inputs.to_vec(),
            secret_inputs: secret_inputs.to_vec(),
            server_key: server_key.clone(),
        }
    }

    pub fn public_inputs(&self) -> &[u8] {
        &self.public_inputs
    }

    pub fn secret_inputs(&self) -> &[FheUInt8] {
        &self.secret_inputs
    }

    pub fn server_key(&self) -> &ServerKey {
        &self.server_key
    }
}

impl Serializable for InputData {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.server_key.write_into(target);

        target.write_usize(self.secret_inputs.len());
        for secret in self.secret_inputs.iter() {
            secret.write_into(target);
        }

        target.write_usize(self.public_inputs.len());
        target.write_bytes(&self.public_inputs);
    }
}

impl Deserializable for InputData {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let server_key = ServerKey::read_from(source)?;

        let sec_len = source.read_usize()?;
        let mut secret_inputs = Vec::new();

        for _ in 0..sec_len {
            secret_inputs.push(FheUInt8::read_from(source)?);
        }

        let pub_len = source.read_usize()?;
        let public_inputs = source.read_vec(pub_len)?;

        Ok(InputData {
            public_inputs,
            secret_inputs,
            server_key,
        })
    }
}

#[derive(Clone)]
pub struct OutputData {
    hash: Hash,
    proof: Proof,
    output: [BaseElement; 16],
}

impl OutputData {
    pub fn new(hash: Hash, proof: Proof, output: [BaseElement; 16]) -> OutputData {
        OutputData { hash, proof, output }
    }

    pub fn hash(&self) -> &Hash {
        &self.hash
    }

    pub fn proof(&self) -> &Proof {
        &self.proof
    }

    pub fn output(&self) -> [BaseElement; 16] {
        self.output
    }
}

impl Serializable for OutputData {
    fn write_into<W: ByteWriter>(&self, target: &mut W) {
        self.hash.write_into(target);
        self.proof.write_into(target);

        target.write_usize(self.output.len());
        for secret in self.output.iter() {
            secret.write_into(target);
        }
    }
}

impl Deserializable for OutputData {
    fn read_from<R: ByteReader>(source: &mut R) -> Result<Self, DeserializationError> {
        let hash = Hash::read_from(source)?;
        let proof = Proof::read_from(source)?;
        let output_len = source.read_usize()?;

        let mut out = Vec::new();

        for _ in 0..output_len {
            out.push(BaseElement::read_from(source)?);
        }

        match out.try_into() {
            Ok(output) => Ok(OutputData { hash, proof, output }),
            Err(_) => Err(DeserializationError::UnknownError(
                "expected an array containing f128::BaseElement of length 16".to_string(),
            )),
        }
    }
}
