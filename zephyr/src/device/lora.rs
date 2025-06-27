//! Device wrappers for LoRa

use super::{NoStatic, Unique};
use crate::raw;

/// A LoRa device
#[allow(dead_code)]
pub struct Lora {
    pub(crate) device: *const raw::device,
}

/// Modulation bandwidth
pub enum Bandwidth {
    /// 125 kHz
    Bw125Khz,
    /// 250 kHz
    Bw250Khz,
    /// 500 kHz
    Bw500Khz,
}

/// Spreading factor
///
/// The LoRa spread spectrum modulation is performed by representing each data bit of the
/// packet payload by multiple chips of information. The rate at which the spread information is
/// sent, is referred to as the symbol rate (Rs). The ratio between the nominal data rate and the
/// chip rate is the spreading factor (SF). It represents the number of symbols per data bit.
///
/// Chips per symbol (c/s) listed per variant
pub enum SpreadingFactor {
    /// c/s: 64
    Sf6,
    /// c/s: 128
    Sf7,
    /// c/s: 256
    Sf8,
    /// c/s: 512
    Sf9,
    /// c/s: 1024
    Sf10,
    /// c/s: 2048
    Sf11,
    /// c/s: 4096
    Sf12,
}

/// Forward error correction coding rate
///
/// data bits/total coded bits (d/c) listed per variant
pub enum CodingRate {
    /// d/c: 4/5
    Cr4_5,
    /// d/c: 4/6
    Cr4_6,
    /// d/c: 4/7
    Cr4_7,
    /// d/c: 4/8
    Cr4_8,
}

impl Lora {
    #[allow(dead_code)]
    pub(crate) unsafe fn new(unique: &Unique, _static: &NoStatic, device: *const raw::device) -> Option<Lora> {
        if !unique.once() {
            return None;
        }

        Some(Lora { device })
    }

    /// Configure the LoRa modem
    ///
    /// The LoRa symbol rate (Rs) is defined as $ Rs = \rm{BW} / 2^{\rm{SF}} $
    pub fn config(
        &self,
        frequency: u32,
        bandwidth: Bandwidth,
        spreading_factor: SpreadingFactor,
        coding_rate: CodingRate,
        preamble_len: u16,
        tx_power: i8,
        tx: bool,
        iq_inverted: bool,
        public_network: bool,
    ) -> crate::Result<()> {
        let mut config_mut = raw::lora_modem_config {
            frequency,
            bandwidth: match bandwidth {
                Bandwidth::Bw125Khz => raw::lora_signal_bandwidth_BW_125_KHZ,
                Bandwidth::Bw250Khz => raw::lora_signal_bandwidth_BW_250_KHZ,
                Bandwidth::Bw500Khz => raw::lora_signal_bandwidth_BW_500_KHZ,
            },
            datarate: match spreading_factor {
                SpreadingFactor::Sf6 => raw::lora_datarate_SF_6,
                SpreadingFactor::Sf7 => raw::lora_datarate_SF_7,
                SpreadingFactor::Sf8 => raw::lora_datarate_SF_8,
                SpreadingFactor::Sf9 => raw::lora_datarate_SF_9,
                SpreadingFactor::Sf10 => raw::lora_datarate_SF_10,
                SpreadingFactor::Sf11 => raw::lora_datarate_SF_11,
                SpreadingFactor::Sf12 => raw::lora_datarate_SF_12,
            },
            coding_rate: match coding_rate {
                CodingRate::Cr4_5 => raw::lora_coding_rate_CR_4_5,
                CodingRate::Cr4_6 => raw::lora_coding_rate_CR_4_6,
                CodingRate::Cr4_7 => raw::lora_coding_rate_CR_4_7,
                CodingRate::Cr4_8 => raw::lora_coding_rate_CR_4_8,
            },
            preamble_len,
            tx_power,
            tx,
            iq_inverted,
            public_network,
        };
        unsafe { crate::error::to_result_void(raw::lora_config(self.device, &mut config_mut)) }
    }

    /// Send data
    pub fn send(&self, data: &[u8]) -> crate::Result<()> {
        unsafe {
            crate::error::to_result_void(raw::lora_send(
                self.device,
                data.as_ptr() as *mut u8,
                data.len()
                    .try_into()
                    .expect("The data length can't fit an u32"),
            ))
        }
    }
}
