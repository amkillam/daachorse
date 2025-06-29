use alloc::vec::Vec;

use crate::serializer::Serializable;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde"))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct U24(u32);

impl U24 {
    #[inline(always)]
    pub const fn get(self) -> u32 {
        self.0
    }

    pub const MAX: u32 = 0x00ff_ffff;
}

impl TryFrom<u32> for U24 {
    type Error = &'static str;

    #[inline(always)]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::MAX {
            Ok(Self(value))
        } else {
            Err("value must be smaller than 2^24")
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde"))]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
pub struct U24nU8(u32);

impl U24nU8 {
    #[inline(always)]
    pub const fn a(self) -> U24 {
        U24(self.0 >> 8)
    }

    #[inline(always)]
    pub fn b(self) -> u8 {
        u8::try_from(self.0 & u32::from(u8::MAX)).unwrap()
    }

    #[inline(always)]
    pub fn set_a(&mut self, a: U24) {
        self.0 = (a.get() << 8) | u32::from(self.b());
    }

    #[inline(always)]
    pub fn set_b(&mut self, b: u8) {
        self.0 = (self.a().get() << 8) | u32::from(b);
    }
}

impl Serializable for U24nU8 {
    #[inline(always)]
    fn serialize_to_vec(&self, dst: &mut Vec<u8>) {
        self.0.serialize_to_vec(dst);
    }

    #[inline(always)]
    fn deserialize_from_slice(src: &[u8]) -> (Self, &[u8]) {
        let (x, src) = u32::deserialize_from_slice(src);
        (Self(x), src)
    }

    #[inline(always)]
    fn serialized_bytes() -> usize {
        u32::serialized_bytes()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(any(feature = "serde", feature = "bitcode"))]
    use super::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_u24() {
        let x = U24(42);
        let serialized = serde_json::to_string(&x).unwrap();
        let y: U24 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "bitcode")]
    #[test]
    fn test_bitcode_u24() {
        let x = U24(42);
        let encoded = bitcode::encode(&x);
        let y = bitcode::decode::<U24>(&encoded).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_u24nu8() {
        let x = U24nU8(0x1234_5678);
        let serialized = serde_json::to_string(&x).unwrap();
        let y: U24nU8 = serde_json::from_str(&serialized).unwrap();
        assert_eq!(x, y);
    }

    #[cfg(feature = "bitcode")]
    #[test]
    fn test_bitcode_u24nu8() {
        let x = U24nU8(0x1234_5678);
        let encoded = bitcode::encode(&x);
        let y = bitcode::decode::<U24nU8>(&encoded).unwrap();
        assert_eq!(x, y);
    }
}
