RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
else if (ACK)
{
Reset Error Count
Mask ACK
}

The application is expected to write the data packets into the transmit FIFO when the
space is available in the transmit FIFO and the request queue. The application can
make use of the NPTXFE interrupt in OTG_GINTSTS to find the transmit FIFO space.
b)

Bulk/control IN

Unmask (TXERR/XFRC/BBERR/STALL/DTERR)
if (XFRC)
{
Reset Error Count
Unmask CHH
Disable Channel
Reset Error Count
Mask ACK
}
else if (TXERR or BBERR or STALL)
{
Unmask CHH
Disable Channel
if (TXERR)
{
Increment Error Count
Unmask ACK
}
}
else if (CHH)
{
Mask CHH
if (Transfer Done or (Error_count == 3))
{
De-allocate Channel
}
else
{
Re-initialize Channel
}
}
else if (ACK)
{
Reset Error Count
Mask ACK
}

RM0456 Rev 6

<!-- pagebreak -->

