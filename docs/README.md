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

* What language do you want to use
* What platform do you want to 

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


