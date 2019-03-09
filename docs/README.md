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

There are tons of details to be filled in. And this is just the beginning:
users have the least of tolerance on bugs of database;
users have least intention to switch, until they are pushed to.

So why do the developers reinvent the wheel if there are so many decisions?
The answer is they have to. Before internet era, the community of
open source databases are relatively smaller than those of commercial competitors.
Commercial databases are more feature complete and most important,
they provide service and warranty. However, in the era of internet,
users of database gradually stand in the frontier.
The have to find out the new demands of databases and they have to 
implement features to meet the demands. With money from VC flowing into
the internet industry, the companies become able to maintain teams to
develop in house, instead of waiting for next version of commercial products.

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


