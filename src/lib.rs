//! # 🐎 daachorse: Double-Array Aho-Corasick
//!
//! A fast implementation of the Aho-Corasick algorithm
//! using the compact double-array data structure.
//!
//! ## Overview
//!
//! Daachorse is a crate for fast multiple pattern matching using the
//! [Aho-Corasick algorithm](https://dl.acm.org/doi/10.1145/360825.360855), running in linear time
//! over the length of the input text. This crate uses the
//! [compact double-array data structure](https://doi.org/10.1016/j.ipm.2006.04.004) for
//! implementing the pattern match automaton for time and memory efficiency. The data structure not
//! only supports constant-time state-to-state traversal but also represents each state in the
//! space of only 12 bytes.
//!
//! The main technical ideas behind this library appear in the following paper:
//!
//! > Shunsuke Kanda, Koichi Akabe, and Yusuke Oda.
//! > [Engineering faster double-array Aho-Corasick automata](https://doi.org/10.1002/spe.3190).
//! > *Software: Practice and Experience (SPE)*
//! > ([arXiv](https://arxiv.org/abs/2207.13870))
//!
//! ## Example: Finding overlapped occurrences
//!
//! To search for all occurrences of registered patterns that allow for positional overlap in the
//! input text, use [`DoubleArrayAhoCorasick::find_overlapping_iter()`].
//!
//! When you use [`DoubleArrayAhoCorasick::new()`] for construction, the library assigns a unique
//! identifier to each pattern in the input order. The match result has the byte positions of the
//! occurrence and its identifier.
//!
//! ```
//! use daachorse::DoubleArrayAhoCorasick;
//!
//! let patterns = vec!["bcd", "ab", "a"];
//! let pma = DoubleArrayAhoCorasick::new(patterns).unwrap();
//!
//! let mut it = pma.find_overlapping_iter("abcd");
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 1, 2), (m.start(), m.end(), m.value()));
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 2, 1), (m.start(), m.end(), m.value()));
//!
//! let m = it.next().unwrap();
//! assert_eq!((1, 4, 0), (m.start(), m.end(), m.value()));
//!
//! assert_eq!(None, it.next());
//! ```
//!
//! ## Example: Finding non-overlapped occurrences with standard matching
//!
//! If you do not want to allow positional overlap, use [`DoubleArrayAhoCorasick::find_iter()`]
//! instead.
//!
//! This function performs the search on the Aho-Corasick automaton and reports patterns first
//! found in each iteration.
//!
//! ```
//! use daachorse::DoubleArrayAhoCorasick;
//!
//! let patterns = vec!["bcd", "ab", "a"];
//! let pma = DoubleArrayAhoCorasick::new(patterns).unwrap();
//!
//! let mut it = pma.find_iter("abcd");
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 1, 2), (m.start(), m.end(), m.value()));
//!
//! let m = it.next().unwrap();
//! assert_eq!((1, 4, 0), (m.start(), m.end(), m.value()));
//!
//! assert_eq!(None, it.next());
//! ```
//!
//! ## Example: Finding non-overlapped occurrences with longest matching
//!
//! If you want to search for the longest pattern without positional overlap in each iteration, use
//! [`DoubleArrayAhoCorasick::leftmost_find_iter()`] with specifying
//! [`MatchKind::LeftmostLongest`] in the construction.
//!
//! ```
//! use daachorse::{DoubleArrayAhoCorasickBuilder, MatchKind};
//!
//! let patterns = vec!["ab", "a", "abcd"];
//! let pma = DoubleArrayAhoCorasickBuilder::new()
//!     .match_kind(MatchKind::LeftmostLongest)
//!     .build(&patterns)
//!     .unwrap();
//!
//! let mut it = pma.leftmost_find_iter("abcd");
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 4, 2), (m.start(), m.end(), m.value()));
//!
//! assert_eq!(None, it.next());
//! ```
//!
//! ## Example: Finding non-overlapped occurrences with leftmost-first matching
//!
//! If you want to find the earliest registered pattern among ones starting from the search
//! position, use [`DoubleArrayAhoCorasick::leftmost_find_iter()`] with specifying
//! [`MatchKind::LeftmostFirst`].
//!
//! This semantic is the so-called *leftmost first match*, a tricky search option supported in the
//! [aho-corasick](https://github.com/BurntSushi/aho-corasick) crate. For example, in the
//! following code, `ab` is reported because it is the earliest registered one.
//!
//! ```
//! use daachorse::{DoubleArrayAhoCorasickBuilder, MatchKind};
//!
//! let patterns = vec!["ab", "a", "abcd"];
//! let pma = DoubleArrayAhoCorasickBuilder::new()
//!     .match_kind(MatchKind::LeftmostFirst)
//!     .build(&patterns)
//!     .unwrap();
//!
//! let mut it = pma.leftmost_find_iter("abcd");
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 2, 0), (m.start(), m.end(), m.value()));
//!
//! assert_eq!(None, it.next());
//! ```
//!
//! ## Example: Associating arbitrary values with patterns
//!
//! To build the automaton from pairs of a pattern and user-defined value, instead of assigning
//! identifiers automatically, use [`DoubleArrayAhoCorasick::with_values()`].
//!
//! ```
//! use daachorse::DoubleArrayAhoCorasick;
//!
//! let patvals = vec![("bcd", 0), ("ab", 10), ("a", 20)];
//! let pma = DoubleArrayAhoCorasick::with_values(patvals).unwrap();
//!
//! let mut it = pma.find_overlapping_iter("abcd");
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 1, 20), (m.start(), m.end(), m.value()));
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 2, 10), (m.start(), m.end(), m.value()));
//!
//! let m = it.next().unwrap();
//! assert_eq!((1, 4, 0), (m.start(), m.end(), m.value()));
//!
//! assert_eq!(None, it.next());
//! ```
//!
//! ## Example: Building faster automaton on multibyte characters
//!
//! To build a faster automaton on multibyte characters, use [`CharwiseDoubleArrayAhoCorasick`]
//! instead.
//!
//! The standard version [`DoubleArrayAhoCorasick`] handles strings as UTF-8 sequences and defines
//! transition labels using byte values. On the other hand, [`CharwiseDoubleArrayAhoCorasick`] uses
//! Unicode code point values, reducing the number of transitions and faster matching.
//!
//! ```
//! use daachorse::CharwiseDoubleArrayAhoCorasick;
//!
//! let patterns = vec!["全世界", "世界", "に"];
//! let pma = CharwiseDoubleArrayAhoCorasick::new(patterns).unwrap();
//!
//! let mut it = pma.find_iter("全世界中に");
//!
//! let m = it.next().unwrap();
//! assert_eq!((0, 9, 0), (m.start(), m.end(), m.value()));
//!
//! let m = it.next().unwrap();
//! assert_eq!((12, 15, 2), (m.start(), m.end(), m.value()));
//!
//! assert_eq!(None, it.next());
//! ```

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

