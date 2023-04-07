use random_number::random;
use winapi::{
    shared::windef::{COLORREF, HDC__, HICON__, HWND__, RECT},
    um::{wingdi::*, winuser::*},
};

use crate::utils;

// A structure for defining the desktop
#[derive(Debug, Clone, Copy)]
pub struct Window {
    hwnd: *mut HWND__,
    dc: *mut HDC__,
    w: i32,
    h: i32,
    x: i32,
    y: i32,
}

impl Window {
    pub fn get_width(&self) -> i32 {
        self.w
    }

    pub fn get_height(&self) -> i32 {
        self.h
    }

    pub fn get_dc(&self) -> *mut HDC__ {
        self.dc
    }

    pub fn get_hwnd(&self) -> *mut HWND__ {
        self.hwnd
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    // Call when done with object
    pub unsafe fn close(&self) {
        ReleaseDC(self.get_hwnd(), self.get_dc());
    }

    // Some basic presets that look alright

    pub unsafe fn rgb_glitch(&self) {
        BitBlt(
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h,
            self.dc,
            self.x + 1,
            self.y - 1,
            SRCINVERT,
        );
    }

    pub unsafe fn copy_glitch(&self) {
        BitBlt(
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h,
            self.dc,
            self.x + 1,
            self.y - 1,
            SRCCOPY,
        );
    }

    pub unsafe fn paint_glitch(&self) {
        BitBlt(
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h,
            self.dc,
            self.x + 1,
            self.y - 1,
            SRCPAINT,
        );
    }

    pub unsafe fn erode_glitch(&self) {
        BitBlt(
            self.dc,
            self.x,
            self.y + 1,
            self.w,
            self.h,
            self.dc,
            self.x,
            self.y,
            SRCAND,
        );
    }

    pub unsafe fn invert(&self) {
        PatBlt(self.dc, self.x, self.y + 1, self.w, self.h, PATINVERT);
    }

    pub unsafe fn set_white(&self) {
        PatBlt(self.dc, self.x, self.y, self.w, self.h, WHITENESS);
    }

    pub unsafe fn set_black(&self) {
        PatBlt(self.dc, self.x, self.y, self.w, self.h, BLACKNESS);
    }

    pub unsafe fn fill_rect(&self, rect: *mut RECT, color: COLORREF) {
        FillRect(self.dc, rect, CreateSolidBrush(color));
    }

    // Simple screen melting effect
    pub unsafe fn melt_step(&self) {
        let width = random!(0, self.w);
        BitBlt(
            self.dc, self.x, self.y, width, self.w, self.dc, self.x, 0, SRCCOPY,
        );
    }

    pub unsafe fn set_pixel(&self, x: i32, y: i32, color: COLORREF) {
        SetPixel(self.dc, x, y, color);
    }

    pub unsafe fn stretch_glitch(&self, amount: i32) {
        StretchBlt(
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h + amount,
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h,
            SRCCOPY,
        );
    }

    pub unsafe fn stretch_glitch_neg(&self, amount: i32) {
        StretchBlt(
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h - amount,
            self.dc,
            self.x,
            self.y,
            self.w,
            self.h,
            SRCCOPY,
        );
    }

    pub unsafe fn draw_text(&self, text: String, rect: &mut RECT) {
        DrawTextW(
            self.dc,
            utils::to_utf16(&text).as_ptr(),
            text.len() as i32,
            rect,
            DT_CENTER,
        );
    }

    // To get a HICON instance take a look at load_icon() in utils
    pub unsafe fn draw_icon(&self, x: i32, y: i32, icon: *mut HICON__) {
        DrawIcon(self.dc, x, y, icon);
    }
}

// Get a instance of the desktop
pub fn get_desktop() -> Window {
    unsafe {
        let hwnd = GetDesktopWindow();
        let dc = GetDC(hwnd);
        let w = GetSystemMetrics(SM_CXSCREEN);
        let h = GetSystemMetrics(SM_CYSCREEN);
        let x = SM_CXSCREEN;
        let y = SM_CYSCREEN;
        Window {
            hwnd,
            dc,
            w,
            h,
            x,
            y,
        }
    }
}
