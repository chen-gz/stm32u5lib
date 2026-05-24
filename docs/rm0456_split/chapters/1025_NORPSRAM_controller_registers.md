Bit number

Bit name

Value to set

31-30

DATAHLD

Don’t care

29:28

ACCMOD

0x0

27-24

DATLAT

Data latency

23-20

CLKDIV

0x0 to get CLK = HCLK
0x1 to get CLK = 2 × HCLK

19-16

BUSTURN

Time between NEx high to NEx low (BUSTURN HCLK).

15-8

DATAST

Don’t care

7-4

ADDHLD

Don’t care

3-0

ADDSET

Don’t care

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

27.6.6

NOR/PSRAM controller registers
SRAM/NOR-flash chip-select control register for bank x
(FMC_BCRx)
Address offset: 0x00 + 0x8 * (x - 1), (x = 1 to 4)
Reset value: 0x0000 30DB, 0x0000 30D2, 0x0000 30D2, 0x0000 30D2
This register contains the control information of each memory bank, used for SRAMs,
PSRAM, FRAM and NOR flash memories.

31
FMCEN

30

29

28

27

26

25

24

23

22

20

19

WFDIS

CCLK
EN

CBURST
RW

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

ASYNC
WAIT

EXT
MOD

WAIT
EN

WREN

WAIT
CFG

Res.

WAIT
POL

BURST
EN

Res.

FACC
EN

rw

rw

rw

rw

rw

rw

rw

rw

NBLSET[1:0]

21

rw

CPSIZE[2:0]

MWID[1:0]

MTYP[1:0]

MUX
EN

MBK
EN

rw

rw

rw

rw

rw

rw

Bit 31 FMCEN: FMC controller enable
This bit enables or disables the FMC controller.
0: Disable the FMC controller
1: Enable the FMC controller
Note: The FMCEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the
FMC_BCR1 register.
Bits 30:24 Reserved, must be kept at reset value.
Bits 23:22 NBLSET[1:0]: Byte lane (NBL) setup
These bits configure the NBL setup timing from NBLx low to chip select NEx low.
00: NBL setup time is 0 AHB clock cycle
01: NBL setup time is 1 AHB clock cycle
10: NBL setup time is 2 AHB clock cycles
11: NBL setup time is 3 AHB clock cycles
Bit 21 WFDIS: Write FIFO disable
This bit disables the Write FIFO used by the FMC controller.
0: Write FIFO enabled (Default after reset)
1: Write FIFO disabled
Note: The WFDIS bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the
FMC_BCR1 register.

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Bit 20 CCLKEN: Continuous clock enable
This bit enables the FMC_CLK clock output to external memory devices.
0: The FMC_CLK is only generated during the synchronous memory access (read/write
transaction). The FMC_CLK clock ratio is specified by the programmed CLKDIV value in the
FMC_BCRx register (default after reset).
1: The FMC_CLK is generated continuously during asynchronous and synchronous access. The
FMC_CLK clock is activated when the CCLKEN is set.
Note: The CCLKEN bit of the FMC_BCR2..4 registers is don’t care. It is only enabled through the
FMC_BCR1 register. Bank 1 must be configured in Synchronous mode to generate the
FMC_CLK continuous clock.
Note: If CCLKEN bit is set, the FMC_CLK clock ratio is specified by CLKDIV value in the FMC_BTR1
register. CLKDIV in FMC_BWTR1 is don’t care.
Note: If the Synchronous mode is used and CCLKEN bit is set, the synchronous memories
connected to other banks than Bank 1 are clocked by the same clock (the CLKDIV value in the
FMC_BTR2..4 and FMC_BWTR2..4 registers for other banks has no effect.)
Bit 19 CBURSTRW: Write burst enable
For PSRAM (CRAM) operating in Burst mode, the bit enables synchronous accesses during write
operations. The enable bit for synchronous read accesses is the BURSTEN bit in the FMC_BCRx
register.
0: Write operations are always performed in Asynchronous mode.
1: Write operations are performed in Synchronous mode.
Bits 18:16 CPSIZE[2:0]: CRAM page size
These are used for CellularRAM™ 1.5 which does not allow burst access to cross the address
boundaries between pages. When these bits are configured, the FMC controller splits automatically
the burst access when the memory page size is reached (refer to memory datasheet for page size).
000: No burst split when crossing page boundary (default after reset)
001: 128 bytes
010: 256 bytes
011: 512 bytes
100: 1024 bytes
Others: Reserved, must not be used
Bit 15 ASYNCWAIT: Wait signal during asynchronous transfers
This bit enables/disables the FMC to use the wait signal even during an asynchronous protocol.
0: NWAIT signal is not taken in to account when running an asynchronous protocol (default after
reset).
1: NWAIT signal is taken in to account when running an asynchronous protocol.
Bit 14 EXTMOD: Extended mode enable
This bit enables the FMC to program the write timings for non multiplexed asynchronous accesses
inside the FMC_BWTR register, thus resulting in different timings for read and write operations.
0: values inside FMC_BWTR register are not taken into account (default after reset)
1: values inside FMC_BWTR register are taken into account
Note: When the Extended mode is disabled, the FMC can operate in mode 1 or mode 2 as follows:
–
Mode 1 is the default mode when the SRAM/PSRAM memory type is selected
(MTYP = 0x0 or 0x01)
–
Mode 2 is the default mode when the NOR memory type is selected (MTYP = 0x10).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

