# Amlogic's fastboot based `adnl`

This is both the protocol name and a corresponding CLI tool, found here:
https://github.com/khadas/utils/tree/master/aml-flash-tool/tools/adnl/ 

## `tree .`

```
├── adnl
├── adnl_burn_pkg
├── asd
└── usb_flow
    ├── AmlImagePack.so
    ├── libamlfastboot.so
    ├── libaml_usb_flow.so
    ├── libaulextend.so
    └── liblua53.so
```

## `adnl` binary

Fork of fastboot with vendor specific and some hidden commands.
Based on the old C++ implementation, looking at the symbols, prior or equal to
commit [`2d08ae57`](https://android.googlesource.com/platform/system/core/+/2d08ae57d46fe1626459fd8b67bb50526f3a97ae).

mainline: https://android.googlesource.com/platform/system/core/+/refs/heads/main/fastboot

### `adnl`

```
Amlogic DNL protocol tool V[2.6.3] at Aug 20 2021
usage: adnl [ -s <specific para> ] <command> <command para>
commands:
  devices                                  List all connected dnl devices
  getvar <variable>                        Display a BL1/BL2/BL33 variable.
                                           if not supported, this does nothing.
  setvar <variable> <value>                Set a BL1 <variable> with <value>.
  setkey <keyPath>                         Set a BL1 <variable> with <value>.
  oem/cmd <cmd>                                Send a BL33 string<cmd>.
  download <FilePath> [<size> <offset>]
          Download file to RAM[, with <size>bytes from <offset>]
  run   boot code in RAM, after cmd 'download'.
  bl1_boot -f <bootloaderPath>             BL1 download and boot bl2.
  bl2_boot -f <bootloaderPath>             BL2 download and boot TPL.
  partition <-p partName>  <-f imgPath>  [<-m media>] [<-t imgType>] [-v vryFile]
            @media:    mem/store/key, default store
            @imgType:  sparse/normal, default normal
  upload <-p partName>  <-f dumpFilePath>  <-z dumpSz [K|M|G]> [<-m media>] [<-o offset>]
            @media:    mem/store/key, default store
            @offset:   start offset in part @partname, default 0
  reboot [mode]                            Reboot device [into supported mode].

other options:
  -s <specific para>                     specify a USB device, command 'devices' to get specific para
  -h, --help                             show this message.
Amlogic DNL protocol tool V[2.6.3] at Aug 20 2021
```

### `adnl devices`

```
0056000142a21c0500000000        Aml_DNL
```

This is the same as `fastboot devices`, except for the name/mode:

```
0056000142a21c0500000000        fastboot
```

### `adnl mwrite`

Hidden command found via binary ninja at Chaospott 2023-10-19 :-)

```
1f162:       e8 ae e5 ff ff          call   1d715 <_Z15fb_queue_mwritePvlPKc>
```

### gdb

```
break _ZL26match_fastboot_with_serialP12usb_ifc_infoPKc
```

```
_ZL21list_devices_callbackP12usb_ifc_info
```

### USB traffic analysis

Since the USB transfer is built-in, instead of debugging, let us sniff:

https://github.com/liquidctl/liquidctl/blob/main/docs/developer/capturing-usb-traffic.md
https://github.com/liquidctl/liquidctl/blob/main/docs/developer/techniques-for-analyzing-usb-protocols.md

### SRAM (?)

```
f700ab90
```


## `usb_flow` directory

### `file usb_flow/*`

```
usb_flow/AmlImagePack.so:    ELF 64-bit LSB shared object, x86-64, version 1 (GNU/Linux), dynamically linked, BuildID[sha1]=8307fea9b094ed7d961056ced125500ed01d082e, with debug_info, not stripped
usb_flow/libaml_usb_flow.so: ELF 64-bit LSB shared object, x86-64, version 1 (GNU/Linux), dynamically linked, BuildID[sha1]=97d0f35ffca6778a2e44208b49a9aff5f22c6e2c, not stripped
usb_flow/libamlfastboot.so:  ELF 64-bit LSB shared object, x86-64, version 1 (GNU/Linux), dynamically linked, BuildID[sha1]=21adba3502063aa1a07cdd072f9c365c9dcb2b1a, not stripped
usb_flow/libaulextend.so:    ELF 64-bit LSB shared object, x86-64, version 1 (GNU/Linux), dynamically linked, BuildID[sha1]=339c969328614dc2160c800de1611c6776f369d8, not stripped
usb_flow/liblua53.so:        ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, BuildID[sha1]=677b3d2a2e6d4d2c4b20bca3ac10fbe03d44fd0c, not stripped
```

