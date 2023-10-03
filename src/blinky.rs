use crate::protocol::{read_reg, write_mem, Handle};
use std::{thread::sleep, time::Duration};

// From S905 Public Datasheet V1.1.4
const S905_PERIPHS_MUX_BASE: u32 = 0xC883_4400;
/*
const S905_PERIPHS_PIN_MUX_0: u32 = S905_PERIPHS_MUX_BASE + 0x00b0;
const S905_PERIPHS_PIN_MUX_1: u32 = S905_PERIPHS_MUX_BASE + 0x00b4;
const S905_PERIPHS_PIN_MUX_2: u32 = S905_PERIPHS_MUX_BASE + 0x00b8;
const S905_PERIPHS_PIN_MUX_3: u32 = S905_PERIPHS_MUX_BASE + 0x00bc;
const S905_PERIPHS_PIN_MUX_4: u32 = S905_PERIPHS_MUX_BASE + 0x00c0;
const S905_PERIPHS_PIN_MUX_5: u32 = S905_PERIPHS_MUX_BASE + 0x00c4;
const S905_PERIPHS_PIN_MUX_6: u32 = S905_PERIPHS_MUX_BASE + 0x00c8;
*/
const S905_GPIOZ_OE: u32 = S905_PERIPHS_MUX_BASE + 0x0015 * 4;
const S905_GPIOZ_OUT: u32 = S905_PERIPHS_MUX_BASE + 0x0016 * 4;
/*
const S905_GPIOX_OE: u32 = S905_PERIPHS_MUX_BASE + 0x0018 * 4;
const S905_GPIOX_OUT: u32 = S905_PERIPHS_MUX_BASE + 0x0019 * 4;
*/

/*
const S905_AO_RTI_PIN_MUX_BASE: usize = 0xC810_0000;
const S905_AO_RTI_PIN_MUX_1: usize = S905_AO_RTI_PIN_MUX_BASE + 0x0014;
const S905_AO_RTI_PIN_MUX_2: usize = S905_AO_RTI_PIN_MUX_BASE + 0x0018;
*/
const S905_GPIO_AO_BASE: u32 = 0xC810_0000;
// bits 0-13: output enable; bits 16-29: output state
const S905_GPIO_AO_OUT: u32 = S905_GPIO_AO_BASE + 0x0024;

// Let the white LED blink. NOTE: Values are little endian.
pub fn vim1_blink(h: &Handle, t: Duration) {
    crate::protocol::nop(h, t);
    // On Khadas VIM1, GPIO AO 9 is the SYS LED.
    let ao = S905_GPIO_AO_OUT;
    // Z14 / Z15 are ethernet link / active indicators.
    let z = S905_GPIOZ_OUT;
    // Enable GPIOZ_{14,15}
    if false {
        let v = read_reg(h, t, S905_GPIOZ_OE).unwrap();
        // I _think_ this _should_ be correct... but what do I know?
        let v = [v[0], v[1] & 0b0011_1111, v[2], v[3]];
        // FIXME: This runs into IO or t errors; something crashes?!
        write_mem(h, t, S905_GPIOZ_OE, &v).unwrap();
    }
    // read_mem(h, t, ao, 4).unwrap();
    // [0xff, 0x3f, 0xff, 0xbf];
    // i.e., 0xbfff_3fff (0b1011_1111__1111_1111___0011_1111__1111_1111)
    // want  0xbdff_3dff (0b1011_1101__1111_1111___0011_1101__1111_1111)
    //                             ^on                    ^enable
    println!("Blink the SYS LED on Khadas VIM1");
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf = [0xff, 0x3d, 0xff, 0xbd];
        write_mem(h, t, ao, &buf).unwrap();
        sleep(dur);
        let buf = [0xff, 0x3d, 0xff, 0xbf];
        write_mem(h, t, ao, &buf).unwrap();
        sleep(dur);
        if false {
            // initial values: 0xff 0xff 0xff 0xff
            // let v = read_reg(h, t, z).unwrap();
            // let v = [v[0], v[1] & 0b1111_0011, v[2], v[3]];
            // let v = [v[0], 0, v[2], v[3]];
            let v = [0, 0, 0, 0];
            write_mem(h, t, z, &v).unwrap();
            sleep(dur);
            // let v = read_reg(h, t, z).unwrap();
            // let v = [v[0], v[1] | 0b0000_1100, v[2], v[3]];
            let v = [0xff, 0xff, 0xff, 0xff];
            write_mem(h, t, z, &v).unwrap();
            sleep(dur);
        }
    }
}

// from AML S905D3 / A311D manual
const S905D3_PERIPHS_MUX_BASE: usize = 0xff63_4400;

const S905D3_PREG_PAD_GPIO2_EN: usize = S905D3_PERIPHS_MUX_BASE + 0x0058;
const S905D3_PREG_PAD_GPIO2_O: usize = S905D3_PERIPHS_MUX_BASE + 0x005C;

// const S905D3_AO_RTI_PIN_MUX_BASE: usize = 0xff80_0000;

// see Libre Computer AML-A311D-CC V0.2 schematics
const LED1: u8 = 1 << 3;
const LED2: u8 = 1 << 6;
const LED3: u8 = 1 << 7;

// NOTE: This is all active low.
pub fn lc_a311d_cc_blink(h: &Handle, t: Duration) {
    let addr = S905D3_PREG_PAD_GPIO2_EN as u32;
    let buf = [0b0011_0111, 0xff, 0xff, 0xff];
    write_mem(h, t, addr, &buf).unwrap();
    println!("Blink the LEDs on Libre Computer AML-A311D-CC");
    let addr = S905D3_PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf = [LED1 | LED3, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
        let buf = [LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
        let buf = [LED1 | LED2, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
        let buf = [LED1 | LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
    }
}

// WIP: This should be the same as for the A311D, but runs into ts, then
// errors with "NoDevice".
pub fn lc_s905d3_cc_blink(h: &Handle, t: Duration) {
    let addr = S905D3_PREG_PAD_GPIO2_EN as u32;
    let buf = [0b0011_0111, 0xff, 0xff, 0xff];
    write_mem(h, t, addr, &buf).unwrap();
    println!("Blink the LEDs on Libre Computer AML-A311D-CC");
    let addr = S905D3_PREG_PAD_GPIO2_O as u32;
    let dur = Duration::from_millis(300);
    for _ in 0..4 {
        let buf = [LED1 | LED3, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
        let buf = [LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
        let buf = [LED1 | LED2, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
        let buf = [LED1 | LED2 | LED3, 0x00, 0x00, 0x00];
        write_mem(h, t, addr, &buf).unwrap();
        sleep(dur);
    }
}
