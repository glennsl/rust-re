extern mod extra;

use std::num;
use std::u8;
use std::str;
use std::from_str;
use std::iter;
use std::vec;
use extra::sort;

pub enum QuantifierType {
	Greedy,
	NonGreedy
}

pub enum Expression {
    Literal(char),
    AnyLiteral, // .
    RangeLiteral(char, char), 
    CharacterClass(~[(char, char)]),
    Concatenate(~Expression, ~Expression), // implied
    Alternate(~Expression, ~Expression), // |
    SubExpression(~Expression, Option<uint>),
    Question(~Expression, QuantifierType),
    Star(~Expression, QuantifierType),
    Plus(~Expression, QuantifierType),
    ExactRepetition(~Expression, uint, QuantifierType), // {x}
    UnboundedRepetition(~Expression, uint, QuantifierType), // {x,}
    BoundedRepetition(~Expression, uint, uint, QuantifierType), // {x, y}
    AssertStart, // ^
    AssertEnd, // $
    AssertWordBoundary, // \b
    AssertNonWordBoundary // \B
}

struct Parser<'self> {
    pattern: &'self str,
    pos: uint,
    next: uint,
    captures: uint
}

impl<'self> Parser<'self> {
    fn new<'r> (pattern: &'r str) -> Parser<'r>  {
        Parser{
            pattern: pattern,
            pos: 0,
            next: 0,
            captures: 0
        }
    }

    fn next(&mut self) -> Option<char> {
        self.pos = self.next;

        if (self.pos >= self.pattern.len()) {
            return None;
        } else {
            let str::CharRange { ch, next } = self.pattern.char_range_at(self.pos);
            self.next = next;
            return Some(ch);
        }
    }

    fn peek(&mut self, offset: uint) -> Option<char> {
        let mut pos = self.next;
        let mut ret = None;

        for _ in iter::range(0, offset) {
            if (pos >= self.pattern.len()) {
                return None;
            } else {
                let str::CharRange { ch, next } = self.pattern.char_range_at(pos);
                pos = next;
                ret = Some(ch);
            }
        }

        return ret;
    }

    fn fail(&mut self, message: &str) -> ! {
    	// Too verbose for teting, since task fails always prints the fail message
    	//let ptr = str::from_chars(vec::from_elem(self.pos, ' ')) + "^";
    	//let s = format!("Parse error: {}\n  {}\n  {}\n", message, self.pattern, ptr);
    	let s = "Parse error: " + message;
        fail!(s);
    }
}

pub fn parse(pattern: &str) -> ~Expression {
	let mut parser = Parser::new(pattern);

    let e = parse_recursive(&mut parser);

    if parser.pos < pattern.len() {
    	// Inferred since parse_Recursive only terminates on end of string or 
    	// encountering a ')'. And since we haven't reached end of string...
    	parser.fail("Unexpected ')' encountered.");
    }

    return e;
}

fn do_concat(stack: &mut ~[~Expression]) {
    while stack.len() > 1 {
        let (right, left) = (stack.pop(), stack.pop());
        stack.push(~Concatenate(left, right));
    }
}

fn do_alternate(stack: &mut ~[~Expression]) {
    while stack.len() > 1 {
        let (right, left) = (stack.pop(), stack.pop());
        stack.push(~Alternate(left, right));
    }
}

fn negate_charclass(ranges: &[(char, char)]) -> ~Expression {
    let mut inverted_ranges = ~[];

    let sorted_ranges = sort::merge_sort(ranges, |v1, v2| v1.first() <= v2.first());

    let mut start = '\0';
    let mut end = '\0';
    for &(rstart, rend) in sorted_ranges.iter() {
        if rstart > end {
            inverted_ranges.push((start, (rstart as u8 - 1) as char));
        }
        start = num::max(end as u8, rend as u8 + 1) as char;
        end = start;
    }
    inverted_ranges.push((start, u8::max_value as char));
    /*
    for &(s, e) in inverted_ranges.iter() {
        println("(" + (s as u8).to_str() + "[" + str::from_char(s) + "], " + (e as u8).to_str() + "[" + str::from_char(e) + "])");
    }
    */
    return ~CharacterClass(inverted_ranges);
}

