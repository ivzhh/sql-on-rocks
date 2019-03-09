# Tutorial: Write An SQL Engine on RocksDB

## Table of Contents

### Chapters

- [How to Map Table to Key-value](./how-to-map-table.md)

### Appendices

- [Choose An Encoding](./choose-an-encoding.md)

## Introduction

When you read the title, you may think:
people say "Don't re-invent the wheel" or "If it ain't broke, don't fix it".
But in the industry, it has never been the case. Re-implementations are like
Dandelion on your lawn: they come back, stronger.
Software is a special kind of wheel: it is an ecosystem.
For example, if you want to implement a relational database, then besides the
core B-tree structure, you have also consider:

* Language of implementation
* Is it cross-platform?
* Embedded or standalone?
* OLAP or OLTP
* SQL language completeness; dialect of SQL to use
* Isolation level and locking mechanism
* Replication (physical and logical)
* Distributed or not? (Spanner-like or Calvin-like)
* Toolset: dump; importing; user management; etc.

There are tons of details to be filled in. 
The evolvement gradually hides the designs, which are always
neat and straightforward in the beginning, with code for 
error handling, compatibility, performance, etc. 
The code becomes much less self-explanatory for the novice.
For example, MySQL has implemented SQL engine on 
MyISAM and InnoDB decades ago[^1]. However, if you want to know
how they did it, you have to read documents/code and 
use accurate keywords to dig it out of Google 
(MySQL supports the era of Internet; there are so many articles
on it and thus the a lot of details are buried in thousands of 
pages; most of the articles are either general on introduction
or specific on tuning for cluster).

# Motivation

I don't know why I prefer database topics. My daily browsing always
leads to database related articles. I pay more attention to keywords
like Postgres, TiDB, RocksDB, etc. 

A year ago, I learned Rust and as the result, I have a `hg status`
implemented in Rust. But since than, I work

## Data Structure

Log-structure Merge Tree (LSM) is a classic paper in 1996 and it shows 
the power of append-only structure in disk-oriented file writing.
But "The devil is in the detail": implementing an efficient LSM is not easy.
LevelDB gets very famous as its SSTable has been well tested in production as part of BigTable.
It has been proven to be efficient implementation as 
["implementation details matter a great deal"](https://www.igvita.com/2012/02/06/sstable-and-log-structured-storage-leveldb/).
However, the good greed drives industry to further dig out more from best practice;
as a happy result, Facebook rolled out their RocksDB, 
an enhanced "Swiss Army knife" built upon LevelDB. Atop that, 
Facebook then starts to build MyRocks, a storage engine for MySQL 5.6.
MyRocks utilizes.

[^1]: [SQL in CockroachDB: Mapping Table Data to Key-Value Storage](https://www.cockroachlabs.com/blog/sql-in-cockroachdb-mapping-table-data-to-key-value-storage/)
