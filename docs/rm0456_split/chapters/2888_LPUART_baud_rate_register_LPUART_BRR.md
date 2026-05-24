21

RM0456 Rev 6

Res.
rw

2

17

16

DMAEN SAIEN
rw

rw

1

0

PRTCFG[1:0]

MODE[1:0]

rw

rw

rw

rw

RM0456

Serial audio interface (SAI)

Bit 27 MCKEN: Master clock generation enable
0: The master clock is not generated
1: The master clock is generated independently of SAIEN bit
Bit 26 OSR: Oversampling ratio for master clock
This bit is meaningful only when NODIV bit is set to 0.
0: Master clock frequency = FFS x 256
1: Master clock frequency = FFS x 512
Bits 25:20 MCKDIV[5:0]: Master clock divider
These bits are set and cleared by software.
000000: Divides by 1 the kernel clock input (sai_x_ker_ck).
Otherwise, The master clock frequency is calculated according to the formula given in
Section 69.4.8: SAI clock generator.
These bits have no meaning when the audio block is slave.
They have to be configured when the audio block is disabled.
Bit 19 NODIV: No divider
This bit is set and cleared by software.
0: the ratio between the Master clock generator and frame synchronization is fixed to 256 or 512
1: the ratio between the Master clock generator and frame synchronization depends on FRL[7:0]
Bit 18 Reserved, must be kept at reset value.
Bit 17 DMAEN: DMA enable
This bit is set and cleared by software.
0: DMA disabled
1: DMA enabled
Note: Since the audio block defaults to operate as a transmitter after reset, the MODE[1:0] bits must
be configured before setting DMAEN to avoid a DMA request in receiver mode.
Bit 16 SAIEN: Audio block enable
This bit is set by software.
To switch off the audio block, the application software must program this bit to 0 and poll the bit till it
reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it
is set to 0, otherwise the enable command is not taken into account.
This bit enables to control the state of the SAI audio block. If it is disabled when an audio frame
transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this
audio frame transfer.
0: SAI audio block disabled
1: SAI audio block enabled.
Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the
SAI block input before setting SAIEN bit.
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 OUTDRIV: Output drive
This bit is set and cleared by software.
0: Audio block output driven when SAIEN is set
1: Audio block output driven immediately after the setting of this bit.
Note: This bit has to be set before enabling the audio block and after the audio block configuration.

RM0456 Rev 6

<!-- pagebreak -->

