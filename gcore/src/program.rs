// This file is part of Gear.

// Copyright (C) 2021 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

mod sys {
    extern "C" {
        pub fn gr_submit_program(
            code_ptr: *const u8,
            code_len: u32,
            nonce: u64,
            init_payload_ptr: *const u8,
            init_payload_len: u32,
            init_gas_limit: u64,
            init_value: *const u8,
        );
    }
}

pub fn submit(code: &[u8], nonce: u64, init_payload: &[u8], init_gas_limit: u64, init_value: u128) {
    unsafe {
        sys::gr_submit_program(
            code.as_ptr(),
            code.len() as _,
            nonce,
            init_payload.as_ptr(),
            init_payload.len() as _,
            init_gas_limit,
            init_value.to_le_bytes().as_ptr(),
        );
    }
}
