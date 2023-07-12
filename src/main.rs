use std::time::Duration;

const USB_VID_AMLOGIC: u16 = 0x1b8e;
const USB_PID_S905X3: u16 = 0xc003;
const USB_PID_S905X4: u16 = 0xc004;

// see https://vovkos.github.io/doxyrest/samples/libusb-sphinxdoc/enum_libusb_endpoint_direction.html#doxid-group-libusb-desc-1ga86c880af878493aa8f805c2aba654b8b
// IN
const REQ_TYPE_AMLIN: u8 = 0xc0;
// OUT
const REQ_TYPE_AMLOUT: u8 = 0x40;

const REQ_READ_MEM: u8 = 0x02;

const REQ_IDENTIFY_HOST: u8 = 0x20;

const REQ_TPL_CMD: u8 = 0x30;
const REQ_PASSWORD: u8 = 0x35;
const REQ_NOP: u8 = 0x36;

// This is on a TV box based on S905X4
const FB_ADDR: u32 = 0x7f80_0000;

fn int_to_bool_str(v: u8) -> &'static str {
    match v {
        1 => "yes",
        _ => "no",
    }
}

fn main() {
    println!("Searching for Amlogic USB devices...");
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        let vid = device_desc.vendor_id();
        let pid = device_desc.product_id();

        if vid == USB_VID_AMLOGIC && (pid == USB_PID_S905X3 || pid == USB_PID_S905X4) {
            let s_type = if pid == USB_PID_S905X3 {
                "S905X, S905X2 or S905X3"
            } else {
                "S905X4"
            };
            println!(
                "Found {vid:04x}:{pid:04x} ({s_type}) on bus {:03}, device {:03}",
                device.bus_number(),
                device.address(),
            );

            let timeout = Duration::from_millis(2500);
            let handle = device.open();

            match handle {
                Ok(handle) => {
                    if true {
                        println!("nop");
                        let buf: [u8; 0] = [0; 0];
                        let r =
                            handle.write_control(REQ_TYPE_AMLOUT, REQ_NOP, 0x0, 0x0, &buf, timeout);
                        match r {
                            Ok(_) => println!("Ok"),
                            Err(_) => println!("Nope"),
                        }
                    }
                    if true {
                        println!("read chip ID");
                        let mut buf: [u8; 8] = [0; 8];
                        let chip_id = handle.read_control(
                            REQ_TYPE_AMLIN,
                            REQ_IDENTIFY_HOST,
                            0x0,
                            0x0,
                            &mut buf,
                            timeout,
                        );
                        match chip_id {
                            Ok(_) => {
                                println!("ROM version:   {}.{}", buf[0], buf[1]);
                                println!("Stage version: {}.{}", buf[2], buf[3]);
                                println!("Need password: {}", int_to_bool_str(buf[4]));
                                println!("Password OK:   {}", int_to_bool_str(buf[5]));
                            }
                            Err(e) => println!("chip_id err: {e:?}"),
                        }
                    }
                    if false {
                        println!("read FB memory");
                        let addr = FB_ADDR;
                        let mut buf: [u8; 64] = [0; 64];
                        let fb_mem = handle.read_control(
                            REQ_TYPE_AMLIN,
                            REQ_READ_MEM,
                            (addr >> 16) as u16,
                            addr as u16,
                            &mut buf,
                            timeout,
                        );
                        match fb_mem {
                            Ok(_) => {
                                println!("fb_mem: {buf:?}");
                            }
                            Err(e) => println!("fb_mem err: {e:?}"),
                        }
                    }
                    if false {
                        println!("password");
                        // password size is 64 bytes
                        let buf: [u8; 64] = [0; 64];
                        let r = handle.write_control(
                            REQ_TYPE_AMLOUT,
                            REQ_PASSWORD,
                            0x0,
                            0x0,
                            &buf,
                            timeout,
                        );
                        println!("{r:?}");
                    }
                    if false {
                        println!("fastboot");
                        let sub_code: u8 = 1;
                        let cmd = std::ffi::CString::new("fastboot").expect("C sucks");
                        let buf = cmd.as_bytes_with_nul();
                        let res = handle.write_control(
                            REQ_TYPE_AMLOUT,
                            REQ_TPL_CMD,
                            0,
                            sub_code as u16,
                            buf,
                            timeout,
                        );
                        println!("{res:?}");
                    }
                }
                Err(e) => {
                    println!("Error opening USB device {e:?}");
                }
            }
        }
    }
}