### `ldd usb_flow/*`

```
usb_flow/AmlImagePack.so:
        linux-vdso.so.1 (0x00007ffe7f2f2000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f2589200000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f2589931000)
usb_flow/libaml_usb_flow.so:
        linux-vdso.so.1 (0x00007ffd95d67000)
        liblua53.so => not found
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f7af5000000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f7af5660000)
usb_flow/libamlfastboot.so:
        linux-vdso.so.1 (0x00007ffd82971000)
        libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f21048bb000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f2104200000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f21048df000)
usb_flow/libaulextend.so:
        linux-vdso.so.1 (0x00007ffc0a675000)
        liblua53.so => not found
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f28d6200000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f28d69f1000)
usb_flow/liblua53.so:
        linux-vdso.so.1 (0x00007ffe49ae5000)
        libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fb4ae494000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fb4ade00000)
        /lib64/ld-linux-x86-64.so.2 (0x00007fb4ae59a000)`
```

### `strings usb_flow/libaml_usb_flow.so`

#### UUIDs

No hit in Google. What might they be? Possibly GPT partitions?

```
B97A28CB-A108-4B36-9151-FE17455570F0
B97A28CB-A108-4B36-9151-FE17455570F4
```

#### Aml custom Lua stuff

`strings usb_flow/libaml_usb_flow.so | rg -i lua | rg aml | less`

No Google result for `aml_load_lua_flow.cpp`.

Neither for `aml_img_load.lua`.

```
aml_lua_state_init
aml_lua_call_loader
Fail call aml_img_load.lua, ret %d
aml_load_lua_flow.cpp
_ZZ18aml_lua_state_initPP9lua_StateE8__func__
_ZZ19aml_lua_call_loaderP9lua_StatePKcPiRiE8__func__
_ZL25l_aml_dev_get_api_versionP9lua_State
_ZZL25l_aml_dev_get_api_versionP9lua_StateE8__func__
_ZL18l_aml_dev_get_pathP9lua_State
_ZZL18l_aml_dev_get_pathP9lua_StateE8__func__
_ZL18l_aml_dev_get_typeP9lua_State
_ZZL18l_aml_dev_get_typeP9lua_StateE8__func__
...
_ZL24l_aml_key_ui_api_versionP9lua_State
_ZZL24l_aml_key_ui_api_versionP9lua_StateE8__func__
_ZL24l_aml_key_set_left_countP9lua_State
_ZZL24l_aml_key_set_left_countP9lua_StateE8__func__
_ZL18l_aml_key_dll_initP9lua_State
_ZZL18l_aml_key_dll_initP9lua_StateE8__func__
_ZL20l_aml_key_dll_uninitP9lua_State
_ZL25l_aml_key_dll_get_versionP9lua_State
_ZZL25l_aml_key_dll_get_versionP9lua_StateE8__func__
_ZL26l_aml_key_dll_need_dev_infP9lua_State
_ZZL26l_aml_key_dll_need_dev_infP9lua_StateE8__func__
_ZL25l_aml_key_dll_get_dev_infP9lua_State
_ZZL25l_aml_key_dll_get_dev_infP9lua_StateE8__func__
...
_ZL13l_aml_ui_dmsgP9lua_State
_ZZL13l_aml_ui_dmsgP9lua_StateE8__func__
_Z18aml_lua_state_exitP9lua_State
_Z19aml_lua_call_loaderP9lua_StatePKcPiRi
_Z18aml_lua_state_initPP9lua_State
_Z21l_register_aml_deviceP9lua_State
_Z18l_register_aml_keyP9lua_State
_Z20l_register_aml_imageP9lua_State
_Z17l_register_aml_uiP9lua_State
```

The ELF is built for x86. What do they use Lua for, maybe testing?

