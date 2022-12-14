#![cfg_attr(not(feature = "std"), no_std)]

/// An ID construct based on base58 encoding.
///
/// - `P`: the number of bytes of the id prefix, e.g. `usr = 3` for an id prefixed with `usr`
/// - `N`: the number of bytes of the full id, e.g. `P = 3` prefixed with `usr`, `E = 8` suffixed with a 8-byte timestamp, `N = 43`, id content is a 256-bit (32 bytes) hash
/// - `E`: the number of bytes of the id extension to store extra information, e.g. 8 for a timestamp
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct ID<const P: usize, const E: usize, const N: usize>([u8; N]);

impl<const P: usize, const E: usize, const N: usize> ID<P, E, N> {
    /// Create a new ID from a byte array.
    #[inline]
    pub const fn new(bytes: [u8; N]) -> Self {
        Self(bytes)
    }

    /// Get the prefix of the ID.
    #[inline]
    pub fn prefix(&self) -> &[u8] {
        &self.0[..P]
    }

    /// Get the extension of the ID.
    #[inline]
    pub fn suffix(&self) -> &[u8] {
        &self.0[N - E..]
    }

    /// Get the content of the ID.
    #[inline]
    pub fn content(&self) -> &[u8] {
        &self.0[P..N - E]
    }

    /// Get the byte representation of the ID.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl<const P: usize, const E: usize, const N: usize> From<[u8; N]> for ID<P, E, N> {
    #[inline]
    fn from(bytes: [u8; N]) -> Self {
        Self::new(bytes)
    }
}

impl<const P: usize, const E: usize, const N: usize> TryFrom<&[u8]> for ID<P, E, N> {
    type Error = ParseIDError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != N {
            return Err(ParseIDError {
                n: N,
                src: value.len(),
            });
        }
        Ok(value.try_into().map(Self::new).unwrap())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ParseIDError {
    n: usize,
    src: usize,
}

impl core::fmt::Display for ParseIDError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "buid: expected {} bytes, got {}", self.n, self.src)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseIDError {}
