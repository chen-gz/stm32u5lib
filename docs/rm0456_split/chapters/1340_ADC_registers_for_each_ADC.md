1377

Analog-to-digital converter (ADC12)

RM0456

Table 320. ADC interrupts
Event
flag

Enable
Control
bit

ADC ready

ADRDY

ADRDYIE

End of conversion of a
regular group

EOC

EOCIE

End of conversion
sequence of a regular
group

EOS

EOSIE

End of conversion of an
injected group

JEOC

JEOCIE

Interrupt
vector

ADC

Interrupt event

End of conversion
sequence of an injected
group

JEOS

JEOSIE

Analog watchdog 1 status
bit is set

AWD1

AWD1IE

Analog watchdog 2 status
bit is set

AWD2

AWD2IE

Analog watchdog 3 status
bit is set

AWD3

AWD3IE

End of sampling phase

EOSMP EOSMPIE

Overrun

33.6

OVR

Exit from
Interrupt
Sleep
clear method
mode

Set by
hardware and
cleared by
software

Exit from
Stop 0,
Stop 1
and Stop
2 modes

Exit from
Stop 3,
Standby
modes

No

No

Yes

OVRIE

ADC registers (for each ADC)
Refer to Section 1.2 for a list of abbreviations used in register descriptions.

33.6.1

ADC interrupt and status register (ADC_ISR)
Address offset: 0x00
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

LDO
RDY

Res.

Res.

AWD3

AWD2

AWD1

JEOS

JEOC

OVR

EOS

EOC

EOSMP ADRDY

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

Res.

Res.

r

Bits 31:13 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rc_w1

RM0456

Analog-to-digital converter (ADC12)

Bit 12 LDORDY: ADC voltage regulator ready
This bit is set by hardware. It indicates that the ADC internal supply is ready. The ADC is available
after tADCVREG_SETUP time.
0: ADC voltage regulator disabled
1: ADC voltage regulator enabled
Bits 11:10 Reserved, must be kept at reset value.
Bit 9 AWD3: Analog watchdog 3 flag
This bit is set by hardware when the converted voltage crosses the values programmed in the fields
LT3[7:0] and HT3[7:0] of ADC_LTR3 & ADC_HTR3 register. It is cleared by software writing 1 to it.
0: No analog watchdog 3 event occurred (or the flag event was already acknowledged and cleared
by software)
1: Analog watchdog 3 event occurred
Bit 8 AWD2: Analog watchdog 2 flag
This bit is set by hardware when the converted voltage crosses the values programmed in the fields
LT2[7:0] and HT2[7:0] of ADC_LTR2 & ADC_HTR2 register. It is cleared by software writing 1 to it.
0: No analog watchdog 2 event occurred (or the flag event was already acknowledged and cleared
by software)
1: Analog watchdog 2 event occurred
Bit 7 AWD1: Analog watchdog 1 flag
This bit is set by hardware when the converted voltage crosses the values programmed in the fields
LT1[11:0] and HT1[11:0] of ADC_LTR1, & ADC_HTR1 register. It is cleared by software. writing 1 to
it.
0: No analog watchdog 1 event occurred (or the flag event was already acknowledged and cleared
by software)
1: Analog watchdog 1 event occurred
Bit 6 JEOS: Injected channel end of sequence flag
This bit is set by hardware at the end of the conversions of all injected channels in the group. It is
cleared by software writing 1 to it.
0: Injected conversion sequence not complete (or the flag event was already acknowledged and
cleared by software)
1: Injected conversions complete
Bit 5 JEOC: Injected channel end of conversion flag
This bit is set by hardware at the end of each injected conversion of a channel when a new data is
available in the corresponding ADC_JDRy register. It is cleared by software writing 1 to it or by
reading the corresponding ADC_JDRy register
0: Injected channel conversion not complete (or the flag event was already acknowledged and
cleared by software)
1: Injected channel conversion complete
Bit 4 OVR: ADC overrun
This bit is set by hardware when an overrun occurs on a regular channel, meaning that a new
conversion has completed while the EOC flag was already set. It is cleared by software writing 1 to
it.
0: No overrun occurred (or the flag event was already acknowledged and cleared by software)
1: Overrun has occurred

RM0456 Rev 6

<!-- pagebreak -->

