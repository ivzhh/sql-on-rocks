# Choose An Encoding for Keys And Values

## Msgpack

### Inconsistency

An example of inconsistency of msgpack is [number 42](https://docs.rs/rmp/0.8.7/rmp/#detailed).
It can be encoded as either of fixed number, 8/16/32/64 bits unsigned numbers.
This inconsistency is very dangerous: this can cause the `scan` with prefix missing the keys.
The problem is with this variant integer types. Variant integers can be expressed in many ways.
based on the type of the integer (8/16/32/64 bits). The encoder should be able to
detect not only the integer type, but also integer value range.
A small test for number 42 is added in the source code to verify this assumption.

## JSON