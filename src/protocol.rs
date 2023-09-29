use std::time::Duration;

// keeping it short :)
pub type Handle = rusb::DeviceHandle<rusb::GlobalContext>;

// from https://dn.odroid.com/S905/DataSheet/S905_Public_Datasheet_V1.1.4.pdf
// const SYS_AHB_BASE: u32 = 0xC800_0000;
// FIXME: Not working on S905X, taken from pyamlboot PROTOCOL.md
// also found in khadas update tool
// const CHIP_ID_ADDR_X: u32 = SYS_AHB_BASE + 0x0001_3c24;

// from khadas tools / update (objdump is your friend :))
const CHIP_ID_ADDR_S905X: u32 = 0xd900_d400;

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

const REQ_IDENTIFY_HOST: u8 = 0x20;

const REQ_TPL_CMD: u8 = 0x30;
const REQ_PASSWORD: u8 = 0x35;
const REQ_NOP: u8 = 0x36;

// whatever nop does, useful for testing communication
pub fn nop(handle: &Handle, timeout: Duration) {
    println!("nop");

    let buf: [u8; 0] = [0; 0];
    let r = handle.write_control(REQ_TYPE_AMLOUT, REQ_NOP, 0x0, 0x0, &buf, timeout);
    match r {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Nope"),
    }
}

// TODO: move to lib?
fn int_to_bool_str(v: u8) -> &'static str {
    match v {
        1 => "yes",
        _ => "no",
    }
}

pub fn info(handle: &Handle, timeout: Duration) {
    println!("Read chip information\n");
    let mut buf: [u8; 8] = [0; 8];
    match handle.read_control(
        REQ_TYPE_AMLIN,
        REQ_IDENTIFY_HOST,
        0x0,
        0x0,
        &mut buf,
        timeout,
    ) {
        Ok(_) => {
            println!("  ROM version:   {}.{}", buf[0], buf[1]);
            println!("  Stage version: {}.{}", buf[2], buf[3]);
            println!("  Need password: {}", int_to_bool_str(buf[4]));
            println!("  Password OK:   {}", int_to_bool_str(buf[5]));
            println!();
        }
        Err(e) => println!("chip_id err: {e:?}"),
    }
    // TODO: SoC/board specifics
    let is_vim1 = false; // Khadas VIM1 board
    if is_vim1 {
        println!("Chip ID:");
        read_mem(handle, timeout, CHIP_ID_ADDR_S905X, 12).unwrap();
        // CPU power states, p47
        println!("Power states (?):");
        read_mem(&handle, timeout, 0xc810_00e0, 8).unwrap();
    }
}

// We can read max. 64 bytes at a time.
pub fn read_mem(handle: &Handle, timeout: Duration, addr: u32, size: u8) -> Result<(), &'static str> {
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("read memory @{addr_h:04x}{addr_l:04x}");
    if size > 64 {
        return Err("Memory read size is 64 max");
    }
    let mut buf = vec![0; size as usize];
    match handle.read_control(
        REQ_TYPE_AMLIN,
        REQ_READ_MEM,
        addr_h,
        addr_l,
        &mut buf,
        timeout,
    ) {
        Ok(_) => {
            println!("read_mem: {buf:02x?}");
        }
        Err(e) => println!("read_mem err: {e:?}"),
    }
    Ok(())
}

pub fn write_mem(
    handle: &Handle,
    timeout: Duration,
    addr: u32,
    buf: &[u8],
) -> Result<(), &'static str> {
    let addr_l = addr as u16;
    let addr_h = (addr >> 16) as u16;
    println!("write to memory @{addr_h:04x}{addr_l:04x}");
    if buf.len() > 64 {
        return Err("Memory write size is 64 max");
    }

    match handle.write_control(REQ_TYPE_AMLOUT, REQ_WRITE_MEM, addr_h, addr_l, buf, timeout) {
        Ok(n) => {
            println!("write_mem success, {n} bytes");
        }
        Err(e) => println!("write_mem err: {e:?}"),
    }
    Ok(())
}

// The command needs 0-byte termination, hence CString.
pub fn tpl_cmd(handle: &Handle, timeout: Duration, cmd: &str) {
    println!("tpl_cmd {cmd}");
    let cmd = std::ffi::CString::new(cmd).expect("C sucks");
    let buf = cmd.as_bytes_with_nul();
    let res = handle.write_control(
        REQ_TYPE_AMLOUT,
        REQ_TPL_CMD,
        0,
        1, // aka sub code - always 1 though?
        buf,
        timeout,
    );
    println!("{res:?}");
}

// Just for reference; untested as per pyamlboot
// Password size is 64 bytes
pub fn password(handle: &Handle, timeout: Duration, buf: &[u8; 64]) {
    println!("password: {buf:02x?}");
    let r = handle.write_control(REQ_TYPE_AMLOUT, REQ_PASSWORD, 0x0, 0x0, buf, timeout);
    println!("{r:?}");
}

// NOTE: not yet working, just an attempt
pub fn password_test(handle: &Handle, timeout: Duration) {
    nop(handle, timeout);

    let pw: [u8; 64] = [0xff; 64];
    password(handle, timeout, &pw);

    /*
    let size = 16;
    let mut buf = vec![0; size as usize];
    for cmd in 0xa0..=0xff {
        println!("Try command {cmd:02x}");
        match handle.read_control(REQ_TYPE_AMLIN, cmd, 0, 0, &mut buf, timeout) {
            Ok(n) => {
                println!("res ({n}): {buf:02x?}");
            }
            Err(e) => println!("err: {e:?}"),
        }
    }
    */
}
