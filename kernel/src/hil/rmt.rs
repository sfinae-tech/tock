// Licensed under the Apache License, Version 2.0 or the MIT License
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2024

// Author: Kacper Kołodziej <kacper@sfinae.dev>

use crate::ErrorCode;

#[derive(Copy, Clone, Debug)]
pub enum DataOrder {
    MSBFirst,
    LSBFirst,
}
