#![no_main]

use k256::elliptic_curve::Group;
use k256::{AffinePoint, ProjectivePoint, Scalar};
use std::ops::{Add, Mul};

valida_rs::entrypoint!(main);

#[no_mangle]
fn main() {
    let private_key_str = valida_rs::io::read_line::<String>().unwrap();
    let private_key = serde_json::from_str::<Scalar>(&private_key_str).unwrap();

    let amount = valida_rs::io::read_line::<u64>().unwrap();

    let g_str = valida_rs::io::read_line::<String>().unwrap();
    let g = serde_json::from_str::<AffinePoint>(&g_str).unwrap();

    let h_str = valida_rs::io::read_line::<String>().unwrap();
    let h =  serde_json::from_str::<AffinePoint>(&h_str).unwrap();

    let commitment = ProjectivePoint::from(g)
        .mul(private_key)
        .add(ProjectivePoint::from(h).mul(Scalar::from(amount)));

    println!("Commitment: {}", serde_json::to_string_pretty(&commitment.to_affine()).unwrap());
}
