#![no_main]

use k256::elliptic_curve::group::{Curve, GroupEncoding};
use k256::elliptic_curve::Group;
use k256::Scalar;
use std::ops::{Add, Mul};

valida_rs::entrypoint!(main);
fn main() {
    let private_key = Scalar::from(12345678909876554321u128);
    let amount = Scalar::from(12345u64);

    let g = k256::ProjectivePoint::GENERATOR;
    let h = g.mul(k256::Scalar::from(12345u32));

    let commitment = g.mul(private_key).add(h.mul(amount));

    valida_rs::io::println(&hex::encode(commitment.to_bytes()));
    valida_rs::io::println(&hex::encode(private_key.to_bytes()));
    valida_rs::io::println(&hex::encode(h.to_bytes()));
    valida_rs::io::println(&hex::encode(h.to_bytes()));
}
