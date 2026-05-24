1599

Multi-function digital filter (MDF)

RM0456

The application to select the wanted stream via the MDF bitstream matrix control register x
(MDF_BSMXxCR). This selection is intended to be static.
Figure 332. BSMX overview

MDF
ADCITF[2,1]

BSMX
SCD0
DFLT0

bs_flt0

bs0_r
bs0_f
bs1_r
bs1_f
...
bs(N-1)_r
bs(N-1)_f

bs0_r
bs0_f

SITF0

MDF_BSMX0CR.BSSEL

SCD1
DFLT1

bs_flt1

SCD(N-1)
DFLT(N-1)

bs_flt(N-1)

MDF_BSMX(N-1)CR.BSSEL

bs0_r
bs0_f
bs1_r
bs1_f
...
bs(N-1)_r
bs(N-1)_f

bs1_r
bs1_f

SITF1

...

...

...

MDF_BSMX1CR.BSSEL

bs0_r
bs0_f
bs1_r
bs1_f
...
bs(N-1)_r
bs(N-1)_f

bs(N-1)_r
bs(N-1)_f

SITF(N-1)

MSv62658V1

BSMX programming sequence example
The BSSEL[4:0] bitfield cannot be changed if the corresponding SCDx, OLDx or DFLTx is
enabled. The following steps are needed to change the value of BSMX, for filter path x:

39.4.7

1.

Set SCDEN of SCDx to 0.

2.

Set DFLTEN of DFLTx to 0.

3.

Set OLDEN of OLDx to 0.

4.

Wait for BSMXACTIVE = 0.

5.

Program BSSEL[4:0] of filter path x.

6.

Set SCDEN of SCDx to 1 (optional).

7.

Set DFLTEN of DFLTx to 1 (optional).

8.

Set OLDEN of OLDx block to 1 (optional).

Short-circuit detectors (SCD)
The SCDx detects, with a very fast response time, if an analog signal reached saturated
values (out-of-full scale ranges) and remained on this value for a given time.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
This behavior can detect short-circuit or open-circuit errors (such as over current or over
voltage). An interrupt event or/and a break signal can be generated.
Figure 333. SCD functional view

MDF

SCD
scd_brkx
(to break interface)

MDF_SCDxCR.SCDT[7:0]

Comparator
scd_evtx
(to interrupts interface)

scd_cnt[7:0]
Transition
detector

mdf_ker_ck

reset

Counter

bs_fltx_dat
bs_fltx
(from BSMX)

bs_fltx_ck

MSv62659V1

The SCDx is counting consecutive zeros or consecutive ones received from the serial
interface (SITFx). A counter is restarted if there is a change in the data stream received. If
this counter reaches a short-circuit threshold value (SCDT[7:0] in the MDF short circuit
detector control register x (MDF_SCDxCR)), then a short-circuit event is invoked. Each
BSMX output has its own short-circuit detector. Any BSMX output can be continuously
monitored by setting the corresponding SCDEN bit to 1 in MDF_SCDxCR. Each SCD has
its own threshold settings (SCDT) and its own status bit (SCDF).
The figure below shows an example where SCDT[7:0] is set to six. The break signal
remains high as long as the short circuit condition is present.
No overrun event is generated when the interrupt event is pending while a new short-circuit
condition is detected.
Figure 334. SCD timing example
bs_fltx_dat

bs_fltx_ck

scd_cnt[7:0]

...

2

3

4

1

2

3

4

1

2

3

4

5

6

1

1

2

3

4

5

6

1

2 1

mdf_ker_ck

scd_brkx

mdf_evt_itx/mdf_itx
(due to scd_evtx)

Writing 1 to SCDF
MSv62660V1

RM0456 Rev 6

<!-- pagebreak -->

