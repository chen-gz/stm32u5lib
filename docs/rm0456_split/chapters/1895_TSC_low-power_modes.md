RM0456 Rev 6

RM0456

Touch sensing controller (TSC)
In order to improve the system immunity, the Schmitt trigger hysteresis of the GPIOs
controlled by the TSC must be disabled by resetting the corresponding Gx_IOy bit in the
TSC_IOHCR register.

47.4

TSC low-power modes
Table 459. Effect of low-power modes on TSC
Mode

47.5

Description

Sleep

No effect. Peripheral interrupts cause the device to exit Sleep mode.

Stop

Peripheral registers content is kept.

Standby

Powered-down. The peripheral must be reinitialized after exiting Standby mode.

TSC interrupts
Table 460. Interrupt control bits

Interrupt event

Enable
control bit

Event flag

Clear flag
bit

Exit the
Sleep mode

Exit the
Stop mode

Exit the
Standby mode

End of acquisition

EOAIE

EOAIF

EOAIC

Yes

No

No

Max count error

MCEIE

MCEIF

MCEIC

Yes

No

No

47.6

TSC registers
Refer to Section 1.2 of the reference manual for a list of abbreviations used in register
descriptions.
The peripheral registers can be accessed by words (32-bit).

47.6.1

TSC control register (TSC_CR)
Address offset: 0x00
Reset value: 0x0000 0000

31

30

29

28

27

CTPH[3:0]

26

25

24

23

22

21

CTPL[3:0]

20

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

SSPSC
rw

PGPSC[2:0]
rw

rw

Res.
rw

Res.

Res.

19

18

17

SSD[6:0]

Res.

MCV[2:0]
rw

RM0456 Rev 6

rw

rw

16
SSE

rw

rw

rw

rw

4

3

2

1

0

IODEF

SYNC
POL

AM

START

TSCE

rw

rw

rw

rw

rw

<!-- pagebreak -->

1904

Touch sensing controller (TSC)

RM0456

Bits 31:28 CTPH[3:0]: Charge transfer pulse high
These bits are set and cleared by software. They define the duration of the high state of the
charge transfer pulse (charge of CX).
0000: 1x tPGCLK
0001: 2x tPGCLK
...
1111: 16x tPGCLK
Note: These bits must not be modified when an acquisition is ongoing.
Bits 27:24 CTPL[3:0]: Charge transfer pulse low
These bits are set and cleared by software. They define the duration of the low state of the
charge transfer pulse (transfer of charge from CX to CS).
0000: 1x tPGCLK
0001: 2x tPGCLK
...
1111: 16x tPGCLK
Note: These bits must not be modified when an acquisition is ongoing.
Note: Some configurations are forbidden. Refer to the Section 47.3.4: Charge transfer
acquisition sequence for details.
Bits 23:17 SSD[6:0]: Spread spectrum deviation
These bits are set and cleared by software. They define the spread spectrum deviation which
consists in adding a variable number of periods of the SSCLK clock to the charge transfer
pulse high state.
0000000: 1x tSSCLK
0000001: 2x tSSCLK
...
1111111: 128x tSSCLK
Note: These bits must not be modified when an acquisition is ongoing.
Bit 16 SSE: Spread spectrum enable
This bit is set and cleared by software to enable/disable the spread spectrum feature.
0: Spread spectrum disabled
1: Spread spectrum enabled
Note: This bit must not be modified when an acquisition is ongoing.
Bit 15 SSPSC: Spread spectrum prescaler
This bit is set and cleared by software. It selects the AHB clock divider used to generate the
spread spectrum clock (SSCLK).
0: fHCLK
1: fHCLK /2
Note: This bit must not be modified when an acquisition is ongoing.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Touch sensing controller (TSC)

Bits 14:12 PGPSC[2:0]: Pulse generator prescaler
These bits are set and cleared by software.They select the AHB clock divider used to
generate the pulse generator clock (PGCLK).
000: fHCLK
001: fHCLK /2
010: fHCLK /4
011: fHCLK /8
100: fHCLK /16
101: fHCLK /32
110: fHCLK /64
111: fHCLK /128
Note: These bits must not be modified when an acquisition is ongoing.
Note: Some configurations are forbidden. Refer to the Section 47.3.4: Charge transfer
acquisition sequence for details.
Bits 11:8 Reserved, must be kept at reset value.
Bits 7:5 MCV[2:0]: Max count value
These bits are set and cleared by software. They define the maximum number of charge
transfer pulses that can be generated before a max count error is generated.
000: 255
001: 511
010: 1023
011: 2047
100: 4095
101: 8191
110: 16383
111: reserved
Note: These bits must not be modified when an acquisition is ongoing.
Bit 4 IODEF: I/O Default mode
This bit is set and cleared by software. It defines the configuration of all the TSC I/Os when
there is no ongoing acquisition. When there is an ongoing acquisition, it defines the
configuration of all unused I/Os (not defined as sampling capacitor I/O or as channel I/O).
0: I/Os are forced to output push-pull low
1: I/Os are in input floating
Note: This bit must not be modified when an acquisition is ongoing.
Bit 3 SYNCPOL: Synchronization pin polarity
This bit is set and cleared by software to select the polarity of the synchronization input pin.
0: Falling edge only
1: Rising edge and high level

RM0456 Rev 6

<!-- pagebreak -->

