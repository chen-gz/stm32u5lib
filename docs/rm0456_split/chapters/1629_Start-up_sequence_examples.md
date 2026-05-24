RM0456 Rev 6

RM0456

Audio digital filter (ADF)
In the example shown in Figure 377, the discard function is used to drop the first five
samples provided by the digital filter (S1 to S5). The first sample transfered to the
RXFIFO is S6.
Figure 377. Discard function example

Output filter
Provided to
RXFIFO

Dropped

S5

S4

S6

S7

S8

S9...

S3
TPCM

S2

S1

time

40.4.9

MSv63621V1

Start-up sequence examples
Figure 378 details a start of acquisition sequence of a digital filter triggered by DFLTEN
(ACQMOD[2:0] = 0), with NBDIS[7:0] = 3 (three samples to discard before acquisition).
The DFLT0 is configured for audio application: MCIC, RSFLT and HPF activated.The data
interface (SITF0 or ADCITF) is assumed to be already activated.

Note:

NBDIS[7:0] is set on purpose to a small value to simplify the drawing.
Figure 378. Start sequence with DFLTEN, in continuous mode, audio configuration
Re-sync of DFLTEN with adf_proc_ck

ADCITF/SITF
status

Enabled
000

ACQMOD
DFLTEN
adf_proc_ck

bs0_[r|f]_ck
MCIC
dec. counter

0

1 2 ... N 1 2 ... N

...

1 2 ... N

...

1 2 ... N

...

1 2 ... N

...

1 2 ... N

...

1 2 ... N

...

1 2 ... N

...

MCIC_OUT

x

MCIC_S0

...

MCIC_S3

...

MCIC_S7

...

MCIC_S11

...

MCIC_S15

...

MCIC_S19

...

MCIC_S23

...

RSFLT
dec. counter

0

1...

...

...4

...

...4

...

...4

...

...4

...

...4

...

...4

...

P1
x

RSFLT_OUT

RSFLT_S0

RSFLT_S1

RSFLT_S2

RSFLT_S3

RSFLT_S4

RSFLT_S5

P2
HPF_OUT

NBDIS_CNTR
Sample stored
in RXFIFO

x
x

HPF_S0

3

HPF_S1

HPF_S2

2

1

x

HPF_S3

HPF_S4

HPF_S5

0

HPF_S3

HPF_S4

HPF_S5
MSv63664V1

The DFLTEN bit is re-sampled into the ADF processing clock domain. When DFLTEN is
detected high, the filter chain is enabled, and the decimation counter of the MCIC filter is
incremented at the rate of the bitstream clock.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

When the MCIC decimation counter reached its programmed value N, a sample is available
for the RSFLT.
The RSFLT processes all the samples provided by the MCIC, and delivers a sample to the
HPF every-time it processes four samples (decimation by 4). The RSFLT needs up to 24
cycles of adf_proc_ck clock before delivering a sample (P1).
The HPF processes all the samples provided by the RSFLT, but the NBDIS function
prevents the data writing in the RXFIFO as long as NBDIS_CNTR does not reach 0.
When NBDIS_CNTR reaches 0, the samples provided by the HPF are stored into the
RXFIFO.

40.4.10

Sound activity detection (SAD)
The SAD is based on the computation of the ambient noise level (ANLVL) and of the
short-term sound level (SDLVL). The SAD offers the following ways to detect a sound:
•

when the SDLVL reaches a threshold referenced to the ambient noise level

•

when the SDLVL reaches a fixed threshold

•

when the ANLVL reaches a fixed threshold

As shown in Figure 379, the SAD takes the 16 MSB samples from the DFLT0 output.
Figure 379. SAD block diagram

SAD
Logic

ADF_SADCR.SADEN
ADF_SADCR.DATCAP[1:0]
ADF_SADCR.SADST[1:0]
SDLVL computation

PCM[23:8]

xn

ANLVL update
Thresholds update

Sound level
(SDLVL)
computation

Learning phase
ANLVL computation

SAD trigger
logic

sddet_evt

Detection
sdlvl_evt

DFLT0

RXFIFO0
PCM[23:0]

MSv63623V1

The SAD is highly configurable, and the application can adjust several parameters:

<!-- pagebreak -->

•

SAD detection behavior (SADMOD)

•

number of samples used to compute the sound level (FRSIZE)

•

number of frames used to compute the ambient noise level during the learning phase
(LFRNB)

•

slope of the ambient noise estimator (ANSLP)

•

minimum expected ambient noise level (ANMIN)

•

threshold level (SNTHR)

•

threshold hysteresis (HYSTEN)

•

hangover window in order to filter spurious transitions between DETECT and
MONITOR states (HGOVR)

•

data capture mode (DATCAP)

RM0456 Rev 6

RM0456

Audio digital filter (ADF)

SAD detection behavior
The SAD can use the following ways to detect a sound, selected by SADMOD[1:0]:
•

When SADMOD[1:0] = 0, the SAD works like a voice-activity detection. In this mode,
the SAD estimates the ambient noise level according to the computed sound level
values. The threshold of the trigger is elaborated from the estimated ambient noise.
Finally the current sound level is compared to this threshold. In a first approximation,
the SAD triggers if the peak-to-average value of the input signal reaches a level
defined by SNTHR[3:0].

