#![crate_name="testlib_rust"]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[doc(inline)]

use std::{mem, ffi::CStr};

#[path = "bindings.rs"] mod CTestLib;

#[derive(Debug)]
pub enum TestLibPattern {
    pattern_ab = CTestLib::_TESTLIB_PATTERN_PATTERN_AB as isize,
    pattern_xy = CTestLib::_TESTLIB_PATTERN_PATTERN_XY as isize,
    pattern_tp = CTestLib::_TESTLIB_PATTERN_PATTERN_TP as isize,
}

#[derive(Debug)]
/// TestLibInfo struct
pub struct TestLibInfo {
    pub name: String,
    pub cameraId: i32,
    pub maxHeight: i32,
    pub maxWidth: i32,

    pub isColorCam: bool,
    pub bayerPattern: Option<TestLibPattern>
}

impl TestLibInfo {
    /// Returns info from TestLib
    /// 
    /// # Examples
    /// 
    /// ```
    /// use testlib_rust::TestLibInfo;
    /// let info = TestLibInfo::getInfo();
    /// ```
    pub fn getInfo() -> Option<TestLibInfo> {
        let mut info_prv: CTestLib::_TESTLIB_INFO = unsafe {mem::zeroed()};
        let result = unsafe {CTestLib::testlib_getinfo(&mut info_prv)};
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
                        CTestLib::_TESTLIB_PATTERN_PATTERN_AB => Some(TestLibPattern::pattern_ab),
                        CTestLib::_TESTLIB_PATTERN_PATTERN_XY => Some(TestLibPattern::pattern_xy),
                        CTestLib::_TESTLIB_PATTERN_PATTERN_TP => Some(TestLibPattern::pattern_tp),
                        CTestLib::_TESTLIB_PATTERN_PATTERN_NONE => None,
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
            Some(_x) => {
                // println!("{:?}", x.name.as_bytes());
                true
            }
        });
    }
}
