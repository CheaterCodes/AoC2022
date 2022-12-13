use std::{error::Error, fs::File, io::{BufReader, BufRead}, cmp::Ordering};

#[derive(PartialEq, Eq)]
enum PacketData {
    Integer(i64),
    List(Vec<PacketData>),
}

impl PacketData {
    fn parse(input: &mut StringParser) -> PacketData {
        if input.parse_string("[").is_some() {
            let mut list = Vec::new();
            if !input.parse_string("]").is_some() {
                loop {
                    list.push(PacketData::parse(input));
                    if input.parse_string("]").is_some() {
                        break;
                    }
                    input.parse_string(",").unwrap();
                }
            }

            PacketData::List(list)
        } else {
            PacketData::Integer(input.parse_integer().unwrap())
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketData::Integer(left) => match other {
                PacketData::Integer(right) => left.cmp(right),
                PacketData::List(_) => PacketData::List(vec![PacketData::Integer(*left)]).cmp(other),
            },
            PacketData::List(left) => match other {
                PacketData::Integer(right) => self.cmp(&PacketData::List(vec![PacketData::Integer(*right)])),
                PacketData::List(right) => {
                    let mut left_iter = left.iter();
                    let mut right_iter = right.iter();
                    loop {
                        let next_left = left_iter.next();
                        let next_right = right_iter.next();
                        if next_left.is_none() {
                            if next_right.is_none() {
                                break Ordering::Equal;
                            } else {
                                break Ordering::Less;
                            }
                        } else {
                            if next_right.is_none() {
                                break Ordering::Greater;
                            } else {
                                let cmp = next_left.unwrap().cmp(next_right.unwrap());
                                if cmp.is_eq() {
                                    continue;
                                }
                                break cmp;
                            }
                        }
                    }
                },
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day13/input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut pairs = Vec::new();
    for chunk in lines.chunks(3) {
        pairs.push((
            PacketData::parse(&mut StringParser::new(&chunk[0])),
            PacketData::parse(&mut StringParser::new(&chunk[1])),
        ));
    }

    let mut total = 0;
    for (index, pair) in pairs.iter().enumerate() {
        if pair.0 <= pair.1 {
            total += index + 1;
        }
    }
    println!("Total: {total}");
    
    Ok(())
}

struct StringParser<'a> {
    input: &'a str
}

impl<'a> StringParser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn parse_string(&mut self, string: &str) -> Option<&str> {
        if !self.input.is_char_boundary(string.len()) {
            return None;
        }

        let (head, tail) = self.input.split_at(string.len());
        if head == string {
            self.input = tail;
            Some(head)
        } else {
            None
        }
    }

    fn parse_integer(&mut self) -> Option<i64> {
        let (sign, mut input) = match &self.input[0..1] {
            "-" => (-1, &self.input[1..]),
            "+" => (1, &self.input[1..]),
            _ => (1, self.input),
        };

        let mut val = 0;

        let first = input.chars().next();
        if !first.is_some() || !first.unwrap().is_ascii_digit() {
            return None;
        }

        while let Some(next) = input.chars().next() {
            if next.is_ascii_digit() {
                val *= 10;
                val += next as i64 - '0' as i64;
                input = &input[1..];
            } else {
                break;
            }
        }

        self.input = input;
        Some(sign * val)
    }
}
