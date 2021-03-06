// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use super::stream::SBStream;
use sys;

/// A file specification that divides the path into a
/// directory and basename.
///
/// The string values of the paths are put into uniqued string pools
/// for fast comparisons and efficient memory usage.
pub struct SBFileSpec {
    /// The underlying raw `SBFileSpecRef`.
    pub raw: sys::SBFileSpecRef,
}

impl SBFileSpec {
    /// Construct a new `SBFileSpec`.
    pub fn wrap(raw: sys::SBFileSpecRef) -> SBFileSpec {
        SBFileSpec { raw: raw }
    }

    /// Construct a new `Some(SBFileSpec)` or `None`.
    pub fn maybe_wrap(raw: sys::SBFileSpecRef) -> Option<SBFileSpec> {
        if unsafe { sys::SBFileSpecIsValid(raw) != 0 } {
            Some(SBFileSpec { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBFileSpec` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBFileSpecIsValid(self.raw) != 0 }
    }

    /// Does this file exist?
    pub fn exists(&self) -> bool {
        unsafe { sys::SBFileSpecExists(self.raw) != 0 }
    }

    /// The path file name.
    pub fn filename(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBFileSpecGetFilename(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }

    /// The path directory name.
    pub fn directory(&self) -> &str {
        unsafe {
            match CStr::from_ptr(sys::SBFileSpecGetDirectory(self.raw)).to_str() {
                Ok(s) => s,
                _ => panic!("Invalid string?"),
            }
        }
    }
}

impl fmt::Debug for SBFileSpec {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBFileSpecGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBFileSpec {{ {} }}", stream.data())
    }
}

impl Drop for SBFileSpec {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBFileSpec(self.raw) };
    }
}
