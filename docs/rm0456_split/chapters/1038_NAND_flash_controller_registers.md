1045

Flexible static memory controller (FSMC)

RM0456

These two ECC blocks are identical and associated with Bank 2 and Bank 3. As a
consequence, no hardware ECC computation is available for memories connected to
Bank 4.
The ECC algorithm implemented in the FMC can perform 1-bit error correction and 2-bit
error detection per 256, 512, 1 024, 2 048, 4 096 or 8 192 bytes read or written from/to the
NAND flash memory. It is based on the Hamming coding algorithm and consists in
calculating the row and column parity.
The ECC modules monitor the NAND flash data bus and read/write signals (NCE and NWE)
each time the NAND flash memory bank is active.
The ECC operates as follows:
•

When accessing NAND flash memory bank 2 or bank 3, the data present on the
D[15:0] bus is latched and used for ECC computation.

•

When accessing any other address in NAND flash memory, the ECC logic is idle, and
does not perform any operation. As a result, write operations to define commands or
addresses to the NAND flash memory are not taken into account for ECC computation.

Once the desired number of bytes has been read/written from/to the NAND flash memory by
the host CPU, the FMC_ECCR registers must be read to retrieve the computed value. Once
read, they must be cleared by resetting the ECCEN bit to ‘0’. To compute a new data block,
the ECCEN bit must be set to one in the FMC_PCR registers.
To perform an ECC computation:

27.7.7

1.

Enable the ECCEN bit in the FMC_PCR register.

2.

Write data to the NAND flash memory page. While the NAND page is written, the ECC
block computes the ECC value.

3.

Read the ECC value available in the FMC_ECCR register and store it in a variable.

4.

Clear the ECCEN bit and then enable it in the FMC_PCR register before reading back
the written data from the NAND page. While the NAND page is read, the ECC block
computes the ECC value.

5.

Read the new ECC value available in the FMC_ECCR register.

6.

If the two ECC values are the same, no correction is required, otherwise there is an
ECC error and the software correction routine returns information on whether the error
can be corrected or not.

NAND flash controller registers
NAND flash control registers (FMC_PCR)
Address offset: 0x80
Reset value: 0x0000 0018

31

30

29

28

27

26

25

24

23

22

21

20

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

TAR[2:0]
rw

rw

<!-- pagebreak -->

11

10

9

TCLR[3:0]
rw

rw

rw

rw

8

7

6

Res.

Res.

ECCEN

rw

rw

RM0456 Rev 6

5

4

PWID[1:0]
rw

rw

19

18

17

ECCPS[2:0]

TAR3

rw

rw

rw

3

2

1

PTYP PBKEN PWAITEN
rw

rw

16

rw

rw
0
Res.

RM0456

Flexible static memory controller (FSMC)

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:17 ECCPS[2:0]: ECC page size
Defines the page size for the extended ECC:
000: 256 bytes
001: 512 bytes
010: 1024 bytes
011: 2048 bytes
100: 4096 bytes
101: 8192 bytes
Bits 16:13 TAR[3:0]: ALE to RE delay
Sets time from ALE low to RE low in number of AHB clock cycles (HCLK).
Time is: t_ar = (TAR + SET + 2) × THCLK where THCLK is the HCLK clock period
0000: 1 HCLK cycle (default)
1111: 16 HCLK cycles
Note: SET is MEMSET or ATTSET according to the addressed space.
Bits 12:9 TCLR[3:0]: CLE to RE delay
Sets time from CLE low to RE low in number of AHB clock cycles (HCLK).
Time is t_clr = (TCLR + SET + 2) × THCLK where THCLK is the HCLK clock period
0000: 1 HCLK cycle (default)
1111: 16 HCLK cycles
Note: SET is MEMSET or ATTSET according to the addressed space.
Bits 8:7 Reserved, must be kept at reset value.
Bit 6 ECCEN: ECC computation logic enable bit
0: ECC logic is disabled and reset (default after reset),
1: ECC logic is enabled.
Bits 5:4 PWID[1:0]: Data bus width
Defines the external memory device width.
00: 8 bits
01: 16 bits (default after reset).
10: reserved.
11: reserved.
Bit 3 PTYP: Memory type
Defines the type of device attached to the corresponding memory bank:
0: Reserved, must be kept at reset value
1: NAND flash (default after reset)
Bit 2 PBKEN: NAND flash memory bank enable bit
Enables the memory bank. Accessing a disabled memory bank causes an ERROR on AHB
bus
0: Corresponding memory bank is disabled (default after reset)
1: Corresponding memory bank is enabled
Bit 1 PWAITEN: Wait feature enable bit
Enables the Wait feature for the NAND flash memory bank:
0: disabled
1: enabled
Bit 0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

FIFO status and interrupt register (FMC_SR)
Address offset: 0x84
Reset value: 0x0000 0040
This register contains information about the FIFO status and interrupt. The FMC features a
FIFO that is used when writing to memories to transfer up to 16 words of data from the AHB.
This is used to quickly write to the FIFO and free the AHB for transactions to peripherals
other than the FMC, while the FMC is draining its FIFO into the memory. One of these
register bits indicates the status of the FIFO, for ECC purposes.
The ECC is calculated while the data are written to the memory. To read the correct ECC,
the software must consequently wait until the FIFO is empty.
31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

FEMPT

IFEN

ILEN

IREN

IFS

ILS

IRS

r

rw

rw

rw

rw

rw

rw

