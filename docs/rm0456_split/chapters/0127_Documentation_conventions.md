RM0456 Rev 6

RM0456

Documentation conventions

1

Documentation conventions

1.1

General information
The STM32U5 series devices have an Arm®(a) Cortex®-M33 core, with Arm® TrustZone®.

1.2

List of abbreviations for registers
The following abbreviations(b) are used in register descriptions:

read/write (rw)

Software can read and write to this bit.

read-only (r)

Software can only read this bit.

write-only (w)

Software can only write to this bit. Reading this bit returns the reset value.

read/clear write0 (rc_w0)

Software can read as well as clear this bit by writing 0. Writing 1 has no
effect on the bit value.

read/clear write1 (rc_w1)

Software can read as well as clear this bit by writing 1. Writing 0 has no
effect on the bit value.

read/clear write (rc_w)

Software can read as well as clear this bit by writing to the register. The
value written to this bit is not important.

read/clear by read (rc_r)

Software can read this bit. Reading this bit automatically clears it to 0.
Writing this bit has no effect on the bit value.

read/set by read (rs_r)

Software can read this bit. Reading this bit automatically sets it to 1.
Writing this bit has no effect on the bit value.

read/set (rs)

Software can read as well as set this bit. Writing 0 has no effect on the bit
value.

read/write once (rwo)

Software can only write once to this bit and can also read it at any time.
Only a reset can return the bit to its reset value.

toggle (t)

The software can toggle this bit by writing 1. Writing 0 has no effect.

read-only write trigger (rt_w1) Software can read this bit. Writing 1 triggers an event but has no effect on
the bit value.
Reserved (Res.)

1.3

Reserved bit, must be kept at reset value.

Register reset value
Bits (binary notation) or bits nibbles (hexadecimal notation) of which the reset value is
undefined are marked as X.

a. Arm is a registered trademark of Arm Limited (or its subsidiaries) in the US and/or elsewhere.
b. This is an exhaustive list of all abbreviations applicable to STMicroelectronics microcontrollers, some of
them may not be used in the current document.

RM0456 Rev 6

<!-- pagebreak -->

