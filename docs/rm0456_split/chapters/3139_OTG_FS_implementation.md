3291

USB on-the-go full-speed (OTG_FS)

RM0456
Figure 903. Receive FIFO read task

•

Bulk and control OUT/SETUP transactions
A typical bulk or control OUT/SETUP pipelined transaction-level operation is shown in
Figure 904. See channel 1 (ch_1). Two bulk OUT packets are transmitted. A control
SETUP transaction operates in the same way but has only one packet. The
assumptions are:

•

–

The application is attempting to send two maximum-packet-size packets (transfer
size = 1, 024 bytes).

–

The non-periodic transmit FIFO can hold two packets (128 bytes for FS).

–

The non-periodic request queue depth = 4.

Normal bulk and control OUT/SETUP operations
The sequence of operations in (channel 1) is as follows:

<!-- pagebreak -->

