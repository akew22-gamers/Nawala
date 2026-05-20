// Minimal nomor_surat module for Task 4
// This module provides basic constants for nomor surat generation

/// Default pattern for nomor surat
pub const DEFAULT_PATTERN: &str = "{seq:4}/{kode}/{kode_desa}/{romawi:bulan}/{tahun}";

/// Roman numerals for months (1-12)
pub const ROMAWI: [&str; 13] = [
    "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII"
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_pattern() {
        assert!(!DEFAULT_PATTERN.is_empty());
    }

    #[test]
    fn test_romawi_array() {
        assert_eq!(ROMAWI.len(), 13);
        assert_eq!(ROMAWI[5], "V");
    }
}
