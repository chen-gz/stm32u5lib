1599

Multi-function digital filter (MDF)

RM0456

The table below shows which interrupt line is affected by which event, and how to clear and
activate each interrupt/event.
Table 384. MDF interrupt requests

MDF_FLTx_EVT(4)

MDF_FLTx(2)

MDF_FLTx_RX(3)

Interrupt
vectors

-

-

-

-

Event flag

Event/interrupt
clearing method

RXFIFO threshold
reached

FTHF

Read MDF_DFLTxDR
until RXFIFO level is
lower than the
threshold.

Snapshot data ready

SSDRF

Write SSDRF to 1.

Snapshot data
overrun

SSOVRF

Write SSOVRF to 1.

RXFIFO overrun

DOVRF

Write DOVRF to 1.

RSFLT overrun

RFOVRF

Write RFOVRF to 1.

Short-circuit detector

SCDF

Write SCDF to 1.

Saturation detection

SATF

Write SATF to 1.

Channel clock
absence detection

CKABF

Write CKABF to 1.

OLDF

Write OLDF to 1.

THHF
THLF

Interrupt event

Out-of-limit detector

Exit Sleep
mode

Exit Stop
modes(1)

Exit Standby
mode

Yes

Yes

No

Write OLDF to 1.

-

-

-

Write OLDF to 1.

-

-

-

1. Refer to Section 39.3: MDF implementation for details.
2. MDF_FLTx vector corresponds to the assertion of mdf_fltx_it signal
3. MDF_FLTx_RX vector corresponds to the assertion of mdf_fltx_rx signal.
4. MDF_FLTx_EVT vector corresponds to the assertion of mdf_fltx_evt signal.

39.7

MDF application informations

39.7.1

MDF configuration examples for audio capture
Table 385 gives some examples of the MDF settings for the digital microphones, focusing
on 16 and 48 kHz output data rate. In these examples, the following is expected:
•

The INT block is bypassed (INTVAL = 0).

•

The offset error compensation is disabled (OFFSET = 0).

Configurations #1 and #2 are for very low-power use-cases and have a reduced signal-tonoise ratio. The user must also insure that the selected digital microphone can work
properly at 512 kHz. These configurations can be used for sound detection. The RSFLT is
not used to reduce as much as possible the frequency of the kernel clock (mdf_ker_ck).
Configurations #3, #4, #9, #10, #11 give signal-to-noise ratios around 115 dB, with an ideal
microphone model, with a sinus signal of 997 Hz. Using the RSFLT allows a good control on
the in-band ripple and a good image rejection.

<!-- pagebreak -->

