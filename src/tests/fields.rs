use ark_ff::{
    ark_ff_macros::SmallFpConfig,
    fields::{Fp128, Fp64, MontBackend, MontConfig},
    BigInt, SmallFp,
};
use ark_ff::{Fp2, Fp2Config, Fp4, Fp4Config, SqrtPrecomputation};
#[derive(MontConfig)]
#[modulus = "19"]
#[generator = "2"]
pub struct F19Config;
pub type F19 = Fp64<MontBackend<F19Config, 1>>;

#[derive(MontConfig)]
#[modulus = "2147483647"] // 2 ^ 31 - 1
#[generator = "2"]
pub struct M31Config;
pub type M31 = Fp64<MontBackend<M31Config, 1>>;

#[derive(MontConfig)]
#[modulus = "18446744069414584321"] // q = 2^64 - 2^32 + 1
#[generator = "2"]
pub struct F64Config;
pub type F64 = Fp64<MontBackend<F64Config, 1>>;

#[derive(MontConfig)]
#[modulus = "143244528689204659050391023439224324689"] // q = 143244528689204659050391023439224324689
#[generator = "2"]
pub struct F128Config;
pub type F128 = Fp128<MontBackend<F128Config, 2>>;

// #[derive(SmallFpConfig)]
// #[modulus = "65521"]
// #[generator = "2"]
// #[backend = "montgomery"]
// pub struct SmallF16ConfigMont;
// pub type SmallF16 = SmallFp<SmallF16ConfigMont>;

