//! Defensive module boundary. Remote or stealth actions are disabled by default.
pub fn scope_required() -> bool { true }
