//! Common methods for xor filters.

pub mod fuse;
pub mod xor;

use crate::murmur3;

/// A set of hashes indexing three blocks.
pub struct HashSet {
    /// Key hash
    pub hash: u64,
    /// Indexing hashes h_0, h_1, h_2 created with `hash`.
    pub hset: [usize; 3],
}

/// The hash of a key and the index of that key in the construction array H.
#[derive(Default, Copy, Clone)]
pub struct KeyIndex {
    pub hash: u64,
    pub index: usize,
}

/// A set in the construction array H. Elements are encoded via xor with the mask.
#[derive(Default, Clone)]
pub struct HSet {
    pub count: u32,
    pub mask: u64,
}

/// Applies a finalization mix to a randomly-seeded key, resulting in an avalanched hash. This
/// helps avoid high false-positive ratios (see Section 4 in the paper).
#[inline]
const fn mix(key: u64, seed: u64) -> u64 {
    murmur3::mix64(key.overflowing_add(seed).0)
}

/// Computes a fingerprint.
#[doc(hidden)]
#[macro_export]
macro_rules! fingerprint(
    ($hash:expr) => {
        $hash ^ ($hash >> 32)
    };
);

/// Rotate left
#[doc(hidden)]
#[macro_export]
macro_rules! rotl64(
    ($n:expr, by $c:expr) => {
        ($n << ($c & 63)) | ($n >> ((-$c) & 63))
    };
);

/// [A fast alternative to the modulo reduction](http://lemire.me/blog/2016/06/27/a-fast-alternative-to-the-modulo-reduction/)
#[doc(hidden)]
#[macro_export]
macro_rules! reduce(
    ($hash:ident on interval $n:expr) => {
        (($hash as u64 * $n as u64) >> 32) as usize
    };
);

/// Creates a block of sets, each set being of type T.
#[doc(hidden)]
#[macro_export]
macro_rules! make_block(
    (with $size:ident sets) => {
        {
            let sets_block = vec![Default::default(); $size];
            sets_block.into_boxed_slice()
        }
    };
);

/// Creates a block of sets, each set being of type T.
#[doc(hidden)]
#[macro_export]
macro_rules! try_enqueue(
    (block $H_block:expr, set $idx:ident; queue block $Q_block:expr, with size $qblock_size:expr) => {
        if $H_block[$idx].count == 1 {
            $Q_block[$qblock_size].index = $idx;
            // If there is only one key, the mask contains it wholly.
            $Q_block[$qblock_size].hash = $H_block[$idx].mask;
            $qblock_size += 1;
        }
    };
);
