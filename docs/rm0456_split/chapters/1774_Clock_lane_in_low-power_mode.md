1861

DSI Host (DSI)

RM0456

Keep the maximum number of bytes read from the peripheral to a minimum, to issue the
read commands in a line time. Ensure that MRD_TIME x lane byte clock period is less than
LPSIZE x 16 x escape clock period of the host, otherwise, the read commands are
dispatched on the last line of a frame. If it is necessary to read a large number of parameters
(> 16), increase the MRD_TIME while the read command is executed. When the read has
completed, decrease the MRD_TIME to a lower value.
If a read command is issued on the last line of a frame, the LTDC interface is halted and
stays halted until the read command is in progress. The video transmission must be stopped
during this period.

44.9.5

Clock lane in low-power mode
To reduce the power consumption of the D-PHY, the DSI Host, when not transmitting in the
high-speed mode, allows the clock lane to enter into the low-power mode. The controller
automatically handles the transition of the clock lane from HS (clock lane active sending
clock) to LP state without direct intervention by the software. This feature can be enabled by
configuring the DPCC and the ACR bits of the DSI_CLCR register.
In the command mode, the DSI Host can put the clock lane in the low-power mode when it
does not have any HS packets to transmit.
In the video mode (LTDC interface), the DSI Host controller uses its internal video and PHY
timing configurations to determine if there is time available for the clock line to enter the lowpower mode and not compromise the video data transmission of pixel data and sync events.
Along with a correct configuration of the video mode (see Section 44.5: Functional
description: video mode on LTDC interface), the DSI Host needs to know the time required
by the clock lane to go from high-speed to low-power mode and vice-versa. The values
required can be obtained from the D-PHY specification: program the DSI_CLTCR register
with the following values:
•

HS2LP_TIME = time from HS to LP in clock lane / byte clock period in HS (lanebyteclk)

•

LP2HS_TIME = time from LP to HS in clock lane / byte clock period in HS (lanebyteclk)

Based on the programmed values, the DSI Host calculates if there is enough time for the
clock lane to enter the low-power mode during inactive regions of the video frame.
The DSI Host decides the best approach to follow regarding power saving out of the three
possible scenarios:
•

there is no enough time to go to the low-power mode. Therefore, blanking period is
added as shown in Figure 435

•

there is enough time for the data lanes to go to the low-power mode but not enough
time for the clock lane to enter the low-power mode, see Figure 436.

•

there is enough time for both data lanes and clock lane to go to the low-power mode,
as in Figure 437.
Figure 435. Clock lane and data lane in HS

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
Figure 436. Clock lane in HS and data lanes in LP

Figure 437. Clock lane and data lane in LP

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

44.10

RM0456

Functional description: virtual channels
The DSI Host supports choosing the virtual channel (VC) for use for each interface. Using
multiple virtual channels, the system can address multiples displays at the same time, when
each display has a different virtual channel identifier.
When the LTDC interface is configured for a particular virtual channel, it is possible to use
the APB register interface to issue the commands while the video stream is being
transmitted. With this, it is possible to send the commands through the ongoing video
stream, addressing different virtual channels and thus enable the interface with multiple
displays. During the video mode, the video stream transmission has the maximum priority.
Therefore, the transmission of sideband packets such as the ones from the generic interface
are only transported when there is time available within the video stream transmission. The
DSI Host identifies the available time periods and uses them to transport the generic
interface packets. Figure 438 illustrates where the DSI Host inserts the packets from the
APB generic interface within the video stream transmitted by the LTDC interface.

Short packets transmitted through the DPI interface
Payload transmitted through the DPI interface
Commands transmitted through the APB register interface (different VC)
Areas where the controller can transmit commands when there is enough time

MS35885V2

Figure 438. Command transmission by the generic interface

It is also possible to address the multiple displays with only the generic interface, using
different virtual channels. Because the generic interface is not restricted to any particular
virtual channel through configuration, it is possible to issue the packets with different virtual
channels. This enables the interface to time multiplex the packets to be provided to the
displays with different virtual channels.
You can use the following configuration registers to select the virtual channel ID associated
with transmissions over the LTDC and APB register interfaces:

<!-- pagebreak -->

