2093

Public key accelerator (PKA)

RM0456

Data formats
The format of the input data and the results in the PKA RAM are specified, for each
operation, in Section 53.4.

Executing a PKA operation
Each of the supported PKA operation is executed using the following procedure:

Note:

1.

Load initial data into the PKA internal RAM, which is located at address offset 0x400.

2.

Write in the MODE field of PKA_CR register, specifying the operation which is to be
executed and then assert the START bit, also in PKA_CR register.

3.

Wait until the PROCENDF bit in the PKA_SR register is set to 1, indicating that the
computation is complete.

4.

Read the result data from the PKA internal RAM, then clear PROCENDF bit by setting
PROCENDFC bit in PKA_CLRFR.

When PKA is busy (BUSY = 1) any access by the application to PKA RAM is ignored, and
the flag RAMERRF is set in PKA_SR.
Selecting an illegal or unknown operation in step 2 triggers an OPERRF error, and step 3
(PROCENDF = 1) never happens. See Section 53.3.7 for details.

Using precomputed Montgomery parameters (PKA Fast mode)
As explained in Section 53.3.4, when computing many operations with the same modulus it
can be beneficial for the application to compute only once the corresponding Montgomery
parameter (see, for example, Section 53.4.5). This is know as “Fast mode”.
To manage the usage of Fast mode it is recommended to follow the procedure described
below:

53.3.7

1.

Load in PKA RAM the modulus size and value information. Such information is
compiled in Section 53.5.1.

2.

Program in PKA_CR register the PKA in Montgomery parameter computation mode
(MODE=”0x1”) then assert the START bit.

3.

Wait until the PROCENDF bit in the PKA_SR register is set to 1, then read back from
PKA memory the corresponding Montgomery parameter, and then clear PROCENDF
bit by setting PROCENDFC bit in PKA_CLRFR.

4.

Proceed with the required PKA operation, loading on top of regular input data the
Montgomery information R2 mod m. All addresses are indicated in Section 53.4.

PKA error management
When PKA is used some errors can occur:

<!-- pagebreak -->

