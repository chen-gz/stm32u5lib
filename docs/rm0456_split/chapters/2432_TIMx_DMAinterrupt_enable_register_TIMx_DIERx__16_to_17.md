RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)
The counter can be reloaded on the fly by setting FRFCRx in GFXTIM_TCR. This force
reload neither stop the timer (even if RFCxCM = 0 in GFXTIM_TCR), nor set RFCxRF
(reload flag) in GFXTIM_ISR.
The relative frame counter can be disabled by setting RFCxDIS (disable) in GFXTIM_TDR.

59.3.7

Tearing-effect detection
A tearing-effect line can work in one of the two configurations shown in the figure below.
Figure 763. Tearing-effect configurations
Display refresh duration
TE (VSYNC)
Line
duration

TE (VSYNC + HSYNC)
MSv66921V1

A tearing-effect event can be generated on rising or falling edge depending on TEPOL
(tearing-effect polarity) in GFXTIM_CR.
When a tearing-effect event is detected, TEF (tearing-event flag) is set in GFXTIM_ISR, and
an interrupt is generated if TEIE (interrupt enable) is set in GFXTIM_IER.

59.3.8

Event generator
The event generator can combine timer events into complex events. Up to four combined
events can be generated.
The events can be used for:
•

interrupt generation

•

watchdog clocking

•

external trigger generation

A complex event is a combination between a frame event and a line event.
Once a frame event occurs, the GFXTIM waits for the line event to occur before generating
the complex event.
The frame event is selected by the corresponding FESx (frame event selection x)
in GFXTIM_EVSR (events selection y register). The line event is selected by
the corresponding LESx in GFXTIM_EVSR.
The complex event generation is enabled by setting the corresponding EVxEN (event x
enable) in GFXTIM_EVCR (event control register).
It is recommended to disable the event generation prior to any event configuration to avoid
spurious complex event generation
When a complex event occurs, the corresponding EVxF (event x flag) is set
in GFXTIM_ISR, and an interrupt is generated if EVxIE (event x interrupt enable) is set
in GFXTIM_IER.
Each of the events can be connected to another peripherals (such as DMA) to generate
hardware triggers.

RM0456 Rev 6

<!-- pagebreak -->

