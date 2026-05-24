1599

Multi-function digital filter (MDF)

39.8.1

RM0456

MDF global control register (MDF_GCR)
Address offset: 0x000
Reset value: 0x0000 0000
This register is used for controls common to all digital filters.

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

7

6

5

4

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

Res.

Res.

ILVNB[3:0]
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

TRGO

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:4 ILVNB[3:0]: Interleaved number
This bitfield is set and reset by software. it enables or disables the interleaved-transfer mode
and defines how many digital filters work in this mode.
This bitfield cannot be changed when DFLTEN = 1 in MDF_DFLT0CR.
0000: Interleaved-transfer mode disabled
0001: Data from DFLT0 and DFLT1 are interleaved.
0010: Data from DFLT0, DFLT1 and DFLT2 are interleaved.
...
1111: Data from DFLT0 to DFLT15 are interleaved.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 TRGO: Trigger output control
This bit is set by software and reset by hardware. It is used to start the acquisition of several
filters synchronously. It is also able to synchronize several MDF together by controlling the
mdf_trgo signal.
0: Write 0 has no effect. Read 0 means that the trigger can be set again to 1.
1: Write 1 generates a positive pulse on mdf_trgo signal and triggers the acquisition on the
enabled filters having ACQMOD[2:0] = 0x1 and selecting TRGO as trigger. Read 1 means
that the trigger pulse is still active.

39.8.2

MDF clock generator control register (MDF_CKGCR)
Address offset: 0x004
Reset value: 0x0000 0000
This register is used to control the clock generator. The mdf_proc_ck clock must be enabled
before enabling other MDF parts.

31

30

29

28

CKGA
CTIVE

27

26

25

24

PROCDIV[6:0]

r

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

Res.

TRGSE
NS

TRGSRC[3:0]
rw

rw

<!-- pagebreak -->

rw

Res.
rw

Res.

23

22

21

20

Res.

Res.

Res.

Res.

rw

19

Res.

rw

RM0456 Rev 6

6

5

4

CCK1D CCK0D CKGM
IR
IR
OD
rw

rw

rw

17

16

CCKDIV[3:0]
rw

7

18

3
Res.

rw

rw

2

1

CCK1E CCK0E
N
N
rw

rw

rw
0
CKGD
EN
rw

RM0456

Multi-function digital filter (MDF)

Bit 31 CKGACTIVE: Clock generator active flag
This bit is set and cleared by hardware. This flag must be used by the application to check if
the clock generator is effectively enabled (active) or not. The protected fields of this function
can only be updated when CKGACTIVE = 0 (refer to Section 39.4.15 for details). The delay
between a transition on CKGDEN and a transition on CKGACTIVE is two periods of AHB
clock and two periods of mdf_proc_ck.
0: The clock generator is not active and can be configured if needed.
1: The clock generator is active and protected fields cannot be configured.
Bits 30:24 PROCDIV[6:0]: Divider to control the serial interface clock
This bitfield is set and reset by software. It is used to adjust the frequency of the clock
provided to the SITF.
F mdf_ker_ck
F mdf_itf_ck = ------------------------------------------( PROCDIV + 1 )
This bitfield must not be changed if one of the filters is enabled (DFTEN = 1).
0: mdf_ker_ck provided to the SITF
1: mdf_ker_ck/2 provided to the SITF
2: mdf_ker_ck/3 provided to the SITF
...
127: mdf_ker_ck/128 provided to the SITF
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 23:20 Reserved, must be kept at reset value.
Bits 19:16 CCKDIV[3:0]: Divider to control the MDF_CCK clock
This bitfield is set and reset by software. It is used to adjust the frequency of the MDF_CCK
clock. The input clock of this divider is the clock provided to the SITF. More globally, the
frequency of the MDF_CCK is given by the following formula:
F mdf_ker_ck
F MDF_CCK = ----------------------------------------------------------------------------------------( PROCDIV + 1 ) × ( CCKDIV + 1 )
This bitfield must not be changed if one of the filters is enabled (DFTEN = 1).
0000: The MDF_CCK clock is mdf_proc_ck.
0001: The MDF_CCK clock is mdf_proc_ck / 2.
0010: The MDF_CCK clock is mdf_proc_ck / 3.
...
1111: The MDF_CCK clock is mdf_proc_ck / 16.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 15:12 TRGSRC[3:0]: Digital filter trigger signal selection
This bitfield is set and cleared by software.It is used to select which external signals trigger
for the corresponding filter. This bitfield is not significant if CKGMOD = 0.
000x: TRGO selected
0010: mdf_trg[0] selected
0011: mdf_trg[1] selected
...
1111: mdf_trg[13] selected
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details.)
Bits 11:9 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

Bit 8 TRGSENS: CKGEN trigger sensitivity selection
This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger
signals. This bit is not significant if the CKGMOD = 0.
0: A rising edge event triggers the activation of CKGEN dividers.
1: A falling edge even triggers the activation of CKGEN dividers.
Note: When the trigger source is TRGO, TRGSENS value is not taken into account. When
TRGO is selected, the sensitivity is forced to falling edge. This bit can be write-protected
(refer to Section 39.4.15 for details).
Bit 7 Reserved, must be kept at reset value.
Bit 6 CCK1DIR: MDF_CCK1 direction
This bit is set and reset by software.It is used to control the direction of the MDF_CCK1 pin.
0: MDF_CCK1 pin direction is in input.
1: MDF_CCK1 pin direction is in output.
Note: This bit can be write-protected (refer to Section 39.4.15 for details).
Bit 5 CCK0DIR: MDF_CCK0 direction
This bit is set and reset by software.It is used to control the direction of the MDF_CCK0.
0: MDF_CCK0 pin direction is in input.
1: MDF_CCK0 pin direction is in output.
Note: This bit can be write-protected (refer to Section 39.4.15 for details).
Bit 4 CKGMOD: Clock generator mode
This bit is set and reset by software. It is used to define the way the clock generator is
enabled. This bit must not be changed if one of the filters is enabled (DFTEN = 1).
0: The kernel clock is provided to the dividers as soon as CKGDEN is set to 1.
1: The kernel clock is provided to the dividers when CKGDEN is set to 1 and the trigger
condition met.
Note: This bit can be write-protected (refer to Section 39.4.15 for details).
Bit 3 Reserved, must be kept at reset value.
Bit 2 CCK1EN: MDF_CCK1 clock enable
This bit is set and reset by software. It is used to control the generation of the bitstream clock
on the MDF_CCK1 pin.
0: Bitstream clock not generated
1: Bitstream clock generated on the MDF_CCK1 pad
Bit 1 CCK0EN: MDF_CCK0 clock enable
This bit is set and reset by software.It is used to control the generation of the bitstream clock
on the MDF_CCK0 pin.
0: Bitstream clock not generated
1: Bitstream clock generated on the MDF_CCK0 pad
Bit 0 CKGDEN: CKGEN dividers enable
This bit is set and reset by software. It is used to enable/disable the clock dividers of the
CKGEN: PROCDIV and CCKDIV.
0: CKGEN dividers disabled
1: CKGEN dividers enabled

<!-- pagebreak -->

