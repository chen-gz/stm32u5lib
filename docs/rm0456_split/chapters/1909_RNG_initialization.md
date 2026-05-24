RM0456 Rev 6

RM0456

True random number generator (RNG)
The RNG implements the following health check features in accordance with NIST
SP800-90B. The described thresholds correspond to the value recommended for register
RNG_HTCR (configuration A in Section 48.6.2).
1.

2.

3.

4.

Startup health tests, performed after reset and before the first use of the RNG as
entropy source:
–

Repetition count test, flagging an error when the noise source has provided more
than 40 consecutive bits at a constant value (0 or 1).

–

Adaptive proportion test running on a window of 1024 consecutive bits: the RNG
verifies that the first bit on the outputs of the noise source is not repeated more
than 688 times.

–

Known-answer tests, to verify the conditioning stage.

Continuous health tests, running indefinitely on the outputs of the noise source:
–

Repetition count test, similar to the one running in startup tests.

–

Adaptive proportion test, similar to the one running in startup tests.

Vendor specific continuous tests
–

Transition count test, flagging an error when the noise source has delivered more
than 32 consecutive occurrences of 2-bit patterns (01 or 10).

–

Real-time “too slow” sampling clock detector, flagging an error when one RNG
clock cycle (before divider) is smaller than AHB clock cycle divided by 32.

On-demand test of digitized noise source (raw data)
–

Supported by restarting the entropy source and rerunning the startup tests (see
software reset sequence in Section 48.3.4). Other kinds of on-demand testing
(software based) are not supported.

The CECS and SECS status bits in the RNG_SR register indicate when an error condition is
detected, as detailed in Section 48.3.7.
Note:

An interrupt can be generated when an error is detected.
Above the health test thresholds are modified by changing the value in the RNG_HTCR
register. See Section 48.6 for details.

48.3.4

RNG initialization
The RNG simplified state machine is pictured on Figure 458.
After enabling the RNG (RNGEN = 1 in RNG_CR), the following chain of events occurs:
1.

The analog noise source is enabled, and by default the RNG waits 16 cycles of RNG
clock cycles (before divider) before starting to sample the analog output and filling the
128-bit conditioning shift register.

2.

The conditioning hardware initializes, automatically triggering startup behavior test on
the raw data samples and known-answer tests.

3.

When startup health tests are completed. During this time, three 128-bit noise source
samples are used.

4.

The conditioning stage internal input data buffer is filled again with 128-bit and a
number of conditioning rounds defined by the RNG configuration (NIST or non-NIST) is
performed. The output buffer is then filled with the post processing result.

5.

The output buffer is refilled automatically according to the RNG usage.

RM0456 Rev 6

<!-- pagebreak -->

