use crate::{num_bigint, tx_mock::big_int_to_i64, DebugApi};
use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Rem, Shl, Shr, Sub},
};
use multiversx_sc::{
    api::{BigIntApiImpl, HandleConstraints, HandleTypeInfo, ManagedBufferApiImpl},
    err_msg,
    types::heap::BoxedBytes,
};
use num_bigint::BigInt;
use num_traits::{pow, sign::Signed};
use std::convert::TryInto;

fn assert_positive(bi: &num_bigint::BigInt) {
    assert!(
        bi.sign() != num_bigint::Sign::Minus,
        "{}",
        err_msg::BIG_INT_BITWISE_OPERATION_NEGATIVE,
    );
}

macro_rules! binary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(
            &self,
            dest: Self::BigIntHandle,
            x: Self::BigIntHandle,
            y: Self::BigIntHandle,
        ) {
            let bi_x = self.bi_get(x);
            let bi_y = self.bi_get(y);
            let result = bi_x.$rust_op_name(bi_y);
            self.bi_overwrite(dest, result);
        }
    };
}

macro_rules! binary_bitwise_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(
            &self,
            dest: Self::BigIntHandle,
            x: Self::BigIntHandle,
            y: Self::BigIntHandle,
        ) {
            let bi_x = self.bi_get(x);
            assert_positive(&bi_x);
            let bi_y = self.bi_get(y);
            assert_positive(&bi_y);
            let result = bi_x.$rust_op_name(bi_y);
            self.bi_overwrite(dest, result);
        }
    };
}

macro_rules! unary_op_method {
    ($method_name:ident, $rust_op_name:ident) => {
        fn $method_name(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle) {
            let bi_x = self.bi_get(x);
            let result = bi_x.$rust_op_name();
            self.bi_overwrite(dest, result);
        }
    };
}

impl DebugApi {
    pub(crate) fn bi_new_from_big_int(
        &self,
        value: num_bigint::BigInt,
    ) -> <Self as HandleTypeInfo>::BigIntHandle {
        self.m_types_borrow_mut().bi_new_from_big_int(value).into()
    }

    pub(crate) fn bi_overwrite(
        &self,
        destination: <Self as HandleTypeInfo>::BigIntHandle,
        value: num_bigint::BigInt,
    ) {
        let mut managed_types = destination.context.m_types_borrow_mut();
        managed_types.bi_overwrite(destination.get_raw_handle_unchecked(), value)
    }

    pub(crate) fn bi_get(&self, handle: <Self as HandleTypeInfo>::BigIntHandle) -> BigInt {
        let managed_types = handle.context.m_types_borrow();
        managed_types.bi_get(handle.get_raw_handle_unchecked())
    }

    pub(crate) fn bi_get_unsigned_bytes(
        &self,
        handle: <Self as HandleTypeInfo>::BigIntHandle,
    ) -> BoxedBytes {
        handle
            .context
            .m_types_borrow()
            .bi_get_unsigned_bytes(handle.get_raw_handle_unchecked())
    }

    pub(crate) fn bi_get_signed_bytes(
        &self,
        handle: <Self as HandleTypeInfo>::BigIntHandle,
    ) -> BoxedBytes {
        handle
            .context
            .m_types_borrow()
            .bi_get_signed_bytes(handle.get_raw_handle_unchecked())
    }
}

impl BigIntApiImpl for DebugApi {
    #[allow(dead_code)]
    fn bi_new(&self, value: i64) -> Self::BigIntHandle {
        self.bi_new_from_big_int(num_bigint::BigInt::from(value))
    }

    fn bi_set_int64(&self, destination: Self::BigIntHandle, value: i64) {
        self.bi_overwrite(destination, num_bigint::BigInt::from(value))
    }

    fn bi_to_i64(&self, handle: Self::BigIntHandle) -> Option<i64> {
        let bi = self.bi_get(handle);
        big_int_to_i64(&bi)
    }

    binary_op_method! {bi_add, add}
    binary_op_method! {bi_sub, sub}
    binary_op_method! {bi_mul, mul}
    binary_op_method! {bi_t_div, div}
    binary_op_method! {bi_t_mod, rem}

    unary_op_method! {bi_abs, abs}
    unary_op_method! {bi_neg, neg}

    fn bi_sign(&self, x: Self::BigIntHandle) -> multiversx_sc::api::Sign {
        let bi = self.bi_get(x);
        match bi.sign() {
            num_bigint::Sign::Minus => multiversx_sc::api::Sign::Minus,
            num_bigint::Sign::NoSign => multiversx_sc::api::Sign::NoSign,
            num_bigint::Sign::Plus => multiversx_sc::api::Sign::Plus,
        }
    }

    fn bi_cmp(&self, x: Self::BigIntHandle, y: Self::BigIntHandle) -> Ordering {
        let bi_x = self.bi_get(x);
        let bi_y = self.bi_get(y);
        bi_x.cmp(&bi_y)
    }

    unary_op_method! {bi_sqrt, sqrt}

    fn bi_pow(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, y: Self::BigIntHandle) {
        let bi_x = self.bi_get(x);
        let bi_y = self.bi_get(y);
        let exp = big_int_to_i64(&bi_y).unwrap().try_into().unwrap();
        let result = pow(bi_x, exp);
        self.bi_overwrite(dest, result);
    }

    fn bi_log2(&self, x: Self::BigIntHandle) -> u32 {
        let bi_x = self.bi_get(x);
        bi_x.bits() as u32 - 1
    }

    binary_bitwise_op_method! {bi_and, bitand}
    binary_bitwise_op_method! {bi_or, bitor}
    binary_bitwise_op_method! {bi_xor, bitxor}

    fn bi_shr(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        let bi_x = self.bi_get(x);
        assert_positive(&bi_x);
        let result = bi_x.shr(bits);
        self.bi_overwrite(dest, result);
    }

    fn bi_shl(&self, dest: Self::BigIntHandle, x: Self::BigIntHandle, bits: usize) {
        let bi_x = self.bi_get(x);
        assert_positive(&bi_x);
        let result = bi_x.shl(bits);
        self.bi_overwrite(dest, result);
    }

    fn bi_to_string(&self, x: Self::BigIntHandle, str_handle: Self::ManagedBufferHandle) {
        let s = {
            let bi_x = self.bi_get(x);
            bi_x.to_string()
        };
        self.mb_overwrite(str_handle, s.as_bytes());
    }
}
