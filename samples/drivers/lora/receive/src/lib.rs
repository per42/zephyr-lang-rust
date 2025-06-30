// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_use]
extern crate alloc;

use log::info;
use core::primitive::str;

use zephyr::time::{sleep, Duration, Forever};

use zephyr::raw::lora_coding_rate;
use zephyr::raw::lora_datarate;
use zephyr::raw::lora_modem_config;
use zephyr::raw::lora_signal_bandwidth;

const MAX_DATA_LEN: u8 = 255;

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    let lora0 = zephyr::devicetree::aliases::lora0::get_instance().unwrap();

    lora0
        .config(&lora_modem_config {
            frequency: 865100000,
            bandwidth: lora_signal_bandwidth::BW_125_KHZ,
            datarate: lora_datarate::SF_10,
            coding_rate: lora_coding_rate::CR_4_5,
            preamble_len: 8,
            tx_power: 4,
            tx: false,
            iq_inverted: false,
            public_network: false,
        })
        .unwrap();

    /* Receive 4 packets synchronously */
    info!("Synchronous reception");
    for counter in 0..4 {
        let digit = counter % 10;
        let (data, rssi, snr) = lora0
            .recv(MAX_DATA_LEN, Forever.into())
            .unwrap();
        info!("LoRa RX RSSI: {} dBm, SNR: {} dB", rssi, snr);
        info!("LoRa RX payload: {}", str::from_utf8(&data).expect("Message should be in UTF8"));
    }
}
