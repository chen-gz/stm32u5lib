363

Embedded flash memory (FLASH)

RM0456

If an error occurs during a secure or nonsecure program or erase operation, one of the
following programming error flags is set:
•

nonsecure programming error flags: PROGERR, SIZERR, PGAERR, PGSERR,
OPTWRERR, or WRPERR is set in FLASH_NSSR.
If ERRIE is set in FLASH_NSCR, an interrupt is generated and the operation error flag
OPERR is set in the FLASH_NSSR register.

•

Secure programming error flags: PROGERR, SIZERR, PGAERR, PGSERR, or
WRPERR is set in FLASH_SECSR.
If ERRIE is set in FLASH_SECCR, an interrupt is generated and the operation error
flag OPERR is set in FLASH_SECSR.

Note:

If several successive errors are detected (for example, in case of DMA transfer to the flash
memory), the error flags cannot be cleared until the end of the successive write requests.
Any programming error flushes the write buffer.

7.3.10

Read-while-write (RWW)
The flash memory is divided into two banks allowing read-while-write operations.
This feature allows a read operation to be performed from one bank while erase or program
operation is performed to the other bank.

Note:

Write-while-write operations are not allowed. As an example, It is not possible to perform
an erase operation on one bank while programming the other one.

Read from bank 1 while page erasing in bank 2 (or vice versa)
While executing a program code from bank 1, it is possible to perform a page erase
operation on bank 2 (and vice versa).

Read from bank 1 while mass erasing bank 2 (or vice versa)
While executing a program code from bank 1, it is possible to perform a mass erase
operation on bank 2 (and vice versa).

Read from bank 1 while programming bank 2 (or vice versa)
While executing a program code from bank 1, it is possible to perform a program operation
on the bank 2 (and vice versa).
Note:

Due to the Cortex-M33 unified C-Bus, the user software must ensure to not stall C-Bus with
multiple consecutive writes. It is recommended to wait for the BSY flag to be cleared before
programming the next quad-word.

7.3.11

Power-down during FLASH programming or erase operation
The contents of the flash memory currently being accessed are not guaranteed
if a power-down occurs during a flash memory program or erase operation.

7.3.12

Reset during FLASH programming or erase operation
The contents of the flash memory currently being accessed are not guaranteed if a reset
occurs during a flash memory program or erase operation. The status of the flash memory
can be recovered from FLASH_OPSR when a system reset occurs during a flash memory
program or erase operation.

<!-- pagebreak -->

