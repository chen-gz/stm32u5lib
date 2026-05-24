1.

Optionally turn on external oscillator and/or device PLL.

2.

Clear SUSPEN bit of USB_CNTR register.

3.

If the resume triggering event has to be identified, bits RXDP and RXDM in the
USB_FNR register can be used according to Table 739, which also lists the intended
software action in all the cases. If required, the end of resume or reset sequence can
be detected monitoring the status of the above mentioned bits by checking when they
reach the “10” configuration, which represent the idle bus state; moreover at the end of
a reset sequence the RST_DCON bit in USB_ISTR register is set to 1, issuing an
interrupt if enabled, which must be handled as usual.

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
Table 739. Resume event detection
[RXDP,RXDM] status

Wake-up event

Required resume software action

“00”

Root reset

None

“10”

None (noise on bus)

Go back in Suspend mode

“01”

Root resume

None

“11”

Not allowed (noise on bus)

Go back in Suspend mode

A device may require to exit from suspend mode as an answer to particular events not
directly related to the USB protocol (for example a mouse movement wakes up the whole
system). In this case, the resume sequence can be started by setting the L2RES bit in the
USB_CNTR register to 1 and resetting it to 0 after an interval between 1 ms and 15 ms (this
interval can be timed using ESOF interrupts, occurring with a 1 ms period when the system
clock is running at nominal frequency). Once the L2RES bit is clear, the resume sequence is
completed by the host PC and its end can be monitored again using the RXDP and RXDM
bits in the USB_FNR register.
Note:

The L2RES bit must be anyway used only after the USB peripheral has been put in suspend
mode, setting the SUSPEN bit in USB_CNTR register to 1.

Suspend and resume in Host mode
The basics of the suspend and resume mechanism has been described in the previous
section.
From the host stand-point, suspend is entered by writing the SUSPEN bit in USB_CNTR.
When suspend entry is confirmed, SUSPRDY (also in USB_CNTR) is set.
Once in suspend, and when the application want to resume the bus, this can be done by
setting the L2RES bit in USB_CNTR to 1.
Below in Table 740, the different actions recommended after a wake-up event are indicated.
According to the different line states after a wake-up event, the interpretation of the event
and the suggested behavior are shown. Note that, this table here is somewhat expanded
when compared to the previously shown device table, as the host may encounter both full
speed and low speed devices which use different line states for both suspend and resume.

RM0456 Rev 6

<!-- pagebreak -->

