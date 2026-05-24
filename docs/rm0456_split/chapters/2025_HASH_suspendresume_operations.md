RM0456 Rev 6

RM0456

51.4.8

Hash processor (HASH)

HASH suspend/resume operations
Overview
It is possible to interrupt a hash/HMAC operation to perform another processing with a
higher priority. The interrupted process completes later when the higher-priority task has
been processed, as shown in Figure 503.
Figure 503. HASH suspend/resume mechanism
Message 1

Message 2

Block 1

Block 2
New higher
priority message
2 to be processed

HASH suspend
sequence
Block 3
Block 1

Block 2
(last block)
Block 4
HASH resume
sequence

Block 5

Block 6

...

MSv41985V2

To do so, the context of the interrupted task must be saved from the HASH registers to
memory, and then be restored from memory to the HASH registers.
The procedures where the data flow is controlled by software or by DMA are described
hereafter.

RM0456 Rev 6

<!-- pagebreak -->

2037

Hash processor (HASH)

RM0456

Data loaded by software
When the DMA is not used to load the message into the hash processor, the context can be
saved only when no block processing is ongoing.
To suspend the processing of a message, proceed as follows after writing the number of
words defined in NBWE:
1.

In Polling mode, wait for BUSY = 0, then poll if the DINIS status bit is set to 1.
In Interrupt mode, implement the next step in DINIS interrupt handler (recommended).

2.

Store the contents of the following registers into memory:
–

HASH_IMR

–

HASH_STR

–

HASH_CR

–

HASH_CSR0 to HASH_CSR37. HASH_CSR38 to HASH_CSR53 registers must
also be saved if an HMAC operation was ongoing.

To resume the processing of a message, proceed as follows:
1.

Write the following registers with the values saved in memory: HASH_IMR,
HASH_STR and HASH_CR.

2.

Initialize the hash processor by setting the INIT bit in the HASH_CR register.

3.

Write the HASH_CSRx registers with the values saved in memory.

4.

Restart the processing from the point where it has been interrupted.

Data loaded by DMA
When the DMA is used to load the message into the hash processor, it is recommended to
suspend and then restore a secure digest computing is described below. In this sequence
the DMA channel allocated to the hash peripheral remains allocated to the processing of
message 1 (see Figure 503).
To suspend the processing of a message using DMA, proceed as follows:

<!-- pagebreak -->

