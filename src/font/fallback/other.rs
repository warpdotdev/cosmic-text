// SPDX-License-Identifier: MIT OR Apache-2.0

use unicode_script::Script;

// Fallbacks to use after any script specific fallbacks
pub fn common_fallback() -> &'static [&'static str] {
    &[]
}

// Fallbacks to never use
pub fn forbidden_fallback() -> &'static [&'static str] {
    &[
        "PasswordCircle",
        // Hack is Warp's bundled terminal font. It shares the font db with the UI fonts on
        // this platform, so without this exclusion it can silently answer for glyphs the
        // requested UI font lacks (e.g. the mac Shift glyph), even when a purpose-picked
        // fallback family also covers them.
        "Hack",
    ]
}

// Fallbacks to use per script
pub fn script_fallback(_script: Script, _locale: &str) -> &'static [&'static str] {
    &[]
}
