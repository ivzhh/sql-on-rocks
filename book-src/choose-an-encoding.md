# Choose An Encoding for Keys And Values

Basic we have two types of choices: binary encoding for machine,
and text encoding for human. 

So a human-oriented encoding prefers to define "a boundary" to define
a element, like quote, line breaker and etc. For example, in Redis protocol,
the string with length is encoded as `$6\r\nfoobar\r\n` [^1]. You need
`\r\n` as some boundary to separate length from string content. 
If we use text-based stream message to encode our key and value,
we need to use a similar method. (we cannot use Redis protocol,
as the length is encoded in the beginning).

Binary encoding is very intuitive on machine. We define 
byte ranges for different purposes and we don't need the boundary marks.

The following will examine two types of encoding: msgpack (binary) and
JSON (text). And after the discussion, I will bring up a fix for 
msgpack for prefix encoding.

## Two Case-studies

### Msgpack

#### Inconsistency

An example of inconsistency of msgpack is [number 42](https://docs.rs/rmp/0.8.7/rmp/#detailed).
It can be encoded as either of fixed number, 8/16/32/64 bits unsigned numbers.
This inconsistency is very dangerous: this can cause the `scan` with prefix missing the keys.
The problem is with this variant integer types. Variant integers can be expressed in many ways.
based on the type of the integer (8/16/32/64 bits). The encoder should be able to
detect not only the integer type, but also integer value range.
A small test for number 42 is added in the source code to verify this assumption.

#### Prefix Incomparable

RocksDB and its derivatives utilize prefix scan to replace B+ tree index.
However, a string can not fit into this requirement. For example, two strings
`abc` and `b`. The length is encoded in the first byte and thus `b` is smaller
than `abc`; however, alphabetical sort should put `abc` before `b`.

This is a key failure in choosing msgpack. When you construct a prefix,
you cannot enumerate all lengths of prefixes. Thus you cannot construct
a proper prefix.

A solution will be stated in later section.

### JSON

[^1]: [Redis Protocol specification](https://redis.io/topics/protocol)