#[cfg(not(feature = "alloc"))]
compile_error!("`alloc` feature is currently required to build this crate");

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("`target_pointer_width` must be 32 or 64");

#[macro_use]
extern crate alloc;

mod build_helper;
pub mod bytewise;
pub mod charwise;
pub mod errors;
mod intpack;
mod nfa_builder;
mod serializer;
mod utils;

use core::num::NonZeroU32;

use alloc::vec::Vec;

use build_helper::BuildHelper;
pub use bytewise::{DoubleArrayAhoCorasick, DoubleArrayAhoCorasickBuilder};
pub use charwise::{CharwiseDoubleArrayAhoCorasick, CharwiseDoubleArrayAhoCorasickBuilder};
pub use serializer::Serializable;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde"))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "V: serde::Serialize",
        deserialize = "V: serde::Deserialize<'de>",
    ))
)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
struct Output<V> {
    value: V,
    length: u32,
    #[cfg_attr(
        feature = "serde",
        serde(with = "crate::serializer::nzu32_option_serde")
    )]
    parent: Option<NonZeroU32>,
}

impl<V> Output<V>
where
    V: Copy,
{
    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    pub fn new(value: V, length: u32, parent: Option<NonZeroU32>) -> Self {
        Self {
            value,
            length,
            parent,
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    pub fn value(self) -> V {
        self.value
    }

    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    pub fn length(self) -> u32 {
        self.length
    }

    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    pub fn parent(self) -> Option<NonZeroU32> {
        self.parent
    }
}

impl<V> Serializable for Output<V>
where
    V: Serializable,
{
    #[inline(always)]
    fn serialize_to_vec(&self, dst: &mut Vec<u8>) {
        self.value.serialize_to_vec(dst);
        self.length.serialize_to_vec(dst);
        self.parent.serialize_to_vec(dst);
    }

    #[inline(always)]
    fn deserialize_from_slice(src: &[u8]) -> (Self, &[u8]) {
        let (value, src) = V::deserialize_from_slice(src);
        let (length, src) = u32::deserialize_from_slice(src);
        let (parent, src) = Option::<NonZeroU32>::deserialize_from_slice(src);
        (
            Self {
                value,
                length,
                parent,
            },
            src,
        )
    }

    #[inline(always)]
    fn serialized_bytes() -> usize {
        V::serialized_bytes() + u32::serialized_bytes() + Option::<NonZeroU32>::serialized_bytes()
    }
}

/// Match result.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde"))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "V: serde::Serialize",
        deserialize = "V: serde::Deserialize<'de>",
    ))
)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct Match<V> {
    length: usize,
    end: usize,
    value: V,
}

