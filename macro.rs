mod fields {
    use ark_ff::{
        ark_ff_macros::SmallFpConfig, fields::{Fp128, Fp64, MontBackend, MontConfig},
        BigInt, SmallFp,
    };
    use ark_ff::{Fp2, Fp2Config, Fp4, Fp4Config, SqrtPrecomputation};
    #[modulus = "65521"]
    #[generator = "2"]
    #[backend = "montgomery"]
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
}
