use std::ptr::null_mut;

use random_number::random;
use winapi::{
    shared::windef::{COLORREF, HICON__, RECT},
    um::{wingdi::RGB, winnt::LPCWSTR, winuser::LoadIconW},
};

pub fn rgb_to_colorref(r: u8, g: u8, b: u8) -> COLORREF {
    RGB(r, g, b)
}

// Example:
// let icon = load_icon(IDI_ERROR); // 'IDI_ERROR' From winapi
pub unsafe fn load_icon(icon: LPCWSTR) -> *mut HICON__ {
    LoadIconW(null_mut(), icon)
}

// Sometimes you need to use .as_ptr
pub fn to_utf16(input: &str) -> Vec<u16> {
    let mut v: Vec<u16> = input.encode_utf16().collect();
    v.push(0);
    v
}

// Utility function to create a win32 rectangle with ease
pub unsafe fn create_rect(x: i32, y: i32, w: i32, h: i32) -> RECT {
    RECT {
        left: x,
        top: y,
        right: x + w,
        bottom: y + h,
    }
}

pub fn random_color() -> COLORREF {
    RGB(random!(0, 255), random!(0, 255), random!(0, 255))
}