impl<V> Match<V>
where
    V: Copy,
{
    /// Starting position of the match.
    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    #[must_use]
    pub fn start(&self) -> usize {
        self.end - self.length
    }

    /// Ending position of the match.
    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    #[must_use]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Value associated with the pattern.
    #[allow(clippy::missing_const_for_fn)]
    #[inline(always)]
    #[must_use]
    pub fn value(&self) -> V {
        self.value
    }
}

/// A search option of the Aho-Corasick automaton
/// specified in [`DoubleArrayAhoCorasickBuilder::match_kind`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde"))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[repr(u8)]
pub enum MatchKind {
    /// The standard match semantics, which enables
    /// [`find_iter()`](DoubleArrayAhoCorasick::find_iter()),\
    /// [`find_overlapping_iter()`](DoubleArrayAhoCorasick::find_overlapping_iter()), and
    /// [`find_overlapping_no_suffix_iter()`](DoubleArrayAhoCorasick::find_overlapping_no_suffix_iter()).
    /// Patterns are reported in the order that follows the normal behaviour of the Aho-Corasick
    /// algorithm.
    Standard = 0,

    /// The leftmost-longest match semantics, which enables
    /// [`leftmost_find_iter()`](DoubleArrayAhoCorasick::leftmost_find_iter()).
    /// When multiple patterns are started from the same positions, the longest pattern will be
    /// reported. For example, when matching patterns `ab|a|abcd` over `abcd`, `abcd` will be
    /// reported.
    LeftmostLongest = 1,

    /// The leftmost-first match semantics, which enables
    /// [`leftmost_find_iter()`](DoubleArrayAhoCorasick::leftmost_find_iter()).
    /// When multiple patterns are started from the same positions, the pattern that is registered
    /// earlier will be reported. For example, when matching patterns `ab|a|abcd` over `abcd`,
    /// `ab` will be reported.
    LeftmostFirst = 2,
}

impl MatchKind {
    fn is_standard(self) -> bool {
        self == Self::Standard
    }

    fn is_leftmost(self) -> bool {
        self == Self::LeftmostFirst || self == Self::LeftmostLongest
    }

