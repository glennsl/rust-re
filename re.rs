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
/*
    fn partial_match(&self, input: &str) -> Option<~[Match]> {
        let mut threads = vec::with_capacity(self.code.len()); // clist
        let mut next_threads = vec::with_capacity(self.code.len()); // nlist
        let mut matched = None;

        let mut thread_id = 0;
        let add_thread = |l: &mut ~[~Thread], pc, sp, captures: ~[Option<Match>], registers: ~[uint]| {
            let t = ~Thread {
                pc: pc,
                id: thread_id,
                match_start: sp,
                captures: captures,
                registers: registers
            };
            (*l).push(t);
            thread_id += 1;
        };

        for (sp, c) in input.iter().chain("\x03".iter()).enumerate() {
            debug!("Input {}", c);

            if matched.is_none() {
                debug!("\tAdd thread {}: {}", thread_id, sp);

                add_thread(&mut next_threads, 0, sp, ~[None, ..10], vec::from_elem(self.registers, 0u));
            }

            util::swap(&mut threads, &mut next_threads);
            threads.reverse();
            next_threads.clear();

            'threads: loop {
                let mut thread = match threads.pop_opt() {
                    Some(thread) => thread,
                    None => break
                };

                'thread: loop {
                    debug!("\tThread {}", thread.id);
                    
                    match self.code[thread.pc] {
                        Char(ch) => {
                            debug!("\t\tChar({}): ", ch);
                            
                            if c == ch {
                                debug!("\t\t\tMatch");
                                
                                thread.pc += 1;
                                next_threads.push(thread);
                            } else {
                                debug!("\t\t\tFail");
                            }
                            break;
                        }
                        Any => {
                            debug!("\t\tAny");
                            
                            thread.pc += 1;
                            next_threads.push(thread);
                            break;
                        }
                        Range(start, end) => {
                            debug!("\t\tRange({}, {}) <{}, {}>", start as u8, end as u8, start, end);
                            
                            if c >= start && c <= end {
                                thread.pc += 1;
                                next_threads.push(thread);
                            }
                            break;
                        }
                        Fork(pc1, pc2) => {
                            debug!("\t\tFork({}, {}): add thread {}: {}", pc1, pc2, thread_id, thread.match_start);
                            
                            add_thread(&mut threads, pc2, thread.match_start, thread.captures.clone(), thread.registers.clone());
                            thread.pc = pc1;
                        }
                        Jump(new_pc) => {
                            debug!("\t\tJump({})", new_pc);
                            
                            thread.pc = new_pc;
                        }
                        ConditionalJumpEq(register, value, new_pc) => {
                            debug!("\t\tConditionalJump({} == {}): ", thread.registers[register], value);

                            if thread.registers[register] == value {
                                debug!("\t\t\tJump({})", new_pc);

                                thread.pc = new_pc;
                            } else {
                                debug!("\t\t\tFail");

                                thread.pc += 1;
                            }
                        }
                        ConditionalJumpLE(register, value, new_pc) => {
                            debug!("\t\tConditionalJump({} <= {}): ", thread.registers[register], value);

                            if thread.registers[register] < value {
                                debug!("\t\t\tJump({})", new_pc);

                                thread.pc = new_pc;
                            } else {
                                debug!("\t\t\tFail");

                                thread.pc += 1;
                            }
                        }
                        Increment(register) => {
                            debug!("\t\tIncrement({})", register);

                            thread.registers[register] += 1;
                            thread.pc += 1;
                        }
                        SaveStart(group) => {
                            debug!("\t\tSaveStart({}): {}", group, sp);

                            thread.captures = thread.captures.clone();

                            if group < thread.captures.len() {
                                thread.captures[group] = Some(Match{ start: sp, end: sp });
                            }
                            thread.pc +=1;
                        }
                        SaveEnd(group) => {
                            debug!("\t\tSaveEnd({}): {}", group, sp);
                            
                            if group < thread.captures.len() {
                                match thread.captures[group] {
                                    Some(ref mut m) => m.end = sp,
                                    None => unreachable!()
                                }
                            }
                            thread.pc +=1;
                        }
                        AssertStart => {
                            debug!("\t\tAssert ^: {}", sp == 0);

                            if sp == 0 {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        AssertEnd => {
                            debug!("\t\tAssert $: {}", sp == input.char_len());

                            if sp == input.char_len() {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        AssertWordBoundary => {
                            let a = if sp == 0 {
                                false
                            } else {
                                is_word_char(input[sp - 1] as char)
                            };
                            let b = is_word_char(c);

                            debug!("\t\tAssert b: ({}, {}): ", a, b);

                            if (a && !b) || (!a && b) {
                                debug!("\t\t\ttrue");

                                thread.pc += 1;
                            } else {
                                debug!("\t\t\tfalse");

                                break;
                            }
                        }
                        AssertNonWordBoundary => {
                            let a = if sp == 0 {
                                false
                            } else {
                                is_word_char(input[sp - 1] as char)
                            };
                            let b = is_word_char(c);

                            debug!("\t\tAssert b: ({}, {}): ", a, b);
                            
                            if (a && !b) || (!a && b) {
                                debug!("\t\t\tfalse");

                                break;
                            } else {
                                debug!("\t\t\ttrue");

                                thread.pc += 1;
                            }
                        }
                        Accept => {
                            debug!("\t\tAccept");
                            
                            let mut matches = ~[];
                            matches.push(Match{ start: thread.match_start, end: sp });
                            for capture in thread.captures.iter() {
                                match capture {
                                    &Some(m) => matches.push(m),
                                    &None => ()
                                }
                            }
                            matched = Some(matches);
                            break 'threads;
                        }
                    }
                }
            }
        }

        return matched;
    }
    */
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