# AML GX-CHIP protocol reversing

See existing notes in `pyamlboot` for known-so-far commands:
<https://github.com/superna9999/pyamlboot/blob/master/PROTOCOL.md>

The following assembly excerpts are `objdump`ed from the vendor tool `update`;
specifically, commit hash `79e367b56cc203895350a436fae48e0c02bcf0a0` from
<https://github.com/althafvly/aml-flash-tool>.

## First look

The vendor tool uses `libusb`, as we can see via `ldd update`:

```
        linux-vdso.so.1 (0x00007ffda51f4000)
        libusb-0.1.so.4 => /lib/x86_64-linux-gnu/libusb-0.1.so.4 (0x00007f4ebd53c000)
        libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f4ebd200000)
        /lib64/ld-linux-x86-64.so.2 (0x00007f4ebd568000)
```

Let's see what it does now.

### `update`

```
====>Amlogic update USB tool(Ver 1.7.2) 2018/04<=============
update  <command>       [device name]   <arg0>  <arg1>  <arg2>  ...

Common Commands:
update <partition>: Burn a partition with a partition image
update <mwrite>   : Burn data to media or memory
update <mread>    : Dump a data from media or memory to pc and save as a file
update <tplcmd>   : like bulkcmd
update <bulkcmd>  : pass and exec a command platform bootloader can support
update <write>    : Down a file to memory
update <run>      : Run code from memory address
update <read>     : Dump data from memory:
update <wreg>     : set one 32bits reg:
update <rreg>     : Dump data from reg:
update <password> : unlock chip:
update <chipinfo> : get chip info at page index:
update <chipid>   : get chip id
update <bl2_boot> : boot fip format u-boot.bin
...
```

Those two `chipid` and `chipinfo` look interesting. The former sounds familiar.

### `update chipid`

```
AmlUsbIdentifyHost
This firmware version is 3-2-0-0
[update]idVer is 0x302
[update]get chpid by chip info page
ChipID is:0x505046363434080000060a01
```

That is what we already have, too.

### `update chipinfo`

```
[update]ERR(L1117):paraNum(2) too small for chipinfo
```

This command require another argument, apparently an index. Let's start at `0`.

Note: The first 4 bytes look like ASCII characters. They are in reverse order
(little endian). In the following, they are human-readable in parentheses.

#### `update chipinfo 0` (INDX)

```
00000000: 58444e49 0000000f 00000000 00000000
00000010: 00000000 00000000 00000000 00000000
00000020: 00000000 00000000 00000000 00000000
00000030: 00000000 00000000 00000000 00000000
```

#### `update chipinfo 1` (CHIP)

```
00000000: 50494843 0000002b 11111113 00720040
00000010: 01111111 36465050 00083434 010a0600
00000020: 00000000 a0f83180 20282000 00000367
00000030: 00000000 00000000 00000000 00000000
```

second row bytes 4-15 are chip ID

#### `update chipinfo 2` (OPS_)

```
00000000: 5f53504f 00000000 00000000 00000000
00000010: 00000000 00000000 00000000 00000000
00000020: 00017201 36465050 00083434 010a0600
00000030: 2298fa40 80068091 7ea9aa01 300016a9
```

#### `update chipinfo 3` (ROMV)

```
00000000: 564d4f52 00000000 00000000 00000000
00000010: 00000000 00000000 00000000 00000000
00000020: 00000000 00000000 00000000 00000000
00000030: 00000000 00000000 00000000 00000000
```

A lot of the above looks empty, some bits are unknown. That's for another day.

Let's now look at some assembly: `objdump -D update`

## Read memory

`_Z22IOCTL_READ_MEM_Handler12usbDevIoCtrl` - we know this one, let's see...

```
  4030e6:       41 89 c8                mov    %ecx,%r8d
  4030e9:       89 d1                   mov    %edx,%ecx
  4030eb:       ba 02 00 00 00          mov    $0x2,%edx
  4030f0:       be c0 00 00 00          mov    $0xc0,%esi
  4030f5:       48 89 c7                mov    %rax,%rdi
  4030f8:       e8 d3 f5 ff ff          call   4026d0 <usb_control_msg@plt>
```

Those values are familiar from existing protocol documentation in `pyamlboot`:

