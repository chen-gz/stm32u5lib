1674

Audio digital filter (ADF)

RM0456

Table 406. Programming sequence (CIC4) (continued)
Operations

Comments

SAD setting:
ADF_SADCR = 0x0000 1400

SADMOD = 1: Trigger based on sound level values
FRSIZE = 4: Sound level computed on 128 samples
HYSTEN = 0: No hysteresis
DETCFG = 0: interrupt generated when the SAD enters in DETECT state
DATCAP = 0: samples not transferred to memory
SADEN = 0: SAD not yet enabled

SAD setting:
ADF_SADCFGR = 0x0025 0001

ANMIN = 0x25: Sound threshold of 37 * 10^SNTHR/20 = 74 LSB(1)
HGOVR = 0: four frames (4 x 128 = 512 samples)
LFRNB = 0: Learning phase set to two frames. Not useful in this mode.
ANSLP = 0: Not used in this mode
SNTHR = 1: Threshold set to 6 dB (see ANMIN)

Clear status flags:
ADF_DFLT0ISR = 0x0000 0FFF

Clear all the status flags before running the ADF

Enable the SAD:
ADF_SADCR = 0x0000 1401
Wait for SADACTIVE flag = 1

Enable first the SAD, in order to be sure that data transfer to memory are
blocked by the SAD.

Enable the filter:
ADF_DFLT0CR = 0x0000 0001

NBDIS = 0: no samples discarded
ACQMOD = 0: Asynchronous continuous acquisition mode
DMAEN = 0: DMA interface not used
DFLTEN = 1: Filter enabled

1. SNTHR and ANMIN values are computed using the same approach than in Threshold programming with SADMOD = 01
(detection of a sound higher than 63 dBSPL).

Example 2
This example describes the programming of ADF for the capture of a signal coming from a
digital microphone, using the CIC5, and the RSFLT, with a total decimation of 64.
Table 407. Programming sequence (CIC5)
Operations

Comments

Adjust the proper kernel clock frequency via the RCC

Assuming that the RCC is programmed to provide a kernel clock
(adf_ker_ck) of 6.144 MHz

Select the proper ADF kernel clock
source via the RCC

Refer to the RCC of the product.

Enable the ADF clocks via the RCC

Refer to the RCC of the product.

Reset the ADF via the RCC

Refer to the RCC of the product.

AFMUX programming

Program the AFMUX to select ADF_SD0 and ADF_CCK0 functions.

Enable ADF processing clock:
ADF_CKGCR = 0x0201 0023

PROCDIV = 2 (division by 3): adf_proc_ck frequency is 6.144 MHz.
CCKDIV = 1 (division by 2): ADF_CCK0 clock frequency is 1.024 MHz.
The ADF_CCK0 pin is set in output and generates a clock so that the
microphone can exit from low-power mode.

<!-- pagebreak -->

