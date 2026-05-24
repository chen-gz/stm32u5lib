1861

DSI Host (DSI)

44.5.1

RM0456

Video transmission mode
There are different video transmission modes, namely:
•

Burst mode

•

Non-burst mode
–

Non-burst mode with sync pulse

–

Non-burst mode with sync event

Burst mode
In this mode, the entire active pixel line is buffered into a FIFO and transmitted in a single
packet with no interruptions. This transmission mode requires that the DPI pixel FIFO has
the capacity to store a full line of active pixel data inside it. This mode is optimally used
when the difference between the pixel required bandwidth and DSI link bandwidth is
significant, it enables the DSI Host to quickly dispatch the entire active video line in a single
burst of data and then return to low-power mode.

Non-burst mode
In this mode, the processor uses the partitioning properties of the DSI Host to divide the
video line transmission into several DSI packets. This is done to match the pixel required
bandwidth with the DSI link bandwidth. With this mode, the controller configuration does not
require a full line of pixel data to be stored inside the LTDC interface pixel FIFO. It requires
only the content of one video packet.

Guidelines for selecting the burst or non-burst mode
Selecting the burst and non-burst mode is mainly dependent on the system configuration
and the device requirements. Choose the video transmission mode that suits the application
scenario. The burst mode is more beneficial because it increases the probability of the link
spending more time in the low-power mode, decreasing power consumption. The following
conditions must be met to get the maximum benefits from the burst mode of operation:
•

The DSI Host core must have sufficient pixel memory to store an entire pixel line to
avoid the overflow of the internal FIFOs.

•

The display device must support receiving a full pixel line in a single packet burst to
avoid the overflow on the reception buffer.

•

The DSI output bandwidth must be higher than the LTDC interface input bandwidth in a
relation that enables the link to go to low-power once per line.

If the system cannot meet these requirements, it is likely that the pixel data is lost causing
the malfunctioning of the display device while using the burst mode. These errors are
related to the capabilities of the system to store the temporary pixel data.
If all the conditions for using the burst mode cannot be met, use the non-burst mode to avoid
errors. The non-burst mode provides a better matching of rates for pixel transmission,
enabling:
•

only a certain number of pixels to be stored in the memory, and not requiring a full pixel
line (lesser LTDC interface RAM requirements in the DSI Host)

•

operation with devices that support only a small amount of pixel buffering (less than a
full pixel line).

The DSI non-burst mode must be configured so that the DSI output pixel ratio matches with
the LTDC interface input pixel ratio, reducing the memory requirements on both host and/or

<!-- pagebreak -->

