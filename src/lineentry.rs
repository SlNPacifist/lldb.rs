// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use super::address::SBAddress;
use super::filespec::SBFileSpec;
use super::stream::SBStream;
use sys;

/// Specifies an association with a contiguous range of
/// instructions and a source file location.
pub struct SBLineEntry {
    /// The underlying raw `SBLineEntryRef`.
    pub raw: sys::SBLineEntryRef,
}

impl SBLineEntry {
    /// Construct a new `SBLineEntry`.
    pub fn wrap(raw: sys::SBLineEntryRef) -> SBLineEntry {
        SBLineEntry { raw: raw }
    }

    /// Construct a new `Some(SBLineEntry)` or `None`.
    pub fn maybe_wrap(raw: sys::SBLineEntryRef) -> Option<SBLineEntry> {
        if unsafe { sys::SBLineEntryIsValid(raw) != 0 } {
            Some(SBLineEntry { raw: raw })
        } else {
            None
        }
    }

    /// Check whether or not this is a valid `SBLineEntry` value.
    pub fn is_valid(&self) -> bool {
        unsafe { sys::SBLineEntryIsValid(self.raw) != 0 }
    }

    /// The start address for this line entry.
    pub fn start_address(&self) -> SBAddress {
        SBAddress::wrap(unsafe { sys::SBLineEntryGetStartAddress(self.raw) })
    }

    /// The end address for this line entry.
    pub fn end_address(&self) -> SBAddress {
        SBAddress::wrap(unsafe { sys::SBLineEntryGetEndAddress(self.raw) })
    }

    /// The file (`SBFileSpec`) for this line entry.
    pub fn filespec(&self) -> SBFileSpec {
        SBFileSpec::wrap(unsafe { sys::SBLineEntryGetFileSpec(self.raw) })
    }

    /// The 1-based line number for this line entry.
    ///
    /// A return value of `0` indicates that no line information is
    /// available.
    pub fn line(&self) -> u32 {
        unsafe { sys::SBLineEntryGetLine(self.raw) }
    }

    /// The 1-based column number for this line entry.
    ///
    /// A return value of `0` indicates that no column information is
    /// available.
    pub fn column(&self) -> u32 {
        unsafe { sys::SBLineEntryGetColumn(self.raw) }
    }
}

impl fmt::Debug for SBLineEntry {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let stream = SBStream::new();
        unsafe { sys::SBLineEntryGetDescription(self.raw, stream.raw) };
        write!(fmt, "SBLineEntry {{ {} }}", stream.data())
    }
}

impl Drop for SBLineEntry {
    fn drop(&mut self) {
        unsafe { sys::DisposeSBLineEntry(self.raw) };
    }
}
