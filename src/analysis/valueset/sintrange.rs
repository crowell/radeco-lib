use super::{ValueSet, KnownBits, UIntMultiple, UIntRange, SIntRange};
use std::cmp::{min, max};
use std::ops::{BitAnd, BitOr};

impl ValueSet<u64> for SIntRange {
	fn contains(&self, value: u64) -> bool { (self.min <= (value as i64)) && ((value as i64) <= self.max) }
}

/// A value set that includes all i64 values between a minimum and a maximum
impl SIntRange {
	// TODO
	fn as_knownbits(&self) -> KnownBits { KnownBits {zerobits: 0, onebits: 0} }
	fn as_umultiple(&self) -> UIntMultiple { UIntMultiple {modulus: 0, residue: 0} }
	fn as_urange(&self) -> UIntRange { UIntRange {min: 0, max: 0} }
}

impl<'a, 'b> BitAnd<&'a SIntRange> for &'b SIntRange {
	type Output = SIntRange;

	fn bitand(self, rhs: &SIntRange) -> SIntRange {
		SIntRange {
			min: max(self.min, rhs.min),
			max: min(self.max, rhs.max)
		}
	}
}

impl<'a, 'b> BitOr<&'a SIntRange> for &'b SIntRange {
	type Output = SIntRange;

	fn bitor(self, rhs: &SIntRange) -> SIntRange {
		SIntRange {
			min: min(self.min, rhs.min),
			max: max(self.max, rhs.max)
		}
	}
}
