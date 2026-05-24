RM0456 Rev 6

RM0456

Clock recovery system (CRS)
The result must be always rounded up to the nearest integer value to obtain the best
trimming response. If frequent trimming actions are not needed in the application, the
hysteresis can be increased by slightly increasing the FELIM value.
The reset value of the FELIM field corresponds to (fTARGET / fSYNC) = 48000, and to a typical
trimming step size of 0.14%.

Note:

The trimming step size depends upon the product, check the datasheet for accurate setting.

Caution:

There is no hardware protection from a wrong configuration of the RELOAD and FELIM
fields, this can lead to an erratic trimming response. The expected operational mode
requires proper setup of the RELOAD value (according to the synchronization source
frequency), which is also greater than 128 * FELIM value (OUTRANGE limit).

12.5

CRS in low-power modes
Table 124. Effect of low-power modes on CRS
Mode

Description

Sleep

No effect. CRS interrupts cause the device to exit the Sleep mode.

Stop

CRS registers are frozen. The CRS stops operating until the Stop mode is exited and the
HSI48 oscillator is restarted.

Standby

The peripheral is powered down and must be reinitialized after exiting Standby mode.

Shutdown The peripheral is powered down and must be reinitialized after exiting Shutdown mode.

12.6

CRS interrupts
Table 125. Interrupt control bits
Interrupt event

Event flag

Enable
control bit

Clear
flag bit

Expected synchronization

ESYNCF

ESYNCIE

ESYNCC

Synchronization OK

SYNCOKF

SYNCOKIE

SYNCOKC

Synchronization warning

SYNCWARNF

SYNCWARNIE

SYNCWARNC

Synchronization or trimming error
(TRIMOVF, SYNCMISS, SYNCERR)

ERRF

ERRIE

ERRC

RM0456 Rev 6

<!-- pagebreak -->

