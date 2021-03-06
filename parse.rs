use std::num;
use std::str;
use std::char;
use std::from_str;
use std::iter;
//use std::vec;
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
    pos: uint,  // in bytes
    next: uint, // in bytes
    current: Option<char>,
    captures: uint
}

impl<'self> Parser<'self> {
    fn new<'r> (pattern: &'r str) -> Parser<'r>  {
        Parser{
            pattern: pattern,
            pos: 0,
            next: 0,
            current: None,
            captures: 0
        }
    }

    fn next(&mut self) -> Option<char> {
        self.pos = self.next;

        if (self.pos >= self.pattern.len()) {
            self.current = None;
        } else {
            let str::CharRange { ch, next } = self.pattern.char_range_at(self.pos);
            self.next = next;
            self.current = Some(ch);
        }

        return self.current;
    }

    fn consume_chars(&mut self, n: uint) {
        do n.times {
            self.next();
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

pub fn parse(pattern: &str) -> Expression {
	let mut parser = Parser::new(pattern);

    let e = parse_recursive(&mut parser);

    if parser.pos < pattern.len() {
    	// Inferred since parse_recursive only terminates on end of string or 
    	// encountering a ')'. And since we haven't reached end of string...
    	parser.fail("Unexpected ')' encountered.");
    }

    return e;
}

#[inline]
fn do_concat(stack: &mut ~[Expression]) {
    while stack.len() > 1 {
        let (right, left) = (stack.pop(), stack.pop());
        stack.push(Concatenate(~left, ~right));
    }
}

#[inline]
fn do_alternate(stack: &mut ~[Expression]) {
    while stack.len() > 1 {
        let (right, left) = (stack.pop(), stack.pop());
        stack.push(Alternate(~left, ~right));
    }
}

#[inline]
fn negate_char_ranges(ranges: &[(char, char)]) -> Expression {
    let mut inverted_ranges = ~[];

    let sorted_ranges = sort::merge_sort(ranges, |v1, v2| v1.first() <= v2.first());

    let mut start = '\0';
    let mut end = '\0';
    for &(rstart, rend) in sorted_ranges.iter() {
        if rstart > end {
            match char::from_u32(rstart as u32 - 1) {
                Some(c) => inverted_ranges.push((start, c)),
                None => inverted_ranges.push((start, rstart))
            }
        }
        start = match char::from_u32(num::max(end as u32, rend as u32 + 1)) {
            Some(c) => c,
            None => rend
        };
        end = start;
    }

    inverted_ranges.push((start, char::MAX));

    return CharacterClass(inverted_ranges);
}

#[inline]
fn parse_charclass(parser: &mut Parser) -> Expression {
    let mut ranges: ~[(char, char)] = ~[];
    let mut negated = false;

    loop {
        match parser.next() {
            Some('^') if ranges.is_empty() && !negated => negated = true,
            Some('\\') => {
            	match parse_charclass_escape(parser) {
            		CharacterClass(r) => ranges.push_all(r),
            		Literal(c) => ranges.push((c, c)),
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
        return negate_char_ranges(ranges);
    } else {
        return CharacterClass(ranges);
    }
}

#[inline]
fn parse_common_escape(c: char) -> Option<Expression> {
    let ranges = match c {
        'd' |
        'D' => ~[('0', '9')],

        's' |
        'S' => ~[('\t', '\t'), // Tab
                 ('\r', '\r'), // Carriage Return
                 ('\n', '\n'), // Line Feed
                 ('\x0b', '\x0b'), // Vertical Tab
                 ('\x0c', '\x0c'), // Form Feed
                 ('\u2028', '\u2028'), // Line Separator
                 ('\u2029', '\u2029'), // Paragraph Separator
                 ('\u00a0', '\u00a0'), // No-break Space
                 ('\ufeff', '\ufeff')], // Byte Order Mark

        'w' |
        'W' => ~[('a', 'z'), ('A', 'Z'), ('0', '9'), ('_', '_')],

        _ => return None
    };

    if c.is_uppercase() {
        Some(negate_char_ranges(ranges))
    } else {
        Some(CharacterClass(ranges))
    }
}

#[inline]
fn parse_charclass_escape(parser: &mut Parser) -> Expression {
	match parser.next() {
        Some(c) => {
            match parse_common_escape(c) {
                Some(e) => e,
                None => Literal(c)
            }
        }
		None => parser.fail("Incomplete escape sequence.")
	}
}

#[inline]
fn parse_escape(parser: &mut Parser) -> Expression {
    match parser.next() {
        Some('b') => AssertWordBoundary,
        Some('B') => AssertNonWordBoundary,
        Some(c) => {
            match parse_common_escape(c) {
                Some(e) => e,
                None => Literal(c)
            }
        }
        None => parser.fail("Incomplete escape sequence.")
    }
}

#[inline]
fn parse_group(parser: &mut Parser) -> Expression {
    let mut capture = false;

	match parser.peek(1) {
		Some ('?') => {
			match parser.peek(2) {
                // Non-capturing group
				Some(':') => parser.consume_chars(2),

                //Positive lookahead
				Some('=') => parser.fail("NOT IMPLEMENTED"),

                //Negative lookahead
				Some('!') =>parser.fail("NOT IMPLEMENTED"),

                // Normal capturing group
				Some(_) => capture = true,

				None => parser.fail("Unterminated sub expression.")
			}
		}
        // Normal capturing group
		Some(_) => capture = true,

		None => parser.fail("Unterminated sub expression.")
	}

	
	let capture_slot = if capture {
        parser.captures += 1;
        Some(parser.captures)
    } else {
        None
    };

	let e = parse_recursive(parser);

    match parser.current {
        Some(')') => return SubExpression(~e, capture_slot),
        _ => parser.fail("Unterminated group")
    }	
}

#[inline]
fn parse_repetition(parser: &mut Parser, expr: Expression) -> Expression {
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
						Some(n) => return UnboundedRepetition(~expr, n, Greedy),
						None => parser.fail("Illegal empty repetition.")
					}
				} else {
					let n = from_str::from_str(buffer).unwrap();
					match low {
						Some(l) => return BoundedRepetition(~expr, l, n, Greedy),
						None => return ExactRepetition(~expr, n, Greedy)
					}
				}
			}
			Some(_) => parser.fail("Non-numeric character in repetition."),
			None => parser.fail("Unterminated repetition.")
		}
	}
}

