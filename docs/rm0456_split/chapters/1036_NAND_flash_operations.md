1045

Flexible static memory controller (FSMC)

27.7.4

RM0456

NAND flash operations
The command latch enable (CLE) and address latch enable (ALE) signals of the NAND
flash memory device are driven by address signals from the FMC controller. This means
that to send a command or an address to the NAND flash memory, the CPU has to perform
a write to a specific address in its memory space.
A typical page read operation from the NAND flash device requires the following steps:

27.7.5

1.

Program and enable the corresponding memory bank by configuring the FMC_PCR
and FMC_PMEM (and for some devices, FMC_PATT, see Section 27.7.5: NAND flash
prewait functionality) registers according to the characteristics of the NAND flash
memory (PWID bits for the data bus width of the NAND flash, PTYP = 1, PWAITEN = 0
or 1 as needed, see Section 27.5.2: NAND flash memory address mapping for timing
configuration).

2.

The CPU performs a byte write to the common memory space, with data byte equal to
one flash command byte (for example 0x00 for Samsung NAND flash devices). The LE
input of the NAND flash memory is active during the write strobe (low pulse on NWE),
thus the written byte is interpreted as a command by the NAND flash memory. Once
the command is latched by the memory device, it does not need to be written again for
the following page read operations.

3.

The CPU can send the start address (STARTAD) for a read operation by writing four
bytes (or three for smaller capacity devices), STARTAD[7:0], STARTAD[16:9],
STARTAD[24:17] and finally STARTAD[25] (for 64 Mb x 8 bit NAND flash memories) in
the common memory or attribute space. The ALE input of the NAND flash device is
active during the write strobe (low pulse on NWE), thus the written bytes are
interpreted as the start address for read operations. Using the attribute memory space
makes it possible to use a different timing configuration of the FMC, which can be used
to implement the prewait functionality needed by some NAND flash memories (see
details in Section 27.7.5: NAND flash prewait functionality).

4.

The controller waits for the NAND flash memory to be ready (R/NB signal high), before
starting a new access to the same or another memory bank. While waiting, the
controller holds the NCE signal active (low).

5.

The CPU can then perform byte read operations from the common memory space to
read the NAND flash page (data field + Spare field) byte by byte.

6.

The next NAND flash page can be read without any CPU command or address write
operation. This can be done in three different ways:
–

by simply performing the operation described in step 5

–

a new random address can be accessed by restarting the operation at step 3

–

a new command can be sent to the NAND flash device by restarting at step 2

NAND flash prewait functionality
Some NAND flash devices require that, after writing the last part of the address, the
controller waits for the R/NB signal to go low. (see Figure 140).

<!-- pagebreak -->