Bits 31:7 Reserved, must be kept at reset value.
Bit 6 FEMPT: FIFO empty
Read-only bit that provides the status of the FIFO
0: FIFO not empty
1: FIFO empty
Bit 5 IFEN: Interrupt falling edge detection enable bit
0: Interrupt falling edge detection request disabled
1: Interrupt falling edge detection request enabled
Bit 4 ILEN: Interrupt high-level detection enable bit
0: Interrupt high-level detection request disabled
1: Interrupt high-level detection request enabled
Bit 3 IREN: Interrupt rising edge detection enable bit
0: Interrupt rising edge detection request disabled
1: Interrupt rising edge detection request enabled
Bit 2 IFS: Interrupt falling edge status
The flag is set by hardware and reset by software.
0: No interrupt falling edge occurred
1: Interrupt falling edge occurred
Note: If this bit is written by software to 1 it is set.
Bit 1 ILS: Interrupt high-level status
The flag is set by hardware and reset by software.
0: No Interrupt high-level occurred
1: Interrupt high-level occurred

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

Bit 0 IRS: Interrupt rising edge status
The flag is set by hardware and reset by software.
0: No interrupt rising edge occurred
1: Interrupt rising edge occurred
Note: If this bit is written by software to 1 it is set.

Common memory space timing register (FMC_PMEM)
Address offset: 0x88
Reset value: 0xFCFC FCFC
The FMC_PMEM read/write register contains the timing information for NAND flash memory
bank. This information is used to access either the common memory space of the NAND
flash for command, address write access and data read/write access.
31

30

29

28

rw

rw

rw

rw

15

14

13

12

27

26

25

24

23

22

21

rw

rw

rw

rw

rw

rw

rw

rw

11

10

9

8

7

6

5

4

MEMHIZ[7:0]

rw

rw

rw

rw

19

18

17

16

rw

rw

rw

rw

3

2

1

0

rw

rw

rw

MEMHOLD[7:0]

MEMWAIT[7:0]
rw

20

MEMSET[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 MEMHIZ[7:0]: Common memory x data bus Hi-Z time
Defines the number of HCLK clock cycles during which the data bus is kept Hi-Z after the
start of a NAND flash write access to common memory space on socket. This is only valid for
write transactions:
0000 0000: 1 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: reserved.
Bits 23:16 MEMHOLD[7:0]: Common memory hold time
Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for
read access during which the address is held (and data for write accesses) after the
command is deasserted (NWE, NOE), for NAND flash read or write access to common
memory space on socket x:
0000 0000: reserved.
0000 0001: 1 HCLK cycle for write access / 3 HCLK cycles for read access
1111 1110: 254 HCLK cycles for write access / 256 HCLK cycles for read access
1111 1111: reserved.
Bits 15:8 MEMWAIT[7:0]: Common memory wait time
Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE,
NOE), for NAND flash read or write access to common memory space on socket. The
duration of command assertion is extended if the wait signal (NWAIT) is active (low) at the
end of the programmed value of HCLK:
0000 0000: reserved
0000 0001: 2HCLK cycles (+ wait cycle introduced by deasserting NWAIT)
1111 1110: 255 HCLK cycles (+ wait cycle introduced by deasserting NWAIT)
1111 1111: reserved.

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Bits 7:0 MEMSET[7:0]: Common memory x setup time
Defines the number of HCLK (+1) clock cycles to set up the address before the command
assertion (NWE, NOE), for NAND flash read or write access to common memory space on
socket x:
0000 0000: 1 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: reserved

Attribute memory space timing register (FMC_PATT)
Address offset: 0x8C
Reset value: 0xFCFC FCFC
The FMC_PATT read/write register contains the timing information for NAND flash memory
bank. It is used for 8-bit accesses to the attribute memory space of the NAND flash for the
last address write access if the timing must differ from that of previous accesses (for
Ready/Busy management, refer to Section 27.7.5: NAND flash prewait functionality).
31

30

29

28

27

26

25

24

23

22

21

ATTHIZ[7:0]

20

19

18

17

16

ATTHOLD[7:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

ATTWAIT[7:0]
rw

rw

ATTSET[7:0]
rw

rw

Bits 31:24 ATTHIZ[7:0]: Attribute memory data bus Hi-Z time
Defines the number of HCLK clock cycles during which the data bus is kept in Hi-Z after the
start of a NAND flash write access to attribute memory space on socket. Only valid for writ
transaction:
0000 0000: 1 HCLK cycle
1111 1110: 255 HCLK cycles
1111 1111: reserved.
Bits 23:16 ATTHOLD[7:0]: Attribute memory hold time
Defines the number of HCLK clock cycles for write access and HCLK (+2) clock cycles for
read access during which the address is held (and data for write access) after the command
deassertion (NWE, NOE), for NAND flash read or write access to attribute memory space on
socket:
0000 0000: reserved
0000 0001: 1 HCLK cycle for write access / 3 HCLK cycles for read access
1111 1110: 254 HCLK cycles for write access / 256 HCLK cycles for read access
1111 1111: reserved.
Bits 15:8 ATTWAIT[7:0]: Attribute memory wait time
Defines the minimum number of HCLK (+1) clock cycles to assert the command (NWE,
NOE), for NAND flash read or write access to attribute memory space on socket x. The
duration for command assertion is extended if the wait signal (NWAIT) is active (low) at the
end of the programmed value of HCLK:
0000 0000: reserved
0000 0001: 2 HCLK cycles (+ wait cycle introduced by deassertion of NWAIT)
1111 1110: 255 HCLK cycles (+ wait cycle introduced by deasserting NWAIT)
1111 1111: reserved.

<!-- pagebreak -->

