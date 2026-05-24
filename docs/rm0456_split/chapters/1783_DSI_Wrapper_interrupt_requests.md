RM0456 Rev 6

RM0456

DSI Host (DSI)
where:
•

CLKIN is in the 4 to 100 MHz range

•

DSI_WRPCR.NDIV is in the 1 to 511 range

•

DSI_WRPCR.IDF is in the 1 to 511 range

•

FPFD is in the 2 to 100 MHz range

•

FVCO is in the 500 MHz to 1 GHz range

•

DSI_WRPCR.ODF can be 1 to 511

•

PHI is in the 31.25 to 500 MHz range

The PLL is enabled by setting the PLLEN bit in the DSI_WRPCR register.
Once the PLL is locked, the PLLLIF bit is set in the DSI_WISR. If the PLLLIE bit is set in the
DSI_WIER, an interrupt is generated.
The PLL status (lock or unlock) can be monitored with the PLLLS flag in the DSI_WISR
register.
If the PLL gets unlocked, the PLLUIF bit of the DSI_WISR is set. If the PLLUIE bit of the
DSI_WIER register is set, an interrupt is generated.
The DSI PLL settings can be changed only when the PLL is disabled.

44.12.5

D-PHY bias control
The bias providing the reference to the D-PHY is enabled setting the PWRUP bit of the
DSI_BCFGR register.

44.13

Functional description: interrupts and errors
The interrupts can be generated either by the DSI Host or by the DSI Wrapper.
All the interrupts are merged in one interrupt lane going to the interrupt controller.

44.13.1

DSI Wrapper interrupts
An interrupt can be produced on the following events:
•

tearing effect event

•

end of refresh

•

PLL locked

•

PLL unlocked

Separate interrupt enable bits are available for flexibility.
Table 446. DSI Wrapper interrupt requests
Interrupt event

Event flag in DSI_WISR

Enable control bit in DSI_WIER

Tearing effect

TEIF

TEIE

End of refresh

ERIF

ERIE

PLL locked

PLLLIF

PLLLIE

PLL unlocked

PLLUIF

PLLUIE

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.13.2

RM0456

DSI Host interrupts and errors
The DSI_ISR0 and DSI_ISR1 registers are associated with error condition reporting. These
registers can trigger an interrupt to inform the system about the occurrence of errors.
The DSI Host has one interrupt line that is set high when an error occurs in either the
DSI_ISR0 or the DSI_ISR1 register.
The triggering of the interrupt can be masked by programming the mask registers DSI_IER0
and DSI_IER1. By default all errors are masked. When any bit of these registers is set to 1,
it enables the interrupt for a specific error. The error bit is always set in the respective
DSI_ISR register. The DSI_ISR0 and DSI_ISR1 registers are always cleared after a read
operation. The interrupt line is cleared if all registers that caused the interrupt are read.
The interrupt force registers (DSI_FIR0 and DSI_FIR1) are used for test purposes: they
allow triggering the interrupt events individually without the need to activate the conditions
that trigger the interrupt sources (it is extremely complex to generate the stimuli for that
purpose). This feature also facilitates the development and testing of the software
associated with the interrupt events. Setting any bit of these registers to 1 triggers the
corresponding interrupt.

<!-- pagebreak -->

