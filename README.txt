a rust tool to poke at IT8987 EC found in x1e80100 machines, including
lenovo yoga slim 7x extensions

Common Registers:
0x20: Fan EC Interface status???
0x21: Fan status
0x22: Fan speed

0x23: Modern standby status
0x24: Fan profile
0x25: Fan trip points
0x26: Number of profiles
0x27: Number of LUTs
0x28: LUT data

(I2C6._DSM, 0, DSME, 0x06, <thermistor>):
0x29: Thermistor 1 (CPU+GPU), TZ39
0x2a: Thermistor 2 (Battery / Battery controller), TZ40
0x2b: Thermistor 3 (GPU), TZ41
0x2c: Thermistor 4 (GPU), TZ42
0x2d: Thermistor 5 (CPU+GPU), TZ43
0x2e: Thermistor 6 (GPU), TZ44
0x2f: Thermistor 7 (GPU), TZ45

0x30: Fan debug mode
0x32: Thermistor threshold
0x34: Thermistor sample rate
0x35: IRQ Enable/disable

Slim7x extensions:
0x70: Caps lock LED
0x71: Set keyboard ID
0x72: Get keyboard ID
0x73: Power and novo button test
0x74: Cold boot test
0x77: Power and novo button status
0x81: "SetFeatureValue" WMI
0x82: Smart fan mode
0x83: Keyboard status. 4: auto, 2: half-bright, 3: full-bright, 1: off
0x84: Mic-mute LED: 1: on, 2: off
0x88: Key debug?
0x89: Key debug?
0x8e: AI/NPU
0x8f: unknown, referenced in dsdt
0x90: AC status

Asus vivobook s15 extensions:
0x0e: ???
0x50: ???
0x51: ???
0x52: ???
0x54: ???
0x60: ???
0x61: ???
0x62: ???

Interrupt handling in DSDT:

Fan status change:
\_SB.I2C6._DSM (DSME, 0x00, 0x01, <fan number>)
Which then reads status and stores it in FECI and FAN1/FAN2 device

Fan speed change:
\_SB.I2C6._DSM (DSME, 0x00, 0x02, <fan number>)
Which then reads status and stores it in FECI and FAN1/FAN2 device

EC LUT update complete:
\_SB.I2C6._DSM (DSME, 0x00, 0x03, 0x00)
Stores 0 into FECI.LUTC and notifies FECI with 0x85

EC Fan profile switch completed:
\_SB.I2C6._DSM (DSME, 0x00, 0x04, 0x00)
Stores 0 into FECI.LUTC and FECI.LPSC, notifies FECI with 0x84

Thermistor temperature threshold cross:
Notify (\_SB.TZ.., 0x81) // Thermal Trip Point Change

EC Recovery from reset:
calls \_SB.I2C6._DSM (DSME, Zero, EF05, Zero), which is "Fan Debug Mode"

Lenovo extensions:

F4, LSK, FnQ, FnM PTP, FnR, Fnlk on, Fnlk off, FnN:
Stores a key value into LSK2 and notifies WMI

Fn Space:
Reads from 0x83, stores value + 0x3f into LSK2 and notifies WMI

AI:
Invokes SMBus block process with reg 0x8e and arg 0x05.
Stores the result into LAIM and notifies WMI

NPU:
Invokes SMBus block process with reg 0x8e and arg 0x08.
Stores the result into LAIM and notifies WMI

notes:
- Lenovo Yoga Slim7x and Honor Magicbook Art 14 seem to have a somewhat-compatible firmware with random extensions
- Microsoft Surface laptops have a completely different firmware
- ThinkPad T14s also appears to have classic H8 EC found in x86 ThinkPads

mysteries:
- why do some devices reference 0x76, and others 0x36