| reg | val  |                           |
| --- | ---- | ------------------------- |
| rdx | 0x02 | bRequest (read memory)    |
| rsi | 0xc0 | bmRequestType (data in)   |

## Get chip info

`_Z23Aml_Libusb_get_chipinfoPvPciii` does not ring a bell.
So let's see and apply our knowledge! (`!` and `>` are annotations)

```
  40481e:       b8 14 fd ff ff          mov    $0xfffffd14,%eax
  404823:       eb 6b                   jmp    404890 <_Z23Aml_Libusb_get_chipinfoPvPciii+0x119>
  404825:       48 8b 45 e8             mov    -0x18(%rbp),%rax
  404829:       48 89 45 d8             mov    %rax,-0x28(%rbp)
  40482d:       83 7d 9c 40             cmpl   $0x40,-0x64(%rbp)
! 404831:       7e 22                   jle    404855 <_Z23Aml_Libusb_get_chipinfoPvPciii+0xde>
  404833:       8b 45 9c                mov    -0x64(%rbp),%eax
  404836:       89 c2                   mov    %eax,%edx
  404838:       be b0 a2 49 00          mov    $0x49a2b0,%esi
  40483d:       bf 80 9f 49 00          mov    $0x499f80,%edi
  404842:       b8 00 00 00 00          mov    $0x0,%eax
  404847:       e8 3a e6 ff ff          call   402e86 <_Z10aml_printfPKcz>
  40484c:       c7 45 b0 0d fd ff ff    movl   $0xfffffd0d,-0x50(%rbp)
  404853:       eb 2c                   jmp    404881 <_Z23Aml_Libusb_get_chipinfoPvPciii+0x10a>
> 404855:       4c 8b 4d d0             mov    -0x30(%rbp),%r9
  404859:       44 8b 45 c4             mov    -0x3c(%rbp),%r8d
  40485d:       8b 4d c0                mov    -0x40(%rbp),%ecx
  404860:       8b 55 b8                mov    -0x48(%rbp),%edx
  404863:       8b 75 b4                mov    -0x4c(%rbp),%esi
  404866:       48 8b 45 d8             mov    -0x28(%rbp),%rax
  40486a:       8b 7d 94                mov    -0x6c(%rbp),%edi
  40486d:       57                      push   %rdi
  40486e:       8b 7d 9c                mov    -0x64(%rbp),%edi
  404871:       57                      push   %rdi
  404872:       48 89 c7                mov    %rax,%rdi
! 404875:       e8 56 de ff ff          call   4026d0 <usb_control_msg@plt>
```

=> `gdb ./update`

- set breakpoint: `break usb_control_msg@plt`
- `run chipinfo 1`