pub struct SmallF16ConfigMont;
const _: () = {
    use ark_ff::{SmallFp, SmallFpConfig};
    impl SmallFpConfig for SmallF16ConfigMont {
        type T = u16;
        const MODULUS: Self::T = 65521u128 as Self::T;
        const MODULUS_U128: u128 = 65521u128;
        const GENERATOR: SmallFp<Self> = SmallFp::new(30u128 as Self::T);
        const ZERO: SmallFp<Self> = SmallFp::new(0 as Self::T);
        const ONE: SmallFp<Self> = SmallFp::new(15u128 as Self::T);
        const NEG_ONE: SmallFp<Self> = SmallFp::new(65506u128 as Self::T);
        const TWO_ADICITY: u32 = 4u32;
        const TWO_ADIC_ROOT_OF_UNITY: SmallFp<Self> = SmallFp::new(
            65506u128 as Self::T,
        );
        const SQRT_PRECOMP: Option<ark_ff::SqrtPrecomputation<SmallFp<Self>>> = {
            const TRACE_MINUS_ONE_DIV_TWO: [u64; 2] = [2047u64, 0u64];
            Some(ark_ff::SqrtPrecomputation::TonelliShanks {
                two_adicity: 4u32,
                quadratic_nonresidue_to_trace: SmallFp::new(7306u128 as Self::T),
                trace_of_modulus_minus_one_div_two: &TRACE_MINUS_ONE_DIV_TWO,
            })
        };
        #[inline(always)]
        fn add_assign(a: &mut SmallFp<Self>, b: &SmallFp<Self>) {
            let (mut val, overflow) = a.value.overflowing_add(b.value);
            if overflow {
                val = Self::T::MAX - Self::MODULUS + 1 + val;
            }
            if val >= Self::MODULUS {
                val -= Self::MODULUS;
            }
            a.value = val;
        }
        #[inline(always)]
        fn sub_assign(a: &mut SmallFp<Self>, b: &SmallFp<Self>) {
            if a.value >= b.value {
                a.value -= b.value;
            } else {
                a.value = Self::MODULUS - (b.value - a.value);
            }
        }
        #[inline(always)]
        fn double_in_place(a: &mut SmallFp<Self>) {
            let tmp = *a;
            Self::add_assign(a, &tmp);
        }
        #[inline(always)]
        fn neg_in_place(a: &mut SmallFp<Self>) {
            if a.value != (0 as Self::T) {
                a.value = Self::MODULUS - a.value;
            }
        }
        #[inline(always)]
        fn mul_assign(a: &mut SmallFp<Self>, b: &SmallFp<Self>) {
            const MODULUS_MUL_TY: u32 = 65521u128 as u32;
            const MODULUS_TY: u16 = 65521u128 as u16;
            const N_PRIME: u16 = 61167u128 as u16;
            const MASK: u32 = 65535u128 as u32;
            const K_BITS: u32 = 16u32;
            let a_val = a.value as u32;
            let b_val = b.value as u32;
            let tmp = a_val * b_val;
            let carry1 = (tmp >> K_BITS) as u16;
            let r = (tmp & MASK) as u16;
            let m = r.wrapping_mul(N_PRIME);
            let tmp = (r as u32) + ((m as u32) * MODULUS_MUL_TY);
            let carry2 = (tmp >> K_BITS) as u16;
            let mut r = (carry1 as u32) + (carry2 as u32);
            if r >= MODULUS_MUL_TY {
                r -= MODULUS_MUL_TY;
            }
            a.value = r as u16;
        }
        #[inline(always)]
        fn sum_of_products<const T: usize>(
            a: &[SmallFp<Self>; T],
            b: &[SmallFp<Self>; T],
        ) -> SmallFp<Self> {
            match T {
                1 => {
                    let mut prod = a[0];
                    Self::mul_assign(&mut prod, &b[0]);
                    prod
                }
                2 => {
                    let mut prod1 = a[0];
                    Self::mul_assign(&mut prod1, &b[0]);
                    let mut prod2 = a[1];
                    Self::mul_assign(&mut prod2, &b[1]);
                    Self::add_assign(&mut prod1, &prod2);
                    prod1
                }
                _ => {
                    let mut acc = SmallFp::new(0 as Self::T);
                    for (x, y) in a.iter().zip(b.iter()) {
                        let mut prod = *x;
                        Self::mul_assign(&mut prod, y);
                        Self::add_assign(&mut acc, &prod);
                    }
                    acc
                }
            }
        }
        #[inline(always)]
        fn square_in_place(a: &mut SmallFp<Self>) {
            let tmp = *a;
            Self::mul_assign(a, &tmp);
        }
        fn inverse(a: &SmallFp<Self>) -> Option<SmallFp<Self>> {
            if a.value == 0 {
                return None;
            }
            let mut result = Self::ONE;
            let mut base = *a;
            let mut exp = Self::MODULUS - 2;
            while exp > 0 {
                if exp & 1 == 1 {
                    Self::mul_assign(&mut result, &base);
                }
                let mut sq = base;
                Self::square_in_place(&mut sq);
                base = sq;
                exp >>= 1;
            }
            Some(result)
        }
        fn from_bigint(a: ark_ff::BigInt<2>) -> Option<SmallFp<Self>> {
            let val = (a.0[0] as u128) + ((a.0[1] as u128) << 64);
            if val > Self::MODULUS_U128 {
                None
            } else {
                let reduced_val = val % 65521u128;
                let mut tmp = SmallFp::new(reduced_val as Self::T);
                let r2_elem = SmallFp::new(225u128 as Self::T);
                <Self as SmallFpConfig>::mul_assign(&mut tmp, &r2_elem);
                Some(tmp)
            }
        }
        fn into_bigint(a: SmallFp<Self>) -> ark_ff::BigInt<2> {
            let mut tmp = a;
            let one = SmallFp::new(1 as Self::T);
            <Self as SmallFpConfig>::mul_assign(&mut tmp, &one);
            let val = tmp.value as u128;
            let lo = val as u64;
            let hi = (val >> 64) as u64;
            ark_ff::BigInt([lo, hi])
        }
    }
    impl SmallF16ConfigMont {
        pub fn new(value: <Self as SmallFpConfig>::T) -> SmallFp<Self> {
            let reduced_value = value % <Self as SmallFpConfig>::MODULUS;
            let mut tmp = SmallFp::new(reduced_value);
            let r2_elem = SmallFp::new(225u128 as <Self as SmallFpConfig>::T);
            <Self as SmallFpConfig>::mul_assign(&mut tmp, &r2_elem);
            tmp
        }
        pub fn exit(a: &mut SmallFp<Self>) {
            let mut tmp = *a;
            let one = SmallFp::new(1 as <Self as SmallFpConfig>::T);
            <Self as SmallFpConfig>::mul_assign(&mut tmp, &one);
            a.value = tmp.value;
        }
    }
};
pub type SmallF16 = SmallFp<SmallF16ConfigMont>;

#[derive(SmallFpConfig)]
#[modulus = "2147483647"] // 2 ^ 31 - 1
#[generator = "2"]
#[backend = "montgomery"]
pub struct SmallM31ConfigMont;
pub type SmallM31 = SmallFp<SmallM31ConfigMont>;

#[derive(SmallFpConfig)]
#[modulus = "18446744069414584321"] // Goldilock's prime 2^64 - 2^32 + 1
#[generator = "2"]
#[backend = "montgomery"]
pub struct SmallF64ConfigMont;
pub type SmallGoldilocks = SmallFp<SmallF64ConfigMont>;

// SmallM31 extensions
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fp2SmallM31Config;

impl Fp2Config for Fp2SmallM31Config {
    type Fp = SmallM31;

    // Use const_new to build compile-time constants
    const NONRESIDUE: SmallM31 = SmallM31::new(3);

    // These Frobenius coeffs aren't used for arithmetic benchmarks anyway
    const FROBENIUS_COEFF_FP2_C1: &'static [SmallM31] = &[SmallM31::new(1), SmallM31::new(3)];
}

pub type Fp2SmallM31 = Fp2<Fp2SmallM31Config>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fp4SmallM31Config;

impl Fp4Config for Fp4SmallM31Config {
    type Fp2Config = Fp2SmallM31Config;

