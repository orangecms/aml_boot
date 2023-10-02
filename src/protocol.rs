use std::time::Duration;

// keeping it short :)
pub type Handle = rusb::DeviceHandle<rusb::GlobalContext>;

// from https://dn.odroid.com/S905/DataSheet/S905_Public_Datasheet_V1.1.4.pdf
// const SYS_AHB_BASE: u32 = 0xC800_0000;
// FIXME: Not working on S905X, taken from pyamlboot PROTOCOL.md
// also found in khadas update tool
// const CHIP_ID_ADDR_X: u32 = SYS_AHB_BASE + 0x0001_3c24;

// from Khadas tools / update (objdump is your friend :))
const S905X_CHIP_ID_ADDR: u32 = 0xd900_d400;
// from S905X manual, p47
const S905X_CPU_POWER_STATE: u32 = 0xc810_00e0;

// these are also taken from khadas update tool
// const X_ADDR3: u32 = 0xfffc_d400;
// const X_ADDR4: u32 = 0xffff_fc84;

/* Request types - just one per direction */
// see https://vovkos.github.io/doxyrest/samples/libusb-sphinxdoc/enum_libusb_endpoint_direction.html#doxid-group-libusb-desc-1ga86c880af878493aa8f805c2aba654b8b
// IN
const REQ_TYPE_AMLIN: u8 = 0xc0;
// OUT
const REQ_TYPE_AMLOUT: u8 = 0x40;

/* Actual commands */
const REQ_WRITE_MEM: u8 = 0x01;
const REQ_READ_MEM: u8 = 0x02;

const REQ_CHIP_GEN: u8 = 0x12;
const REQ_IDENTIFY_HOST: u8 = 0x20;
const REQ_CHIPINFO: u8 = 0x40;

const REQ_TPL_CMD: u8 = 0x30;
const REQ_PASSWORD: u8 = 0x35;
const REQ_NOP: u8 = 0x36;

// whatever nop does, useful for testing communication
pub fn nop(h: &Handle, t: Duration) {
    println!("nop");

    let buf = [0u8; 0];
    let r = h.write_control(REQ_TYPE_AMLOUT, REQ_NOP, 0x0, 0x0, &buf, t);
    match r {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Nope"),
    }
}

// How do we read this? Example output from Libre Computer S905D3-CC:
// [10, 03, 47, 00, 58, 00, 2d, 00, 43, 00, 48, 00, 49, 00, 50, 00]
// Twiddling those as 2 bytes per entry, little endian, reveals ASCII:
// 03 10 ??
// 00 47 G
// 00 58 X
// 00 2d -
// 00 43 C
// 00 48 H
// 00 49 I
// 00 50 P
// NOTE: "GX-CHIP" is also the USB product string.
// 0x03 might be the chip generation, maybe 0x10 is a family / variant?
// We get the same output on the Khadas VIM1 (S905X).
// NOTE: args appear to have no effect; to be verified
pub fn chip_gen(h: &Handle, t: Duration) {
    println!("Read chip generation");
    // This appears to be constant?!
    let mut buf = vec![0; 16];
    match h.read_control(REQ_TYPE_AMLIN, REQ_CHIP_GEN, 0, 0, &mut buf, t) {
        Ok(_) => {
            let fam = buf[0];
            let gen = buf[1];
            let mut s = String::new();
            for i in (2..16).step_by(2) {
                let r = u16::from_le_bytes(buf[i..i + 2].try_into().unwrap());
                s.push(r as u8 as char);
            }

            println!("Chip: {s:} {fam:02x?}/{gen:02x?}");
        }
        Err(e) => println!("chip_gen err: {e:?}"),
    }
}

// TODO: move to lib?
fn int_to_bool_str(v: u8) -> &'static str {
    match v {
        1 => "yes",
        _ => "no",
    }
}

pub fn info(h: &Handle, t: Duration) {
    println!("Read chip information\n");
    let mut buf = [0u8; 6];
    match h.read_control(REQ_TYPE_AMLIN, REQ_IDENTIFY_HOST, 0x0, 0x0, &mut buf, t) {
        Ok(_) => {
            println!("  ROM version:   {}.{}", buf[0], buf[1]);
            println!("  Stage version: {}.{}", buf[2], buf[3]);
            println!("  Need password: {}", int_to_bool_str(buf[4]));
            println!("  Password OK:   {}", int_to_bool_str(buf[5]));
            println!();
        }
        Err(e) => println!("chip_id err: {e:?}"),
    }
}

fn print_64u8_as_16u32(buf: &[u8; 64]) {
    let v: &mut Vec<u32> = &mut Vec::new();
    for i in (0..64).step_by(4) {
        let chunk = buf[i..i + 4].try_into().unwrap();
        v.push(u32::from_le_bytes(chunk));
    }
    for i in (0..16).step_by(4) {
        println!(
            "    {:08x?} {:08x?} {:08x?} {:08x?}",
            v[i],
            v[i + 1],
            v[i + 2],
            v[i + 3]
        );
    }
}

