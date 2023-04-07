use std::time::Duration;

use winapi::um::{
    wingdi::{
        CreateHatchBrush, PatBlt, StretchBlt, DSTINVERT, HS_FDIAGONAL, HS_HORIZONTAL, HS_VERTICAL,
        SRCCOPY,
    },
    winuser::{FillRect, PaintDesktop},
};

use random_number::random;

use crate::{
    effects::Window,
    utils::{create_rect, random_color},
};

#[derive(Clone, Copy)]
pub enum EFFECTS {
    SCALE,
    INVERT,
    RGB,
    COPY,
    PAINT,
    ERODE,
    WHITE,
    BLACK,
    MELTSTEP,
    STRETCH,
    SHRINK,
    STRETCHRGB,
    MELT,
    FLIPH,
    FLIPV,
    RANDOMNOISE,
    RANDOMRECTS,
    RANDOMINVERT,
    DESKTOPOVERLAY,
    HORIZONTALLINES,
    VERTICALLINES,
    DIAGLINES,
}

pub struct SingleEffect {
    effect: EFFECTS,
}

impl SingleEffect {
    pub fn new(effect: EFFECTS) -> SingleEffect {
        SingleEffect { effect }
    }

    pub unsafe fn execute(&self, target: &Window) {
        execute(target, &vec![self.effect], 0);
    }
}

pub struct EffectChain {
    delay: u64,
    effects: Vec<EFFECTS>,
}

impl EffectChain {
    pub fn new(delay: u64, effects: Vec<EFFECTS>) -> EffectChain {
        EffectChain { delay, effects }
    }

    /*
       delay: Delay inbetween drawing effects in milliseconds
       effects: Effects to draw one by one
    */
    pub unsafe fn execute(&self, target: &Window) {
        execute(target, &self.effects, self.delay);
    }
}

unsafe fn execute(target: &Window, effects: &Vec<EFFECTS>, delay: u64) {
    let dc = target.get_dc();
    let mut frect = create_rect(0, 0, target.get_width(), target.get_height());

    for effect in effects.iter() {
        match effect {
            EFFECTS::SCALE => target.stretch_glitch(10),
            EFFECTS::INVERT => target.invert(),
            EFFECTS::RGB => target.rgb_glitch(),
            EFFECTS::COPY => target.copy_glitch(),
            EFFECTS::PAINT => target.paint_glitch(),
            EFFECTS::ERODE => target.erode_glitch(),
            EFFECTS::WHITE => target.set_white(),
            EFFECTS::BLACK => target.set_black(),
            EFFECTS::MELTSTEP => target.melt_step(),
            EFFECTS::STRETCH => target.stretch_glitch(10),
            EFFECTS::SHRINK => target.stretch_glitch_neg(10),
            EFFECTS::STRETCHRGB => {
                target.stretch_glitch(10);
                target.rgb_glitch();
            }
            EFFECTS::MELT => {
                for _ in 0..10 {
                    target.melt_step();
                }
            }
            EFFECTS::FLIPH => {
                StretchBlt(
                    dc,
                    target.get_width(),
                    0,
                    -target.get_width(),
                    target.get_height(),
                    dc,
                    0,
                    0,
                    target.get_width(),
                    target.get_height(),
                    SRCCOPY,
                );
            }
            EFFECTS::FLIPV => {
                StretchBlt(
                    dc,
                    0,
                    target.get_height(),
                    target.get_width(),
                    -target.get_height(),
                    dc,
                    0,
                    0,
                    target.get_width(),
                    target.get_height(),
                    SRCCOPY,
                );
            }
            EFFECTS::RANDOMNOISE => {
                for _ in 0..100 {
                    target.set_pixel(
                        random!(0, target.get_width()),
                        random!(0, target.get_height()),
                        random_color(),
                    )
                }
            }
            EFFECTS::RANDOMRECTS => {
                for _ in 0..10 {
                    let mut rect = create_rect(
                        random!(-target.get_width(), target.get_width()),
                        random!(-target.get_height(), target.get_height()),
                        random!(-target.get_width(), target.get_width()),
                        random!(-target.get_height(), target.get_height()),
                    );
                    target.fill_rect(&mut rect, random_color())
                }
            }
            EFFECTS::RANDOMINVERT => {
                for _ in 0..10 {
                    PatBlt(
                        dc,
                        random!(-target.get_width(), target.get_width()),
                        random!(-target.get_height(), target.get_height()),
                        random!(-target.get_width(), target.get_width()),
                        random!(-target.get_height(), target.get_height()),
                        DSTINVERT,
                    );
                }
            }
            EFFECTS::DESKTOPOVERLAY => {
                PaintDesktop(dc);
            }
            EFFECTS::HORIZONTALLINES => {
                let brush = CreateHatchBrush(HS_HORIZONTAL as i32, random_color());
                FillRect(dc, &mut frect, brush);
            }
            EFFECTS::VERTICALLINES => {
                let brush = CreateHatchBrush(HS_VERTICAL as i32, random_color());
                FillRect(dc, &mut frect, brush);
            }
            EFFECTS::DIAGLINES => {
                let brush = CreateHatchBrush(HS_FDIAGONAL as i32, random_color());
                FillRect(dc, &mut frect, brush);
            }
        }
        std::thread::sleep(Duration::from_millis(delay));
    }
}
