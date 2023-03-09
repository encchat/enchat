use curve25519_dalek::{Scalar, constants::ED25519_BASEPOINT_TABLE, MontgomeryPoint, EdwardsPoint};
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use rand::{CryptoRng, RngCore};
use sha2::{Sha512, Digest};
use subtle::ConstantTimeEq;
use x25519_dalek::StaticSecret;

const SIGNATURE_LENGTH: usize = 64;
pub type Signature = [u8; SIGNATURE_LENGTH];

// Based on https://github.com/signalapp/libsignal/blob/39293fa9067c8b305a76b8d748f6931e645a8f15/rust/protocol/src/curve/curve25519.rs#L52
pub fn calculate_signature<R>(
    secret: &StaticSecret,
    csprng: &mut R,
    message: &[u8]
) -> Signature
    where R: CryptoRng + RngCore
{
    let mut random_bytes = [0u8; 64];
    csprng.fill_bytes(&mut random_bytes);
    let key_data = secret.to_bytes();
    let a = Scalar::from_bits(key_data);
    let ed_public_key_point = &a * ED25519_BASEPOINT_TABLE;
    let ed_public_key = ed_public_key_point.compress();
    let sign_bit = ed_public_key.as_bytes()[31] & 0b1000_0000_u8;

    let mut hash1 = Sha512::new();
    let hash_prefix = [
        0xFEu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
        0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8,
    ];
    hash1.update(&hash_prefix[..]);
    hash1.update(&key_data[..]);
    hash1.update(message);
    hash1.update(&random_bytes[..]);
    let r = Scalar::from_hash(hash1);
    let cap_r = (&r * ED25519_BASEPOINT_TABLE).compress();
    let mut hash = Sha512::new();
    hash.update(cap_r.as_bytes());
    hash.update(ed_public_key.as_bytes());
    hash.update(message);

    let h = Scalar::from_hash(hash);
    let s = (h * a) + r;
    let mut result = [0u8; SIGNATURE_LENGTH];
    result[..32].copy_from_slice(cap_r.as_bytes());
    result[32..].copy_from_slice(s.as_bytes());
    result[SIGNATURE_LENGTH - 1] &= 0b0111_1111_u8;
    result[SIGNATURE_LENGTH - 1] |= sign_bit;
    result
}

pub fn verify_signature(
    their_public_key: &[u8; PUBLIC_KEY_LENGTH],
    message: &[u8],
    signature: &Signature,
) -> bool {
    let mont_point = MontgomeryPoint(*their_public_key);
    let ed_pub_key_point =
        match mont_point.to_edwards((signature[SIGNATURE_LENGTH - 1] & 0b1000_0000_u8) >> 7) {
            Some(x) => x,
            None => return false,
        };
    let cap_a = ed_pub_key_point.compress();
    let mut cap_r = [0u8; 32];
    cap_r.copy_from_slice(&signature[..32]);
    let mut s = [0u8; 32];
    s.copy_from_slice(&signature[32..]);
    s[31] &= 0b0111_1111_u8;
    if (s[31] & 0b1110_0000_u8) != 0 {
        return false;
    }
    let minus_cap_a = -ed_pub_key_point;

    let mut hash = Sha512::new();
    // Explicitly pass a slice to avoid generating multiple versions of update().
    hash.update(&cap_r[..]);
    hash.update(cap_a.as_bytes());
    hash.update(message);

    let h = Scalar::from_hash(hash);

    let cap_r_check_point = EdwardsPoint::vartime_double_scalar_mul_basepoint(
        &h,
        &minus_cap_a,
        &Scalar::from_bits(s),
    );
    let cap_r_check = cap_r_check_point.compress();

    bool::from(cap_r_check.as_bytes().ct_eq(&cap_r))
}