// Read and dump chip info at index n.
pub fn chip_info_n(h: &Handle, t: Duration, n: u16) {
    let mut buf = [0u8; 64];
    match h.read_control(REQ_TYPE_AMLIN, REQ_CHIPINFO, 0x0, n, &mut buf, t) {
        Ok(_) => print_64u8_as_16u32(&buf),
        Err(e) => println!("chip_info err: {e:?}"),
    }
}

// Read and dump all four chip info blocks.
// NOTE: On Khadas VIM1 / S905X, we get 4x 16 bytes only, always the same:
// 00470310 002d0058 00480043 00500049 (0x03 0x10 GX-CHIP, as in chip_gen)
pub fn chip_info(h: &Handle, t: Duration) {
    println!("Read chip information\n");
    println!("- INDX");
    chip_info_n(h, t, 0x0);
    println!();
    println!("- CHIP");
    chip_info_n(h, t, 0x1);
    println!();
    println!("- OPS_");
    chip_info_n(h, t, 0x2);
    println!();
    println!("- ROM version");
    chip_info_n(h, t, 0x3);
    println!();
}

pub fn chip_id(h: &Handle, t: Duration) {
    println!("Chip ID:");
    read_mem(h, t, S905X_CHIP_ID_ADDR, 12).unwrap();
}

pub fn power_states(h: &Handle, t: Duration) {
    println!("Power states (?):");
    read_mem(h, t, S905X_CPU_POWER_STATE, 8).unwrap();
}

pub fn read_reg(h: &Handle, t: Duration, addr: u32) -> Result<[u8; 4], &'static str> {
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("read memory @{addr_h:04x}{addr_l:04x}");
    let mut buf = vec![0; 4usize];
    match h.read_control(REQ_TYPE_AMLIN, REQ_READ_MEM, addr_h, addr_l, &mut buf, t) {
        Ok(_) => {
            println!("read_mem: {buf:02x?}");
        }
        Err(e) => println!("read_mem err: {e:?}"),
    }
    Ok(buf.try_into().unwrap())
}

// Read [size] bytes (max. 64) from memory starting at address [addr].
pub fn read_mem(h: &Handle, t: Duration, addr: u32, size: u8) -> Result<(), &'static str> {
    // We can read max. 64 bytes at a time.
    if size > 64 {
        return Err("Memory read size is 64 max");
    }
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("read memory @{addr_h:04x}{addr_l:04x}");
    let mut buf = vec![0; size as usize];
    match h.read_control(REQ_TYPE_AMLIN, REQ_READ_MEM, addr_h, addr_l, &mut buf, t) {
        Ok(_) => {
            println!("read_mem: {buf:02x?}");
        }
        Err(e) => println!("read_mem err: {e:?}"),
    }
    Ok(())
}

pub fn write_mem(h: &Handle, t: Duration, addr: u32, buf: &[u8]) -> Result<(), &'static str> {
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("write to memory @{addr_h:04x}{addr_l:04x}");
    if buf.len() > 64 {
        return Err("Memory write size is 64 max");
    }
    match h.write_control(REQ_TYPE_AMLOUT, REQ_WRITE_MEM, addr_h, addr_l, buf, t) {
        Ok(n) => {
            println!("write_mem success, {n} bytes");
        }
        Err(e) => println!("write_mem err: {e:?}"),
    }
    Ok(())
}

// The command needs 0-byte termination, hence CString.
pub fn tpl_cmd(h: &Handle, t: Duration, cmd: &str) {
    println!("tpl_cmd {cmd}");
    let cmd = std::ffi::CString::new(cmd).expect("C sucks");
    let buf = cmd.as_bytes_with_nul();
    let res = h.write_control(
        REQ_TYPE_AMLOUT,
        REQ_TPL_CMD,
        0,
        1, // aka sub code - always 1 though?
        buf,
        t,
    );
    println!("{res:?}");
}

// Just for reference; untested as per pyamlboot
// Password size is 64 bytes
pub fn password(h: &Handle, t: Duration, buf: &[u8; 64]) {
    println!("password: {buf:02x?}");
    let r = h.write_control(REQ_TYPE_AMLOUT, REQ_PASSWORD, 0x0, 0x0, buf, t);
    println!("{r:?}");
}

// NOTE: not yet working, just an attempt
pub fn password_test(h: &Handle, t: Duration) {
    nop(h, t);

    let pw = [0xffu8; 64];
    password(h, t, &pw);

    /*
    let size = 16;
    let mut buf = vec![0; size as usize];
    for cmd in 0xa0..=0xff {
        println!("Try command {cmd:02x}");
        match h.read_control(REQ_TYPE_AMLIN, cmd, 0, 0, &mut buf, t) {
            Ok(n) => {
                println!("res ({n}): {buf:02x?}");
            }
            Err(e) => println!("err: {e:?}"),
        }
    }
    */
}
