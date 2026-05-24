1710

Parallel synchronous slave interface (PSSI)

RM0456

Figure 405. Bidirectional PSSI_DE/PSSI_RDY waveform

PSSI_PDCK

PSSI_D[15:0]

PSSI_DE
PSSI_RDY
MSv48849V2

Figure 406. Bidirectional PSSI_DE/PSSI_RDY connection diagram

PSSI_D[15:0]
PSSI_PDCK
Master transmitter

PSSI
PSSI_DE_RDY

MSv48850V2

42.4

PSSI interrupts
The PSSI generates only one interrupt (IT_OVR). It is consequently equivalent to the global
interrupt (pssi_it). Refer to Table 426 for the list of interrupts.
The PSSI and the DCMI share the same interrupt vector. When the PSSI ENABLE bit (bit 14
of PSSI_CR) is set to 1, these interrupts are triggered by the PSSI. Otherwise, they are
controlled by the DCMI.
The DCMI ENABLE bit (bit 14 of DCMI_CR) and PSSI ENABLE bit must not be set to 1 at
the same time.
Table 426. PSSI interrupt requests
Interrupt
acronym

IT_OVR

<!-- pagebreak -->