fn parse_charclass(parser: &mut Parser) -> ~Expression {
    let mut ranges: ~[(char, char)] = ~[];
    let mut negated = false;

    loop {
        match parser.next() {
            Some('^') if ranges.is_empty() && !negated => negated = true,
            Some('\\') => {
            	match parse_charclass_escape(parser) {
            		~CharacterClass(r) => ranges.push_all(r),
            		~Literal(c) => ranges.push((c, c)),
            		_ => unreachable!()
            	}
            }
            Some('-') => {
                match ranges.pop_opt() {
                    Some((last_start, last_end)) => {
                        match parser.next() {
                            Some(']') => {
                            	ranges.push(('-', '-'));
                            	break;
                            }
                            Some(end) => {
		                        if (last_start != last_end) {
		                        	parser.fail("Unexpected '-' in character class. Missing start of range.");
		                        }
                                if last_start >= end {
                                    parser.fail("Character class range start is larger than or equal to range end.");
                                }
                                ranges.push((last_start, end));
                            }
                            None => parser.fail("Unterminated character class.")
                        }
                    }
                    None => ranges.push(('-', '-'))
                }
            }
            Some(']') => break,
            Some(c) => ranges.push((c, c)),
            None => parser.fail("Unterminated character class.")
        }
    }

    if ranges.is_empty() {
        parser.fail("Empty character class");
    }
    if negated {
        return negate_charclass(ranges);
    } else {
        return ~CharacterClass(ranges);
    }
}

fn parse_charclass_escape(parser: &mut Parser) -> ~Expression {
	match parser.next() {
		Some('d') => ~CharacterClass(~[('0', '9')]),
		Some('D') => negate_charclass([('0', '9')]),
		Some('s') => ~CharacterClass(~[
							('\t', '\t'), // Tab
							('\r', '\r'), // Carriage Return
							('\n', '\n'), // Line Feed
							('\x0b', '\x0b'), // Vertical Tab
							('\x0c', '\x0c'), // Form Feed
							('\u2028', '\u2028'), // Line Separator
							('\u2029', '\u2029'), // Paragraph Separator
							('\u00a0', '\u00a0'), // No-break Space
							('\ufeff', '\ufeff') // Byte Order Mark
					 ]),
		Some('S') => negate_charclass([
							('\t', '\t'), // Tab
							('\r', '\r'), // Carriage Return
							('\n', '\n'), // Line Feed
							('\x0b', '\x0b'), // Vertical Tab
							('\x0c', '\x0c'), // Form Feed
							('\u2028', '\u2028'), // Line Separator
							('\u2029', '\u2029'), // Paragraph Separator
							('\u00a0', '\u00a0'), // No-break Space
							('\ufeff', '\ufeff') // Byte Order Mark
					 ]),
		Some('w') => ~CharacterClass(~[('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')]),
		Some('W') => negate_charclass([('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')]),
		Some(c) => ~Literal(c),
		None => parser.fail("Incomplete escape sequence.")
	}
}

fn parse_escape(parser: &mut Parser) -> ~Expression {
	match parser.next() {
		Some('d') => ~CharacterClass(~[('0', '9')]),
		Some('D') => negate_charclass([('0', '9')]),
		Some('s') => ~CharacterClass(~[
							('\t', '\t'), // Tab
							('\r', '\r'), // Carriage Return
							('\n', '\n'), // Line Feed
							('\x0b', '\x0b'), // Vertical Tab
							('\x0c', '\x0c'), // Form Feed
							('\u2028', '\u2028'), // Line Separator
							('\u2029', '\u2029'), // Paragraph Separator
							('\u00a0', '\u00a0'), // No-break Space
							('\ufeff', '\ufeff') // Byte Order Mark
					 ]),
		Some('S') => negate_charclass([
							('\t', '\t'), // Tab
							('\r', '\r'), // Carriage Return
							('\n', '\n'), // Line Feed
							('\x0b', '\x0b'), // Vertical Tab
							('\x0c', '\x0c'), // Form Feed
							('\u2028', '\u2028'), // Line Separator
							('\u2029', '\u2029'), // Paragraph Separator
							('\u00a0', '\u00a0'), // No-break Space
							('\ufeff', '\ufeff') // Byte Order Mark
					 ]),
		Some('w') => ~CharacterClass(~[('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')]),
		Some('W') => negate_charclass([('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')]),
		Some('b') => ~AssertWordBoundary,
		Some('B') => ~AssertNonWordBoundary,
		Some(c) => ~Literal(c),
		None => parser.fail("Incomplete escape sequence.")
	}
}

