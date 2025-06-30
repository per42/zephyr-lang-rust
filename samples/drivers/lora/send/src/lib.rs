// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

#[macro_use]
extern crate alloc;

use log::info;

use zephyr::time::{sleep, Duration};

use zephyr::raw::lora_coding_rate;
use zephyr::raw::lora_datarate;
use zephyr::raw::lora_modem_config;
use zephyr::raw::lora_signal_bandwidth;

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
            tx: true,
            iq_inverted: false,
            public_network: false,
        })
        .unwrap();

    for counter in 0.. {
        let digit = counter % 10;
        lora0
            .send(format!("helloworld {}", digit).as_bytes())
            .unwrap();
        info!("Data sent {}!", digit);

        /* Send data at 1s interval */
        sleep(Duration::secs_at_least(1));
    }
}
