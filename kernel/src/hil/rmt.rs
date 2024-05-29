// Licensed under the Apache License, Version 2.0 or the MIT License
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024

// Author: Kacper Kołodziej <kacper@sfinae.dev>

use crate::ErrorCode;
use crate::hil::gpio;

#[derive(Copy, Clone, Debug)]
pub enum DataOrder {
    MSBFirst,
    LSBFirst,
}

pub struct RMT<'a, P: gpio::Pin> {
    pub pin: &'a P,
}

impl<'a, P: gpio::Pin> RMT<'a, P> {
    fn new(p: &'a P) -> Self {
        Self { pin: p }
    }

    fn init(&self) {
        self.pin.make_output();
    }
}
