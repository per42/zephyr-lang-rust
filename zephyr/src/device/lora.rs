//! Device wrappers for LoRa

use super::{NoStatic, Unique};
use crate::raw;

/// A LoRa device
#[allow(dead_code)]
pub struct Lora {
    pub(crate) device: *const raw::device,
}

impl Lora {
    #[allow(dead_code)]
    pub(crate) unsafe fn new(
        unique: &Unique,
        _static: &NoStatic,
        device: *const raw::device,
    ) -> Option<Lora> {
        if !unique.once() {
            return None;
        }

        Some(Lora { device })
    }

    /// Configure the LoRa modem
    pub fn config(&self, config: &raw::lora_modem_config) -> crate::Result<()> {
        unsafe {
            crate::error::to_result_void(raw::lora_config(
                self.device,
                (&raw const *config).cast_mut(),
            ))
        }
    }

    /// Send data
    pub fn send(&self, data: &[u8]) -> crate::Result<()> {
        unsafe {
            crate::error::to_result_void(raw::lora_send(
                self.device,
                data.as_ptr().cast_mut(),
                data.len()
                    .try_into()
                    .expect("The data length can't fit an u32"),
            ))
        }
    }
}