    pub(crate) fn is_leftmost_first(self) -> bool {
        self == Self::LeftmostFirst
    }
}

impl From<u8> for MatchKind {
    fn from(src: u8) -> Self {
        match src {
            1 => Self::LeftmostLongest,
            2 => Self::LeftmostFirst,
            _ => Self::Standard,
        }
    }
}

impl From<MatchKind> for u8 {
    fn from(src: MatchKind) -> Self {
        match src {
            MatchKind::Standard => 0,
            MatchKind::LeftmostLongest => 1,
            MatchKind::LeftmostFirst => 2,
        }
    }
}

impl Serializable for MatchKind {
    #[inline(always)]
    fn serialize_to_vec(&self, dst: &mut Vec<u8>) {
        dst.push(u8::from(*self));
    }

    #[inline(always)]
    fn deserialize_from_slice(src: &[u8]) -> (Self, &[u8]) {
        (Self::from(src[0]), &src[1..])
    }

    #[inline(always)]
    fn serialized_bytes() -> usize {
        1
    }
}

/// Empty value type.
///
/// This is a unit struct with no field. This type can be used to reduce memory consumption when
/// the value corresponding to each pattern is not used.
///
/// # Example
///
/// ```
/// use daachorse::{DoubleArrayAhoCorasick, Empty};
///
/// let patterns = vec!["bcd", "ab", "a"];
/// let pma = DoubleArrayAhoCorasick::<Empty>::new(patterns).unwrap();
///
/// let mut it = pma.find_overlapping_iter("abcd");
///
/// let m = it.next().unwrap();
/// assert_eq!((0, 1), (m.start(), m.end()));
/// ```
#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde"))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct Empty;

impl From<usize> for Empty {
    fn from(_value: usize) -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_output() {
        let x = Output {
            value: 42u32,
            length: 57,
            parent: NonZeroU32::new(13),
        };
        let mut data = vec![];
        x.serialize_to_vec(&mut data);
        assert_eq!(data.len(), Output::<u32>::serialized_bytes());
        let (y, rest) = Output::deserialize_from_slice(&data);
        assert!(rest.is_empty());
        assert_eq!(x, y);
    }

    #[test]
    fn test_serialize_match_kind() {
        let x = MatchKind::LeftmostLongest;
        let mut data = vec![];
        x.serialize_to_vec(&mut data);
        assert_eq!(data.len(), MatchKind::serialized_bytes());
        let (y, rest) = MatchKind::deserialize_from_slice(&data);
        assert!(rest.is_empty());
        assert_eq!(x, y);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_output() {
        let x = Output {
            value: 42u32,
            length: 57,
            parent: NonZeroU32::new(13),
        };
        let serialized = serde_json::to_string(&x).unwrap();
        let y: Output<u32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "bitcode")]
    #[test]
    fn test_bitcode_output() {
        let x = Output {
            value: 42u32,
            length: 57,
            parent: NonZeroU32::new(13),
        };
        let encoded = bitcode::encode(&x);
        let y = bitcode::decode::<Output<u32>>(&encoded).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_match() {
        let x = Match {
            length: 4,
            end: 10,
            value: 42u32,
        };
        let serialized = serde_json::to_string(&x).unwrap();
        let y: Match<u32> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "bitcode")]
    #[test]
    fn test_bitcode_match() {
        let x = Match {
            length: 4,
            end: 10,
            value: 42u32,
        };
        let encoded = bitcode::encode(&x);
        let y = bitcode::decode::<Match<u32>>(&encoded).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_match_kind() {
        let x = MatchKind::LeftmostLongest;
        let serialized = serde_json::to_string(&x).unwrap();
        let y: MatchKind = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "bitcode")]
    #[test]
    fn test_bitcode_match_kind() {
        let x = MatchKind::LeftmostLongest;
        let encoded = bitcode::encode(&x);
        let y = bitcode::decode::<MatchKind>(&encoded).unwrap();
        assert_eq!(x, y);
    }
}
