// Copyright (C) 2017-2018 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![crate_name = "helloworldsampleenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_tunittest;

use sgx_types::*;
use std::string::String;
use std::vec::Vec;
use std::io::{self, Write};
use std::slice;
use std::panic;
use sgx_tunittest::*;

extern crate num_bigint_dig as num_bigint;
extern crate num_traits;
extern crate num_integer;
extern crate rand;
extern crate smallvec;
//extern crate rand_chacha;
extern crate rand_isaac;
extern crate rand_xorshift;
extern crate serde_test;

#[macro_use]
mod macros;
mod consts;
mod bigint;
mod bigint_bitwise;
mod bigint_scalar;
mod biguint;
mod biguint_scalar;
mod modpow;
mod rand_tests;
mod roots;
mod serde;
mod torture;

#[no_mangle]
pub extern "C" fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    // A sample &'static string
    let rust_raw_string = "This is a in-Enclave ";
    // An array
    let word:[u8;4] = [82, 117, 115, 116];
    // An vector
    let word_vec:Vec<u8> = vec![32, 115, 116, 114, 105, 110, 103, 33];

    // Construct a string from &'static string
    let mut hello_string = String::from(rust_raw_string);

    // Iterate on word array
    for c in word.iter() {
        hello_string.push(*c as char);
    }

    // Rust style convertion
    hello_string += String::from_utf8(word_vec).expect("Invalid UTF-8")
                                               .as_str();

    // Ocall to normal world for output
    println!("{}", &hello_string);

    rsgx_unit_tests!(
bigint::test_from_bytes_be,
bigint::test_to_bytes_be,
bigint::test_from_bytes_le,
bigint::test_to_bytes_le,
bigint::test_to_signed_bytes_le,
bigint::test_from_signed_bytes_le,
bigint::test_to_signed_bytes_be,
bigint::test_from_signed_bytes_be,
bigint::test_signed_bytes_be_round_trip,
bigint::test_signed_bytes_le_round_trip,
bigint::test_cmp,
bigint::test_hash,
bigint::test_convert_i64,
bigint::test_convert_i128,
bigint::test_convert_u64,
bigint::test_convert_u128,
bigint::test_convert_f32,
bigint::test_convert_f64,
bigint::test_convert_to_biguint,
bigint::test_convert_from_uint,
bigint::test_convert_from_int,
bigint::test_convert_from_biguint,
bigint::test_add,
bigint::test_add_mut,
bigint::test_sub,
bigint::test_mul,
bigint::test_div_mod_floor,
bigint::test_div_rem,
bigint::test_checked_add,
bigint::test_checked_sub,
bigint::test_checked_mul,
bigint::test_checked_div,
bigint::test_gcd,
bigint::test_lcm,
bigint::test_abs_sub,
bigint::test_from_str_radix,
bigint::test_lower_hex,
bigint::test_upper_hex,
bigint::test_binary,
bigint::test_octal,
bigint::test_display,
bigint::test_neg,
bigint::test_negative_shr,
bigint::test_random_shr,
bigint::test_iter_sum,
bigint::test_iter_product,
bigint::test_iter_sum_generic,
bigint::test_iter_product_generic,
bigint::test_pow,
bigint_bitwise::test_not,
bigint_bitwise::test_not_i64,
bigint_bitwise::test_bitwise,
bigint_bitwise::test_bitwise_i64,
bigint_scalar::test_scalar_add,
bigint_scalar::test_scalar_sub,
bigint_scalar::test_scalar_mul,
bigint_scalar::test_scalar_div_rem,
biguint::test_from_bytes_be,
biguint::test_to_bytes_be,
biguint::test_from_bytes_le,
biguint::test_to_bytes_le,
biguint::test_cmp,
biguint::test_hash,
biguint::test_bitand,
biguint::test_bitor,
biguint::test_bitxor,
biguint::test_shl,
biguint::test_shr,
biguint::test_convert_i64,
biguint::test_convert_i128,
biguint::test_convert_u64,
biguint::test_convert_u128,
biguint::test_convert_f32,
biguint::test_convert_f64,
biguint::test_convert_to_bigint,
biguint::test_convert_from_uint,
biguint::test_add,
biguint::test_sub,
biguint::test_mul,
biguint::test_div_rem,
biguint::test_checked_add,
biguint::test_checked_sub,
biguint::test_checked_mul,
biguint::test_mul_overflow,
biguint::test_checked_div,
biguint::test_gcd,
biguint::test_lcm,
biguint::test_is_even,
biguint::test_to_str_radix,
biguint::test_from_and_to_radix,
biguint::test_from_str_radix,
biguint::test_all_str_radix,
biguint::test_lower_hex,
biguint::test_upper_hex,
biguint::test_binary,
biguint::test_octal,
biguint::test_display,
biguint::test_factor,
biguint::test_bits,
biguint::test_iter_sum,
biguint::test_iter_product,
biguint::test_iter_sum_generic,
biguint::test_iter_product_generic,
biguint::test_pow,
biguint_scalar::test_scalar_add,
biguint_scalar::test_scalar_sub,
biguint_scalar::test_scalar_mul,
biguint_scalar::test_scalar_div_rem,
modpow::biguint::test_modpow_single,
modpow::biguint::test_modpow_big,
modpow::bigint::test_modpow,
modpow::bigint::test_modpow_big,
modpow::bigint::test_modpow_regressions,
rand_tests::biguint::test_rand,
rand_tests::biguint::test_rand_bits,
rand_tests::biguint::test_rand_range,
rand_tests::biguint::test_rand_uniform,
//rand_tests::biguint::test_chacha_value_stability,
rand_tests::biguint::test_isaac_value_stability,
rand_tests::biguint::test_xorshift_value_stability,
|| should_panic!(rand_tests::biguint::test_zero_rand_range()),
|| should_panic!(rand_tests::biguint::test_negative_rand_range()),
rand_tests::bigint::test_rand,
rand_tests::bigint::test_rand_bits,
rand_tests::bigint::test_rand_range,
rand_tests::bigint::test_rand_uniform,
//rand_tests::bigint::test_chacha_value_stability,
rand_tests::bigint::test_isaac_value_stability,
rand_tests::bigint::test_xorshift_value_stability,
|| should_panic!(rand_tests::bigint::test_zero_rand_range()),
|| should_panic!(rand_tests::bigint::test_negative_rand_range()),
rand_tests::prime::test_prime_small,
rand_tests::prime::test_gen_prime_1024,
roots::biguint::test_sqrt,
roots::biguint::test_cbrt,
roots::biguint::test_nth_root,
|| should_panic!(roots::biguint::test_nth_root_n_is_zero()),
roots::biguint::test_nth_root_big,
roots::biguint::test_nth_root_googol,
roots::biguint::test_nth_root_twos,
roots::biguint::test_roots_rand,
roots::biguint::test_roots_rand1,
roots::bigint::test_nth_root,
|| should_panic!(roots::bigint::test_nth_root_x_neg_n_even()),
|| should_panic!(roots::bigint::test_sqrt_x_neg()),
roots::bigint::test_cbrt,
serde::biguint_zero,
serde::bigint_zero,
serde::biguint_one,
serde::bigint_one,
serde::bigint_negone,
serde::biguint_factorial_100,
serde::bigint_factorial_100,
torture::test_mul_divide_torture,
);
    sgx_status_t::SGX_SUCCESS
}
