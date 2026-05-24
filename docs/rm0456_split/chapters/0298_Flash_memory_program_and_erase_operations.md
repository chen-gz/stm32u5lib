363

Embedded flash memory (FLASH)

RM0456

Waking up both bank 1 and bank 2 is done in one of the following cases:
•

upon a valid mass erase

•

upon an option byte modification

•

upon an option byte loading

•

upon system reset

Note:

The software can reduce the flash bank wake-up time by enabling HSI16 before waking up
the bank.

7.3.5

Flash memory program and erase operations
The embedded flash memory can be programmed using in-circuit programming (ICP) or
in-application programming (IAP).
The ICP method is used to update the entire contents of the flash memory, using the JTAG,
SWD protocol, or the bootloader to load the user application into the microcontroller. The
ICP offers quick and efficient design iterations, and eliminates unnecessary package
handling or socketing of devices.
The IAP can use any communication interface supported by the microcontroller (such as
I/Os, USB, CAN, UART, I2C, or SPI) to download programming data into the memory.
The IAP allows the user to reprogram the flash memory while the application is running. Part
of the application must have been previously programmed in the flash memory using ICP.
An ongoing flash memory operation does not block the CPU as long as the CPU does not
access the same flash memory bank. Code or data fetches are possible on one bank while
a write/erase operation is performed to the other bank (refer to Section 7.3.10).
On the contrary, during a program/erase operation to the flash memory, any attempt to read
the same flash memory bank stalls the bus. The read operation proceeds correctly once the
program/erase operation has been completed.
The MCU supports TrustZone that defines secure and nonsecure areas in the flash
memory. All program and erase operations can be performed in secure mode through the
secure registers or in nonsecure mode through the nonsecure registers.
For more information, refer to Section 7.5.

Unlock the secure/nonsecure FLASH control registers
After reset, write is not allowed in FLASH_SECCR and FLASH_NSCR in order to protect
the flash memory against possible unwanted operations (due, for example, to electric
disturbances).
The following sequence is used to unlock these registers:
1.

Write KEY1 = 0x45670123 in FLASH_SECKEYR or FLASH_NSKEYR.

2.

Write KEY2 = 0xCDEF89AB in FLASH_SECKEYR or FLASH_NSKEYR).

Any wrong sequence locks up FLASH_SECCR or FLASH_NSCR until the next system
reset. In the case of a wrong key sequence, a bus error is detected and a HardFault
interrupt is generated.
FLASH_NSCR (resp. FLASH_SECCR) can be locked again by software by setting LOCK in
FLASH_NSCR (resp. FLASH_SECCR).

<!-- pagebreak -->