•

When SADMOD[1:0] = 01, the SAD compares the current sound level (SDLVL) to a
fixed trigger value defined by the application via SNTHR[3:0] and ANMIN[12:0]. This
mode allows a fast SAD reaction as the amount of samples used to compute the sound
level can be configured via FRSIZE[2:0].

•

When SADMOD[1:0] = 1x, the SAD compares the estimated ambient noise level
(ANLVL) to a fixed trigger value defined by the application via SNTHR[3:0] and
ANMIN[12:0]. This mode avoids unwanted triggers, due to peak levels, but the SAD
reacts more slowly to an input signal variation. It is nevertheless possible to adjust the
reaction time via FRSIZE[2:0] and ANSLP[2:0].

SAD states
As shown in Figure 379, the SAD works as follows:
1.

When enabled (SADEN = 1), the SAD is first in LEARN state to perform a first
estimation of the ambient noise level.

2.

The SAD continuously computes the short-term sound level (SDLVL) using the
samples provided by the DFLT0. The amount of samples used to compute the sound
level is given by FRSIZE[2:0]. The samples processed by the DFLT0 can be
transferred into the memory or not depending on DATCAP[1:0] value.

3.

The initial ambient noise level (ANLVL) is computed using the consecutive sound level
values. The application can define how much sound level values are used to perform
the computation of this initial ambient noise estimation (LFRNB).

4.

When the initial ambient noise level (ANLVL) is computed, the SAD switches to the
MONITOR state.

5.

Every time a new short-term sound level value is available, the SAD updates the
ambient noise level and the thresholds according to the selected detection mode.

6.

7.

If the SAD triggers, then the following happens:
–

The SAD switches to DETECT state.

–

The sddet_evt event is asserted.

–

The adf_sad_det signal is set to high.

The hangover function insures that the DETECT state is maintained even if the sound
level goes below the threshold level for a time given by HGOVR.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456
Figure 380. SAD flow diagram
SAD enable
SADST= LEARN
Compute sound level (SDLVL)
(using FRSIZE samples)

When SADMOD = 1x

When SADMOD = 01

When SADMOD = 0

Update ANLVL

Update THRH, THRL values

Update ANLVL

(Using SDLVL, ANSLP)

(Using ANMIN, HYSTEN, SNTHR)

(Using SDLVL, ANSLP, ANMIN)

SELSIG = SDLVL

(Using ANMIN, HYSTEN, SNTHR)

(Using ANLVL, HYSTEN, SNTHR)

SELSIG = ANLVL

SELSIG = SDLVL

N

SADST =
MONITOR?

THR = THRL

N

HGCNT
=0?

HGCNT = HGCNT - 1

SADST
= LEARN ?

Y

Compute initial ambient noise level (ANLVL)
(using LFRNB, SDLVL)

Update THRH, THRL values

Update THRH, THRL values

N

N

ANLVL
available ?
N

Y

SADST= MONITOR

Y

THR = THRH

SELSIG >
THR ?

Y

SADST = DETECT

Y

HGCNT = HOVR

SADST= MONITOR

ANLVL update
if SADMOD = 1x

ANLVL update

MSv62687V1

Sound level computation (SDLVL)
Once enabled, the SAD computes continuously the sound level value. The sound level
represents the average of the absolute value of an amount of PCM samples given by
FRSIZE[2:0].
N FRSIZE

1
SDLVL = ---------------------- ×
N FRSIZE

∑ PCM ( n )
n=1

where NFRSIZE is the amount of PCM samples given by FRSIZE[2:0].

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Audio digital filter (ADF)

Ambient noise estimation (ANLVL)
The ambient noise level (ANLVL) is computed when SADMOD[1:0] is 00 or 10.
The ambient noise level is computed differently according to the state of the SAD as
detailed below:
•

ANLVL computation during the LEARN state
Every time the SAD is enabled, a learning phase is initiated in order to estimate a first
value of the ambient noise level. During this phase, the SAD cannot trigger.
During the LEARN phase, the ambient noise level is computed as follows:
N LFRNB

1
ANLVL = -------------------- ×
N LFRNB

∑ SDLVL ( n )
n=1

where NLFRNB is the amount of frames given by LFRNB[2:0] bitfield.
•

ANLVL computation during the MONITOR or DETECT state
When the learning phase is completed, the SAD updates the ambient noise level in the
following way:
–

The SAD computes the new possible values for the ambient noise level:
ANLVL_UP = ANLVL * (1+2^(ANSLP-12))
ANLVL_DN = ANLVL * (1-2^(ANSLP-10))

–

The ANLVL takes the ANLVL_DN value if the current sound level is lower than
ANLVL_DN, otherwise ANLVL takes the value of ANLVL_UP.
The ANLVL is not updated if the current sound level is higher than the threshold
level, except if SADMOD[1:0] = 10.

–

When SADMOD[1:0] = 0, if the new ANLVL value is lower than ANMIN[12:0],
ANLVL is replaced by ANMIN.

The slope of the noise estimator can be adjusted to optimize the detection of the
wanted signal. This slope is adjusted via ANSLP[2:0] in the ADF SAD configuration
register (ADF_SADCFGR).

RM0456 Rev 6

<!-- pagebreak -->

