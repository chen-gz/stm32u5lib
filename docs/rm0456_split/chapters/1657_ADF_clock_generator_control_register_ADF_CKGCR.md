RM0456 Rev 6

RM0456

Audio digital filter (ADF)

40.8.2

ADF clock generator control register (ADF_CKGCR)
Address offset: 0x004
Reset value: 0x0000 0000
This register is used to control the clock generator. The clock adf_proc_ck must be enabled
before enabling other ADF parts.

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

7

6

5

4

Res.

rw

CCK1D CCK0D CKGM
IR
IR
OD
rw

rw

rw

19

18

17

16

CCKDIV[3:0]
rw

rw

rw

3

2

1

Res.

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

Bit 31 CKGACTIVE: Clock generator active flag
This bit is set and cleared by hardware. Ii is used by the application to check if the clock
generator is effectively enabled (active) or not. The protected fields of this function can only
be updated when CKGACTIVE = 0 (see Section 40.4.13: Register protection for details).
The delay between a transition on CKGDEN and a transition on CKGACTIVE is two periods
of AHB clock and two 2 periods of adf_proc_ck.
0: The clock generator is not active and can be configured if needed.
1: The clock generator is active and protected fields cannot be configured.
Bits 30:24 PROCDIV[6:0]: Divider to control the serial interface clock
This bitfield is set and reset by software. It is used to adjust the frequency of the clock
provided to the SITF.
F adf_ker_ck
F adf_itf_ck = ------------------------------------------( PROCDIV + 1 )
This bitfield must not be changed if the filter is enabled (DFTEN = 1).
0: adf_ker_ck provided to the SITF
1: adf_ker_ck / 2 provided to the SITF
2: adf_ker_ck / 3 provided to the SITF
...
127: adf_ker_ck / 128 provided to the SITF
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 23:20 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

Bits 19:16 CCKDIV[3:0]: Divider to control the ADF_CCK clock
This bitfield is set and reset by software. It is used to adjust the frequency of the ADF_CCK
clock. The input clock of this divider is the clock provided to the SITF. More globally, the
frequency of the ADF_CCK is given by the following formula:
F adf_ker_ck
F ADF_CCK = ----------------------------------------------------------------------------------------( PROCDIV + 1 ) × ( CCKDIV + 1 )
This bitfield must not be changed if the filter is enabled (DFTEN = 1).
0000: The ADF_CCK clock is adf_proc_ck.
0001: The ADF_CCK clock is adf_proc_ck / 2.
0010: The ADF_CCK clock is adf_proc_ck / 3.
...
1111: The ADF_CCK clock is adf_proc_ck / 16.
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 15:12 TRGSRC[3:0]: Digital filter trigger signal selection
This bitfield is set and cleared by software. It is used to select which external signals trigger
the corresponding filter. This bitfield is not significant if the CKGMOD = 0.
000x: TRGO selected
0010: adf_trg1 selected
Others: Reserved
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 TRGSENS: CKGEN trigger sensitivity selection
This bit is set and cleared by software. It is used to select the trigger sensitivity of the trigger
signals. This bit is not significant if the CKGMOD = 0.
0: A rising edge event triggers the activation of CKGEN dividers.
1: A falling edge even triggers the activation of CKGEN dividers.
Note: When the trigger source is TRGO, the sensitivity is forced to falling edge, thus
TRGSENS value is not taken into account. This bit can be write-protected
(see Section 40.4.13: Register protection for details).
Bit 7 Reserved, must be kept at reset value.
Bit 6 CCK1DIR: ADF_CCK1 direction
This bit is set and reset by software. It is used to control the direction of the ADF_CCK1 pin.
0: The ADF_CCK1 pin direction is in input.
1: The ADF_CCK1 pin direction is in output.
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bit 5 CCK0DIR: ADF_CCK0 direction
This bit is set and reset by software. It is used to control the direction of the ADF_CCK0 pin.
0: The ADF_CCK0 pin direction is in input.
1: The ADF_CCK0 pin direction is in output.
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bit 4 CKGMOD: Clock generator mode
This bit is set and reset by software. It is used to define the way the clock generator is
enabled. This bit must not be changed if the filter is enabled (DFTEN = 1).
0: The kernel clock is provided to the dividers as soon as CKGDEN is set to 1.
1: The kernel clock is provided to the dividers when CKGDEN is set to 1 and the trigger
condition met.
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).

<!-- pagebreak -->

