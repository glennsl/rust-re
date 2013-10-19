pub enum TestResult {
    Match,
    NoMatch,
    ParseError
}

// Tuple parameters:
//      1: pattern
//      2: input
//      3: expected result
//      4: expected match string
//      5: list of expected captures
pub static TestCases: &'static [(&'static str, &'static str, TestResult, &'static str, &'static [&'static str])] = &'static [

    // Character classes
    ("[^^]+", "abc", Match, "abc", &'static []),
    ("[^^]+", "^", NoMatch, "", &'static []),
    ("[^al-obc]+", "kpd", Match, "kpd", &'static []),
    ("[^al-obc]+", "abc", NoMatch, "", &'static []),
    ("[al-obc]+", "almocb", Match, "almocb", &'static []),
    ("[al-obc]+", "defzx", NoMatch, "", &'static []),

    // Real-life example scenarios
    // From http://www.regular-expressions.info/examples.html
    // Grabbing HTML Tags
    ("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three", Match, "<TAG>two</TAG>", &'static ["two"]),
    // IP Addresses
    ("\\b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\b", "127.0.0.1", Match, "127.0.0.1", &'static ["127", "0", "0", "1"]),
    ("\\b(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\b", "127.0.0.999", NoMatch, "", &'static []),
    // Floating Point Numbers
    ("^[-+]?[0-9]*\\.?[0-9]+$", "3.14", Match, "3.14", &'static []),
    ("^[-+]?[0-9]*\\.?[0-9]+([eE][-+]?[0-9]+)?$", "1.602e-19", Match, "1.602e-19", &'static ["e-19"]),
    // E-mail Addresses
    ("\\b[a-zA-Z0-9._%+-]+@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,4}\\b", "john@server.department.company.com", Match, "john@server.department.company.com", &'static []),
    ("\\b[a-zA-Z0-9._%+-]+@(?:[a-zA-Z0-9-]+\\.)+[a-zA-Z]{2,4}\\b", "john@aol...com", NoMatch, "", &'static []),
    ("[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?", "john@server.department.company.com", Match, "john@server.department.company.com", &'static []),
    ("[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?", "john@aol...com", NoMatch, "", &'static []),
    // Date Validation
    ("^((?:19|20)\\d\\d)[- /.](0[1-9]|1[012])[- /.](0[1-9]|[12][0-9]|3[01])$", "1999-01-01", Match, "1999-01-01", &'static ["1999", "01", "01"]),
    ("^((?:19|20)\\d\\d)[- /.](0[1-9]|1[012])[- /.](0[1-9]|[12][0-9]|3[01])$", "1999/01-01", Match, "1999/01-01", &'static ["1999", "01", "01"]),
    ("^((?:19|20)\\d\\d)[- /.](0[1-9]|1[012])[- /.](0[1-9]|[12][0-9]|3[01])$", "1999-13-33", NoMatch, "", &'static []),
    //("^((?:19|20)\\d\\d)([- /.])(0[1-9]|1[012])\2(0[1-9]|[12][0-9]|3[01])$", "199-01-01", Match, "1999-01-01", &'static ["1999", "-", "01", "01"]), NOT IMPLEMENTED
    //("^((?:19|20)\\d\\d)([- /.])(0[1-9]|1[012])\2(0[1-9]|[12][0-9]|3[01])$", "199/01/01", Match, "1999/01/01", &'static ["1999", "/", "01", "01"]), NOT IMPLEMENTED
    //("^((?:19|20)\\d\\d)([- /.])(0[1-9]|1[012])\2(0[1-9]|[12][0-9]|3[01])$", "199/01-01", NoMatch, "", &'static []), NOT IMPLEMENTED
    // Mathing lines (not) containing certain words
    //^(?=.*?\bmust-have\b)(?=.*?\bmandatory\b)((?!avoid|illegal).)*$.
    // Near operator emulation
    ("\\bword1\\W+(?:\\w+\\W+){1,6}?word2\\b", "word1 word2", NoMatch, "", &'static []),
    ("\\bword1\\W+(?:\\w+\\W+){1,6}?word2\\b", "word1 1 word2", Match, "word1 1 word2", &'static []),
    ("\\bword1\\W+(?:\\w+\\W+){1,6}?word2\\b", "word1 1 2 3 4 5 6 word2", Match, "word1 1 2 3 4 5 6 word2", &'static []),
    ("\\bword1\\W+(?:\\w+\\W+){1,6}?word2\\b", "word1 1 2 3 4 5 6 7 word", NoMatch, "", &'static []),
    //("", "", Match, "", &'static []),

    // Unicode
    //("①②③", "①②③", Match, "①②③", &'static []),
    //("①②③", "①②③④⑤", Match, "①②③", &'static []),
    //("①(②)③", "①②③", Match, "①②③", &'static ["②"]),
    //("[①②③]*", "①②③", Match, "①②③", &'static []),
    //("[^④⑤]*", "①②③", Match, "①②③", &'static []),

    // Infinite loop issues
    // A**
    // (a{1, 2}?)
    // c{1,2}+e

    /* NOT IMPLEMENTED
    ('(?P<foo_123', '', ParseError),      # Unterminated group identifier
    ('(?P<1>a)', '', ParseError),         # Begins with a digit
    ('(?P<!>a)', '', ParseError),         # Begins with an illegal char
    ('(?P<foo!>a)', '', ParseError),      # Begins with an illegal char

    # Same tests, for the ?P= form
    ('(?P<foo_123>a)(?P=foo_123', 'aa', ParseError),
    ('(?P<foo_123>a)(?P=1)', 'aa', ParseError),
    ('(?P<foo_123>a)(?P=!)', 'aa', ParseError),
    ('(?P<foo_123>a)(?P=foo_124', 'aa', ParseError),  # Backref to undefined group

    ('(?P<foo_123>a)', 'a', SUCCEED, 'g1', 'a'),
    ('(?P<foo_123>a)(?P=foo_123)', 'aa', SUCCEED, 'g1', 'a'),
    */

    // Test octal escapes
    //("\\1", 'a', ParseError),    // Backreference FAILS
    //("[\\1]", "\\1", Match, "\\1", &'static []), FAILS
    //("\\09", "\x09", Match, "", &'static []), FAILS

    // From http://www.ecma-international.org/ecma-262/5.1/#sec-15.10.2.11
    // If \ is followed by a decimal number n whose first digit is not 0, 
    // then the escape sequence is considered to be a backreference. It is an 
    // error if n is greater than the total number of left capturing parentheses 
    // in the entire regular expression. \0 represents the <NUL> character and 
    // cannot be followed by a decimal digit.
    //('\\141', 'a', SUCCEED, 'found', 'a'),
    //("\\0141", "a", Match, "", &'static []), FAILS
    //('(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)\\119', 'abcdefghijklk9', SUCCEED, 'found+"-"+g11', 'abcdefghijklk9-k'),
    //("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)\\0119", "abcdefghijklk9", Match, "", &'static []), FAILS
    /*
    # Test \0 is handled everywhere
    (r'\0', '\0', SUCCEED, 'found', '\0'),
    (r'[\0a]', '\0', SUCCEED, 'found', '\0'),
    (r'[a\0]', '\0', SUCCEED, 'found', '\0'),
    (r'[^a\0]', '\0', FAIL),

    # Test various letter escapes
    (r'\a[\b]\f\n\r\t\v', '\a\b\f\n\r\t\v', SUCCEED, 'found', '\a\b\f\n\r\t\v'),
    (r'[\a][\b][\f][\n][\r][\t][\v]', '\a\b\f\n\r\t\v', SUCCEED, 'found', '\a\b\f\n\r\t\v'),
    # NOTE: not an error under PCRE/PRE:
    # (r'\u', '', ParseError),    # A Perl escape
    (r'\c\e\g\h\i\j\k\m\o\p\q\y\z', 'ceghijkmopqyz', SUCCEED, 'found', 'ceghijkmopqyz'),
    (r'\xff', '\377', SUCCEED, 'found', chr(255)),
    # new \x semantics
    (r'\x00ffffffffffffff', '\377', FAIL, 'found', chr(255)),
    (r'\x00f', '\017', FAIL, 'found', chr(15)),
    (r'\x00fe', '\376', FAIL, 'found', chr(254)),
    # (r'\x00ffffffffffffff', '\377', SUCCEED, 'found', chr(255)),
    # (r'\x00f', '\017', SUCCEED, 'found', chr(15)),
    # (r'\x00fe', '\376', SUCCEED, 'found', chr(254)),

    (r"^\w+=(\\[\000-\277]|[^\n\\])*", "SRC=eval.c g.c blah blah blah \\\\\n\tapes.c",
     SUCCEED, 'found', "SRC=eval.c g.c blah blah blah \\\\"),

    # Test that . only matches \n in DOTALL mode
    ('a.b', 'acb', SUCCEED, 'found', 'acb'),
    ('a.b', 'a\nb', FAIL),
    ('a.*b', 'acc\nccb', FAIL),
    ('a.{4,5}b', 'acc\nccb', FAIL),
    ('a.b', 'a\rb', SUCCEED, 'found', 'a\rb'),
    ('a.b(?s)', 'a\nb', SUCCEED, 'found', 'a\nb'),
    ('a.*(?s)b', 'acc\nccb', SUCCEED, 'found', 'acc\nccb'),
    ('(?s)a.{4,5}b', 'acc\nccb', SUCCEED, 'found', 'acc\nccb'),
    ('(?s)a.b', 'a\nb', SUCCEED, 'found', 'a\nb'),
    */

    // Python tests suite
    (")", "", ParseError, "", &'static []),
    //("", "", Match, "", &'static []), FAILS no VoidExpression (yet?)
    ("abc", "abc", Match, "abc", &'static []),
    ("abc", "xbc", NoMatch, "", &'static []),
    ("abc", "axc", NoMatch, "", &'static []),
    ("abc", "abx", NoMatch, "", &'static []),
    ("abc", "xabcy", Match, "abc", &'static []),
    ("abc", "ababc", Match, "abc", &'static []),
    ("ab*c", "abc", Match, "abc", &'static []),
    ("ab*bc", "abc", Match, "abc", &'static []),
    ("ab*bc", "abbc", Match, "abbc", &'static []),
    ("ab*bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab+bc", "abbc", Match, "abbc", &'static []),
    ("ab+bc", "abc", NoMatch, "", &'static []),
    ("ab+bc", "abq", NoMatch, "", &'static []),
    ("ab+bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab?bc", "abbc", Match, "abbc", &'static []),
    ("ab?bc", "abc", Match, "abc", &'static []),
    ("ab?bc", "abbbbc", NoMatch, "", &'static []),
    ("ab?c", "abc", Match, "abc", &'static []),
    ("^abc$", "abc", Match, "abc", &'static []),
    ("^abc$", "abcc", NoMatch, "", &'static []),
    ("^abc", "abcc", Match, "abc", &'static []),
    ("^abc$", "aabc", NoMatch, "", &'static []),
    ("abc$", "aabc", Match, "abc", &'static []),
    ("^", "abc", Match, "", &'static []),
    ("$", "abc", Match, "", &'static []),
    ("a.c", "abc", Match, "abc", &'static []),
    ("a.c", "axc", Match, "axc", &'static []),
    ("a.*c", "axyzc", Match, "axyzc", &'static []),
    ("a.*c", "axyzd", NoMatch, "", &'static []),
    ("a[bc]d", "abc", NoMatch, "", &'static []),
    ("a[bc]d", "abd", Match, "abd", &'static []),
    ("a[b-d]e", "abd", NoMatch, "", &'static []),
    ("a[b-d]e", "ace", Match, "ace", &'static []),
    ("a[b-d]", "aac", Match, "ac", &'static []),
    ("a[-b]", "a-", Match, "a-", &'static []),
    ("a[\\-b]", "a-", Match, "a-", &'static []),

    ("a[]b", "-", ParseError, "", &'static []),
    ("a[", "-", ParseError, "", &'static []),
    ("a\\", "-", ParseError, "", &'static []),
    ("abc)", "-", ParseError, "", &'static []),
    ("(abc", "-", ParseError, "", &'static []),
    ("a]", "a]", Match, "a]", &'static []),
    //("a[]]b", "a]b", Match, "", &'static []), FAILS
    //("a[]]b", "a]b", Match, "", &'static []), FAILS
    ("a[^bc]d", "aed", Match, "aed", &'static []),
    ("a[^bc]d", "abd", NoMatch, "", &'static []),
    ("a[^-b]c", "adc", Match, "adc", &'static []),
    ("a[^-b]c", "a-c", NoMatch, "", &'static []),
    //("a[^]b]c", "a]c", NoMatch, "", &'static []), FAILS
    //("a[^]b]c", "adc", Match, "", &'static []), FAILS
    ("\\ba\\b", "a-", Match, "a", &'static []),
    ("\\ba\\b", "-a", Match, "a", &'static []),
    ("\\ba\\b", "-a-", Match, "a", &'static []),
    ("\\by\\b", "xy", NoMatch, "", &'static []),
    ("\\by\\b", "yz", NoMatch, "", &'static []),
    ("\\by\\b", "xyz", NoMatch, "", &'static []),
    ("x\\b", "xyz", NoMatch, "", &'static []),
    ("x\\B", "xyz", Match, "x", &'static []),
    ("\\Bz", "xyz", Match, "z", &'static []),
    ("z\\B", "xyz", NoMatch, "", &'static []),
    ("\\Bx", "xyz", NoMatch, "", &'static []),
    ("\\Ba\\B", "a-", NoMatch, "", &'static []),
    ("\\Ba\\B", "-a", NoMatch, "", &'static []),
    ("\\Ba\\B", "-a-", NoMatch, "", &'static []),
    ("\\By\\B", "xy", NoMatch, "", &'static []),
    ("\\By\\B", "yz", NoMatch, "", &'static []),
    ("\\By\\b", "xy", Match, "y", &'static []),
    ("\\by\\B", "yz", Match, "y", &'static []),
    ("\\By\\B", "xyz", Match, "y", &'static []),
    ("ab|cd", "abc", Match, "ab", &'static []),
    ("ab|cd", "abcd", Match, "ab", &'static []),
    //("()ef", "def", Match, "ef", &'static [""]), FAILS there's no VoidExpression (yet?)
    ("$b", "b", NoMatch, "", &'static []),
    ("a\\(b", "a(b", Match, "a(b", &'static []),
    ("a\\(*b", "ab", Match, "ab", &'static []),
    ("a\\(*b", "a((b", Match, "a((b", &'static []),
    ("a\\\\b", "a\\b", Match, "a\\b", &'static []),
    ("((a))", "abc", Match, "a", &'static ["a", "a"]),
    ("(a)b(c)", "abc", Match, "abc", &'static ["a", "c"]),
    ("a+b+c", "aabbabc", Match, "abc", &'static []),
    ("(a+|b)*", "ab", Match, "ab", &'static ["b"]),
    ("(a+|b)+", "ab", Match, "ab", &'static ["b"]),
    ("(a+|b)?", "ab", Match, "a", &'static ["a"]),
    (")(", "-", ParseError, "", &'static []),
    ("[^ab]*", "cde", Match, "cde", &'static []),
    ("abc", "", NoMatch, "", &'static []),
    ("a*", "", Match, "", &'static []),
    ("a|b|c|d|e", "e", Match, "e", &'static []),
    ("(a|b|c|d|e)f", "ef", Match, "ef", &'static ["e"]),
    ("abcd*efg", "abcdefg", Match, "abcdefg", &'static []),
    ("ab*", "xabyabbbz", Match, "ab", &'static []),
    ("ab*", "xayabbbz", Match, "a", &'static []),
    ("(ab|cd)e", "abcde", Match, "cde", &'static ["cd"]),
    ("[abhgefdc]ij", "hij", Match, "hij", &'static []),
    ("^(ab|cd)e", "abcde", NoMatch, "", &'static []),
    //("(abc|)ef", "abcdef", Match, "ef", &'static [""]), FAILS no VoidExpression (yet?)
    ("(a|b)c*d", "abcd", Match, "bcd", &'static ["b"]),
    ("(ab|ab*)bc", "abc", Match, "abc", &'static ["a"]),
    ("a([bc]*)c*", "abc", Match, "abc", &'static ["bc"]),
    ("a([bc]*)(c*d)", "abcd", Match, "abcd", &'static ["bc", "d"]),
    ("a([bc]+)(c*d)", "abcd", Match, "abcd", &'static ["bc", "d"]),
    ("a([bc]*)(c+d)", "abcd", Match, "abcd", &'static ["b", "cd"]),
    ("a[bcd]*dcdcde", "adcdcde", Match, "adcdcde", &'static []),
    ("a[bcd]+dcdcde", "adcdcde", NoMatch, "", &'static []),
    ("(ab|a)b*c", "abc", Match, "abc", &'static ["ab"]),
    ("((a)(b)c)(d)", "abcd", Match, "abcd", &'static ["abc", "a", "b", "d"]),
    ("[a-zA-Z_][a-zA-Z0-9_]*", "alpha", Match, "alpha", &'static []),
    //("^a(bc+|b[eh])g|.h$", "abh", Match, "bh", &'static [""]), FAILS doesn't capture on paths not taken
    //("(bc+d$|ef*g.|h?i(j|k))", "effgz", Match, "effgz", &'static ["effgz", ""]), FAILS doesn't capture on paths not taken
    ("(bc+d$|ef*g.|h?i(j|k))", "ij", Match, "ij", &'static ["ij", "j"]),
    ("(bc+d$|ef*g.|h?i(j|k))", "effg", NoMatch, "", &'static []),
    ("(bc+d$|ef*g.|h?i(j|k))", "bcdd", NoMatch, "", &'static []),
    //("(bc+d$|ef*g.|h?i(j|k))", "reffgz", Match, "effgz", &'static ["effgz", ""]), FAILS doesn't capture on paths not taken
    ("(((((((((a)))))))))", "a", Match, "a", &'static ["a", "a", "a", "a", "a", "a", "a", "a", "a"]),
    ("multiple words of text", "uh-uh", NoMatch, "", &'static []),
    ("multiple words", "multiple words, yeah", Match, "multiple words", &'static []),
    ("(.*)c(.*)", "abcde", Match, "abcde", &'static ["ab", "de"]),
    ("\\((.*), (.*)\\)", "(a, b)", Match, "(a, b)", &'static ["a", "b"]),
    ("[k]", "ab", NoMatch, "", &'static []),
    ("a[-]?c", "ac", Match, "ac", &'static []),
    //("(abc)\\1", "abcabc", Match, "g1", "abc"), NOT IMPLEMNTED
    //("([a-c]*)\\1", "abcabc", Match, "g1", "abc"), NOT IMPLEMNTED
    ("^(.+)?B", "AB", Match, "AB", &'static ["A"]),
    //("(a+).\\1$", "aaaaa", Match, "aaaaa", &'static ["aa"]), NOT IMPLEMENTED
    //("^(a+).\\1$", "aaaa", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(abc)\\1", "abcabc", Match, "abcabc", &'static ["abc"]), NOT IMPLEMENTED
    //("([a-c]+)\\1", "abcabc", Match, "abcabc", &'static ["abc"]), NOT IMPLEMENTED
    //("(a)\\1", "aa", Match, ""aa, &'static ["a"]), NOT IMPLEMENTED
    //("(a+)\\1", "aa", Match, "aa", &'static ["a"]), NOT IMPLEMENTED
    //("(a+)+\\1", "aa", Match, "aa", &'static ["a"]), NOT IMPLEMENTED
    //("(a).+\\1", "aba", Match, "aba", &'static ["a"]), NOT IMPLEMENTED
    //("(a)ba*\\1", "aba", Match, "aba", &'static ["a"]), NOT IMPLEMENTED
    //("(aa|a)a\\1$", "aaa", Match, "aaa", &'static ["a"]), NOT IMPLEMENTED
    //("(a|aa)a\\1$", "aaa", Match, "aaa", &'static ["a"]), NOT IMPLEMENTED
    //("(a+)a\\1$", "aaa", Match, "aaa", &'static ["a"]), NOT IMPLEMENTED
    //("([abc]*)\\1", "abcabc", Match, "abcabc", &'static ["abc"]), NOT IMPLEMENTED
    //("(a)(b)c|ab", "ab", Match, "ab", &'static ["", ""]), FAILS doesn't capture on paths not taken
    ("(a)+x", "aaax", Match, "aaax", &'static ["a"]),
    ("([ac])+x", "aacx", Match, "aacx", &'static ["c"]),
    ("([^/]*/)*sub1/", "d:msgs/tdir/sub1/trial/away.cpp", Match, "d:msgs/tdir/sub1/", &'static ["tdir/"]),
    ("([^.]*)\\.([^:]*):[T ]+(.*)", "track1.title:TBlah blah blah", Match, "track1.title:TBlah blah blah", &'static ["track1", "title", "Blah blah blah"]),
    ("([^N]*N)+", "abNNxyzN", Match, "abNNxyzN", &'static ["xyzN"]),
    ("([^N]*N)+", "abNNxyz", Match, "abNN", &'static ["N"]),
    ("([abc]*)x", "abcx", Match, "abcx", &'static ["abc"]),
    ("([abc]*)x", "abc", NoMatch, "", &'static []),
    ("([xyz]*)x", "abcx", Match, "x", &'static [""]),
    //("(a)+b|aac", "aac", Match, "aac", &'static [""]), FAILS doesn't capture on paths not taken

    /* NOT IMPLEMENTED
    # Test symbolic groups

    ('(?P<i d>aaa)a', 'aaaa', ParseError),
    ('(?P<id>aaa)a', 'aaaa', SUCCEED, 'found+"-"+id', 'aaaa-aaa'),
    ('(?P<id>aa)(?P=id)', 'aaaa', SUCCEED, 'found+"-"+id', 'aaaa-aa'),
    ('(?P<id>aa)(?P=xd)', 'aaaa', ParseError),

    # Test octal escapes/memory references

    ('\\1', 'a', ParseError),
    ('\\09', chr(0) + '9', SUCCEED, 'found', chr(0) + '9'),
    ('\\141', 'a', SUCCEED, 'found', 'a'),
    ('(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)(l)\\119', 'abcdefghijklk9', SUCCEED, 'found+"-"+g11', 'abcdefghijklk9-k'),
    */

    // All tests from Perl (from the python test suite)
    ("abc", "abc", Match, "abc", &'static []),
    ("abc", "xbc", NoMatch, "", &'static []),
    ("abc", "axc", NoMatch, "", &'static []),
    ("abc", "abx", NoMatch, "", &'static []),
    ("abc", "xabcy", Match, "abc", &'static []),
    ("abc", "ababc", Match, "abc", &'static []),
    ("ab*c", "abc", Match, "abc", &'static []),
    ("ab*bc", "abc", Match, "abc", &'static []),
    ("ab*bc", "abbc", Match, "abbc", &'static []),
    ("ab*bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab{0,}bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab+bc", "abbc", Match, "abbc", &'static []),
    ("ab+bc", "abc", NoMatch, "", &'static []),
    ("ab+bc", "abq", NoMatch, "", &'static []),
    ("ab{1,}bc", "abq", NoMatch, "", &'static []),
    ("ab+bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab{1,}bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab{1,3}bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab{3,4}bc", "abbbbc", Match, "abbbbc", &'static []),
    ("ab{4,5}bc", "abbbbc", NoMatch, "abbbbc", &'static []),
    ("ab?bc", "abbc", Match, "abbc", &'static []),
    ("ab?bc", "abc", Match, "abc", &'static []),
    ("ab{0,1}bc", "abc", Match, "abc", &'static []),
    ("ab?bc", "abbbbc", NoMatch, "", &'static []),
    ("ab?c", "abc", Match, "abc", &'static []),
    ("ab{0,1}c", "abc", Match, "abc", &'static []),
    ("^abc$", "abc", Match, "abc", &'static []),
    ("^abc$", "abcc", NoMatch, "", &'static []),
    ("^abc", "abcc", Match, "abc", &'static []),
    ("^abc$", "aabc", NoMatch, "", &'static []),
    ("abc$", "aabc", Match, "abc", &'static []),
    ("^", "abc", Match, "", &'static []),
    ("$", "abc", Match, "", &'static []),
    ("a.c", "abc", Match, "abc", &'static []),
    ("a.c", "axc", Match, "axc", &'static []),
    ("a.*c", "axyzc", Match, "axyzc", &'static []),
    ("a.*c", "axyzd", NoMatch, "", &'static []),
    ("a[bc]d", "abc", NoMatch, "", &'static []),
    ("a[bc]d", "abd", Match, "abd", &'static []),
    ("a[b-d]e", "abd", NoMatch, "", &'static []),
    ("a[b-d]e", "ace", Match, "ace", &'static []),
    ("a[b-d]", "aac", Match, "ac", &'static []),
    ("a[-b]", "a-", Match, "a-", &'static []),
    ("a[b-]", "a-", Match, "a-", &'static []),
    ("a[b-a]", "-", ParseError, "", &'static []),
    ("a[]b", "-", ParseError, "", &'static []),
    ("a[", "-", ParseError, "", &'static []),
    ("a]", "a]", Match, "a]", &'static []),
    //("a[]]b", "a]b", Match, "", &'static []), FAILS
    ("a[^bc]d", "aed", Match, "aed", &'static []),
    ("a[^bc]d", "abd", NoMatch, "", &'static []),
    ("a[^-b]c", "adc", Match, "adc", &'static []),
    ("a[^-b]c", "a-c", NoMatch, "", &'static []),
    //("a[^]b]c", "a]c", NoMatch, "", &'static []), FAILS
    //("a[^]b]c", "adc", Match, "", &'static []), FAILS
    ("ab|cd", "abc", Match, "ab", &'static []),
    ("ab|cd", "abcd", Match, "ab", &'static []),
    //("()ef", "def", Match, "", &'static []), FAILS no VoidExpression (yet?)
    ("*a", "-", ParseError, "", &'static []),
    ("(*)b", "-", ParseError, "", &'static []),
    ("$b", "b", NoMatch, "", &'static []),
    ("a\\", "-", ParseError, "", &'static []),
    ("a\\(b", "a(b", Match, "a(b", &'static []),
    ("a\\(*b", "ab", Match, "ab", &'static []),
    ("a\\(*b", "a((b", Match, "a((b", &'static []),
    ("a\\\\b", "a\\b", Match, "a\\b", &'static []),
    ("abc)", "-", ParseError, "", &'static []),
    ("(abc", "-", ParseError, "", &'static []),
    ("((a))", "abc", Match, "a", &'static ["a", "a"]),
    ("(a)b(c)", "abc", Match, "abc", &'static ["a", "c"]),
    ("a+b+c", "aabbabc", Match, "abc", &'static []),
    ("a{1,}b{1,}c", "aabbabc", Match, "abc", &'static []),
    ("a**", "-", ParseError, "", &'static []), // Would cause an infinite loop if accepted
    ("a.+?c", "abcabc", Match, "abc", &'static []),
    ("(a+|b)*", "ab", Match, "ab", &'static ["b"]),
    ("(a+|b){0,}", "ab", Match, "ab", &'static ["b"]),
    ("(a+|b)+", "ab", Match, "ab", &'static ["b"]),
    ("(a+|b){1,}", "ab", Match, "ab", &'static ["b"]),
    ("(a+|b)?", "ab", Match, "a", &'static ["a"]),
    ("(a+|b){0,1}", "ab", Match, "a", &'static ["a"]),
    (")(", "-", ParseError, "", &'static []),
    ("[^ab]*", "cde", Match, "cde", &'static []),
    ("abc", "", NoMatch, "", &'static []),
    ("a*", "", Match, "", &'static []),
    ("([abc])*d", "abbbcd", Match, "abbbcd", &'static ["c"]),
    ("([abc])*bcd", "abcd", Match, "abcd", &'static ["a"]),
    ("a|b|c|d|e", "e", Match, "e", &'static []),
    ("(a|b|c|d|e)f", "ef", Match, "ef", &'static ["e"]),
    ("abcd*efg", "abcdefg", Match, "abcdefg", &'static []),
    ("ab*", "xabyabbbz", Match, "ab", &'static []),
    ("ab*", "xayabbbz", Match, "a", &'static []),
    ("(ab|cd)e", "abcde", Match, "cde", &'static ["cd"]),
    ("[abhgefdc]ij", "hij", Match, "hij", &'static []),
    ("^(ab|cd)e", "abcde", NoMatch, "", &'static []),
    //("(abc|)ef", "abcdef", Match, "", &'static []), FAILS no VoidExpression (yet?)
    ("(a|b)c*d", "abcd", Match, "bcd", &'static ["b"]),
    ("(ab|ab*)bc", "abc", Match, "abc", &'static ["a"]),
    ("a([bc]*)c*", "abc", Match, "abc", &'static ["bc"]),
    ("a([bc]*)(c*d)", "abcd", Match, "abcd", &'static ["bc", "d"]),
    ("a([bc]+)(c*d)", "abcd", Match, "abcd", &'static ["bc", "d"]),
    ("a([bc]*)(c+d)", "abcd", Match, "abcd", &'static ["b", "cd"]),
    ("a[bcd]*dcdcde", "adcdcde", Match, "adcdcde", &'static []),
    ("a[bcd]+dcdcde", "adcdcde", NoMatch, "", &'static []),
    ("(ab|a)b*c", "abc", Match, "abc", &'static ["ab"]),
    ("((a)(b)c)(d)", "abcd", Match, "abcd", &'static ["abc", "a", "b", "d"]),
    ("[a-zA-Z_][a-zA-Z0-9_]*", "alpha", Match, "alpha", &'static []),
    //("^a(bc+|b[eh])g|.h$", "abh", Match, "bh", &'static [""]), FAILS doesn't capture on paths not taken
    //("(bc+d$|ef*g.|h?i(j|k))", "effgz", Match, "effgz", &'static ["effgz", ""]), FAILS doesn't capture on paths not taken
    ("(bc+d$|ef*g.|h?i(j|k))", "ij", Match, "ij", &'static ["ij", "j"]),
    ("(bc+d$|ef*g.|h?i(j|k))", "effg", NoMatch, "", &'static []),
    ("(bc+d$|ef*g.|h?i(j|k))", "bcdd", NoMatch, "", &'static []),
    //("(bc+d$|ef*g.|h?i(j|k))", "reffgz", Match, "effgz", &'static ["effgz", ""]), FAILS doesn't capture on paths not taken
    //("((((((((((a))))))))))", "a", Match, "a", &'static ["a", "a", "a", "a", "a", "a", "a", "a", "a", "a"]), FAILS only support for 9 captures
    //("((((((((((a))))))))))\\10", "aa", Match, "", &'static []), NOT IMPLEMENTED
// Python does not have the same rules for \\41 so this is a syntax error
//      ("((((((((((a))))))))))\\41", "aa", NoMatch, "", &'static []),
//      ("((((((((((a))))))))))\\41", "a!", Match, "", &'static []),
    //("((((((((((a))))))))))\\41", "", ParseError, "", &'static []), FAILS
    //("(?i)((((((((((a))))))))))\\41", "", ParseError), NOT IMPLEMENTED
    ("(((((((((a)))))))))", "a", Match, "a", &'static ["a", "a", "a", "a", "a", "a", "a", "a", "a"]),
    ("multiple words of text", "uh-uh", NoMatch, "", &'static []),
    ("multiple words", "multiple words, yeah", Match, "multiple words", &'static []),
    ("(.*)c(.*)", "abcde", Match, "abcde", &'static ["ab", "de"]),
    ("\\((.*), (.*)\\)", "(a, b)", Match, "(a, b)", &'static ["a", "b"]),
    ("[k]", "ab", NoMatch, "", &'static []),
    ("a[-]?c", "ac", Match, "ac", &'static []),
    //("(abc)\\1", "abcabc", Match, "g1", "abc"), NOT IMPLEMENTED
    //("([a-c]*)\\1", "abcabc", Match, "g1", "abc"), NOT IMPLEMENTED
    //("(?i)abc", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc", "XBC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc", "AXC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc", "ABX", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc", "XABCY", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc", "ABABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab*c", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab*bc", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab*bc", "ABBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab*?bc", "ABBBBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{0,}?bc", "ABBBBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab+?bc", "ABBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab+bc", "ABC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab+bc", "ABQ", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{1,}bc", "ABQ", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab+bc", "ABBBBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{1,}?bc", "ABBBBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{1,3}?bc", "ABBBBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{3,4}?bc", "ABBBBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{4,5}?bc", "ABBBBC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab??bc", "ABBC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab??bc", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{0,1}?bc", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab??bc", "ABBBBC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab??c", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab{0,1}?c", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)^abc$", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)^abc$", "ABCC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)^abc", "ABCC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)^abc$", "AABC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc$", "AABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)^", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)$", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a.c", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a.c", "AXC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a.*?c", "AXYZC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a.*c", "AXYZD", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[bc]d", "ABC", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[bc]d", "ABD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[b-d]e", "ABD", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[b-d]e", "ACE", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[b-d]", "AAC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[-b]", "A-", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[b-]", "A-", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[b-a]", "-", ParseError),
    //("(?i)a[]b", "-", ParseError),
    //("(?i)a[", "-", ParseError),
    //("(?i)a]", "A]", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[]]b", "A]B", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[^bc]d", "AED", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[^bc]d", "ABD", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[^-b]c", "ADC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[^-b]c", "A-C", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[^]b]c", "A]C", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[^]b]c", "ADC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab|cd", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab|cd", "ABCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)()ef", "DEF", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)*a", "-", ParseError),
    //("(?i)(*)b", "-", ParseError),
    //("(?i)$b", "B", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a\\", "-", ParseError),
    //("(?i)a\\(b", "A(B", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a\\(*b", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a\\(*b", "A((B", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a\\\\b", "A\\B", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc)", "-", ParseError),
    //("(?i)(abc", "-", ParseError),
    //("(?i)((a))", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a)b(c)", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a+b+c", "AABBABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a{1,}b{1,}c", "AABBABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a**", "-", ParseError),
    //("(?i)a.+?c", "ABCABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a.*?c", "ABCABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a.{0,5}?c", "ABCABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b)*", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b){0,}", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b)+", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b){1,}", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b)?", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b){0,1}", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a+|b){0,1}?", "AB", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i))(", "-", ParseError),
    //("(?i)[^ab]*", "CDE", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)abc", "", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)a*", "", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)([abc])*d", "ABBBCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)([abc])*bcd", "ABCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a|b|c|d|e", "E", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a|b|c|d|e)f", "EF", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)abcd*efg", "ABCDEFG", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab*", "XABYABBBZ", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)ab*", "XAYABBBZ", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(ab|cd)e", "ABCDE", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)[abhgefdc]ij", "HIJ", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)^(ab|cd)e", "ABCDE", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)(abc|)ef", "ABCDEF", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(a|b)c*d", "ABCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(ab|ab*)bc", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a([bc]*)c*", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a([bc]*)(c*d)", "ABCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a([bc]+)(c*d)", "ABCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a([bc]*)(c+d)", "ABCD", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[bcd]*dcdcde", "ADCDCDE", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)a[bcd]+dcdcde", "ADCDCDE", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)(ab|a)b*c", "ABC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)((a)(b)c)(d)", "ABCD", Match, "g1+"-"+g2+"-"+g3+"-"+g4", "ABC-A-B-D"), NOT IMPLEMENTED
    //("(?i)[a-zA-Z_][a-zA-Z0-9_]*", "ALPHA", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)^a(bc+|b[eh])g|.h$", "ABH", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(bc+d$|ef*g.|h?i(j|k))", "EFFGZ", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(bc+d$|ef*g.|h?i(j|k))", "IJ", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(bc+d$|ef*g.|h?i(j|k))", "EFFG", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)(bc+d$|ef*g.|h?i(j|k))", "BCDD", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)(bc+d$|ef*g.|h?i(j|k))", "REFFGZ", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)((((((((((a))))))))))", "A", Match, "g10", "A"), NOT IMPLEMENTED
    //("(?i)((((((((((a))))))))))\\10", "AA", Match, "", &'static []), NOT IMPLEMENTED
    //#("(?i)((((((((((a))))))))))\\41", "AA", NoMatch, "", &'static []), NOT IMPLEMENTED
    //#("(?i)((((((((((a))))))))))\\41", "A!", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(((((((((a)))))))))", "A", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(?:(?:(?:(?:(?:(?:(?:(?:(?:(a))))))))))", "A", Match, "g1", "A"), NOT IMPLEMENTED
    //("(?i)(?:(?:(?:(?:(?:(?:(?:(?:(?:(a|b|c))))))))))", "C", Match, "g1", "C"), NOT IMPLEMENTED
    //("(?i)multiple words of text", "UH-UH", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?i)multiple words", "MULTIPLE WORDS, YEAH", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(.*)c(.*)", "ABCDE", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)\\((.*), (.*)\\)", "(A, B)", Match, "g2+"-"+g1", "B-A"), NOT IMPLEMENTED
    //("(?i)[k]", "AB", NoMatch, "", &'static []), NOT IMPLEMENTED
//#    ("(?i)abcd", "ABCD", Match, "", &'static []),
//#    ("(?i)a(bc)d", "ABCD", Match, "g1+"-"+\\g1+"-"+\\\\g1", "BC-$1-\\BC"),
    //("(?i)a[-]?c", "AC", Match, "", &'static []), NOT IMPLEMENTED
    //("(?i)(abc)\\1", "ABCABC", Match, "g1", "ABC"), NOT IMPLEMENTED
    //("(?i)([a-c]*)\\1", "ABCABC", Match, "g1", "ABC"), NOT IMPLEMENTED
    //("a(?!b).", "abad", Match, "", &'static []), NOT IMPLEMENTED
    //("a(?=d).", "abad", Match, "", &'static []), NOT IMPLEMENTED
    //("a(?=c|d).", "abad", Match, "", &'static []), NOT IMPLEMENTED
    ("a(?:b|c|d)(.)", "ace", Match, "ace", &'static ["e"]),
    ("a(?:b|c|d)*(.)", "ace", Match, "ace", &'static ["e"]),
    ("a(?:b|c|d)+?(.)", "ace", Match, "ace", &'static ["e"]),
    //("a(?:b|(c|e){1,2}?|d)+?(.)", "ace", Match, "ace", &'static ["c", "e"]), FAILS infinite loop
    ("^(.+)?B", "AB", Match, "AB", &'static ["A"]),

    // lookbehind: split by : but not if it is escaped by -.
    //("(?<!-):(.*?)(?<!-):", "a:bc-:de:f", Match, "g1", "bc-:de" ), NOT IMPLEMENTED
    // escaping with \ as we know it
    //("(?<!\\\):(.*?)(?<!\\\):", "a:bc\\:de:f", Match, "g1", "bc\\:de" ), NOT IMPLEMENTED
    // terminating with " and escaping with ? as in edifact
    //("(?<!\\?)"(.*?)(?<!\\?)"", "a"bc?"de"f", Match, "g1", "bc?"de" ), NOT IMPLEMENTED

    // Comments using the (?#...) syntax

    //("w(?# comment", "w", ParseError),
    //("w(?# comment 1)xy(?# comment 2)z", "wxyz", Match, "", &'static []), NOT IMPLEMENTED

    // Check odd placement of embedded pattern modifiers

    // not an error under PCRE/PRE:
    //("w(?i)", "W", Match, "", &'static []), NOT IMPLEMENTED
    //# ("w(?i)", "W", ParseError),

    // Comments using the x embedded pattern modifier
/*
    ("""(?x)w# comment 1
        x y
        # comment 2
        z""", "wxyz", Match, "", &'static []),

    // using the m embedded pattern modifier

    ("^abc", """jkl
abc
xyz""", NoMatch, "", &'static []),
    ("(?m)^abc", """jkl
abc
xyz""", Match, "", &'static []),

    ("(?m)abc$", """jkl
xyzabc
123""", Match, "", &'static []),
*/
    // using the s embedded pattern modifier

    //("a.b", "a\nb", NoMatch, "", &'static []), NOT IMPLEMENTED
    //("(?s)a.b", "a\nb", Match, "", &'static []), NOT IMPLEMENTED

    // test \w, etc. both inside and outside character classes

    ("\\w+", "--ab_cd0123--", Match, "ab_cd0123", &'static []),
    ("[\\w]+", "--ab_cd0123--", Match, "ab_cd0123", &'static []),
    ("\\D+", "1234abc5678", Match, "abc", &'static []),
    ("[\\D]+", "1234abc5678", Match, "abc", &'static []),
    ("[\\da-fA-F]+", "123abc", Match, "123abc", &'static []),
    // not an error under PCRE/PRE:
    // ("[\\d-x]", "-", ParseError),
    //(r"([\s]*)([\S]*)([\s]*)", " testing!1972", Match, " testing!1972", &'static [" ", "testing!1972", ""]), FAILS
    //(r"(\s*)(\S*)(\s*)", " testing!1972", Match, " testing!1972", &'static [" ", "testing!1972", ""]), FAILS

    //(r"\xff", "\377", Match, "\xff", &'static []),
    // new \x semantics
    //(r"\x00ff", "\377", NoMatch, "", &'static []),
    //(r"\x00ff", "\377", Match, "\x00ff", &'static []),
    //(r"\t\n\v\r\f\a\g", "\t\n\v\r\f\ag", Match, "\t\n\v\r\f\ag", &'static []),
    //("\t\n\v\r\f\a\g", "\t\n\v\r\f\ag", Match, "\t\n\v\r\f\ag", &'static []),
    //(r"\t\n\v\r\f\a", "\t\n\v\r\f\a", Match, "", &'static []),
    //(r"[\t][\n][\v][\r][\f][\b]", "\t\n\v\r\f\b", Match, "\t\n\v\r\f\b", &'static []),

    // post-1.5.2 additions

    // xmllib problem
    //(r"(([a-z]+):)?([a-z]+)$", "smil", Match, "smil", &'static ["None", "None", "smil"]), FAILS doesn't capture on paths not taken
    // bug 110866: reference to undefined group
    (r"((.)\1+)", "", ParseError, "", &'static []),
    // bug 111869: search (PRE/PCRE NoMatchs on this one, SRE doesn"t)
    //(r".*d", "abc\nabd", Match, "abd", &'static []), FAILS . matches newlines
    // bug 112468: various expected syntax errors
    (r"(", "", ParseError, "", &'static []),
    //(r"[\41]", "!", Match, "", &'static []),
    // bug 114033: nothing to repeat
    (r"(x?)?", "x", Match, "x", &'static ["x"]),
    // bug 115040: rescan if flags are modified inside pattern
    //(r" (?x)foo ", "foo", Match, "", &'static []), NOT IMPLEMENTED
    // bug 115618: negative lookahead
    //(r"(?<!abc)(d.f)", "abcdefdof", Match, "", &'static []), NOT IMPLEMENTED
    // bug 116251: character class bug
    //(r"[\w-]+", "laser_beam", Match, "laser_beam", &'static []), FAILS not sure why it shouldn't?
    // bug 123769+127259: non-greedy backtracking bug
    (r".*?\S *:", "xx:", Match, "xx:", &'static []),
    (r"a[ ]*?\ (\d+).*", "a   10", Match, "a   10", &'static ["10"]),
    (r"a[ ]*?\ (\d+).*", "a    10", Match, "a    10", &'static ["10"]),
    // bug 127259: \Z shouldn"t depend on multiline mode
    //(r"(?ms).*?x\s*\Z(.*)","xx\nx\n", Match, "g1", ""), NOT IMPLEMENTED
    // bug 128899: uppercase literals under the ignorecase flag
    //(r"(?i)M+", "MMM", Match, "", &'static []), NOT IMPLEMENTED
    //(r"(?i)m+", "MMM", Match, "", &'static []), NOT IMPLEMENTED
    //(r"(?i)[M]+", "MMM", Match, "", &'static []), NOT IMPLEMENTED
    //(r"(?i)[m]+", "MMM", Match, "", &'static []), NOT IMPLEMENTED
    // bug 130748: ^* should be an error (nothing to repeat)
    //(r"^*", "", ParseError, "", &'static []), FAILS infinite loop
    // bug 133283: minimizing repeat problem
    //(r###"(?:"|[^"])*?"###, r###""\""###, Match, r###""\""###, &'static []), FAILS not sure what the original raw string should translate to
    // bug 477728: minimizing repeat problem
    //(r"^.*?$", "one\ntwo\nthree\n", NoMatch, "", &'static []), FAILS . matches newline
    // bug 483789: minimizing repeat problem
    (r"a[^>]*?b", "a>b", NoMatch, "", &'static []),
    // bug 490573: minimizing repeat problem
    (r"^a*?$", "foo", NoMatch, "", &'static []),
    // bug 470582: nested groups problem
    //(r"^((a)c)?(ab)$", "ab", Match, "ab", &'static ["", "", "ab"]), FAILS doesn't capture on paths not taken
    // another minimizing repeat problem (capturing groups in assertions)
    //("^([ab]*?)(?=(b)?)c", "abc", Match, "g1+"-"+g2", "ab-None"), NOT IMPLEMENTED
    //("^([ab]*?)(?!(b))c", "abc", Match, "g1+"-"+g2", "ab-None"), NOT IMPLEMENTED
    //("^([ab]*?)(?<!(a))c", "abc", Match, "g1+"-"+g2", "ab-None"), NOT IMPLEMENTED
];