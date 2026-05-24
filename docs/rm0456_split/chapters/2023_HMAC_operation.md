RM0456 Rev 6

RM0456

51.4.7

Hash processor (HASH)

HMAC operation
Overview
As specified by Internet Engineering Task Force RFC2104 and NIST FIPS PUB 198-1, the
HMAC algorithm is used for message authentication by irreversibly binding the message
being processed to a key chosen by the user. The algorithm consists of two nested hash
operations:
HMAC(message)= Hash((Key | pad) XOR opad |
Hash((Key | pad) XOR ipad | message))

where:

Note:

•

opad = [0x5C]n (outer pad) and ipad = [0x36]n (inner pad)

•

[X]n represents a repetition of X n times, where n equal to the size of the underlying
hash function data block (n = 64 for 512-bit blocks).

•

pad is a sequence of zeroes needed to extend the key to the length n defined above. If
the key length is greater than n, the application must first hash the key using Hash()
function and then use the resultant byte string as the actual key to HMAC.

•

| represents the concatenation operator.

HMAC mode of the hash processor can be used with all supported algorithms.

HMAC processing
Four different steps are required to compute the HMAC:

Note:

Note:

1.

The software writes the INIT bit to 1 with the MODE bit at 1 and the ALGO bits set to
the value corresponding to the desired algorithm. The LKEY bit must also be set to 1 if
the key being used is longer than 64 bytes. In this case, as required by HMAC
specifications, the hash processor uses the hash of the key instead of the real key.

2.

The software provides the key to be used for the inner hash function, using the same
mechanism as the message string loading, that is writing the key data into HASH_DIN
register then completing the transfer by writing DCAL bit to 1 and the correct
NBLW[4:0] to HASH_STR register.

Endianness details can be found in Section 51.4.4: Message data feeding.
3.

Once the processor is ready again (DINIS = 1 in HASH_SR), the software can write the
message string to HASH_DIN. When the last word of the last block is entered and the
software writes DCAL bit to 1 in HASH_STR register, the NBLW[4:0] bitfield must be
written at the same time to a value different from zero if the message length is not an
exact multiple of the block size. Note that the DMA can also be used to feed the
message string, as described in Section 51.4.4: Message data feeding.

4.

Once the processor is ready again (DINIS = 1 in HASH_SR), the software provides the
key to be used for the outer hash function, writing the key data into HASH_DIN register
then completing the transfer by writing DCAL bit to 1 and the correct NBLW[4:0] to
HASH_STR register. The HMAC result can be found in the valid output registers
(HASH_HRx) as soon as DCIS bit is set to 1.

The computation latency of the HMAC primitive depends on the lengths of the keys and
message, as described in Section 51.4.11: HASH processing time.
HMAC example
Below is an example of HMAC SHA-1 algorithm (ALGO = 00 and MODE = 1 in HASH_CR)
as specified by NIST.

RM0456 Rev 6

<!-- pagebreak -->

2037

Hash processor (HASH)

RM0456

Let us assume that the original message is the ASCII binary-coded form of “Sample
message for keylen = blocklen”, of length L = 34 bytes. If the HASH is programmed
in no swapping mode (DATATYPE = 00 in HASH_CR), the following data must be loaded
sequentially into HASH_DIN register:
1.

Inner hash key input (length = 64, that is no padding), specified by NIST. As key
length = 64, LKEY bit is set to 0 in HASH_CR register
00010203 04050607 08090A0B 0C0D0E0F 10111213 14151617
18191A1B 1C1D1E1F 20212223 24252627 28292A2B 2C2D2E2F
30313233 34353637 38393A3B 3C3D3E3F

2.

Message input (length = 34, that is padding required). HASH_STR must be set to
0x20 to start message padding and inner hash computation (see ‘U’ as don’t care)
53616D70 6C65206D 65737361 67652066 6F72206B 65796C65
6E3D626C 6F636B6C 656EUUUU

3.

Outer hash key input (length = 64, that is no padding). A key identical to the inner
hash key is entered here.

4.

Final outer hash computing is then performed by the HASH. The HMAC-SHA1 digest
result is available in the HASH_HRx registers (x = 0 to 4), as shown below:
HASH_HR0 = 0x5FD596EE
HASH_HR1 = 0x78D5553C
HASH_HR2 = 0x8FF4E72D
HASH_HR3 = 0x266DFD19
HASH_HR4 = 0x2366DA29

<!-- pagebreak -->

