//! MIPS register descriptions.

use crate::isa::registers::{RegBank, RegClass, RegClassData, RegInfo, RegUnit};

include!(concat!(env!("OUT_DIR"), "/registers-mips.rs"));

#[cfg(test)]
mod tests {
    use super::{FPR, GPR, HILO, INFO};
    use crate::isa::RegUnit;
    use std::string::{String, ToString};

    #[test]
    fn unit_encodings() {
        assert_eq!(INFO.parse_regunit("zero"), Some(0));
        assert_eq!(INFO.parse_regunit("ra"), Some(31));
        assert_eq!(INFO.parse_regunit("f0"), Some(32));
        assert_eq!(INFO.parse_regunit("f31"), Some(63));
        assert_eq!(INFO.parse_regunit("hi"), Some(64));
        assert_eq!(INFO.parse_regunit("lo"), Some(65));

        assert_eq!(INFO.parse_regunit("t10"), None);
        assert_eq!(INFO.parse_regunit("f32"), None);
    }

    #[test]
    fn unit_names() {
        fn uname(ru: RegUnit) -> String {
            INFO.display_regunit(ru).to_string()
        }

        assert_eq!(uname(0), "%zero");
        assert_eq!(uname(1), "%at");
        assert_eq!(uname(31), "%ra");
        assert_eq!(uname(32), "%f0");
        assert_eq!(uname(33), "%f1");
        assert_eq!(uname(63), "%f31");
        assert_eq!(uname(64), "%hi");
        assert_eq!(uname(65), "%lo");
        assert_eq!(uname(66), "%INVALID66");
    }

    #[test]
    fn classes() {
        assert!(GPR.contains(GPR.unit(0)));
        assert!(GPR.contains(GPR.unit(31)));
        assert!(!FPR.contains(GPR.unit(0)));
        assert!(!FPR.contains(GPR.unit(31)));
        assert!(!GPR.contains(FPR.unit(0)));
        assert!(!GPR.contains(FPR.unit(31)));
        assert!(FPR.contains(FPR.unit(0)));
        assert!(FPR.contains(FPR.unit(31)));
        assert!(HILO.contains(HILO.unit(0)));
        assert!(HILO.contains(HILO.unit(1)));
        assert!(!HILO.contains(HILO.unit(2)));
    }
}
