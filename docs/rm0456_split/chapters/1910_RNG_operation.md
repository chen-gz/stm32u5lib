1921

True random number generator (RNG)

RM0456

The associated initialization time can be found in Section 48.5.
Figure 458. RNG initialization overview

1
Wait for noise
source

Software reset

2
Software reset

Start-up heath tests

Error state
Start-up test(s)
not OK

3
Conditioning keys
init

Continuous test(s)
not OK

4
Generate samples
5
MSv44204V2

Figure 458 also highlights a possible software reset sequence, implemented by:
1.

Writing bits RNGEN = 0 and CONDRST = 1 in the RNG_CR register with the same
RNG configuration and a new CLKDIV if needed.

2.

Then writing RNGEN = 1 and CONDRST = 0 in the RNG_CR register.

3.

Wait for random number to be ready, after initialization completes.

Note:

When the RNG peripheral is reset through RCC (hardware reset), the RNG configuration for
optimal randomness is lost in the RNG registers. Software reset with CONFIGLOCK set
preserves the RNG configuration.

48.3.5

RNG operation
Normal operation
To run the RNG using interrupts, the following steps are recommended:
1.

Check Section 48.6 to verify if a specific RNG configuration is required for the
application.
–

<!-- pagebreak -->

In this case, write in the RNG_CR register the bit CONDRST = 1 together with the
correct RNG configuration. Then perform a second write to the RNG_CR register

RM0456 Rev 6

RM0456

True random number generator (RNG)
with the bit CONDRST = 0, the interrupt enable bit IE = 1 and the RNG enable bit
RNGEN = 1.
–
2.

If it is not the case perform a write to the RNG_CR register with the interrupt
enable bit IE = 1 and the RNG enable bit RNGEN = 1.

An interrupt is now generated when a random number is ready or when an error
occurs. Therefore, at each interrupt, check that:
–

No error occurred. The SEIS and CEIS bits must be set to 0 in the RNG_SR
register.

–

A random number is ready. The DRDY bit must be set to 1 in the RNG_SR
register.

–

If the above two conditions are true the content of the RNG_DR register can be
read up to four consecutive times. If valid data is available in the conditioning
output buffer, four additional words can be read by the application (in this case the
DRDY bit is still high). If one or both of the above conditions are false, the
RNG_DR register must not be read. If an error occurred, the error recovery
sequence described in Section 48.3.7 must be used.

To run the RNG in polling mode following steps are recommended:
1.

2.

3.

Note:

Check Section 48.6 to verify if a specific RNG configuration is required for the
application.
–

If it is the case write in the RNG_CR register the bit CONDRST = 1 together with
the correction RNG configuration. Then perform a second write to the RNG_CR
register with the bit CONDRST = 0 and the RNG enable bit RNGEN = 1.

–

If it is not the case only enable the RNG by setting the RNGEN bit to 1 in the
RNG_CR register.

Read the RNG_SR register and check that:
–

No error occurred (the SEIS and CEIS bits must be set to 0)

–

A random number is ready (the DRDY bit must be set to 1)

If above conditions are true read the content of the RNG_DR register up to four
consecutive times. If valid data is available in the conditioning output buffer four
additional words can be read by the application (in this case the DRDY bit is still high).
If one or both of the above conditions are false, the RNG_DR register must not be read.
If an error occurred, the error recovery sequence described in Section 48.3.7 must be
used.

When data is not ready (DRDY = 0) RNG_DR returns zero.
It is recommended to always verify that RNG_DR is different from zero. Because when it is
the case a seed error occurred between RNG_SR polling and RND_DR output reading (rare
event).
If the random number generation period is a concern to the application and if NIST
compliance is not required it is possible to select a faster RNG configuration by using the
RNG configuration “B”, described in Section 48.6. The gain in random number generation
speed is summarized in Section 48.5.

Low-power operation
If the power consumption is a concern to the application, low-power strategies can be used,
as described in Section 48.3.8.

RM0456 Rev 6

<!-- pagebreak -->

