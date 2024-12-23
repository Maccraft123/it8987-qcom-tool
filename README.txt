a rust tool to poke at IT8987 EC found in x1e80100 machines, including
lenovo yoga slim 7x extensions

Common Registers:
0x20: "Temp"
0x21: Fan status
0x22: Fan speed
0x23: Modern standby status
0x24: Fan profile
0x25: Fan trip points
0x26: Number of profiles
0x27: Number of LUTs
0x28: LUT data
0x29: Thermistor 1
0x2a: Thermistor 2
0x2b: Thermistor 3
0x2c: Thermistor 4
0x2d: Thermistor 5
0x2e: Thermistor 6
0x2f: Thermistor 7
0x30: Fan debug mode
0x32: Thermistor treshold
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
Thermistor temperature treshold cross:
Notify (\_SB.TZ.., 0x81) // Thermal Trip Point Change

EC Recovery from reset:
calls \_SB.I2C6._DSM (DSME, Zero, EF05, Zero), which is "Fan Debug Mode"

AI/NPU:
0x8E register read + WMI notification

notes:
- Lenovo Yoga Slim7x and Honor Magicbook Art 14 seem to have a somewhat-compatible firmware with random extensions
- Microsoft Surface laptops have a completely different firmware
- ThinkPad T14s also has the classic H8 EC found in x86 ThinkPads

mysteries:
- why do some devices reference 0x76, and others 0x36
