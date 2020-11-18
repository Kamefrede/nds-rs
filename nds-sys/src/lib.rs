#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

include!("nds.rs");

const REG_DSPCNT: *mut u32 = 0x04000000 as *mut u32;
const REG_DSPCNT_SUB: *mut u32 = 0x04001000 as *mut u32;

pub fn videoSetMode(mode: u32) {
	unsafe {
		*REG_DSPCNT = mode;
	}
}

pub fn videoSetModeSub(mode: u32) {
	unsafe {
		*REG_DSPCNT_SUB = mode;
	}
}