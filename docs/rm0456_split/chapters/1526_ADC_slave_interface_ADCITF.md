1599

Multi-function digital filter (MDF)

RM0456

When the serial interface is enabled, the MDF must first be synchronized to the incoming
Manchester stream. The synchronization ends when a data transition from 0 to 1 or
from 1 to 0 (pink circle in the Figure 330) is detected.
The end of the synchronization phase can be checked by following the software sequence:
1.

Clear the CKABF flag in the MDF DFLTx interrupt status register x (MDF_DFLTxISR)
by writing CKABF bit to 1. If the serial interface is not yet synchronized, the hardware
immediately sets the CKABF flag to 1.

2.

Read the CKABF flag.
–

If CKABF = 1, go back to step 1.

–

If CKABF = 0, the Manchester interface is synchronized and provides valid data.

Programming example
In the following example, the MDF kernel clock frequency (Fmdf_ker_ck) is 100 MHz and the
received Manchester stream is at about 6 MHz (FSYMB).
1.

Provide a valid mdf_proc_ck to the SITFx.
The mdf_proc_ck frequency must be at least six times higher than the Manchester
symbol frequency (means at least 36 MHz).
PROCDIV is programmed to 1 to perform a division by two of the kernel clock. In that
case, Fmdf_proc_ck = 50 MHz (8.33 times higher than the Manchester symbol
frequency).

2.

Compute STH.
OVR is given by: OVR = Fmdf_proc_ck / FSYMB = 50 MHz / 6 MHz = 8.33.
( 2 × 8.33 ) – 1
3

Then STH[4:0] = round ⎛⎝ ----------------------------------- ⎞⎠ = 5
The minimum allowed frequency for the Manchester stream is then:
1 / (2 × STH × Tmdf_proc_ck) = 1 / (10 x 20 ns) = 5 MHz

The maximum allowed frequency for the Manchester stream is then:
1 / ((STH+1) × Tmdf_proc_ck) = 1 / (6 x 20 ns) = 8.33 MHz

39.4.4

ADC slave interface (ADCITF)
The ADCs are not always connected to the MDF. Refer to Section 39.3 to check the
situation for this product.
The MDF allows the connection of up to two ADCs to the filter path. For each filter, the
DATSRC[1:0] bitfield in the MDF digital filter configuration register x (MDF_DFLTxCICR)
allows the application to select either data from the ADCs.

Warning:

<!-- pagebreak -->

