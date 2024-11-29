#![no_main]

use k256::elliptic_curve::group::{Curve, GroupEncoding};
use k256::elliptic_curve::Group;
use k256::Scalar;
use rand::rngs::OsRng;
use std::ops::{Add, Mul};

valida_rs::entrypoint!(main);
fn main() {
    let mut csprng = OsRng;

    let private_key = Scalar::generate_biased(&mut csprng);
    let amount = 12345u64;
    let g = k256::ProjectivePoint::random(&mut csprng);
    let h = k256::ProjectivePoint::random(&mut csprng);

    let commitment = g.mul(private_key).add(h.mul(Scalar::from(amount)));

    valida_rs::io::println(&hex::encode(commitment.to_bytes()));
    valida_rs::io::println(&hex::encode(private_key.to_bytes()));
    valida_rs::io::println(&hex::encode(h.to_bytes()));
    valida_rs::io::println(&hex::encode(h.to_bytes()));
}
