//! Bring the process to the foreground on macOS after a terminal launch.
//!
//! Launch Services does not yield focus when a GUI binary is started directly
//! from a shell, so GTK's window can appear behind the terminal until clicked.
//! Calling into AppKit after the window is ready restores the usual "new app
//! activates" behaviour for that intentional user launch.

#![allow(unexpected_cfgs)] // objc macros probe `feature = "cargo-clippy"`.

use objc::runtime::{Object, BOOL, YES};
use objc::{class, msg_send, sel, sel_impl};

#[link(name = "AppKit", kind = "framework")]
extern "C" {}

/// Activate this process so its windows can become key.
///
/// Uses `activateIgnoringOtherApps:` (still the path that works for many
/// terminal launches) and, when present, the macOS 14+ `activate` selector.
pub fn activate_running_app() {
    unsafe {
        let app: *mut Object = msg_send![class!(NSApplication), sharedApplication];
        if app.is_null() {
            return;
        }
        let _: () = msg_send![app, activateIgnoringOtherApps: YES];
        let sel_activate = sel!(activate);
        let responds: BOOL = msg_send![app, respondsToSelector: sel_activate];
        if responds == YES {
            let _: () = msg_send![app, activate];
        }
    }
}
