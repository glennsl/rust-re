extern mod extra;

//use std::str;
use std::os;
use std::util;
use std::vec;

use compile::{
    Instruction,
    Char,
    Any,
    Range,
    Fork,
    Jump,
    ConditionalJumpEq,
    ConditionalJumpLE,
    Increment,
    SaveStart,
    SaveEnd,
    AssertStart,
    AssertEnd,
    AssertWordBoundary,
    AssertNonWordBoundary,
    Accept
};

mod parse;
mod compile;
mod debug;
/*
struct Thread<'self> {
    id: uint,
    pc: uint,
    match_start: uint,
    captures: &'self mut [Option<Match>]
}
*/
struct Thread {
    id: uint,
    pc: uint,
    match_start: uint,
    captures: ~[Option<Match>],
    registers: ~[uint]
}

#[deriving(Clone)]
pub struct Match {
    start: uint,
    end: uint
}

pub struct Regex {
    priv code: ~[Instruction],
    priv registers: uint
}

impl Regex {
    fn new(pattern: &str) -> ~Regex {
        let etree = parse::parse(pattern);
        let code = compile::compile(etree);
        let registers = compile::count_registers(code);
        return ~Regex { code: code, registers: registers };
    }

    fn partial_match(&self, input: &str) -> Option<~[Match]> {
        let mut threads = ~[]; // clist
        let mut next_threads = ~[]; // nlist
        let mut matched = None;

        let debug = true;
        //println("Inout: " + input);
        //debug::print_code(self.code);

        let mut thread_id = 0;
        let add_thread = |l: &mut ~[~Thread], pc, sp, captures: ~[Option<Match>], registers: ~[uint]| {
            if thread_id == 50 {
                fail!("Infinite loop?");
            }
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
            if debug {
                println!("Input {}", c);
            }

            if matched.is_none() {
                if debug {
                    println!("\tAdd thread {}: {}", thread_id, sp);
                }
                
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
                    if debug {
                        println!("\tThread {}", thread.id);
                    }
                    
                    match self.code[thread.pc] {
                        Char(ch) => {
                            if debug {
                                print!("\t\tChar({}): ", ch);
                            }
                            
                            if c == ch {
                                if debug {
                                    println("Match");
                                }
                                
                                thread.pc += 1;
                                next_threads.push(thread);
                            } else if debug {
                                println("Fail");
                            }
                            break;
                        }
                        Any => {
                            if debug {
                                println("\t\tAny");
                            }
                            
                            thread.pc += 1;
                            next_threads.push(thread);
                            break;
                        }
                        Range(start, end) => {
                            if debug {
                                println!("\t\tRange({}, {}) <{}, {}>", start as u8, end as u8, start, end);
                            }
                            
                            if c >= start && c <= end {
                                thread.pc += 1;
                                next_threads.push(thread);
                            }
                            break;
                        }
                        Fork(pc1, pc2) => {
                            if debug {
                                println!("\t\tFork({}, {}): add thread {}: {}", pc1, pc2, thread_id, thread.match_start);
                            }
                            
                            add_thread(&mut threads, pc2, thread.match_start, thread.captures.clone(), thread.registers.clone());
                            thread.pc = pc1;
                            //threads.push_back(thread);
                        }
                        Jump(new_pc) => {
                            if debug {
                                println!("\t\tJump({})", new_pc);
                            }
                            
                            thread.pc = new_pc;
                        }
                        ConditionalJumpEq(register, value, new_pc) => {
                            if debug {
                                print!("\t\tConditionalJump({} == {}): ", thread.registers[register], value);
                            }

                            if thread.registers[register] == value {
                                if debug {
                                    println!("Jump({})", new_pc);
                                }

                                thread.pc = new_pc;
                            } else {
                                if debug {
                                    println!("Fail");
                                }

                                thread.pc += 1;
                            }
                        }
                        ConditionalJumpLE(register, value, new_pc) => {
                            if debug {
                                print!("\t\tConditionalJump({} <= {}): ", thread.registers[register], value);
                            }

                            if thread.registers[register] < value {
                                if debug {
                                    println!("Jump({})", new_pc);
                                }

                                thread.pc = new_pc;
                            } else {
                                if debug {
                                    println!("Fail");
                                }

                                thread.pc += 1;
                            }
                        }
                        Increment(register) => {
                            if debug {
                                println!("\t\tIncrement({})", register);
                            }

                            thread.registers[register] += 1;
                            thread.pc += 1;
                        }
                        SaveStart(group) => {
                            if debug {
                                println!("\t\tSaveStart({}): {}", group, sp);
                            }

                            thread.captures = thread.captures.clone();

                            if group < thread.captures.len() {
                                thread.captures[group] = Some(Match{ start: sp, end: sp });
                            }
                            thread.pc +=1;
                        }
                        SaveEnd(group) => {
                            if debug {
                                println!("\t\tSaveEnd({}): {}", group, sp);
                            }
                            
                            if group < thread.captures.len() {
                                match thread.captures[group] {
                                    Some(ref mut m) => m.end = sp,
                                    None => unreachable!()
                                }
                            }
                            thread.pc +=1;
                        }
                        AssertStart => {
                            if debug {
                                println!("\t\tAssert ^: {}", sp == 0);
                            }

                            if sp == 0 {
                                thread.pc += 1;
                            } else {
                                break;
                            }
                        }
                        AssertEnd => {
                            if debug {
                                println!("\t\tAssert $: {}", sp == input.len());
                            }

                            if sp == input.len() {
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

                            if debug {
                                print!("\t\tAssert b: ({}, {}): ", a, b);
                            }

                            if (a && !b) || (!a && b) {
                                if debug {
                                    println("true");
                                }

                                thread.pc += 1;
                            } else {
                                if debug {
                                    println("false");
                                }

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

                            if debug {
                                print!("\t\tAssert b: ({}, {}): ", a, b);
                            }
                            
                            if (a && !b) || (!a && b) {
                                if debug {
                                    println("false");
                                }

                                break;
                            } else {
                                if debug {
                                    println("true");
                                }

                                thread.pc += 1;
                            }
                        }
                        Accept => {
                            if debug {
                                println("\t\tAccept");
                            }
                            
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

                //t += 1;
            }
        }

        return matched;
    }
}

// Not very international, but this is the standard
// http://www.ecma-international.org/ecma-262/5.1/#sec-15.10.2.6
fn is_word_char(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
    (c >= 'A' && c <= 'Z') ||
    (c >= '0' && c <= '9') ||
    c == '_'
}

fn main()  {
    match os::args() {
        [_, pattern, input] => {

            let etree = parse::parse(pattern);
            println("\nExpression Tree");
            println("-----------------");
            debug::print_expression_tree(etree);

            let code = compile::compile(etree);
            println("\nCode");
            println("------");
            debug::print_code(code);

            let re = Regex::new(pattern);
            match re.partial_match(input) {
                Some(matches) => {
                    println("\nYay!");
                    for (i, m) in matches.iter().enumerate() {
                        println!("  {}: {} ({}, {})", i, input.slice(m.start, m.end), m.start, m.end);
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
                        let actual_match = input.slice(m.start, m.end);

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
                                        let capture = input.slice(m.start, m.end).to_owned();
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