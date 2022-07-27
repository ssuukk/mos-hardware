// copyright 2022 mikael lund aka wombat
// 
// licensed under the apache license, version 2.0 (the "license");
// you may not use this file except in compliance with the license.
// you may obtain a copy of the license at
// 
//     http://www.apache.org/licenses/license-2.0
// 
// unless required by applicable law or agreed to in writing, software
// distributed under the license is distributed on an "as is" basis,
// without warranties or conditions of any kind, either express or implied.
// see the license for the specific language governing permissions and
// limitations under the license.

use crate::vic2::*;
use crate::sid::*;
use crate::{peek, poke};

pub mod iomap;

pub const DEFAULT_SCREEN: *mut u8 = (0x0800) as *mut u8;
pub const DEFAULT_UPPERCASE_FONT: *mut u8 = (0x1000) as *mut u8;
pub const DEFAULT_MIXEDCASE_FONT: *mut u8 = (0x1800) as *mut u8;

pub const VIC_II: *const MOSVideoInterfaceControllerII =
    (0xd000) as *const MOSVideoInterfaceControllerII;

pub const SID0: *const MOSSoundInterfaceDevice = (0xd400) as *const MOSSoundInterfaceDevice;
pub const SID1: *const MOSSoundInterfaceDevice = (0xd420) as *const MOSSoundInterfaceDevice;
pub const SID2: *const MOSSoundInterfaceDevice = (0xd440) as *const MOSSoundInterfaceDevice;
pub const SID3: *const MOSSoundInterfaceDevice = (0xd460) as *const MOSSoundInterfaceDevice;

pub const COLOR_RAM: *mut u8 = (0xd800) as *mut u8;

pub enum VicBank {
    Region0000 = 0x11, // Bank 0
    Region4000 = 0x10, // Bank 1
    Region8000 = 0x01, // Bank 2
    RegionC000 = 0x00, // Bank 3
}

/// Generate random byte from hardware register (from mega65 libc)
///
/// @todo Returns constant zero on xemu; check on real hardware.
pub fn rand8() -> u8 {
    let mut random_byte : u8 = 0;
    let mut steps :u8 = 32;
    while steps > 0 {
        steps = steps - 1;
        random_byte = (random_byte << 1) | (random_byte >> 7) ^ (peek!(0xd6de as *mut u8) & 0x01);
        // We then have to wait 10usec before the next value is ready.
        // 1 raster line is more than that, so just wait one raster line
        let raster_position = peek!(0xd052 as *mut u8);
        while peek!(0xd052 as *mut u8) == raster_position {
            continue;
        }
    }
    return random_byte;
}

/// Set CPU speed to 1 Mhz
pub fn speed_mode1() {
    let mut val : u8 = peek!(0xd031 as *mut u8) & 0b1011_1111; // unset FAST bit 
    poke!(0xd031 as *mut u8, val);
    val = peek!(0xd054 as *mut u8) & 0b1011_1111; // unset VFAST bit
    poke!(0xd054 as *mut u8, val);
} 

/// Set CPU speed to 3.5 Mhz
pub fn speed_mode3() {
    let mut val : u8 = peek!(0xd031 as *mut u8) | 0b0100_0000; // set FAST bit
    poke!(0xd031 as *mut u8, val);
    val = peek!(0xd054 as *mut u8) & 0b1011_1111; // unset VFAST
    poke!(0xd054 as *mut u8, val);
} 

/// Set CPU speed to 40 Mhz
pub fn speed_mode40() {
    let mut val : u8 = peek!(0xd031 as *mut u8) | 0b0100_0000; // set FAST bit
    poke!(0xd031 as *mut u8, val);
    val = peek!(0xd054 as *mut u8) | 0b0100_0000; // set VFAST bit
    poke!(0xd054 as *mut u8, val);
} 