Bit 13 WAITEN: Wait enable bit
This bit enables/disables wait-state insertion via the NWAIT signal when accessing the memory in
Synchronous mode.
0: NWAIT signal is disabled (its level not taken into account, no wait state inserted after the
programmed flash latency period).
1: NWAIT signal is enabled (its level is taken into account after the programmed latency period to
insert wait states if asserted) (default after reset).
Bit 12 WREN: Write enable bit
This bit indicates whether write operations are enabled/disabled in the bank by the FMC.
0: Write operations are disabled in the bank by the FMC, an AHB error is reported.
1: Write operations are enabled for the bank by the FMC (default after reset).
Bit 11 WAITCFG: Wait timing configuration
The NWAIT signal indicates whether the data from the memory are valid or if a wait state must be
inserted when accessing the memory in Synchronous mode. This configuration bit determines if
NWAIT is asserted by the memory one clock cycle before the wait state or during the wait state:
0: NWAIT signal is active one data cycle before wait state (default after reset).
1: NWAIT signal is active during wait state (not used for PSRAM).
Bit 10 Reserved, must be kept at reset value.
Bit 9 WAITPOL: Wait signal polarity bit
Defines the polarity of the wait signal from memory used for either in Synchronous or Asynchronous
mode.
0: NWAIT active low (default after reset)
1: NWAIT active high
Bit 8 BURSTEN: Burst enable bit
This bit enables/disables synchronous accesses during read operations. It is valid only for
synchronous memories operating in Burst mode.
0: Burst mode disabled (default after reset). Read accesses are performed in Asynchronous mode.
1: Burst mode enable. Read accesses are performed in Synchronous mode.
Bit 7 Reserved, must be kept at reset value.
Bit 6 FACCEN: Flash access enable
Enables NOR flash memory access operations.
0: Corresponding NOR flash memory access is disabled.
1: Corresponding NOR flash memory access is enabled (default after reset).
Bits 5:4 MWID[1:0]: Memory data bus width
Defines the external memory device width, valid for all type of memories.
00: 8 bits
01: 16 bits (default after reset)
10: reserved
11: reserved
Bits 3:2 MTYP[1:0]: Memory type
Defines the type of external memory attached to the corresponding memory bank.
00: SRAM/FRAM (default after reset for Bank 2...4)
01: PSRAM (CRAM) / FRAM
10: NOR flash/OneNAND flash (default after reset for Bank 1)
11: reserved

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Bit 1 MUXEN: Address/data multiplexing enable bit
When this bit is set, the address and data values are multiplexed on the data bus, valid only with
NOR and PSRAM memories:
0: Address/data non multiplexed
1: Address/data multiplexed on databus (default after reset)
Bit 0 MBKEN: Memory bank enable bit
Enables the memory bank. After reset Bank1 is enabled, all others are disabled. Accessing a
disabled bank causes an ERROR on AHB bus.
0: Corresponding memory bank is disabled.
1: Corresponding memory bank is enabled.

