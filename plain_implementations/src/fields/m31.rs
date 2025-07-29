use ark_ff::fields::{Fp64, MontBackend, MontConfig};
use std::convert::TryInto;

#[derive(MontConfig)]
#[modulus = "2147483647"] // M31 = 2^31 - 1
#[generator = "7"]
pub struct FqConfig;
pub type FpM31 = Fp64<MontBackend<FqConfig, 1>>;
