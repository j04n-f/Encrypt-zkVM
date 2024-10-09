use tempfile::NamedTempFile;

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

// #[test]
// fn test_export_and_import_integer() {
//     let server_key = default_key();

//     let clear_x = 33u8;

//     let x = server_key.encrypt(clear_x);

//     let tmpfile = NamedTempFile::new().unwrap();

//     let path = tmpfile.into_temp_path();

//     x.export_to_file(&path).unwrap();

//     let imported_x = FheUInt8::import_from_file(&path).unwrap();

//     assert_eq!(x.ciphertext(), imported_x.ciphertext());
// }

// #[test]
// fn test_server_key_encryption() {
//     let server_key = default_key();

//     let clear_x = 33u8;

//     let x = server_key.encrypt(clear_x);

//     assert_eq!(clear_x, server_key.decrypt(&x))
// }

// #[test]
// fn test_scalar_addition() {
//     let server_key = default_key();

//     let clear_a = 3u8;
//     let clear_x = 33u8;

//     let x = server_key.encrypt(clear_x);

//     let result = server_key.scalar_add(&clear_a, &x);

//     assert_eq!(clear_a + clear_x, server_key.decrypt(&result))
// }

// #[test]
// fn test_scalar_multiplication() {
//     let server_key = default_key();

//     let clear_a = 2u8;
//     let clear_x = 33u8;

//     let x = server_key.encrypt(clear_x);

//     let result = server_key.scalar_mul(&clear_a, &x);

//     assert_eq!(clear_a * clear_x, server_key.decrypt(&result))
// }

fn default_key() -> ServerKey {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;

    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    ServerKey::new(parameters)
}
