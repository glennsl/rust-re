extern mod extra;

//use std::str;
use std::os;
use std::util;
use std::vec;

use compile::Instruction;

mod parse;
mod compile;
mod matcher;
mod pike;
mod debug;

pub struct Regex {
    priv code: ~[Instruction]
}

impl Regex {
    fn new(pattern: &str) -> ~Regex {
        let etree = parse::parse(pattern);
        let code = compile::compile(&etree);
        return ~Regex { code: code };
    }
    fn partial_match(&self, input: &str) -> Option<~[matcher::Match]> {
        pike::PikeMatcher::do_match(self.code, input)
    }
}

fn main()  {
    match os::args() {
        [_, pattern, input] => {

            let etree = parse::parse(pattern);
            println("\nExpression Tree");
            println("-----------------");
            debug::print_expression_tree(&etree);

            let code = compile::compile(&etree);
            println("\nCode");
            println("------");
            debug::print_code(code);

            let re = Regex::new(pattern);
            match re.partial_match(input) {
                Some(matches) => {
                    println("\nYay!");
                    for (i, m) in matches.iter().enumerate() {
                        println!("  {}: {} ({}, {})", i, input.slice_chars(m.start, m.end), m.start, m.end);
                    }
                    println("");
                }
                None => println("\nAww...\n")
            }
        }
        _ => //println("\nUsage: re <pattern> <input>\n")
        println!("{}", os::args()[1])
    }
}

#[cfg(test)]
mod test {
    use std::task;

    use super::Regex;
    // Apparently this can't be done... ?
    //use tests::{TestResult, NoMatch, ParseError, TestCases};

    #[path="../tests.rs"]
    mod tests;

    #[test]
    fn test_all_the_things() {
        let mut errors = 0u;
        for &(pattern, input, result, expected_match, expected_captures) in tests::TestCases.iter() {
            let task_result = do task::try {
                let re = Regex::new(pattern);
                match re.partial_match(input) {
                    Some(matches) => {
                        let m = matches.head();
                        let captures = matches.tail();
                        let actual_match = input.slice_chars(m.start, m.end);

                        match result {
                            tests::Match => {
                                let mut match_error = false;
                                if  actual_match != expected_match {
                                    println!("Expected match \"{}\". Got \"{}\"", expected_match, actual_match);
                                    match_error = true;
                                }

                                if captures.len() != expected_captures.len() {
                                    println!("Expected {} captures. Got {}", expected_captures.len(), captures.len())
                                    match_error = true;
                                } else {
                                    for (i, m) in captures.iter().enumerate() {
                                        let capture = input.slice_chars(m.start, m.end).to_owned();
                                        if capture != expected_captures[i].to_owned() {
                                            println!("Expected capture {} to be \"{}\". Got \"{}\"", i, expected_captures[i], capture);
                                            match_error = true;
                                        }
                                    }
                                }
                                if match_error {
                                    fail!("Match failed.");
                                }
                            }
                            tests::NoMatch => fail!("Expected NO match. Got match: " + actual_match),
                            tests::ParseError => fail!("Expected parse error. Got match.")
                        }
                    }
                    None => match result {
                        tests::Match => fail!("Expected match. Got nothing."),
                        tests::NoMatch => (),
                        tests::ParseError => fail!("Expected parse error. Got no match.")
                    }
                }
            };
            match task_result {
                Ok(_) => (),
                Err(_) => match result {
                    tests::Match |
                    tests::NoMatch => {
                        println("\tFAIL: Test pattern \"" + pattern + "\" on \"" + input + "\" failed.\n");
                        errors += 1;
                    }
                    tests::ParseError => {
                        // Currently unable to distinguish between expected parse errors and other errors.
                        //println("\tTest pattern \"" + pattern + "\" on \"" + input + "\" failed. Expected parse error.\n");  
                    }
                }
            }
        }
        if errors > 0 {
            fail!(errors.to_str() + " tests FAILED. " + (tests::TestCases.len() - errors).to_str() + " passed.")
        }

        println!("{} tests PASSED", tests::TestCases.len());
    }
}

#[cfg(test)]
mod bench {
    extern mod extra;
/*
    use std::path::Path;
    use std::io;

    use extra::time;
    use extra::test;
*/
    use super::Regex;

    #[bench]
    fn bench_compile_uri(b: &mut extra::test::BenchHarness) {
        let pattern = "([a-zA-Z][a-zA-Z0-9]*)://([^ /]+)(/[^ ]*)?";
        do b.iter {
            Regex::new(pattern);
        }
    }

    #[bench]
    fn bench_compile_email(b: &mut extra::test::BenchHarness) {
        let pattern = "([^ @]+)@([^ @]+)";
        do b.iter {
            Regex::new(pattern);
        }
    }

    #[bench]
    fn bench_compile_date(b: &mut extra::test::BenchHarness) {
        let pattern = "([0-9][0-9]?)/([0-9][0-9]?)/([0-9][0-9]([0-9][0-9])?)";
        do b.iter {
            Regex::new(pattern);
        }
    }

    #[bench]
    fn bench_compile_uri_or_email(b: &mut extra::test::BenchHarness) {
        let pattern = "([a-zA-Z][a-zA-Z0-9]*)://([^ /]+)(/[^ ]*)?|([^ @]+)@([^ @]+)";
        do b.iter {
            Regex::new(pattern);
        }
    }

    #[bench]
    fn bench_match(b: &mut extra::test::BenchHarness) {
        //let pattern = "[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*@(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?";
        //let input = "john@server.department.company.com";
        //let pattern = "^[-+]?[0-9]*\\.?[0-9]+$";
        //let input = "3.14";
        let pattern = "a(b|c)*d";
        let input = "abcd";
        let re = Regex::new(pattern);

        do b.iter {
            re.partial_match(input);
        }
    }
/*
    #[bench]
    fn bench_match_uri(b: &mut extra::test::BenchHarness) {
        bench("([a-zA-Z][a-zA-Z0-9]*)://([^ /]+)(/[^ ]*)?");
        b.iterations = 1;
        do b.iter {}
    }

    #[bench]
    fn bench_match_email(b: &mut extra::test::BenchHarness) {
        bench("([^ @]+)@([^ @]+)");
        b.iterations = 1;
        do b.iter {}
    }

    #[bench]
    fn bench_match_date(b: &mut extra::test::BenchHarness) {
        bench("([0-9][0-9]?)/([0-9][0-9]?)/([0-9][0-9]([0-9][0-9])?)");
        b.iterations = 1;
        do b.iter {}
    }

    #[bench]
    fn bench_match_uri_or_email(b: &mut extra::test::BenchHarness) {
        bench("([a-zA-Z][a-zA-Z0-9]*)://([^ /]+)(/[^ ]*)?|([^ @]+)@([^ @]+)");
        b.iterations = 1;
        do b.iter {}
    }

    fn bench(pattern: &str) {
        let input = read_input();
        let re = Regex::new(pattern);

        let start = time::precise_time_s();
        for line in input.iter() {
            re.partial_match(*line);
        }
        let end = time::precise_time_s();

        println!("\nActual {} s", start - end);
    }

    fn read_input() -> ~[~str] {
        let file = "bench-input.txt";
        io::file_reader(&Path(file)).unwrap().read_lines()
    }
*/
}