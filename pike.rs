use std::vec;
use std::util;

use super::compile;
use super::compile::{
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

use super::matcher;

// Not very international, but this is the standard
// http://www.ecma-international.org/ecma-262/5.1/#sec-15.10.2.6
fn is_word_char(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
    (c >= 'A' && c <= 'Z') ||
    (c >= '0' && c <= '9') ||
    c == '_'
}

/*
struct Thread<'self> {
    id: uint,
    pc: uint,
    match_start: uint,
    captures: &'self mut [Option<matcher::Match>]
}
*/
struct Thread {
    pc: uint,
    match_start: uint,
    captures: ~[Option<matcher::Match>],
    registers: ~[uint]
}

pub struct PikeMatcher<'self> {
    priv code: &'self [Instruction],
    priv threads: ~[~Thread],
    priv next_threads: ~[~Thread],
    priv input: &'self str,
    priv sp: uint,
    priv matched: Option<~[matcher::Match]>,
    priv registers: uint
}

impl<'self> PikeMatcher<'self> {
    pub fn do_match<'r>(code: &'r [Instruction], input: &'r str) -> Option<~[matcher::Match]> {
        let mut matcher = PikeMatcher {
            code: code,
            threads: vec::with_capacity(code.len()),
            next_threads: vec::with_capacity(code.len()),
            input: input,
            sp: 0,
            matched: None,
            registers: compile::count_registers(code)
        };
        
        return matcher.run(input);
    }

    #[inline]
    fn schedule_next(&mut self, mut thread: ~Thread) {
        thread.pc += 1;
        self.next_threads.push(thread);
    }

    fn run(&mut self, input: &str) -> Option<~[matcher::Match]> {
        for (sp, c) in input.iter().chain("\x03".iter()).enumerate() {
            debug!("Input {}", c);

            self.sp = sp;

            if self.matched.is_none() {
                //debug!("\tAdd thread {}: {}", self.thread_id, sp);
                self.next_threads.push(
                    ~Thread {
                        pc: 0,
                        match_start: sp,
                        captures: ~[None, ..10],
                        registers: vec::from_elem(self.registers, 0u) });
            }

            util::swap(&mut self.threads, &mut self.next_threads);
            self.threads.reverse();
            self.next_threads.clear();

            'threads: loop {
                match self.threads.pop_opt() {
                    Some(thread) => {
                        match self.run_thread(thread, c) {
                            Some(_) => break,
                            None => continue
                        }
                    },
                    None => break
                };
            }
        }

        return self.matched.clone();
    }

    #[inline]
    fn run_thread(&mut self, mut thread: ~Thread, c: char) -> Option<~[matcher::Match]> {
        loop {
            //debug!("\tThread {}", thread.id);
            //debug_instruction!(self.code[thread.pc]);
            
            match self.code[thread.pc] {
                Char(ch) if c == ch => {
                    self.schedule_next(thread);
                    return None;
                }
                Char(_) => return None,
                Any => {
                    self.schedule_next(thread);
                    return None;
                }
                Range(start, end) if c >= start && c <= end => {
                    self.schedule_next(thread);
                    return None;
                }
                Range(_, _) => return None,
                Fork(pc1, pc2) => {
                    self.threads.push(
                        ~Thread {
                            pc: pc2,
                            match_start: thread.match_start,
                            captures: thread.captures.clone(),
                            registers: thread.registers.clone() });

                    thread.pc = pc1;
                }
                Jump(new_pc) => thread.pc = new_pc,
                ConditionalJumpEq(register, value, new_pc) => {
                    if thread.registers[register] == value {
                        thread.pc = new_pc;
                    } else {
                        thread.pc += 1;
                    }
                }
                ConditionalJumpLE(register, value, new_pc) => {
                    if thread.registers[register] < value {
                        thread.pc = new_pc;
                    } else {
                        thread.pc += 1;
                    }
                }
                Increment(register) => {
                    thread.registers[register] += 1;
                    thread.pc += 1;
                }
                SaveStart(group) => {
                    thread.captures = thread.captures.clone();

                    if group < thread.captures.len() {
                        thread.captures[group] = Some(matcher::Match{ start: self.sp, end: self.sp });
                    }

                    thread.pc +=1;
                }
                SaveEnd(group) => {
                    if group < thread.captures.len() {
                        match thread.captures[group] {
                            Some(ref mut m) => m.end = self.sp,
                            None => unreachable!()
                        }
                    }

                    thread.pc +=1;
                }
                AssertStart => {
                    if self.sp == 0 {
                        thread.pc += 1;
                    } else {
                        return None;
                    }
                }
                AssertEnd => {
                    if self.sp == self.input.char_len() {
                        thread.pc += 1;
                    } else {
                        return None;
                    }
                }
                AssertWordBoundary => {
                    let a = if self.sp == 0 {
                        false
                    } else {
                        // TODO: char indexing
                        is_word_char(self.input[self.sp - 1] as char)
                    };
                    let b = is_word_char(c);

                    if (a && !b) || (!a && b) {
                        thread.pc += 1;
                    } else {
                        return None;
                    }
                }
                AssertNonWordBoundary => {
                    let a = if self.sp == 0 {
                        false
                    } else {
                        // TODO: char indexing
                        is_word_char(self.input[self.sp - 1] as char)
                    };
                    let b = is_word_char(c);
                    
                    if (a && !b) || (!a && b) {
                        return None;
                    } else {
                        thread.pc += 1;
                    }
                }
                Accept => {
                    let mut matches = ~[];
                    matches.push(matcher::Match{ start: thread.match_start, end: self.sp });
                    for capture in thread.captures.iter() {
                        match capture {
                            &Some(m) => matches.push(m),
                            &None => ()
                        }
                    }
                    self.matched = Some(matches);
                    return self.matched.clone();
                }
            }
        }
    }
}
/*
macro_rules! debug_instruction!(instruction: Instruction) {
    match instruction {
        Char(c) => debug!("\t\tChar({}): ", ch),
        Range(start, end) => debug!("\t\tRange({}, {}) <{}, {}>", start as u8, end as u8, start, end);
    }
}
*/