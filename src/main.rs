#![no_main]

use k256::elliptic_curve::Group;
use k256::Scalar;
use std::ops::{Add, Mul};
use k256::elliptic_curve::group::Curve;
use rand::rngs::OsRng;

valida_rs::entrypoint!(main);
fn main() {

    let mut csprng = OsRng;

    let private_key = Scalar::generate_biased(&mut csprng);
    let amount = 12345u64;
    let g = k256::ProjectivePoint::random(&mut csprng);
    let h = k256::ProjectivePoint::random(&mut csprng);

    let commitment = g
        .mul(private_key)
        .add(h.mul(Scalar::from(amount)));

    valida_rs::io::println("Commitment: {:?}", commitment.to_affine());
    valida_rs::io::println("Private key: {:?}", private_key);
    valida_rs::io::println("G: {:?}", g.to_affine());
    valida_rs::io::println("H: {:?}", h.to_affine());
}
