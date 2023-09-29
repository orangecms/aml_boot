use crate::protocol::{Handle,write_mem};
use std::{thread::sleep, time::Duration};

// From S905 Public Datasheet V1.1.4
const S905_PERIPHS_MUX_BASE: u32 = 0xC883_4400;
const S905_PERIPHS_PIN_MUX_0: u32 = S905_PERIPHS_MUX_BASE + 0x00b0;
const S905_PERIPHS_PIN_MUX_1: u32 = S905_PERIPHS_MUX_BASE + 0x00b4;
const S905_PERIPHS_PIN_MUX_2: u32 = S905_PERIPHS_MUX_BASE + 0x00b8;
const S905_PERIPHS_PIN_MUX_3: u32 = S905_PERIPHS_MUX_BASE + 0x00bc;
const S905_PERIPHS_PIN_MUX_4: u32 = S905_PERIPHS_MUX_BASE + 0x00c0;
const S905_PERIPHS_PIN_MUX_5: u32 = S905_PERIPHS_MUX_BASE + 0x00c4;
const S905_PERIPHS_PIN_MUX_6: u32 = S905_PERIPHS_MUX_BASE + 0x00c8;

const S905_GPIOX_OE: u32 = S905_PERIPHS_MUX_BASE + 0x0018 * 4;
const S905_GPIOX_OUT: u32 = S905_PERIPHS_MUX_BASE + 0x0019 * 4;

// GPIO AO 0-9,
const S905_GPIO_AO_BASE: u32 = 0xC810_0000;
const S905_AO_RTI_PIN_MUX_BASE: usize = 0xC810_0000;
const S905_AO_RTI_PIN_MUX_1: usize = S905_AO_RTI_PIN_MUX_BASE + 0x0014;
const S905_AO_RTI_PIN_MUX_2: usize = S905_AO_RTI_PIN_MUX_BASE + 0x0018;
const S905_GPIO_AO_OUT: u32 = S905_GPIO_AO_BASE + 0x0024;
       
pub fn vim1_blink(handle: &Handle, timeout: Duration) {
    //  On Khadas VIM1, GPIO AO 9 supposedly is the SYS LED.
    //  This seems to be a bit different? It lets the white LED blink.
    let addr = S905_GPIO_AO_OUT;
    //  read_mem(handle, timeout, addr, 4).unwrap();
    // [0xff, 0x3f, 0xff, 0xbf];
    // 9-0: output enable
    // 25-16: OUT
    println!("Blink the SYS LED on Khadas VIM1");
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf: [u8; 4] = [0xff, 0x3d, 0xff, 0xbd];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [0xff, 0x3f, 0xff, 0xbf];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
    }
}

// from AML A311D manual
const PERIPHS_MUX_BASE: usize = 0xff63_4400;
const AO_RTI_PIN_MUX_BASE: usize = 0xff80_0000;

const PREG_PAD_GPIO2_EN: usize = PERIPHS_MUX_BASE + 0x0058;
const PREG_PAD_GPIO2_O: usize = PERIPHS_MUX_BASE + 0x005C;

// see Libre Computer AML-A311D-CC V0.2 schematics
const LED1: u8 = 1 << 3;
const LED2: u8 = 1 << 6;
const LED3: u8 = 1 << 7;

// NOTE: This is all active low.
pub fn lc_a311d_cc_blink(handle: &Handle, timeout: Duration) {
    let addr = PREG_PAD_GPIO2_EN as u32;
    let buf: [u8; 4] = [0b0011_0111, 0xff, 0xff, 0xff];
    write_mem(handle, timeout, addr, &buf).unwrap();
    println!("Blink the LEDs on Libre Computer AML-A311D-CC");
    let addr = PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf: [u8; 4] = [LED1 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED1 | LED2, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED1 | LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
    }
}

// WIP: This should be the same as for the A311D, but runs into timeouts, then
// errors with "NoDevice".
pub fn lc_s905d3_cc_blink(handle: &Handle, timeout: Duration) {
    let addr = PREG_PAD_GPIO2_EN as u32;
    let buf: [u8; 4] = [0b0011_0111, 0xff, 0xff, 0xff];
    write_mem(handle, timeout, addr, &buf).unwrap();
    println!("Blink the LEDs on Libre Computer AML-A311D-CC");
    let addr = PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf: [u8; 4] = [LED1 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED1 | LED2, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
        let buf: [u8; 4] = [LED1 | LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(handle, timeout, addr, &buf).unwrap();
        sleep(dur);
    }
}
