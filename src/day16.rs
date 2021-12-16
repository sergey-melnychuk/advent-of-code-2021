use advent_of_code_2021::util::lines;

type Bit = u64;

struct BitSet {
    bytes: Vec<u8>,
    byte: usize,
    bit: u8,
}

impl BitSet {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            byte: 0,
            bit: 7,
        }
    }

    fn is_empty(&self) -> bool {
        self.byte == self.bytes.len()
    }

    fn get(&mut self) -> Option<Bit> {
        if !self.is_empty() {
            let mask = 1u8 << self.bit;
            let num = ((self.bytes[self.byte] & mask) >> self.bit) as Bit;
            if self.bit == 0 {
                self.bit = 7;
                self.byte += 1;
            } else {
                self.bit -= 1;
            }
            Some(num)
        } else {
            None
        }
    }
}

impl Iterator for BitSet {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        self.get()
    }
}

struct GroupBy<'a, T> {
    src: &'a mut dyn Iterator<Item = T>,
    size: usize,
}

impl<'a, T> Iterator for GroupBy<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            if let Some(item) = self.src.next() {
                group.push(item);
            } else {
                break;
            }
        }
        if group.is_empty() {
            None
        } else {
            Some(group)
        }
    }
}

fn group_by<'a, T: 'a>(
    it: &'a mut (dyn Iterator<Item = T> + 'a),
    size: usize,
) -> impl Iterator<Item = Vec<T>> + 'a {
    GroupBy { src: it, size }
}

fn digit(c: char) -> u8 {
    if c.is_ascii_digit() {
        c as u8 - b'0'
    } else {
        c.to_ascii_uppercase() as u8 - b'A' + 10
    }
}

fn parse(s: &str) -> Vec<u8> {
    group_by(&mut s.chars(), 2)
        .map(|byte| byte.into_iter().fold(0u8, |acc, c| (acc << 4) + digit(c)))
        .collect()
}

fn join(it: &mut dyn Iterator<Item = Bit>, n: usize) -> Option<Bit> {
    Some(pull(it, n)?.into_iter().fold(0, |acc, x| (acc << 1) + x))
}

fn pull(it: &mut dyn Iterator<Item = Bit>, n: usize) -> Option<Vec<Bit>> {
    let vec = it.take(n).collect::<Vec<_>>();
    if vec.len() == n {
        Some(vec)
    } else {
        None
    }
}

fn next(it: &mut dyn Iterator<Item = Bit>) -> Option<Bit> {
    it.next()
}

const TYPE_LITERAL: Bit = 4;

enum Packet {
    Literal(Bit, Bit),
    Group(Bit, Bit, Vec<Packet>),
}

fn match_packet(bits: &mut dyn Iterator<Item = Bit>) -> Option<Packet> {
    let version = join(bits, 3)?;
    let type_id = join(bits, 3)?;

    if type_id == TYPE_LITERAL {
        let mut num: Bit = 0;
        loop {
            let flag = next(bits)?;
            let word = join(bits, 4)?;
            num = (num << 4) + word;
            if flag == 0 {
                break;
            }
        }
        Some(Packet::Literal(version, num))
    } else {
        let len_type_id = next(bits)?;
        if len_type_id == 0 {
            let total_len = join(bits, 15)? as usize;

            let mut packets = Vec::new();
            let chunk = pull(bits, total_len)?;
            let mut it = chunk.into_iter();
            while let Some(packet) = match_packet(&mut it) {
                packets.push(packet);
            }
            Some(Packet::Group(version, type_id, packets))
        } else {
            let total_count = join(bits, 11)?;

            let mut packets = Vec::new();
            for _ in 0..total_count {
                let packet = match_packet(bits)?;
                packets.push(packet);
            }
            Some(Packet::Group(version, type_id, packets))
        }
    }
}

fn iter(packet: &Packet, sum: &mut Bit) {
    match packet {
        Packet::Literal(v, _) => *sum += v,
        Packet::Group(v, _, packets) => {
            *sum += v;
            for p in packets {
                iter(p, sum);
            }
        }
    }
}

fn eval(packet: &Packet) -> Bit {
    match packet {
        Packet::Literal(_, val) => *val,
        Packet::Group(_, id, packets) if *id == 0 => packets.iter().map(|p| eval(p)).sum::<Bit>(),
        Packet::Group(_, id, packets) if *id == 1 => {
            packets.iter().map(|p| eval(p)).product::<Bit>()
        }
        Packet::Group(_, id, packets) if *id == 2 => packets.iter().map(|p| eval(p)).min().unwrap(),
        Packet::Group(_, id, packets) if *id == 3 => packets.iter().map(|p| eval(p)).max().unwrap(),
        Packet::Group(_, id, packets) if *id == 5 => {
            let fst = eval(&packets[0]);
            let snd = eval(&packets[1]);
            if fst > snd {
                1
            } else {
                0
            }
        }
        Packet::Group(_, id, packets) if *id == 6 => {
            let fst = eval(&packets[0]);
            let snd = eval(&packets[1]);
            if fst < snd {
                1
            } else {
                0
            }
        }
        Packet::Group(_, id, packets) if *id == 7 => {
            let fst = eval(&packets[0]);
            let snd = eval(&packets[1]);
            if fst == snd {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    let dump = lines()[0].to_owned();
    let data = parse(&dump);

    let mut bits = BitSet::new(data);
    let packet = match_packet(&mut bits).unwrap();

    let mut sum = 0;
    iter(&packet, &mut sum);
    println!("{}", sum);

    let part2 = eval(&packet);
    println!("{}", part2);
}
