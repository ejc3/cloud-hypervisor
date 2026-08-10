// Copyright 2022 Arm Limited (or its affiliates). All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0

pub mod gic;
pub mod regs;

use serde::{Deserialize, Serialize};

/// Return the MPIDR based on the vCPU ID
/// Based on reset_mpidr() in linux/arch/arm64/kvm/sys_regs.c
pub const fn mpidr_from_vcpu_id(vcpu_id: u64) -> u64 {
    (vcpu_id & 0x0f)                    // Aff0, bits [7:0]
    | (((vcpu_id >> 4) & 0xff) << 8)    // Aff1, bits [15:8]
    | (((vcpu_id >> 12) & 0xff) << 16)  // Aff2, bits [23:16]
    | (1 << 31) // RES1
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExtendedReg {
    pub id: u64,
    pub data: Vec<u8>,
}