SRAM/NOR-flash chip-select timing register for bank x (FMC_BTRx)
Address offset: 0x04 + 0x8 * (x - 1), (x = 1 to 4)
Reset value: 0x0FFF FFFF
This register contains the control information of each memory bank, used for SRAMs,
PSRAM and NOR flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then
this register is partitioned for write and read access, that is, 2 registers are available: one to
configure read accesses (this register) and one to configure write accesses (FMC_BWTRx
registers).
31

30

29

28

27

26

25

24

23

DATLAT[3:0]

22

21

20

19

CLKDIV[3:0]

18

17

16

DATAHLD[1:0]

ACCMOD[1:0]

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

DATAST[7:0]
rw

rw

rw

rw

rw

BUSTURN[3:0]

ADDHLD[3:0]
rw

rw

rw

rw

rw

rw

ADDSET[3:0]
rw

rw

rw

rw

rw

Bits 31:30 DATAHLD[1:0]: Data hold phase duration
These bits are written by software to define the duration of the data hold phase in HCLK
cycles (refer to Figure 121 to Figure 133), used in asynchronous accesses:
For read accesses
00: DATAHLD phase duration = 0 × HCLK clock cycle (default)
01: DATAHLD phase duration = 1 × HCLK clock cycle
10: DATAHLD phase duration = 2 × HCLK clock cycle
11: DATAHLD phase duration = 3 × HCLK clock cycle
For write accesses
00: DATAHLD phase duration = 1 × HCLK clock cycle (default)
01: DATAHLD phase duration = 2 × HCLK clock cycle
10: DATAHLD phase duration = 3 × HCLK clock cycle
11: DATAHLD phase duration = 4 × HCLK clock cycle
Bits 29:28 ACCMOD[1:0]: Access mode
Specifies the asynchronous access modes as shown in the timing diagrams. These bits are
taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
00: Access mode A
01: Access mode B
10: Access mode C
11: Access mode D

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Flexible static memory controller (FSMC)

Bits 27:24 DATLAT[3:0]: (see note below bit descriptions): Data latency for synchronous memory
For synchronous access with read/write Burst mode enabled (BURSTEN / CBURSTRW bits
set), defines the number of memory clock cycles (+2) to issue to the memory before
reading/writing the first data:
This timing parameter is not expressed in HCLK periods, but in FMC_CLK periods.
For asynchronous access, this value is don't care.
0000: Data latency of 2 CLK clock cycles for first burst access
1111: Data latency of 17 CLK clock cycles for first burst access (default value after reset)
Bits 23:20 CLKDIV[3:0]: Clock divide ratio (for FMC_CLK signal)
Defines the period of FMC_CLK clock output signal, expressed in number of HCLK cycles:
0000: FMC_CLK period= 1x HCLK period
0001: FMC_CLK period = 2 × HCLK periods
0010: FMC_CLK period = 3 × HCLK periods
1111: FMC_CLK period = 16 × HCLK periods (default value after reset)
In asynchronous NOR flash, SRAM or PSRAM accesses, this value is don’t care.
Note: Refer to Section 27.6.5: Synchronous transactions for FMC_CLK divider ratio formula)
Bits 19:16 BUSTURN[3:0]: Bus turnaround phase duration
These bits are written by software to add a delay at the end of current read or write
transaction to next transaction on the same bank.
This delay is used to match the minimum time between consecutive transactions (tEHEL from
NEx high to NEx low) and the maximum time needed by the memory to free the data bus
after a read access (tEHQZ, chip enable high to output Hi-Z). This delay is recommended for
mode D and muxed mode. For non-muxed memory, the bus turnaround delay can be set to
minimum value.
(BUSTURN + 1)HCLK period ≥ max(tEHEL min, tEHQZ max)
For FRAM memories, the bus turnaround delay must be configured to match the minimum
tPC (precharge time) timings. The bus turnaround delay is inserted between any consecutive
transactions on the same bank (read/read, write/write, read/write and write/read) to match the
tPC memory timing. The chip select is toggling between any consecutive accesses.
(BUSTURN + 1)HCLK period ≥ tPC min
0000: BUSTURN phase duration = 1 HCLK clock cycle added
...
1111: BUSTURN phase duration = 16 x HCLK clock cycles added (default value after reset)
Bits 15:8 DATAST[7:0]: Data-phase duration
These bits are written by software to define the duration of the data phase (refer to
Figure 121 to Figure 133), used in asynchronous accesses:
0000 0000: Reserved
0000 0001: DATAST phase duration = 1 × HCLK clock cycles
0000 0010: DATAST phase duration = 2 × HCLK clock cycles
...
1111 1111: DATAST phase duration = 255 × HCLK clock cycles (default value after reset)
For each memory type and access mode data-phase duration, refer to the respective figure
(Figure 121 to Figure 133).
Example: Mode 1, write access, DATAST=1: Data-phase duration= DATAST+1 = 2 HCLK
clock cycles.
Note: In synchronous accesses, this value is don’t care.

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Bits 7:4 ADDHLD[3:0]: Address-hold phase duration
These bits are written by software to define the duration of the address hold phase (refer to
Figure 121 to Figure 133), used in mode D or multiplexed accesses:
0000: Reserved
0001: ADDHLD phase duration =1 × HCLK clock cycle
0010: ADDHLD phase duration = 2 × HCLK clock cycle
...
1111: ADDHLD phase duration = 15 × HCLK clock cycles (default value after reset)
For each access mode address-hold phase duration, refer to the respective figure
(Figure 121 to Figure 133).
Note: In synchronous accesses, this value is not used, the address hold phase is always 1
memory clock period duration.
Bits 3:0 ADDSET[3:0]: Address setup phase duration
These bits are written by software to define the duration of the address setup phase (refer to
Figure 121 to Figure 133), used in SRAMs, ROMs, asynchronous NOR flash and PSRAM:
0000: ADDSET phase duration = 0 × HCLK clock cycle
...
1111: ADDSET phase duration = 15 × HCLK clock cycles (default value after reset)
For each access mode address setup phase duration, refer to the respective figure
(Figure 121 to Figure 133).
Note: In synchronous accesses, this value is don’t care.
In Muxed mode or mode D, the minimum value for ADDSET is 1.
In mode 1 and PSRAM memory, the minimum value for ADDSET is 1.

