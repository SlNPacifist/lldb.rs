// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use super::target::SBTarget;
use sys;

/// Creates [`SBTarget`]s, provides access to them and manages
/// the overall debugging experience.
///
/// [`SBTarget`]: struct.SBTarget.html
#[derive(Debug)]
pub struct SBDebugger {
    /// The underlying raw `SBDebuggerRef`.
    pub raw_debugger: sys::SBDebuggerRef,
}

impl SBDebugger {
    /// Initialize LLDB.
    ///
    /// This should be called before LLDB functionality is used.
    pub fn initialize() {
        unsafe {
            sys::SBDebuggerInitialize();
        }
    }

    /// Tear down LLDB.
    ///
    /// This should be called once the application no longer needs
    /// to use LLDB functionality. Typically, this is called as the
    /// application exits.
    pub fn terminate() {
        unsafe {
            sys::SBDebuggerTerminate();
        }
    }

    /// Create a new instance of `SBDebugger`.
    ///
    /// If `source_init_files` is `true`, then `~/.lldbinit` will
    /// be processed.
    pub fn create(source_init_files: bool) -> SBDebugger {
        unsafe { SBDebugger { raw_debugger: sys::SBDebuggerCreate2(source_init_files as u8) } }
    }

    /// Get whether or not the debugger is in async mode.
    ///
    /// When in async mode, the debugger returns immediately when
    /// stepping or continuing without waiting for the process
    /// to change state.
    pub fn async(&self) -> bool {
        unsafe { sys::SBDebuggerGetAsync(self.raw_debugger) != 0 }
    }

    /// Set the debugger to be in async mode or not.
    ///
    /// When in async mode, the debugger returns immediately when
    /// stepping or continuing without waiting for the process
    /// to change state.
    pub fn set_async(&mut self, async: bool) {
        unsafe {
            sys::SBDebuggerSetAsync(self.raw_debugger, async as u8);
        }
    }

    /// Get the LLDB version string.
    pub fn version() -> String {
        unsafe {
            match CStr::from_ptr(sys::SBDebuggerGetVersionString()).to_str() {
                Ok(s) => s.to_owned(),
                _ => panic!("No version string?"),
            }
        }
    }

    /// Get an iterator over the [targets] known to this debugger instance.
    ///
    /// [targets]: struct.SBTarget.html
    pub fn targets(&self) -> DebuggerTargetIter {
        DebuggerTargetIter {
            debugger: self,
            idx: 0,
        }
    }
}

#[doc(hidden)]
pub struct DebuggerTargetIter<'d> {
    debugger: &'d SBDebugger,
    idx: usize,
}

impl<'d> Iterator for DebuggerTargetIter<'d> {
    type Item = SBTarget;

    fn next(&mut self) -> Option<SBTarget> {
        if self.idx < unsafe { sys::SBDebuggerGetNumTargets(self.debugger.raw_debugger) as usize } {
            let r = Some(SBTarget {
                raw_target: unsafe {
                    sys::SBDebuggerGetTargetAtIndex(self.debugger.raw_debugger, self.idx as u32)
                },
            });
            self.idx += 1;
            r
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SBDebugger;

    #[test]
    fn it_works() {
        assert!(!SBDebugger::version().is_empty());
    }
}