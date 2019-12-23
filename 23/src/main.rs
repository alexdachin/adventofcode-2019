use std::collections::{HashMap, VecDeque};
use std::fs;

mod intcode;

#[derive(Debug, Clone)]
struct Packet {
    x: i64,
    y: i64,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the input.");

    let mut original_codes: HashMap<usize, i64> = HashMap::new();
    input
        .trim()
        .split(",")
        .map(|code| code.parse::<i64>().unwrap())
        .enumerate()
        .for_each(|(i, code)| {
            original_codes.insert(i, code);
        });

    let mut computers: Vec<intcode::Computer> = Vec::new();
    let mut packets_queue: Vec<VecDeque<Packet>> = Vec::new();
    for i in 0..50 {
        let mut computer = intcode::Computer::initialize(&original_codes);
        computer.execute();
        computer.provide_input(i);
        computers.push(computer);
        packets_queue.push(VecDeque::new());
    }

    let mut nat: Option<Packet> = None;
    let mut last_y: Option<i64> = None;

    'out_loop: loop {
        let mut is_idle = true;
        for i in 0..50 {
            computers[i].execute();
            if computers[i].has_pending_output {
                is_idle = false;
                let address = computers[i].get_output();
                computers[i].execute();
                let x = computers[i].get_output();
                computers[i].execute();
                let y = computers[i].get_output();

                if address < 50 {
                    packets_queue[address as usize].push_back(Packet { x, y });
                } else {
                    if address == 255 {
                        match nat {
                            None => println!("Part 1: {}", y),
                            _ => {}
                        }

                        nat = Some(Packet { x, y });
                    } else {
                        panic!("Packet sent to unrecognized address {}", address);
                    }
                }
            }

            if computers[i].waiting_for_input {
                let packet_result = packets_queue[i].pop_front();
                match packet_result {
                    None => {
                        computers[i].provide_input(-1);
                    }
                    Some(packet) => {
                        is_idle = false;
                        computers[i].provide_input(packet.x);
                        computers[i].execute();
                        computers[i].provide_input(packet.y);
                    }
                }
            }
        }

        if is_idle {
            match nat {
                None => continue,
                _ => {}
            }

            let nat_packet = nat.clone().unwrap();
            packets_queue[0].push_back(nat_packet.clone());
            match last_y {
                Some(y) => {
                    if y == nat_packet.y {
                        println!("Part 2: {}", y);
                        break 'out_loop;
                    }
                }
                None => {}
            }
            last_y = Some(nat_packet.y);
        }
    }
}
