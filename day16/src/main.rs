use std::{fs::{read_to_string}, str::Chars};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Packet {
    version: u8,
    type_id: u8,
    bit_length: u64,
    length_type_id: u8,
    value: u64 // Represents either value or length
}

fn load() -> String {
    let filename = "./input.txt";
    let text = read_to_string(filename)
        .expect("Could not read file");
    let binary: String = hex::decode(&text).unwrap()
        .into_iter()
        .map(|x| format!("{:08b}",x).chars().collect::<Vec<_>>())
        .flatten()
        .collect();
    return binary;
}

fn binary_to_integer(str: &String) -> u64 {
    return isize::from_str_radix(&str, 2).unwrap().try_into().unwrap();
}

fn try_get_string(pointer: &mut Chars, size: usize) -> Option<String> {
    let val = pointer.take(size).collect::<String>();
    if val.len() < size {
        return None;
    }
    return Some(val);
}

fn parse_packet(pointer: &mut Chars) -> Option<Packet> {
    // First 3 bytes are the version
    let version= match try_get_string(pointer,3) {
        Some(str) => binary_to_integer(&str),
        None => return None
    };
    // Next 3 bytes are the type_id
    let type_id= match try_get_string(pointer,3) {
        Some(str) => binary_to_integer(&str),
        None => return None
    };
    // Figure out value or length
    let mut value: u64 = 0;
    let mut bit_length: u64 = 6;
    let mut length_type_id: u8 = 0;
    if type_id == 4 {
        // Next N bytes are all possible values, iterate through and keep looking
        let mut val_str: Vec<char> = Vec::new();
        loop {
            let end = pointer.next().unwrap() == '0';
            let str= match try_get_string(pointer,4) {
                Some(str) => str,
                None => return None
            };  
            val_str.append(&mut str.chars().collect::<Vec<char>>());
            bit_length += 5;
            if end {
                break;
            }
        }
        value = binary_to_integer(&val_str.into_iter().collect::<String>());
    } else {
        // Next bit indicates length type id
        length_type_id = match pointer.next() {
            Some(c) => c.to_digit(10).unwrap() as u8,
            None => return None
        };
        bit_length += 1;
        let len = if length_type_id == 0 { 15 } else { 11 };
        let val_str = match try_get_string(pointer,len) {
            Some(str) => str,
            None => return None
        };
        value = binary_to_integer(&val_str);
        bit_length += len as u64;
    }
    return Some(Packet {
        version: version as u8,
        type_id: type_id as u8,
        length_type_id: length_type_id,
        bit_length: bit_length,
        value: value
    });
}

fn handle_operator_packet(operator: &Packet, stack: &mut Vec<Packet>) -> Packet {
    let mut parameters: Vec<Packet> = Vec::new();
    let mut count = 0;
    while count < operator.value {
        let param = stack.pop().unwrap();
        parameters.push(param);
        count += if operator.length_type_id == 0 { param.bit_length } else { 1 };
    }
    let value: u64 = match operator.type_id {
        0 => parameters.iter().map(|x| x.value).sum(), 
        1 => parameters.iter().map(|x| x.value).reduce(|a,v| a * v).unwrap(), 
        2 => parameters.iter().map(|x| x.value).min().unwrap(), 
        3 => parameters.iter().map(|x| x.value).max().unwrap(), 
        5 => if parameters[0].value > parameters[1].value { 1 } else { 0 }, 
        6 => if parameters[0].value < parameters[1].value { 1 } else { 0 }, 
        7 => if parameters[0].value == parameters[1].value { 1 } else { 0 }, 
        _ => panic!("invalid operator")
    };
    let total_bit_length = operator.bit_length + parameters.into_iter().map(|x| x.bit_length).sum::<u64>();
    return Packet {
        version: 0,
        type_id: 4,
        length_type_id: 0,
        bit_length: total_bit_length,
        value: value
    }
}

fn part_a(binary: &String) {
    let mut pointer = binary.chars().into_iter();
    let mut packets: Vec<Packet> = Vec::new();
    while let Some(packet) = parse_packet(&mut pointer) {
        packets.push(packet);
    }
    let sum_ver: u64 = (&packets).into_iter().map(|x| x.version as u64).sum();
    println!("A: {}", sum_ver);
}

fn part_b(binary: &String) {
    let mut pointer = binary.chars().into_iter();
    let mut packets: Vec<Packet> = Vec::new();
    while let Some(packet) = parse_packet(&mut pointer) {
        packets.push(packet);
    }
    let mut stack: Vec<Packet> = Vec::new();
    for packet in packets.into_iter().rev() {
        match packet.type_id {
            4 => stack.push(packet), 
            _ => {
                let ret = handle_operator_packet(&packet,&mut stack);
                stack.push(ret);
            }
        };
    }
    // There should only be one value type in the stack at this point with our final value
    println!("B: {}", stack[0].value);
}

fn main() {
    let binary = load();
    part_a(&binary);
    part_b(&binary);
}
