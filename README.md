# sql-on-rocks


When I come to learn more about database, distributed system and
JIT compiler, I am always thankful to the people writing tutorials
and papers. They document how the system evolves and how the 
knowledge develops. I hope I can do that too. CockroachDB and TiDB
provide good article on how they do table-to-storage mapping.
CockroachDB mentions that MySQL did the same since decades ago,
however I failed to retrive a full picture of how MySQL does that.
MySQL is so popular and articles on tutorial, tuning, best practice
have occupy Google's search results.
I can put more effort to read the document and source code,
but I think I could just do it again, showing how this is done.

The other motivation is equally important. I want to show a path
that how I did it. During the process, I will look back to NewSQL,
to NoSQL, and to RDB. I have read a bunch of papers and most of
them just left some impression to me. I will write code,
debate design, and the paper will soon be more obvious to me.
Therefore I will carefully document why I do this design instead of another.
This is more helpful than a completed codebase.

This project is not meant to be a complete SQL engine on RocksDB.
Databases are ecosystem: you need genius, lucks and capitals to make a newcommer
a new ecosystem. I won't do that.
