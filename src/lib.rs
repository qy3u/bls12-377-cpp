use ark_bls12_377::Fr;
use ark_ff::{Zero, One, Field, PrimeField};


#[no_mangle]
#[allow(non_snake_case)]
pub fn Fr_ZERO() -> Fr {
    Fr::zero()
}

#[no_mangle]
#[allow(non_snake_case)]
pub fn Fr_ONE() -> Fr {
    Fr::one()
}

#[no_mangle]
pub unsafe fn print_ptr(p: *const u64) {
    println!("{:p}", p);
}

#[no_mangle]
pub unsafe fn print_arr_ptr(p: *const u64) {
    println!("{:p}", p);
    println!("{:?}", *(p as *const Fr));
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_display(l: *const Fr){
    println!("{:?}", *l);
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_add(l: *const Fr, r: *const Fr) -> Fr {
    *l + *r
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_sub(l: *const Fr, r: *const Fr) -> Fr {
    *l - *r
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_mul(l: *const Fr, r: *const Fr) -> Fr {
    *l * *r
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_div(l: *const Fr, r: *const Fr) -> Fr {
    *l / *r
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_inv(l: *const Fr) -> Fr {
    -*l
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_pow(l: *const Fr, r: *const Fr) -> Fr {
    (*l).pow((&*r).into_repr())
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe fn Fr_from_u64(v: u64) -> Fr {
    Fr::from(v as u128)
}