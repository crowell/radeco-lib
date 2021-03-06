use super::{ValueSet, KnownBits, UIntMultiple, UIntRange, SIntRange};
use std::cmp::{min, max};
use std::ops::{BitAnd, BitOr};

impl ValueSet<u64> for UIntRange {
	fn contains(&self, value: u64) -> bool { (self.min <= value) && (value <= self.max) }
}

/// A value set that includes all u64 values between a minimum and a maximum
impl UIntRange {
	// TODO
	fn as_knownbits(&self) -> KnownBits { KnownBits {zerobits: 0, onebits: 0} }
	fn as_umultiple(&self) -> UIntMultiple { UIntMultiple {modulus: 0, residue: 0} }
	fn as_srange(&self) -> SIntRange { SIntRange {min: 0, max: 0} }
}

impl<'a, 'b> BitAnd<&'a UIntRange> for &'b UIntRange {
	type Output = UIntRange;

	fn bitand(self, rhs: &UIntRange) -> UIntRange {
		UIntRange {
			min: max(self.min, rhs.min),
			max: min(self.max, rhs.max)
		}
	}
}

impl<'a, 'b> BitOr<&'a UIntRange> for &'b UIntRange {
	type Output = UIntRange;

	fn bitor(self, rhs: &UIntRange) -> UIntRange {
		UIntRange {
			min: min(self.min, rhs.min),
			max: max(self.max, rhs.max)
		}
	}
}
