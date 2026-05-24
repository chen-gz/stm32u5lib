•

Main memory block organized as two banks of up to 2 Mbytes each containing
up to 256 pages of 8 Kbytes

•

An information block containing:
–

32 Kbytes for system memory. This area is immutable and reserved for use by
STMicroelectronics. It contains the bootloader that is used to reprogram the flash
memory through one of the user communication interfaces such as USB (DFU).
The system memory is programmed by STMicroelectronics when the device is
manufactured. For further details, refer to the application note STM32
microcontroller system memory boot mode (AN2606).

–

32 Kbytes immutable secure area containing the root security services (RSS and
RSS library) developed by STMicroelectronics

–

512 bytes OTP (one-time programmable) bytes for user data (32 quad-words).
The OTP data cannot be erased and can be written only once.

–

option bytes for user configuration. Unlike user flash memory and system memory,
it is not mapped to any memory address and can be accessed only through the
flash register interface.

RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)
The memory organization is based on a main area and an information block as shown in the
tables below.
Table 51. Flash module 512-Kbyte dual-bank organization for STM32U535/545(1)
Flash area

Bank 1

Main memory

Bank 2

Nonsecure information block

Secure information block

Flash memory address

Size

Name

0x0800 0000 - 0x0800 1FFF

8 Kbytes

Page 0

0x0800 2000 - 0x0800 3FFF

8 Kbytes

Page 1

...

...

...

0x0803 C000 - 0x803 DFFF

8 Kbytes

Page 30

0x0803 E000 - 0x803 FFFF

8 Kbytes

Page 31

0x0804 0000 - 0x0804 3FFF

8 Kbytes

Page 0

0x00804 2000 - 0x0804 3FFF

8 Kbytes

Page 1

...

...

...

0x0807 C000 - 0x0807 DFFF

8 Kbytes

Page 30

0x0807 E000 - 0x0807 FFFF

8 Kbytes

Page 31

0x0BF9 0000 - 0x0BF9 7FFF

32 Kbytes

System memory

0x0BFA 0000 - 0x0BFA 01FF

512 bytes

OTP area

0x0FF8 0000 - 0x0FF8 5FFF

24 Kbytes

RSS

0x0FF8 6000 - 0x0FF8 7FFF

8 Kbytes

RSS library

0x0FFA 0000 - 0x0FFA 01FF

512 bytes

OTP area alias

1. When DUALBANK = 1 in option bytes, the bank 2 base address is 0x0802 0000 for 256-Kbyte, and
0x0801 0000 for 128-Kbyte dual-bank STM32U535/545 devices.

Table 52. Flash module 2-Mbyte dual-bank organization for STM32U575/585(1)
Flash area

Bank 1

Main memory

Bank 2

Nonsecure information block

Secure information block

Flash memory address

Size

Name

0x0800 0000 - 0x0800 1FFF

8 Kbytes

Page 0

0x0800 2000 - 0x0800 3FFF

8 Kbytes

Page 1

...

...

...

0x080F E000 - 0x080F FFFF

8 Kbytes

Page 127

0x0810 0000 - 0x0810 1FFF

8 Kbytes

Page 0

0x0810 2000 - 0x0810 3FFF

8 Kbytes

Page 1

...

...

...

0x081F E000 - 0x081F FFFF

8 Kbytes

Page 127

0x0BF9 0000 - 0x0BF9 7FFF

32 Kbytes

System memory

0x0BFA 0000 - 0x0BFA 01FF

512 bytes

OTP area

0x0FF8 0000 - 0x0FF8 5FFF

24 Kbytes

RSS

0x0FF8 6000 - 0x0FF8 7FFF

8 Kbytes

RSS library

0x0FFA 0000 - 0x0FFA 01FF

512 bytes

OTP area alias

RM0456 Rev 6

<!-- pagebreak -->

