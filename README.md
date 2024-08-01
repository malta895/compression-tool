# Compression tool (Simone SOS)

This tool implements the challenge at https://codingchallenges.fyi/challenges/challenge-huffman

## Binary format

The files produced by the tool will adhere to the following specifications:

1. The file starts with a dynamic-sized header that contains the required information to build a mapping between source text bytes and encoded bit sequences.
1. The remaining bytes represent the encoded source text.

### Header format

The header encodes a sequence of entries representing the couple byte and encoded symbol, the first byte of the header represents the number of entries hereby contained. The byte is written as-is, while the symbol is encoded in the following way:

1. Symbol size is dynamic, so the first byte of the symbol is the number of bits used to encode the symbol, say `n`.
1. The subsequent bytes `n` are the symbol itself.

An entry could be:
```
    / Byte from the source text
   /         / Number of bits in the encoded symbol
  /         /         ------------------ The encoded symbol
 /         /         /        /
00001010 00010000 11101010 00001000 
```