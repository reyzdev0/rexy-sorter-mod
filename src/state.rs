//! State global + helper FFI (string C <-> Rust)
use crate::config::Config;
use crate::gui::Button;
use crate::profile::{self, Profile};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

pub struct State {
    pub cfg: Config,
    pub world: Option<String>,
    pub button: Button,
}

pub static STATE: Mutex<Option<State>> = Mutex::new(None);

pub fn with<R>(f: impl FnOnce(&mut State) -> R) -> Option<R> {
    STATE.lock().ok()?.as_mut().map(f)
}

pub fn persist(s: &State) {
    let _ = s.cfg.save();
    if let Some(w) = &s.world {
        let _ = profile::save(w, &Profile::from_config(&s.cfg));
    }
}

pub unsafe fn cstr(p: *const c_char) -> Option<String> {
    if p.is_null() {
        return None;
    }
    CStr::from_ptr(p).to_str().ok().map(str::to_owned)
}

pub fn to_c(s: String) -> *mut c_char {
    CString::new(s).map(CString::into_raw).unwrap_or(std::ptr::null_mut())
}
