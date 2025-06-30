//! Device wrappers for LoRa

extern crate alloc;

use super::{NoStatic, Unique};
use crate::{raw, time::Timeout};
use alloc::boxed::Box;
use alloc::vec::Vec;

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
        // Verify that the device is ready for use.  At a minimum, this means the device has been
        // successfully initialized.
        if !raw::device_is_ready(device) {
            return None;
        }

        if !unique.once() {
            return None;
        }

        Some(Lora { device })
    }

    /// Configure the LoRa modem
    pub fn config(&self, config: &raw::lora_modem_config) -> crate::Result<()> {
        crate::error::to_result_void(unsafe {
            raw::lora_config(self.device, (&raw const *config).cast_mut())
        })
    }

    /// Send data
    pub fn send(&self, data: &[u8]) -> crate::Result<()> {
        crate::error::to_result_void(unsafe {
            raw::lora_send(
                self.device,
                data.as_ptr().cast_mut(),
                data.len()
                    .try_into()
                    .expect("The data length can't fit an u32"),
            )
        })
    }

    /// Receive data
    pub fn recv(&self, max_size: u8, timeout: Timeout) -> crate::Result<(Box<[u8]>, i16, i8)> {
        let mut data: Vec<u8> = Vec::new();
        data.reserve_exact(max_size.into());
        let mut rssi: i16 = 0;
        let mut snr: i8 = 0;

        match crate::error::to_result(unsafe {
            raw::lora_recv(
                self.device,
                data.as_mut_ptr(),
                max_size,
                timeout.0,
                &raw mut rssi,
                &raw mut snr,
            )
        }) {
            Ok(length) => {
                if length > max_size.into() {
                    panic!("lora_recv returned invalid length");
                }
                unsafe { data.set_len(length.try_into().unwrap()) };
                Ok((data.into_boxed_slice(), rssi, snr))
            }
            Err(err) => Err(err),
        }
    }
}
