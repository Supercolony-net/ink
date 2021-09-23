// Copyright 2018-2021 Parity Technologies (UK) Ltd.
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

#![no_std]
use ink_env::{
    DefaultEnvironment,
    Environment,
};

/// The ECDSA compressed public key.
#[derive(Debug, Copy, Clone)]
pub struct ECDSAPublicKey(pub [u8; 33]);

impl Default for ECDSAPublicKey {
    fn default() -> Self {
        // Default is not implemented for [u8; 33], so we can't derive it for ECDSAPublicKey
        // But clippy thinks that it is possible. So it is workaround for clippy.
        let empty = [0; 33];
        Self { 0: empty }
    }
}

impl core::ops::Deref for ECDSAPublicKey {
    type Target = [u8; 33];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for ECDSAPublicKey {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The address of an Ethereum account.
#[derive(Debug, Default, Copy, Clone)]
pub struct EthereumAddress(pub [u8; 20]);

impl core::ops::Deref for EthereumAddress {
    type Target = [u8; 20];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for EthereumAddress {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ECDSAPublicKey {
    /// Returns Ethereum address from the ECDSA compressed public key.
    ///
    /// # Example
    ///
    /// ```
    /// use ink_eth_compatibility::ECDSAPublicKey;
    /// let pub_key: ECDSAPublicKey = ECDSAPublicKey {
    ///     0: [
    ///         2, 121, 190, 102, 126, 249, 220, 187, 172, 85, 160,  98, 149, 206, 135, 11,
    ///         7,   2, 155, 252, 219,  45, 206,  40, 217, 89, 242, 129,  91,  22, 248, 23,
    ///         152,
    ///     ]
    /// };
    ///
    /// const EXPECTED_ETH_ADDRESS: [u8; 20] = [
    ///     126, 95, 69, 82, 9, 26, 105, 18, 93, 93, 252, 183, 184, 194, 101, 144, 41, 57, 91, 223
    /// ];
    ///
    /// assert_eq!(*pub_key.to_eth_address(), EXPECTED_ETH_ADDRESS);
    /// ```
    pub fn to_eth_address(&self) -> EthereumAddress {
        use ink_env::hash;
        use secp256k1::PublicKey;

        // Transform compressed public key into uncompressed.
        let pub_key = PublicKey::parse_compressed(&self.0)
            .expect("Unable to parse the compressed ECDSA public key");
        let uncompressed = pub_key.serialize();

        // Hash the uncompressed public key by Keccak256 algorithm.
        let mut hash = <hash::Keccak256 as hash::HashOutput>::Type::default();
        // The first byte indicates that the public key is uncompressed.
        // Let's skip it for hashing the public key directly.
        ink_env::hash_bytes::<hash::Keccak256>(&uncompressed[1..], &mut hash);

        // Take the last 20 bytes as an Address
        let mut result = EthereumAddress::default();
        result.copy_from_slice(&hash[12..]);

        result
    }

    /// Returns the default Substrate's `AccountId` from the ECDSA compressed public key.
    /// It hashes the compressed public key with the blake2b256 algorithm like in substrate.
    ///
    /// # Example
    ///
    /// ```
    /// use ink_eth_compatibility::ECDSAPublicKey;
    /// let pub_key: ECDSAPublicKey = ECDSAPublicKey {
    ///     0: [
    ///         2, 121, 190, 102, 126, 249, 220, 187, 172, 85, 160,  98, 149, 206, 135, 11,
    ///         7,   2, 155, 252, 219,  45, 206,  40, 217, 89, 242, 129,  91,  22, 248, 23,
    ///         152,
    ///     ]
    /// };
    ///
    /// const EXPECTED_ACCOUNT_ID: [u8; 32] = [
    ///     41, 117, 241, 210, 139, 146, 182, 232,  68, 153, 184, 59,   7, 151, 239, 82,
    ///     53,  85,  62, 235, 126, 218, 160, 206, 162,  67, 193, 18, 140,  47, 231, 55,
    /// ];
    ///
    /// assert_eq!(pub_key.to_default_account_id(), EXPECTED_ACCOUNT_ID.into());
    pub fn to_default_account_id(
        &self,
    ) -> <DefaultEnvironment as Environment>::AccountId {
        use ink_env::hash;

        let mut output = <hash::Blake2x256 as hash::HashOutput>::Type::default();
        ink_env::hash_bytes::<hash::Blake2x256>(&self.0[..], &mut output);

        output.into()
    }
}