```
Breakpoint 1, 0x00000000004026d0 in usb_control_msg@plt ()
[ Legend: Modified register | Code | Heap | Stack | String ]
────────────────────────────────────────────────────────────────────────────── registers ────
$rax   : 0x00000000006e7980  →  0x0000000000000003
$rbx   : 0x1
$rcx   : 0x0
$rdx   : 0x40
$rsp   : 0x00007fffffffd878  →  0x000000000040487a  →  <Aml_Libusb_get_chipinfo(void*,+0> add rsp, 0x10
$rbp   : 0x00007fffffffd900  →  0x00007fffffffd960  →  0x00007fffffffdd60  →  0x0000000000000003
$rsi   : 0xc0
$rdi   : 0x00000000006e7980  →  0x0000000000000003
$rip   : 0x00000000004026d0  →  <usb_control_msg@plt+0> jmp QWORD PTR [rip+0x2cbb6a]        # 0x6ce240 <usb_control_msg@got.plt>
$r8    : 0x1
$r9    : 0x00000000006e6730  →  0x00000000006e6306  →  0x0000000000000000
$r10   : 0x0
$r11   : 0x246
$r12   : 0x00007fffffffde78  →  0x00007fffffffe1d9  →  "/home/dama/firmware/Amlogic/aml-flash-tool/tools/l[...]"
$r13   : 0x000000000040cc46  →  <main+0> push rbp
$r14   : 0x0
$r15   : 0x00007ffff7ffd040  →  0x00007ffff7ffe2e0  →  0x0000000000000000
$eflags: [ZERO carry PARITY adjust sign trap INTERRUPT direction overflow resume virtualx86 identification]
$cs: 0x33 $ss: 0x2b $ds: 0x00 $es: 0x00 $fs: 0x00 $gs: 0x00
────────────────────────────────────────────────────────────────────────────────── stack ────
0x00007fffffffd878│+0x0000: 0x000000000040487a  →  <Aml_Libusb_get_chipinfo(void*,+0> add rsp, 0x10   ← $rsp
0x00007fffffffd880│+0x0008: 0x0000000000000040 ("@"?)
0x00007fffffffd888│+0x0010: 0x0000000000001f40
0x00007fffffffd890│+0x0018: 0x00001f4000000000
0x00007fffffffd898│+0x0020: 0x0000004000000001
0x00007fffffffd8a0│+0x0028: 0x00000000006e6730  →  0x00000000006e6306  →  0x0000000000000000
0x00007fffffffd8a8│+0x0030: 0x00000000006f0960  →  0x00000000006f1a90  →  0x00000000006f3370  →  0x00000000006f50f0  →  0x0000000000000000
0x00007fffffffd8b0│+0x0038: 0x000000c000000000
──────────────────────────────────────────────────────────────────────────── code:x86:64 ────
     0x4026c0 <__strtof_l@plt+0> jmp    QWORD PTR [rip+0x2cbb72]        # 0x6ce238 <__strtof_l@got.plt>
     0x4026c6 <__strtof_l@plt+6> push   0x44
     0x4026cb <__strtof_l@plt+11> jmp    0x402270
 →   0x4026d0 <usb_control_msg@plt+0> jmp    QWORD PTR [rip+0x2cbb6a]        # 0x6ce240 <usb_control_msg@got.plt>
     0x4026d6 <usb_control_msg@plt+6> push   0x45
     0x4026db <usb_control_msg@plt+11> jmp    0x402270
     0x4026e0 <tolower@plt+0>  jmp    QWORD PTR [rip+0x2cbb62]        # 0x6ce248 <tolower@got.plt>
     0x4026e6 <tolower@plt+6>  push   0x46
     0x4026eb <tolower@plt+11> jmp    0x402270
──────────────────────────────────────────────────────────────────────────────── threads ────
[#0] Id 1, Name: "update", stopped 0x4026d0 in usb_control_msg@plt (), reason: BREAKPOINT
────────────────────────────────────────────────────────────────────────────────── trace ────
[#0] 0x4026d0 → usb_control_msg@plt()
[#1] 0x40487a → Aml_Libusb_get_chipinfo(void*, char*, int, int, int)()
[#2] 0x40b46b → update_sub_cmd_get_chipinfo(AmlUsbRomRW&, char const**, int)()
[#3] 0x40d345 → main()
```

Educated guess: `rbx` and `rcx` make up the argument.

| reg | val  |                           |
| --- | ---- | ------------------------- |
| rbx | 0x01 | argument low double byte  |
| rcx | 0x00 | argument high double byte |
| rdx | 0x40 | bRequest (get chip info)  |
| rsi | 0xc0 | bmRequestType (data in)   |

Ported to our Rust code - yea, it works!

### `aml_boot -c chip-info`

```
Searching for Amlogic USB devices...
Found 1b8e:c003 (S905X, S905X2 or S905X3) on bus 003, device 103
Product string: GX-CHIP

=======

Read chip information

- INDX
    58444e49 0000000f 00000000 00000000
    00000000 00000000 00000000 00000000
    00000000 00000000 00000000 00000000
    00000000 00000000 00000000 00000000

- CHIP
    50494843 0000002b 11111113 00720040
    01111111 36465050 00083434 010a0600
    00000000 a0f83180 20282000 00000367
    00000000 00000000 00000000 00000000

- OPS_
    5f53504f 00000000 00000000 00000000
    00000000 00000000 00000000 00000000
    00017201 36465050 00083434 010a0600
    2298fa40 80068091 7ea9aa01 300016a9

- ROM version
    564d4f52 00000000 00000000 00000000
    00000000 00000000 00000000 00000000
    00000000 00000000 00000000 00000000
    00000000 00000000 00000000 00000000
```
