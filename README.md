rust-re
=======

A regular expression engine written entirely in Rust. It is based on Pike VM (http://swtch.com/~rsc/regexp/regexp2.html), the only significant difference being that it prioritizes threads in the main loop instead of using recursion.

The goal is to at least implement ECMA-262 natively in Rust, and be able to perform reliably and well for all accepted input. It is NOT a goal to be the fastest or the most fully-featured regex implementation. It should be good and maintainable enough to be included in Rust's standard library, but for very special needs, you should use a very special library.

When run, it will currently output the parse tree and compiled code, as well as the result of the regex run on the provided input. There is also an extensive test suite, borrowed mostly from the python regex implementation (http://hg.python.org/cpython/file/178075fbff3a/Lib/test/re_tests.py).

Currently implemented features
------------------------------

* Character classes (e.g. [a-z])
* Negated character clsees (e.g. [^a-z])
* Predefined character classes (., \d, \D, \w, \W, \s and \S)
* Escaping
* Assertions (^, $, \b and \B)
* Capturing groups (e.g. (abc))
* Non-capturing groups ((?:))
* Alternation (e.g. a|b)
* Greedy quantifiers (?, *, +)
* Arbitrary repetitions (e.g. {2}, {2,} and {2, 3})
* Non.greeedy quantifiers (??, *?, +? and {}?)

To be implemented
-----------------

* Backreferences (\1 to \9)
* Positive and negative lookahead ((?=) and (?!))
* Infinite loop resolution
* Options
    * Ignore case
    * Multiline
    * . not matching newline