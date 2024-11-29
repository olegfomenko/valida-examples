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

    println!("Commitment: {:?}", commitment.to_affine());
    println!("Private key: {:?}", private_key);
    println!("G: {:?}", g.to_affine());
    println!("H: {:?}", h.to_affine());
}
