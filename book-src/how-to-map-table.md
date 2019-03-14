# How to Map Table to Key-value

## Where to Start The Implementation

<i>"We can solve any problem by introducing an extra level of indirection."</i> [^3]. 
Database is one of the most complicated software.
Database system engineers must jungle three balls:
develop the features, improve performance, and ensure correctness. 
A modern database evolves a spectrum of multiple technologies:
lower file system; fundamental storage engine; relational model mapper;
query optimizer; concensus protocol and implementation; isolation level,
monitoring etc. Luckily, abstraction is a fundamental tool to address this problem: we can separate
each of these features into different layers and each layer has at least one open source
choice. The era of Internet has established cornerstones for each layer.

The first part of the tutorial is to map SQL's data model to 
lower storage engine. When I became curious about this topic, I did some literature review.
CockroachDB [^1] and TiDB [^2], the two NewSQL implementations, have very good
documentation [^4] on how do they map the relational model to lower storage engine.
I recommend you reading at least CockroachDB's article on this, as the TiDB's article is in Chinese.

## Mapping 101

Key-value store requires keys to be unique and thus we need to uniquely map data in SQL
into unique keys. The data here are not only rows but also indexes.
Normally, when you construct a key, the basic idea is `<namespace>::<data>`.
There are three components here: `<namespace>`, encoding `::` and `<data>`.

`<namespace>` can be many things and the straightforward choice is table/index name.
In SQL, uniqueness of columns is always within a table. 
This is a good choice and we will use table name as the `<namespace>` 
for row data and combination of table and index names for indexes.
In later chapters, the `<namespace>` can be expanded to many other things.

`<data>` can contain the column name or ID but we still need some uniqueness.
The default uniqueness in a table is primary key. 
So a popular way [^2] [^3] to build `<data>` is `table_name::column_name::primary_key_of_row`.

The third component is encoding `::`. Encoding is important as it must satisfy two requirements:

* Stable prefix
* Fast/easy parsing

Key-value databases like RocksDB and LevelDB save keys into different SST files. 
Each SST file covers a range of `[a, b)` (by default, byte array comparison).
When you want to check a key `k`, database looks up in ranges' table and finds
a range `[a_i, b_i)` that satisfy `k in [a_i, b_i)`. Database won't further look into
any SST file till the range matches. This is the LST's way to handle indexes.
RocksDB even has a prefix optimization to assist these operations.
Considering a key `table_name::column_name::column_value::primary_key_of_row`,
the prefix `table_name::column_name::column_value` can easily pick up
all values of the column. Therefore, we need an encoding that preserves the byte order
of the prefix: there is no way for any column of any row has a different 
`table_name::column_name::column_value` format. 

A simple solution is use the format as it is: each component is connected by double colons.
However, the input is pure byte array, which means a key may contains double colons
and this makes it hard to parse the encoding. One difference between human-readable
and machine-parsable format is: machine can compute lengths easily.
So machine-oriented format prefers recording lengths of components as metadata;
human prefer stream without lengths metadata.


[^1]: [SQL in CockroachDB: Mapping Table Data to Key-Value Storage](https://www.cockroachlabs.com/blog/sql-in-cockroachdb-mapping-table-data-to-key-value-storage/)

[^2]: [基于 Raft 构建弹性伸缩的存储系统的一些实践](https://pingcap.com/blog-cn/building-distributed-db-with-raft/)

[^3]: [Fundamental theorem of software engineering](https://en.wikipedia.org/wiki/Fundamental_theorem_of_software_engineering)

[^4]: I am thankful to the trend of open source infrastructures: 
only rare company can implement from A to Z in house and
barely any company wants to be locked in any non open source solutions.

[^5]: [The Internals of PostgreSQL: 1.3. Internal Layout of a Heap Table File](http://www.interdb.jp/pg/pgsql01.html)

[^6]: [RocksDB Prefix Seek API](https://github.com/facebook/rocksdb/wiki/Prefix-Seek-API-Changes)
