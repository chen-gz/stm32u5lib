RM0456 Rev 6

RM0456

DSI Host (DSI)
device side. This is achieved by dividing a pixel line into several chunks of pixels and
optionally interleaving them with null packets.
The following equations show how the DSI Host core transmission parameters must be
programmed in non-burst mode to match the DSI link pixel output ratio (left hand side of the
“=” sign) and LTDC interface pixel input (right hand side of the “=” sign).
When the null packets are enabled:
lanebyteclkperiod * NUMC (VPSIZE * bytes_per_pixel + 12 + NPSIZE) / number_of_lanes
= pixels_per_line * LTDC_Clock_period
When the null packets are disabled:
lanebyteclkperiod * NUMC (VPSIZE * bytes_per_pixel + 6) / number_of_lanes
= pixels_per_line * LTDC_Clock_period

44.5.2

Updating the LTDC interface configuration in video mode
It is possible to update the LTDC interface configuration on the fly without impacting the
current frame. It is done with the help of shadow registers. This feature is controlled by the
DSI Host video shadow control register (DSI_VSCR).
The new configuration is only used when the system requests for it. To update the video
configuration during the transmission of a video frame, the configuration of that frame needs
to be stored in the auxiliary registers. This way, the new frame configurations can be set
through the APB interface without corrupting the current frame.
By default, this feature is disabled. To enable it, set the enable (EN) bit of the DSI Host video
shadow control register (DSI_VSCR) to 1. When this feature is enabled, the system
supplies the configuration stored in the auxiliary registers.
Figure 414 shows the steps needed to update the LTDC interface configuration.
Figure 414. Flow to update the LTDC interface configuration using shadow registers
Configure the LTDC interface

Request update

New resolution

Read DSI Host LTDC
shadow control register

Start video engine

Active

UR

Accepted
MSv35855V1

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

Immediate update
When the shadow register feature is active, the auxiliary registers require the LTDC
configuration before the video engine starts. This means that, after a reset, update register
(UR) bit is immediately granted.
When it is required to immediately update the active registers without the reset (as in
Figure 415), ensure that the enable (EN) and update register (UR) bits of the DSI Host video
shadow control register (DSI_VSCR) are set to 0.
Figure 415. Immediate update procedure
DSI Host

DSI Host

Default DPI Config
DPI CONFIG 1

DPI Config 1
Video Shadow Update

EN=0

EN=1

UR=0

UR=1
MSv35856V2

Updating the configuration during the transmission of a frame using APB
To update the LTDC interface configuration, follow the steps shown in Figure 416:
1.

Ensure that the enable (EN) bit of the DSI Host video shadow control register
(DSI_VSCR) register is set to 1.

2.

Set the update register (UR) bit of DSI Host video shadow control register (DSI_VSCR)
to 1.

3.

Monitor the update register (UR) bit. This bit is set to 0 when the update is complete.
Figure 416. Configuration update during the transmission of a frame
DSI Host

DSI Host

Default DPI Config
DPI CONFIG 1

DPI Config 1
Video Shadow Request

EN=1

EN=1

UR=0

UR=1
MSv35857V2

Requesting a configuration update
It is possible to request for the LTDC interface configuration update at any part of the frame.
DSI Host waits until the end of the frame to change the configuration. However, avoid
sending the update request during the first line of the frame because the data must
propagate between clock domains.

<!-- pagebreak -->