Note:

PSRAMs (CRAMs) have a variable latency due to internal refresh. Therefore these
memories issue the NWAIT signal during the whole latency phase to prolong the latency as
needed.
With PSRAMs (CRAMs) the filled DATLAT must be set to 0, so that the FMC exits its latency
phase soon and starts sampling NWAIT from memory, then starts to read or write when the
memory is ready.
This method can be used also with the latest generation of synchronous flash memories that
issue the NWAIT signal, unlike older flash memories (check the datasheet of the specific
flash memory being used).

SRAM/NOR-flash write timing registers x (FMC_BWTRx)
Address offset: 0x104 + 0x8 * (x - 1), (x = 1 to 4)
Reset value: 0x0FFF FFFF
This register contains the control information of each memory bank. It is used for SRAMs,
PSRAMs and NOR flash memories. When the EXTMOD bit is set in the FMC_BCRx
register, then this register is active for write access.
31

30

DATAHLD[1:0]

28

27

26

25

24

23

22

21

20

ACCMOD[1:0]

29

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
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

15

14

13

12

rw

rw

rw

DATAST[7:0]

<!-- pagebreak -->

rw

rw

19

ADDHLD[3:0]

RM0456 Rev 6

rw

rw

18

17

16

BUSTURN[3:0]

ADDSET[3:0]
rw

rw

rw

RM0456

Flexible static memory controller (FSMC)