fn parse_recursive(parser: &mut Parser) -> Expression {
    let mut stack = ~[];

    while (true) {
        match parser.next() {
            Some('.') => stack.push(AnyLiteral),
            Some('\\') => {
            	let e = parse_escape(parser);
            	stack.push(e);
            }
            Some('|') => {
            	do_concat(&mut stack);
                match stack.pop_opt() {
                    Some(left) => {
                        let right = parse_recursive(parser);
                        stack.push(Alternate(~left, ~right));
                        break;
                    }
                    None => parser.fail("Missing left operand for '|' operator.")
                }
            }
            Some('*') => {
                match stack.pop_opt() {
                	Some(Star(_, _)) |
                	Some(Question(_, _)) => parser.fail("Nothing to repeat."), // Would cause infinite loops
                    Some(e) => stack.push(Star(~e, Greedy)),
                    None => parser.fail("Missing left operand for '*' operator.")
                }
            }
            Some('+') => {
                match stack.pop_opt() {
                	Some(Star(_, _)) |
                	Some(Question(_, _)) => parser.fail("Nothing to repeat."), // Would cause infinite loops
                    Some(e) => stack.push(Plus(~e, Greedy)),
                    None => parser.fail("Missing left operand for '+' operator.")
                }
            }
            Some('?') => {
                match stack.pop_opt() {
                	Some(Question(e, Greedy)) => stack.push(Question(e, NonGreedy)),
                	Some(Star(e, Greedy)) => stack.push(Star(e, NonGreedy)),
                	Some(Plus(e, Greedy)) => stack.push(Plus(e, NonGreedy)),
                	Some(ExactRepetition(e, count, Greedy)) => stack.push(ExactRepetition(e, count, NonGreedy)),
                	Some(UnboundedRepetition(e, low, Greedy)) => stack.push(UnboundedRepetition(e, low, NonGreedy)),
                	Some(BoundedRepetition(e, low, high, Greedy)) => stack.push(BoundedRepetition(e, low, high, NonGreedy)),
                    Some(e) => stack.push(Question(~e, Greedy)),
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
            Some('^') => stack.push(AssertStart),
            Some('$') => stack.push(AssertEnd),
            Some(c) => stack.push(Literal(c)),
            None => break
        }
    }

    do_concat(&mut stack);
    match stack.pop_opt() {
    	Some(e) => return e,
    	None => parser.fail("Illegal empty expression.")
    }
}