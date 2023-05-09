#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{mem, ffi::CStr};

include!("./bindings.rs");

#[derive(Debug)]
pub enum TestLibPattern {
    pattern_ab = _TESTLIB_PATTERN_PATTERN_AB as isize,
    pattern_xy = _TESTLIB_PATTERN_PATTERN_XY as isize,
    pattern_tp = _TESTLIB_PATTERN_PATTERN_TP as isize,
}

#[derive(Debug)]
pub struct TestLibInfo {
    pub name: String,
    pub cameraId: i32,
    pub maxHeight: i32,
    pub maxWidth: i32,

    pub isColorCam: bool,
    pub bayerPattern: Option<TestLibPattern>
}

impl TestLibInfo {
    pub fn getInfo() -> Option<TestLibInfo> {
        let mut info_prv: _TESTLIB_INFO = unsafe {mem::zeroed()};
        let result = unsafe {testlib_getinfo(&mut info_prv)};
        match result {
            0 => {
                let nstr = unsafe {CStr::from_ptr(info_prv.Name.as_ptr()).to_str().unwrap_or("")};
                let nstr = String::from(nstr);
                
                let info: TestLibInfo = TestLibInfo {
                    name: nstr,
                    cameraId: info_prv.CameraID,
                    maxHeight: info_prv.MaxHeight as i32,
                    maxWidth: info_prv.MaxWidth as i32,
                    isColorCam: info_prv.IsColorCam != 0,
                    bayerPattern: match info_prv.BayerPattern {
                        _TESTLIB_PATTERN_PATTERN_AB => Some(TestLibPattern::pattern_ab),
                        _TESTLIB_PATTERN_PATTERN_XY => Some(TestLibPattern::pattern_xy),
                        _TESTLIB_PATTERN_PATTERN_TP => Some(TestLibPattern::pattern_tp),
                        _TESTLIB_PATTERN_PATTERN_NONE => None,
                        _ => None
                    }
                };
                Some(info)
            },
            _ => None
        }
    }
}

#[cfg(test)]

mod tests {
    use crate::TestLibInfo;

    #[test]
    fn test_testlib_getinfo() {
        let var = TestLibInfo::getInfo();
        println!("{:?}", var);
        assert!(match var {
            None => false,
            Some(x) => {
                println!("{:?}", x.name.as_bytes());
                true
            }
        });
    }
}
