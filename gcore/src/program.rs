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

use crate::ProgramId;

mod sys {
    extern "C" {
        pub fn gr_submit_program(code_ptr: *const u8, code_len: u32, program_id_ptr: *mut u8);
    }
}

pub fn submit(code: &[u8]) -> ProgramId {
    unsafe {
        let mut program_id = ProgramId::default();
        sys::gr_submit_program(
            code.as_ptr(),
            code.len() as _,
            program_id.as_mut_slice().as_mut_ptr(),
        );
        program_id
    }
}
