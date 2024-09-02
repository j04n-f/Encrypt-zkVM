use super::*;

#[test]
fn test_server_key_encryption() {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;

    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    let server_key = ServerKey::new(parameters);

    let clear_x = 33u8;

    let x = server_key.encrypt(clear_x);

    assert_eq!(clear_x, server_key.decrypt(&x))
}

#[test]
fn test_scalar_addition() {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;

    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    let server_key = ServerKey::new(parameters);

    let clear_a = 3u8;
    let clear_x = 33u8;

    let x = server_key.encrypt(clear_x);

    let result = server_key.scalar_add(&clear_a, &x);

    assert_eq!(clear_a + clear_x, server_key.decrypt(&result))
}

#[test]
fn test_scalar_multiplication() {
    let plaintext_modulus: u32 = 8u32;
    let ciphertext_modulus: u32 = 128u32;
    let k: usize = 4;
    let std = 2.412_390_240_121_573e-5;

    let parameters = LweParameters::new(plaintext_modulus, ciphertext_modulus, k, std);

    let server_key = ServerKey::new(parameters);

    let clear_a = 2u8;
    let clear_x = 33u8;

    let x = server_key.encrypt(clear_x);

    let result = server_key.scalar_mul(&clear_a, &x);

    assert_eq!(clear_a * clear_x, server_key.decrypt(&result))
}
