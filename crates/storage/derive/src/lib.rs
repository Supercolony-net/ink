// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Custom derive for `ink_storage` traits.
//!
//! This crate provides helpers to define your very own custom storage data
//! structures that work along the `ink_storage` data structures.

extern crate proc_macro;

mod atomic_guard;
mod storage_key_holder;
mod storage_layout;
mod storage_type;

#[cfg(test)]
mod tests;

use self::{
    atomic_guard::atomic_guard_derive,
    storage_key_holder::storage_key_holder_derive,
    storage_layout::storage_layout_derive,
    storage_type::{
        storage_type_derive,
        storage_type_derive2,
    },
};
synstructure::decl_derive!(
    [AtomicGuard] =>
    /// Derives `ink_storage`'s `AtomicGuard<true>` trait for the given `struct`, `enum` or `union`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_storage::traits::AtomicGuard;
    ///
    /// #[derive(AtomicGuard)]
    /// struct NamedFields {
    ///     a: u32,
    ///     b: [u32; 32],
    /// }
    ///
    /// let value: &dyn AtomicGuard<true> = &NamedFields {
    ///     a: 123,
    ///     b: [22; 32],
    /// };
    /// ```
    atomic_guard_derive
);
synstructure::decl_derive!(
    [StorageType] =>
    /// Derives `ink_storage`'s `StorageType` trait for the given `struct` or `enum`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scale::{Encode, Decode};
    /// use ink_primitives::Key;
    /// use ink_storage::traits::{
    ///     SpreadLayout,
    ///     PackedLayout,
    ///     push_packed_root,
    ///     pull_packed_root
    /// };
    ///
    /// # ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
    /// #[derive(Encode, Decode, SpreadLayout, PackedLayout)]
    /// struct NamedFields {
    ///     a: u32,
    ///     b: [u32; 32],
    /// }
    ///
    /// let mut value = NamedFields {
    ///     a: 123,
    ///     b: [22; 32],
    /// };
    ///
    /// push_packed_root(&value, &mut Key::from([0x42; 32]));
    /// let value2: NamedFields = pull_packed_root(&mut Key::from([0x42; 32]));
    /// assert_eq!(value.a, value2.a);
    /// # Ok(())
    /// # });
    /// ```
    storage_type_derive
);
synstructure::decl_derive!(
    [StorageType2] =>
    /// Derives `ink_storage`'s `StorageType2` trait for the given `struct` or `enum`.
    ///
    /// # Examples
    ///
    /// ```
    /// use scale::{Encode, Decode};
    /// use ink_primitives::Key;
    /// use ink_storage::traits::{
    ///     SpreadLayout,
    ///     PackedLayout,
    ///     push_packed_root,
    ///     pull_packed_root
    /// };
    ///
    /// # ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
    /// #[derive(Encode, Decode, SpreadLayout, PackedLayout)]
    /// struct NamedFields {
    ///     a: u32,
    ///     b: [u32; 32],
    /// }
    ///
    /// let mut value = NamedFields {
    ///     a: 123,
    ///     b: [22; 32],
    /// };
    ///
    /// push_packed_root(&value, &mut Key::from([0x42; 32]));
    /// let value2: NamedFields = pull_packed_root(&mut Key::from([0x42; 32]));
    /// assert_eq!(value.a, value2.a);
    /// # Ok(())
    /// # });
    /// ```
    storage_type_derive2
);
synstructure::decl_derive!(
    [StorageKeyHolder] =>
    /// Derives `ink_storage`'s `StorageKeyHolder` trait for the given `struct` or `enum`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_storage::traits::{
    ///     StorageKeyHolder,
    ///     ManualKey,
    ///     AutoKey,
    /// };
    ///
    /// #[derive(StorageKeyHolder)]
    /// struct NamedFields {
    ///     a: u32,
    ///     b: [u32; 32],
    /// }
    ///
    /// assert_eq!(<NameFields as StorageKeyHolder>::KEY, 0);
    ///
    /// #[derive(StorageKeyHolder)]
    /// struct NamedFieldsManualKey<KEY: StorageKeyHolder> {
    ///     a: u32,
    ///     b: [u32; 32],
    /// }
    ///
    /// assert_eq!(<NamedFieldsManualKey<()> as StorageKeyHolder>::KEY, 0);
    /// assert_eq!(<NamedFieldsManualKey<AutoKey> as StorageKeyHolder>::KEY, 0);
    /// assert_eq!(<NamedFieldsManualKey<ManualKey<123>> as StorageKeyHolder>::KEY, 123);
    /// ```
    storage_key_holder_derive
);
synstructure::decl_derive!(
    [StorageLayout] =>
    /// Derives `ink_storage`'s `StorageLayout` trait for the given `struct` or `enum`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ink_metadata::layout::Layout::Struct;
    /// use ink_storage::traits::StorageLayout;
    ///
    /// # ink_env::test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
    /// #[derive(StorageLayout)]
    /// struct NamedFields {
    ///     a: u32,
    ///     b: [u32; 32],
    /// }
    ///
    /// let key = 0x123;
    /// let mut value = NamedFields {
    ///     a: 123,
    ///     b: [22; 32],
    /// };
    ///
    /// if let Struct(layout) = <NamedFields as StorageLayout>::layout(&key) {
    ///     assert_eq!(*layout.fields()[0].name().unwrap(), "a");
    ///     assert_eq!(*layout.fields()[1].name().unwrap(), "b");
    /// }
    /// # Ok(())
    /// # });
    /// ```
    storage_layout_derive
);