Bits 31:30 DATAHLD[1:0]: Data hold phase duration
These bits are written by software to define the duration of the data hold phase in HCLK cycles
(refer to Figure 121 to Figure 133), used in asynchronous write accesses:
00: DATAHLD phase duration = 1 × HCLK clock cycle (default)
01: DATAHLD phase duration = 2 × HCLK clock cycle
10: DATAHLD phase duration = 3 × HCLK clock cycle
11: DATAHLD phase duration = 4 × HCLK clock cycle
Bits 29:28 ACCMOD[1:0]: Access mode.
Specifies the asynchronous access modes as shown in the next timing diagrams.These bits are
taken into account only when the EXTMOD bit in the FMC_BCRx register is 1.
00: Access mode A
01: Access mode B
10: Access mode C
11: Access mode D
Bits 27:20 Reserved, must be kept at reset value.
Bits 19:16 BUSTURN[3:0]: Bus turnaround phase duration
These bits are written by software to add a delay at the end of current write transaction to next
transaction on the same bank.
For FRAM memories, the bus turnaround delay must be configured to match the minimum tPC
(precharge time) timings. The bus turnaround delay is inserted between any consecutive
transactions on the same bank (read/read, write/write, read/write and write/read). The chip select is
toggling between any consecutive accesses.
(BUSTURN + 1)HCLK period ≥ tPC min
0000: BUSTURN phase duration = 1 HCLK clock cycle added
...
1111: BUSTURN phase duration = 16 x HCLK clock cycles added (default value after reset)
Bits 15:8 DATAST[7:0]: Data-phase duration.
These bits are written by software to define the duration of the data phase (refer to Figure 121 to
Figure 133), used in asynchronous SRAM, PSRAM and NOR flash memory accesses:
0000 0000: Reserved
0000 0001: DATAST phase duration = 1 × HCLK clock cycles
0000 0010: DATAST phase duration = 2 × HCLK clock cycles
...
1111 1111: DATAST phase duration = 255 × HCLK clock cycles (default value after reset)
Bits 7:4 ADDHLD[3:0]: Address-hold phase duration.
These bits are written by software to define the duration of the address hold phase (refer to
Figure 130 to Figure 133), used in asynchronous multiplexed accesses:
0000: Reserved
0001: ADDHLD phase duration = 1 × HCLK clock cycle
0010: ADDHLD phase duration = 2 × HCLK clock cycle
...
1111: ADDHLD phase duration = 15 × HCLK clock cycles (default value after reset)
Note: In synchronous NOR flash accesses, this value is not used, the address hold phase is always
1 flash clock period duration.

RM0456 Rev 6

<!-- pagebreak -->

1045

Flexible static memory controller (FSMC)

RM0456

Bits 3:0 ADDSET[3:0]: Address setup phase duration.
These bits are written by software to define the duration of the address setup phase in HCLK cycles
(refer to Figure 121 to Figure 133), used in asynchronous accesses:
0000: ADDSET phase duration = 0 × HCLK clock cycle
...
1111: ADDSET phase duration = 15 × HCLK clock cycles (default value after reset)
Note: In synchronous accesses, this value is not used, the address setup phase is always 1 flash
clock period duration. In muxed mode, the minimum ADDSET value is 1.

PSRAM chip select counter register (FMC_PCSCNTR)
Address offset: 0x20
Reset value: 0x0000 0000
This register contains the PSRAM chip select counter value for Synchronous and
Asynchronous modes. The chip select counter is common to all banks and can be enabled
separately on each bank. During PSRAM read or write accesses, this value is loaded into a
timer which is decremented while the NE signal is held low. When the timer reaches 0, the
PSRAM controller splits the current access, toggles NE to allow PSRAM device refresh, and
restarts a new access. The programmed counter value guarantees a maximum NE pulse
width (tCEM) as specified for PSRAM devices. The counter is reloaded and starts
decrementing each time a new access is started by a transition of NE from high to low.
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

CNTB4EN

CNTB3EN

CNTB2EN

CNTB1EN

h

31

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

CSCOUNT[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bit 19 CNTB4EN: Counter Bank 4 enable
This bit enables the chip select counter for PSRAM/NOR Bank 4.
0: Counter disabled for Bank 4
1: Counter enabled for Bank 4
Bit 18 CNTB3EN: Counter Bank 3 enable
This bit enables the chip select counter for PSRAM/NOR Bank 3.
0: Counter disabled for Bank 3.
1: Counter enabled for Bank 3
Bit 17 CNTB2EN: Counter Bank 2 enable
This bit enables the chip select counter for PSRAM/NOR Bank 2.
0: Counter disabled for Bank 2
1: Counter enabled for Bank 2
Bit 16 CNTB1EN: Counter Bank 1 enable
This bit enables the chip select counter for PSRAM/NOR Bank 1.
0: Counter disabled for Bank 1
1: Counter enabled for Bank 1

<!-- pagebreak -->

