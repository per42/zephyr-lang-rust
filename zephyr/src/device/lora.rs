//! Device wrappers for LoRa

extern crate alloc;

use super::{NoStatic, Unique};
use crate::{raw, time::Timeout};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::null_mut;
use core::slice;

type OnData = dyn FnMut(&[u8], i16, i8);

/// A LoRa device
#[allow(dead_code)]
pub struct Lora {
    pub(crate) device: *const raw::device,
    on_data: Option<Box<OnData>>,
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

        Some(Lora {
            device,
            on_data: None,
        })
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
    ///
    /// returns (data, rssi, snr)
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

    /// Receive by handler
    ///
    /// on_data is called with (data, rssi, snr)
    pub fn recv_async(
        &mut self,
        on_data: impl FnMut(&[u8], i16, i8) + 'static,
    ) -> crate::Result<()> {
        self.on_data = Some(Box::new(on_data));
        let on_data_ptr: *mut _ = self.on_data.as_mut().unwrap();
        crate::error::to_result_void(unsafe {
            raw::lora_recv_async(self.device, Some(recv_async_cb), on_data_ptr.cast())
        })
    }

    /// Stop recv_async
    pub fn recv_async_stop(&mut self) -> crate::Result<()> {
        self.on_data = None;
        crate::error::to_result_void(unsafe { raw::lora_recv_async(self.device, None, null_mut()) })
    }
}

unsafe extern "C" fn recv_async_cb(
    _dev: *const raw::device,
    data: *mut u8,
    size: u16,
    rssi: i16,
    snr: i8,
    user_data: *mut ::core::ffi::c_void,
) {
    let on_data = user_data.cast::<Box<OnData>>().as_mut().unwrap();
    on_data(slice::from_raw_parts(data, size.into()), rssi, snr);
}
