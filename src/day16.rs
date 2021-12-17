use aoc_runner_derive::*;

#[derive(Debug)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
struct Packet {
    version: u32,
    packet_type: PacketType,
    subpackets: Vec<Packet>,
    value: Option<u64>,
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Packet {
    let binary_str = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(16).unwrap())
        .map(|digit| format!("{:04b}", digit).chars().collect::<Vec<_>>())
        .flatten()
        .collect::<String>();

    let (packet, _) = parse_packet(binary_str.clone());
    packet
}

fn parse_packet(binary_str: String) -> (Packet, usize) {
    let version = u32::from_str_radix(&binary_str[0..3], 2).unwrap();
    let packet_type = get_packet_type(u8::from_str_radix(&binary_str[3..6], 2).unwrap());

    match packet_type {
        PacketType::Literal => {
            let (value, length) = parse_literal(&binary_str[6..]);
            (
                Packet {
                    version,
                    packet_type,
                    subpackets: vec![],
                    value: Some(value),
                },
                length + 6,
            )
        }
        _ => {
            let length_type_id = u8::from_str_radix(&binary_str[6..7], 2).unwrap();
            let mut total_length = 7;

            if length_type_id == 1 {
                let packet_count =
                    u16::from_str_radix(&binary_str[total_length..total_length + 11], 2).unwrap();
                total_length += 11;
                let mut subpackets = vec![];
                for _ in 0..packet_count {
                    let (subpacket, length) = parse_packet(binary_str[total_length..].to_string());
                    subpackets.push(subpacket);
                    total_length += length;
                }
                (
                    Packet {
                        version,
                        packet_type,
                        subpackets,
                        value: None,
                    },
                    total_length,
                )
            } else {
                let subpacket_length =
                    usize::from_str_radix(&binary_str[total_length..total_length + 15], 2).unwrap();
                let mut subpackets = vec![];
                let mut i = 0;
                total_length += 15;
                while i < subpacket_length {
                    let (subpacket, length) = parse_packet(
                        binary_str[total_length..].to_string(),
                    );
                    subpackets.push(subpacket);
                    total_length += length;
                    i += length;
                }
                (
                    Packet {
                        version,
                        packet_type,
                        subpackets,
                        value: None,
                    },
                    total_length,
                )
            }
        }
    }
}

fn get_packet_type(num: u8) -> PacketType {
    match num {
        0 => PacketType::Sum,
        1 => PacketType::Product,
        2 => PacketType::Minimum,
        3 => PacketType::Maximum,
        4 => PacketType::Literal,
        5 => PacketType::GreaterThan,
        6 => PacketType::LessThan,
        7 => PacketType::EqualTo,
        _ => panic!("Unknown packet type"),
    }
}

fn parse_literal(binary_str: &str) -> (u64, usize) {
    let (val_str, length) = get_literal(binary_str);
    (u64::from_str_radix(val_str.as_str(), 2).unwrap(), length)
}

fn get_literal(binary_str: &str) -> (String, usize) {
    let value_str = binary_str[1..5].to_string();
    let length = 5;
    if &binary_str[0..1] == "1" {
        let (next_value_str, next_length) = get_literal(&binary_str[5..]);
        (value_str + &next_value_str, length + next_length)
    } else {
        (value_str, length)
    }
}

#[aoc(day16, part1)]
fn part1(input: &Packet) -> u32 {
    sum_versions(input)
}

#[aoc(day16, part2)]
fn part2(input: &Packet) -> u64 {
    match input.packet_type {
        PacketType::Sum => input.subpackets.iter().map(|p| part2(p)).sum(),
        PacketType::Product => input.subpackets.iter().map(|p| part2(p)).product(),
        PacketType::Minimum => input.subpackets.iter().map(|p| part2(p)).min().unwrap(),
        PacketType::Maximum => input.subpackets.iter().map(|p| part2(p)).max().unwrap(),
        PacketType::Literal => input.value.unwrap(),
        PacketType::GreaterThan => {
            let left = part2(&input.subpackets[0]);
            let right = part2(&input.subpackets[1]);
            if left > right {
                1
            } else {
                0
            }
        }
        PacketType::LessThan => {
            let left = part2(&input.subpackets[0]);
            let right = part2(&input.subpackets[1]);
            if left < right {
                1
            } else {
                0
            }
        }
        PacketType::EqualTo => {
            let left = part2(&input.subpackets[0]);
            let right = part2(&input.subpackets[1]);
            if left == right {
                1
            } else {
                0
            }
        }
    }
}

fn sum_versions(packet: &Packet) -> u32 {
    packet.version + packet.subpackets.iter().map(sum_versions).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::parse_literal;

    #[test]
    fn test_parse_literal() {
        assert_eq!(parse_literal("1000101111"), (31, 10));
        assert_eq!(parse_literal("1001011100100011001101011"), (180539, 25));
    }
}
