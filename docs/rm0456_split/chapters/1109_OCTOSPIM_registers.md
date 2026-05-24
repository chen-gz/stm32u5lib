•

OCTOSPI1_NCS and OCTOSPI2_NCS work in the same way, then in
Non-multiplexed mode they have to be assigned to their respective
OCTOSPIM_Pn_NCS.

•

All the other signals are seen by the I/O matrix as if they were seen from OCTOSPI1.

RM0456 Rev 6

RM0456

Octo-SPI I/O manager (OCTOSPIM)

29.5

OCTOSPIM registers

29.5.1

OCTOSPIM control register (OCTOSPIM_CR)
Address offset: 0x0000
Reset value: 0x0000 0000

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

Res.

Res.

Res.

MUXEN

REQ2ACK_TIME[7:0]

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 REQ2ACK_TIME[7:0]: REQ to ACK time
In multiplexed mode (MUXEN = 1), this field defines the time between two transactions.
The value is the number of OCTOSPI clock cycles - 1
Bits 15:1 Reserved, must be kept at reset value.
Bit 0 MUXEN: Multiplexed mode enable
This bit enables the multiplexing of the two OCTOSPIs.
0: No multiplexing, hence no arbitration
1: OCTOSPI1 and OCTOSPI2 are multiplexed over the same bus.

29.5.2

OCTOSPIM Port n configuration register
(OCTOSPIM_PnCR)
Address offset: 0x0000 + 0x4 * n (n = 1 to 2)
Reset value: 0x0301 0111, 0x0705 0333

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

Res.

Res.

Res.

Res.

Res.

IOHSRC[1:0]
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

Res.

Res.

Res.

Res.

Res.

Res.

IOHEN

Res.

Res.

Res.

Res.

Res.

IOLSRC[1:0]
rw

rw

rw

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

NCSSRC NCSEN
rw

rw

DQSSRC DQSEN
rw

rw

17

16
IOLEN

CLKSRC CLKEN
rw

rw

Bits 31:27 Reserved, must be kept at reset value.
Bits 26:25 IOHSRC[1:0]: IO[7:4] source for Port n
This bits select the source of Port n IO[7:4].
00: OCTOSPI1_IO[3:0] in non multiplexed mode / multiplexed_IO[3:0] in multiplexed mode
01: OCTOSPI1_IO[7:4] in non multiplexed mode / multiplexed_IO[7:4] in multiplexed mode
10: OCTOSPI2_IO[3:0] in non multiplexed mode / unused in multiplexed mode
11: OCTOSPI2_IO[7:4] in non multiplexed mode / unused in multiplexed mode

RM0456 Rev 6

<!-- pagebreak -->

1111

Octo-SPI I/O manager (OCTOSPIM)

RM0456

Bit 24 IOHEN: IO[7:4] enable for Port n
This bit enables the Port n IO[7:4].
0: IO[7:4] for Port n disabled
1: IO[7:4] for Port n enabled
Bits 23:19 Reserved, must be kept at reset value.
Bits 18:17 IOLSRC[1:0]: IO[3:0] source for Port n
This bits select the source of Port n IO[3:0].
00: OCTOSPI1_IO[3:0] in non multiplexed mode / multiplexed_IO[3:0] in multiplexed mode
01: OCTOSPI1_IO[7:4] in non multiplexed mode / multiplexed_IO[7:4] in multiplexed mode
10: OCTOSPI2_IO[3:0] in non multiplexed mode / unused in multiplexed mode
11: OCTOSPI2_IO[7:4] in non multiplexed mode / unused in multiplexed mode
Bit 16 IOLEN: IO[3:0] enable for Port n
This bit enables the Port n IO[3:0].
0: IO[3:0] for Port n disabled
1: IO[3:0] for Port n enabled
Bits 15:10 Reserved, must be kept at reset value.
Bit 9 NCSSRC: NCS source for Port n
This bit selects the source of Port n NCS.
0: OCTOSPI1_NCS
1: OCTOSPI2_NCS
Bit 8 NCSEN: NCS enable for Port n
This bit enables the Port n NCS.
0: NCS for Port n is disabled
1: NCS for Port n is enabled
Bits 7:6 Reserved, must be kept at reset value.
Bit 5 DQSSRC: DQS source for Port n
This bit selects the source of Port n DQS.
0: OCTOSPI1_DQS in non multiplexed mode / multiplexed_DQS in multiplexed mode
1: OCTOSPI2_DQS in non multiplexed mode / unused port in multiplexed mode
Bit 4 DQSEN: DQS enable for Port n
This bit enables the Port n DQS.
0: DQS for Port n is disabled
1: DQS for Port n is enabled
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 CLKSRC: CLK/NCLK source for Port n
This bit selects the source of Port n CLK/NCLK.
0: OCTOSPI1_CLK/NCLK in non multiplexed mode / multiplexed_CLK/CLKn in multiplexed
mode
1: OCTOSPI2_CLK/NCLK in non multiplexed mode / unused port in multiplexed mode
Bit 0 CLKEN: CLK/NCLK enable for Port n
This bit enables the Port n CLK/NCLK.
0: CLK/NCLK for Port n is disabled
1: CLK/NCLK for Port n is enabled

<!-- pagebreak -->

