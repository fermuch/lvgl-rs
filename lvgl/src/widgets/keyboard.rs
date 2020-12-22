use crate::widgets::{Keyboard, Textarea};
use crate::{LvResult, NativeObject};
use crate::support::LvError;
use core::convert::TryFrom;

impl Keyboard {
    pub fn set_textarea(&mut self, textarea: &mut Textarea) -> LvResult<()> {
        unsafe {
            lvgl_sys::lv_keyboard_set_textarea(self.core.raw()?.as_mut(), textarea.raw()?.as_mut());
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: KeyboardMode) -> LvResult<()> {
      unsafe {
          lvgl_sys::lv_keyboard_set_mode(self.core.raw()?.as_mut(), mode as u8);
      }
      Ok(())
    }

    pub fn get_mode(&mut self) -> LvResult<KeyboardMode> {
        let mode: u8 = unsafe {
            lvgl_sys::lv_keyboard_get_mode(self.core.raw()?.as_mut())
        };
        
        KeyboardMode::try_from(mode)
    }

    // TODO: lv_keyboard_set_map
    // TODO: lv_keyboard_set_ctrl_map
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum KeyboardMode {
  Lower = lvgl_sys::LV_KEYBOARD_MODE_TEXT_LOWER as u8,
  Upper = lvgl_sys::LV_KEYBOARD_MODE_TEXT_UPPER as u8,
  Special = lvgl_sys::LV_KEYBOARD_MODE_SPECIAL as u8,
  Numeric = lvgl_sys::LV_KEYBOARD_MODE_NUM as u8,
}

impl TryFrom<u8> for KeyboardMode {
    type Error = LvError;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == KeyboardMode::Lower as u8 => Ok(KeyboardMode::Lower),
            x if x == KeyboardMode::Upper as u8 => Ok(KeyboardMode::Upper),
            x if x == KeyboardMode::Special as u8 => Ok(KeyboardMode::Special),
            x if x == KeyboardMode::Numeric as u8 => Ok(KeyboardMode::Numeric),
            _ => Err(LvError::InvalidReference),
        }
    }
}