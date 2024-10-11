use tempfile::NamedTempFile;
use winterfell::math::fields::f128::BaseElement;

use super::*;

#[test]
fn test_export_and_import_server_key() {
    let server_key = default_key();

    let tmpfile = NamedTempFile::new().unwrap();

    let path = tmpfile.into_temp_path();

    server_key.export_to_file(&path).unwrap();

    let imported_key = ServerKey::import_from_file(&path).unwrap();

    assert_eq!(server_key.key(), imported_key.key());
    assert_eq!(server_key.lwe_size(), imported_key.lwe_size());
}

#[test]
fn test_serialize_and_deserialize_server_key() {
    let server_key = default_key();

    let sk_bytes = server_key.to_bytes();

    let read_key = ServerKey::read_from_bytes(&sk_bytes).unwrap();

    assert_eq!(server_key.key(), read_key.key());
    assert_eq!(server_key.lwe_size(), read_key.lwe_size());
}

#[test]
fn test_export_and_import_integer() {
    let server_key = default_key();

    let clear_x = 33u8;

    let x = server_key.encrypt(clear_x);

    let tmpfile = NamedTempFile::new().unwrap();

    let path = tmpfile.into_temp_path();

    x.export_to_file(&path).unwrap();

    let imported_x = FheUInt8::import_from_file(&path).unwrap();

    assert_eq!(x.ciphertext(), imported_x.ciphertext());
}

#[test]
fn test_serialize_and_deserialize_integer() {
    let server_key = default_key();

    let x = server_key.encrypt(33u8);

    let x_bytes = x.to_bytes();

    let read_x = FheUInt8::read_from_bytes(&x_bytes).unwrap();

    assert_eq!(x, read_x);
}

#[test]
fn test_server_key_encryption() {
    let server_key = default_key();

    let clear_x = 33u8;

    let x = server_key.encrypt(clear_x);

    assert_eq!(clear_x, server_key.decrypt(&x))
}

#[test]
fn test_serialize_and_deserialize_parameters() {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;

    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    let parameters_bytes = parameters.to_bytes();

    let read_parameters = LweParameters::read_from_bytes(&parameters_bytes).unwrap();

    assert_eq!(parameters, read_parameters);
}

#[test]
fn test_addition() {
    let server_key = default_key();

    let a = server_key.encrypt(5u8);
    let b = server_key.encrypt(10u8);

    let result = server_key.add(&a, &b);

    assert_eq!(5u8 + 10u8, server_key.decrypt(&result))
}

#[test]
fn test_scalar_addition() {
    let server_key = default_key();

    let a = BaseElement::from(3u8);
    let x = server_key.encrypt(33u8);

    let result = server_key.scalar_add(&a, &x);

    assert_eq!(3u8 + 33u8, server_key.decrypt(&result))
}

#[test]
fn test_scalar_multiplication() {
    let server_key = default_key();

    let a = BaseElement::from(3u8);
    let x = server_key.encrypt(33u8);

    let result = server_key.scalar_mul(&a, &x);

    assert_eq!(3u8 * 33u8, server_key.decrypt(&result))
}

fn default_key() -> ServerKey {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;

    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    ServerKey::new(parameters)
}