fn parse_group(parser: &mut Parser) -> ~Expression {
	match parser.peek(1) {
		Some ('?') => {
			match parser.peek(2) {
				Some(':') => {
					// Non-capturing group
					parser.next();
					parser.next();
					let e = parse_recursive(parser);
					return ~SubExpression(e, None);
				}
				Some('=') => {
					//Positive lookahead
					parser.fail("NOT IMPLEMENTED");
				}
				Some('!') => {
					//Negative lookahead
					parser.fail("NOT IMPLEMENTED");
				}
				Some(_) => (),
				None => parser.fail("Unterminated sub expression.")
			}
		}
		Some(_) => (),
		None => parser.fail("Unterminated sub expression.")
	}

	// Normal capturing group
	parser.captures += 1;
	let capture_slot = parser.captures;
	let e = parse_recursive(parser);
	return ~SubExpression(e, Some(capture_slot));
}

fn parse_repetition(parser: &mut Parser, expr: ~Expression) -> ~Expression {
	let mut low = None;
	let mut buffer = ~"";

	loop {
		match parser.next() {
			Some(c) if c >= '0' && c <= '9' => buffer.push_char(c),
			Some(',') => {
				if buffer.len() == 0 && low.is_none() {
					parser.fail("Unexpected ',' encountered in repetition.");
				}
				low = from_str::from_str(buffer);
				buffer.clear();
			}
			Some('}') => {
				if buffer.len() == 0 {
					match low {
						Some(n) => return ~UnboundedRepetition(expr, n, Greedy),
						None => parser.fail("Illegal empty repetition.")
					}
				} else {
					let n = from_str::from_str(buffer).unwrap();
					match low {
						Some(l) => return ~BoundedRepetition(expr, l, n, Greedy),
						None => return ~ExactRepetition(expr, n, Greedy)
					}
				}
			}
			Some(_) => parser.fail("Non-numeric character in repetition."),
			None => parser.fail("Unterminated repetition.")
		}
	}
}

fn parse_recursive(parser: &mut Parser) -> ~Expression {
    let mut stack = ~[];

    while (true) {
        match parser.next() {
            Some('.') => stack.push(~AnyLiteral),
            Some('\\') => {
            	let e = parse_escape(parser);
            	stack.push(e);
            }
            Some('|') => {
            	do_concat(&mut stack);
                match stack.pop_opt() {
                    Some(e) => {
                        let left = e;
                        let right = parse_recursive(parser);
                        stack.push(~Alternate(left, right));
                        break;
                    }
                    None => parser.fail("Missing left operand for '|' operator.")
                }
            }
            Some('*') => {
                match stack.pop_opt() {
                	Some(~Star(_, _)) |
                	Some(~Question(_, _)) => parser.fail("Nothing to repeat."), // Would cause infinite loops
                    Some(e) => stack.push(~Star(e, Greedy)),
                    None => parser.fail("Missing left operand for '*' operator.")
                }
            }
            Some('+') => {
                match stack.pop_opt() {
                	Some(~Star(_, _)) |
                	Some(~Question(_, _)) => parser.fail("Nothing to repeat."), // Would cause infinite loops
                    Some(e) => stack.push(~Plus(e, Greedy)),
                    None => parser.fail("Missing left operand for '+' operator.")
                }
            }
            Some('?') => {
                match stack.pop_opt() {
                	Some(~Question(e, Greedy)) => stack.push(~Question(e, NonGreedy)),
                	Some(~Star(e, Greedy)) => stack.push(~Star(e, NonGreedy)),
                	Some(~Plus(e, Greedy)) => stack.push(~Plus(e, NonGreedy)),
                	Some(~ExactRepetition(e, count, Greedy)) => stack.push(~ExactRepetition(e, count, NonGreedy)),
                	Some(~UnboundedRepetition(e, low, Greedy)) => stack.push(~UnboundedRepetition(e, low, NonGreedy)),
                	Some(~BoundedRepetition(e, low, high, Greedy)) => stack.push(~BoundedRepetition(e, low, high, NonGreedy)),
                    Some(e) => stack.push(~Question(e, Greedy)),
                    None => parser.fail("Missing left operand for '?' operator.")
                }
            }
            Some('(') => {
                do_concat(&mut stack);
                let e = parse_group(parser);
                stack.push(e);
            }
            Some(')') => break,
            Some('[') => stack.push(parse_charclass(parser)),
            Some('{') => {
            	match stack.pop_opt() {
            		Some(e) => stack.push(parse_repetition(parser, e)),
            		None => parser.fail("Unexpected '{' encountered.")
            	}
            }
            Some('^') => stack.push(~AssertStart),
            Some('$') => stack.push(~AssertEnd),
            Some(c) => stack.push(~Literal(c)),
            None => break
        }
    }

    do_concat(&mut stack);
    match stack.pop_opt() {
    	Some(e) => return e,
    	None => parser.fail("Illegal empty expression.")
    }
}