    const NONRESIDUE: Fp2<Fp2SmallM31Config> =
        Fp2::<Fp2SmallM31Config>::new(SmallM31::new(3), SmallM31::new(7));

    // 👇 now a slice of base‐field elements, not Fp2 elements
    const FROBENIUS_COEFF_FP4_C1: &'static [SmallM31] = &[
        SmallM31::new(1),
        SmallM31::new(3),
        SmallM31::new(9),
        SmallM31::new(27),
    ];
}

pub type Fp4SmallM31 = Fp4<Fp4SmallM31Config>;

// SmallGoldilocks extensions
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Fp2SmallGoldilocksConfig;

impl Fp2Config for Fp2SmallGoldilocksConfig {
    type Fp = SmallGoldilocks;

    /// For Goldilocks, 7 is a quadratic non-residue.
    /// The extension is formed by SmallGoldilocks[u] / (u^2 - 7)
    /// Reference: https://github.com/zhenfeizhang/Goldilocks
    const NONRESIDUE: SmallGoldilocks = SmallGoldilocks::new(7);

    /// Frobenius coefficients are used for computing elements raised to the power of the modulus.
    /// In a quadratic extension, these are often just [1, -1] or precomputed constants.
    const FROBENIUS_COEFF_FP2_C1: &'static [SmallGoldilocks] = &[
        SmallGoldilocks::new(1),
        // This is typically -1 in the field, or the precomputed Frobenius constant.
        SmallGoldilocks::new(18446744069414584320),
    ];
}

pub type Fp2SmallGoldilocks = Fp2<Fp2SmallGoldilocksConfig>;



#[cfg(test)]
mod tests {
    use super::*;

    fn as_bytes<T>(v: &T) -> Vec<u8> {
        unsafe {
            std::slice::from_raw_parts(
                (v as *const T) as *const u8,
                core::mem::size_of::<T>(),
            )
            .to_vec()
        }
    }

    #[test]
    fn smallgoldilocks_vs_goldilocks_raw_and_fmt_should_be_the_same() {
        // SmallFp has `new`, Fp64 commonly supports From<u64>.
        let small = SmallGoldilocks::new(7);
        let normal = F64::from(7u64);

        // Display formatting: should be canonical "7" for both.
        let small_fmt = format!("{}", small); // <-- bug that this IS mont form
        let normal_fmt = format!("{}", normal); // <-- this is 7 (as expected)
        assert_eq!(small_fmt, normal_fmt);

        // whatever is in memory is the same 
        let small_bytes = as_bytes(&small); // <-- TODO: bug that this is NOT mont form? (it's 7)
        let normal_bytes = as_bytes(&normal); // <-- this is mont form (as expected)
        assert_eq!(
            small_bytes, normal_bytes,
            "Raw in-memory bytes differ between SmallGoldilocks and Goldilocks for 7."
        );
    }

    #[test]
    fn debug_smallf16_and_smallm31_and_goldilocks() {

        // --- SmallF16 ---
        println!("========================================");
        println!("Testing SmallF16 (Modulus 65521)");
        println!("========================================");
        
        let small_f16 = SmallF16::new(1);
        let f16_fmt = format!("{}", small_f16);
        let f16_bytes = as_bytes(&small_f16);

        println!("Display Formatted: {}", f16_fmt);
        println!("Raw Bytes (Hex):   {:02x?}", f16_bytes);
        

        // // --- SmallM31 ---
        // println!("\n========================================");
        // println!("Testing SmallM31 (Modulus 2^31 - 1)");
        // println!("========================================");

        // let small_m31 = SmallM31::new(1);
        // let m31_fmt = format!("{}", small_m31);
        // let m31_bytes = as_bytes(&small_m31);

        // println!("Display Formatted: {}", m31_fmt);
        // println!("Raw Bytes (Hex):   {:02x?}", m31_bytes);


        // // --- SmallGoldilocks ---
        // println!("\n========================================");
        // println!("Testing SmallGoldilocks (Modulus 2^64 - 2^32 + 1)");
        // println!("========================================");

        // let small_gld = SmallGoldilocks::new(1);
        // let gld_fmt = format!("{}", small_gld);
        // let gld_bytes = as_bytes(&small_gld);

        // println!("Display Formatted: {}", gld_fmt);
        // println!("Raw Bytes (Hex):   {:02x?}", gld_bytes);
        

        //   // --- SmallGoldilocks ---
        // println!("\n========================================");
        // println!("Testing Goldilocks (Modulus 2^64 - 2^32 + 1)");
        // println!("========================================");

        // let gld = F64::from(1u64);
        // let gld_fmt = format!("{}", gld);
        // let gld_bytes = as_bytes(&gld);

        // println!("Display Formatted: {}", gld_fmt);
        // println!("Raw Bytes (Hex):   {:02x?}", gld_bytes);
    }
}