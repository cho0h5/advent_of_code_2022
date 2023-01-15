use std::cmp::max;

struct Containers {
    containers: Vec<Vec<char>>,
}

impl std::fmt::Debug for Containers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width_of_containers = self.containers.len();
        let mut height_of_containers = 0;
        for line in &self.containers {
            height_of_containers = max(height_of_containers, line.len());
        }
        for i in (0..height_of_containers).rev() {
            for j in 0..width_of_containers {
                if let Some(c) = self.containers[j].get(i) {
                    write!(f, "[{}] ", c);
                } else {
                    write!(f, "    ");
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

fn parse_containers() -> Containers {
    let mut output = vec![];
    let input = "[C]         [S] [H]
[F] [B]     [C] [S]     [W]
[B] [W]     [W] [M] [S] [B]
[L] [H] [G] [L] [P] [F] [Q]
[D] [P] [J] [F] [T] [G] [M] [T]
[P] [G] [B] [N] [L] [W] [P] [W] [R]
[Z] [V] [W] [J] [J] [C] [T] [S] [C]
[S] [N] [F] [G] [W] [B] [H] [F] [N]";
    let input: Vec<&str> = input.split("\n").collect();
    let number_of_stacks = (input[input.len()-1].len()+1)/4;
    for _ in 0..number_of_stacks {
        output.push(vec![]);
    }

    for line in input.iter().rev() {
        for i in 0..number_of_stacks {
            if let Some(c) = line.as_bytes().get(4*i+1) {
                if let Some(ch) = std::char::from_u32(*c as u32) {
                    if ch == ' ' { continue }
                    output[i].push(ch);
                } else { continue }
            } else { break }
        }
    }

    Containers{ containers: output }
}
fn main() {
    let mut containers = parse_containers();
    println!("{:#?}", containers);

    process(&mut containers);

    let mut output = String::new();
    for container in containers.containers {
        let l = container.len();

        output.push(container[l-1]);
    }
    println!("{}", output);
}

fn p(containers: &mut Containers, count: u32, from: usize, to: usize) {
    for _ in 0..count {
        let temp = containers.containers[from-1].pop().unwrap();
        containers.containers[to-1].push(temp);
    }
}

fn process(containers: &mut Containers) {
    let instructions = "move 2 from 5 to 9
move 3 from 1 to 7
move 2 from 3 to 9
move 6 from 9 to 5
move 2 from 3 to 8
move 9 from 7 to 8
move 15 from 8 to 9
move 3 from 1 to 6
move 6 from 4 to 2
move 6 from 5 to 6
move 1 from 4 to 2
move 14 from 6 to 2
move 2 from 1 to 5
move 1 from 7 to 3
move 1 from 4 to 8
move 2 from 5 to 6
move 25 from 2 to 4
move 2 from 6 to 4
move 1 from 8 to 1
move 2 from 9 to 1
move 1 from 6 to 1
move 2 from 1 to 7
move 1 from 7 to 3
move 2 from 1 to 8
move 1 from 2 to 6
move 1 from 3 to 8
move 4 from 5 to 6
move 1 from 5 to 3
move 1 from 9 to 6
move 2 from 3 to 4
move 1 from 2 to 6
move 12 from 9 to 7
move 1 from 9 to 1
move 1 from 5 to 8
move 1 from 3 to 8
move 28 from 4 to 5
move 1 from 4 to 3
move 1 from 2 to 6
move 1 from 3 to 9
move 12 from 7 to 2
move 1 from 9 to 6
move 6 from 6 to 4
move 1 from 7 to 4
move 1 from 1 to 2
move 28 from 5 to 1
move 2 from 2 to 8
move 3 from 8 to 2
move 7 from 4 to 1
move 4 from 8 to 6
move 9 from 2 to 8
move 7 from 6 to 5
move 3 from 5 to 9
move 1 from 9 to 7
move 1 from 7 to 1
move 5 from 8 to 4
move 4 from 1 to 9
move 6 from 9 to 4
move 5 from 1 to 5
move 5 from 2 to 3
move 4 from 8 to 2
move 5 from 1 to 4
move 4 from 5 to 9
move 9 from 4 to 9
move 10 from 9 to 8
move 1 from 9 to 1
move 2 from 2 to 8
move 4 from 3 to 8
move 1 from 2 to 3
move 2 from 9 to 2
move 1 from 2 to 6
move 4 from 4 to 3
move 3 from 5 to 1
move 12 from 1 to 4
move 1 from 5 to 3
move 1 from 5 to 3
move 5 from 8 to 5
move 7 from 8 to 5
move 8 from 3 to 4
move 1 from 5 to 1
move 1 from 6 to 7
move 2 from 1 to 6
move 8 from 5 to 9
move 2 from 5 to 1
move 9 from 1 to 4
move 20 from 4 to 2
move 1 from 5 to 2
move 4 from 4 to 2
move 5 from 9 to 2
move 2 from 8 to 9
move 23 from 2 to 4
move 2 from 2 to 5
move 5 from 1 to 2
move 28 from 4 to 3
move 2 from 8 to 1
move 2 from 5 to 7
move 1 from 6 to 9
move 1 from 4 to 8
move 1 from 8 to 9
move 1 from 4 to 6
move 2 from 7 to 2
move 13 from 3 to 4
move 5 from 9 to 7
move 1 from 9 to 6
move 14 from 2 to 6
move 1 from 4 to 1
move 10 from 3 to 2
move 1 from 6 to 9
move 2 from 3 to 2
move 3 from 1 to 9
move 1 from 3 to 5
move 3 from 9 to 3
move 6 from 7 to 4
move 1 from 9 to 4
move 1 from 9 to 2
move 1 from 5 to 3
move 5 from 3 to 1
move 17 from 4 to 7
move 2 from 2 to 8
move 1 from 3 to 9
move 1 from 8 to 2
move 1 from 9 to 6
move 4 from 6 to 2
move 10 from 6 to 5
move 4 from 1 to 5
move 15 from 2 to 9
move 1 from 8 to 6
move 1 from 2 to 8
move 6 from 9 to 2
move 3 from 4 to 8
move 11 from 7 to 1
move 6 from 9 to 6
move 1 from 6 to 2
move 3 from 9 to 3
move 6 from 2 to 7
move 6 from 7 to 8
move 7 from 1 to 9
move 4 from 1 to 6
move 2 from 1 to 2
move 4 from 6 to 7
move 1 from 2 to 9
move 1 from 2 to 3
move 1 from 2 to 1
move 6 from 8 to 4
move 2 from 6 to 7
move 13 from 5 to 9
move 1 from 5 to 4
move 3 from 4 to 7
move 1 from 1 to 7
move 14 from 9 to 2
move 2 from 9 to 3
move 3 from 8 to 5
move 4 from 3 to 4
move 8 from 4 to 1
move 7 from 1 to 9
move 5 from 6 to 9
move 4 from 9 to 2
move 1 from 1 to 9
move 17 from 2 to 4
move 1 from 6 to 3
move 4 from 7 to 5
move 5 from 7 to 5
move 1 from 6 to 4
move 1 from 8 to 3
move 5 from 7 to 1
move 2 from 7 to 6
move 2 from 3 to 6
move 1 from 2 to 9
move 7 from 9 to 6
move 2 from 3 to 7
move 8 from 6 to 4
move 3 from 9 to 2
move 1 from 6 to 4
move 26 from 4 to 8
move 2 from 7 to 8
move 5 from 5 to 9
move 2 from 6 to 7
move 4 from 9 to 1
move 2 from 7 to 5
move 14 from 8 to 6
move 3 from 2 to 8
move 3 from 6 to 8
move 3 from 6 to 1
move 10 from 8 to 4
move 5 from 9 to 4
move 3 from 8 to 5
move 1 from 8 to 2
move 12 from 4 to 8
move 1 from 9 to 3
move 6 from 6 to 4
move 6 from 8 to 2
move 1 from 3 to 8
move 1 from 8 to 4
move 10 from 1 to 9
move 2 from 1 to 3
move 7 from 4 to 9
move 1 from 2 to 1
move 11 from 8 to 9
move 1 from 3 to 9
move 2 from 2 to 7
move 1 from 3 to 6
move 2 from 7 to 9
move 2 from 4 to 6
move 4 from 6 to 4
move 2 from 2 to 8
move 2 from 8 to 4
move 1 from 1 to 7
move 2 from 2 to 8
move 9 from 5 to 2
move 3 from 5 to 9
move 1 from 8 to 3
move 30 from 9 to 7
move 1 from 6 to 2
move 7 from 4 to 8
move 13 from 7 to 2
move 8 from 7 to 4
move 2 from 4 to 8
move 8 from 8 to 1
move 1 from 8 to 3
move 2 from 8 to 9
move 1 from 3 to 7
move 5 from 7 to 6
move 1 from 3 to 1
move 7 from 4 to 8
move 20 from 2 to 6
move 2 from 2 to 7
move 1 from 9 to 5
move 4 from 7 to 6
move 3 from 7 to 8
move 1 from 7 to 2
move 7 from 8 to 6
move 3 from 6 to 7
move 4 from 9 to 1
move 1 from 2 to 6
move 1 from 9 to 7
move 1 from 2 to 8
move 1 from 7 to 6
move 3 from 6 to 3
move 4 from 8 to 1
move 8 from 6 to 4
move 3 from 7 to 2
move 1 from 3 to 2
move 1 from 4 to 5
move 2 from 3 to 5
move 1 from 4 to 6
move 4 from 1 to 5
move 4 from 2 to 9
move 2 from 1 to 6
move 4 from 9 to 2
move 3 from 2 to 8
move 2 from 8 to 4
move 13 from 6 to 1
move 4 from 5 to 2
move 14 from 6 to 3
move 1 from 2 to 7
move 2 from 2 to 4
move 1 from 8 to 6
move 1 from 6 to 3
move 1 from 7 to 4
move 1 from 2 to 3
move 1 from 2 to 6
move 11 from 4 to 6
move 2 from 5 to 4
move 1 from 5 to 6
move 12 from 3 to 6
move 1 from 3 to 7
move 1 from 5 to 7
move 3 from 3 to 6
move 2 from 7 to 5
move 2 from 5 to 2
move 8 from 6 to 7
move 24 from 1 to 3
move 1 from 4 to 6
move 10 from 3 to 1
move 6 from 1 to 8
move 1 from 6 to 3
move 1 from 4 to 2
move 1 from 3 to 1
move 2 from 2 to 1
move 1 from 7 to 6
move 2 from 7 to 5
move 4 from 3 to 7
move 1 from 2 to 3
move 6 from 1 to 6
move 3 from 7 to 5
move 4 from 7 to 8
move 1 from 1 to 2
move 1 from 2 to 7
move 8 from 3 to 4
move 3 from 4 to 7
move 6 from 8 to 6
move 2 from 3 to 2
move 1 from 3 to 9
move 5 from 5 to 1
move 2 from 8 to 2
move 1 from 9 to 2
move 4 from 1 to 3
move 3 from 2 to 9
move 1 from 1 to 2
move 2 from 9 to 7
move 2 from 2 to 9
move 8 from 7 to 5
move 33 from 6 to 5
move 20 from 5 to 9
move 21 from 5 to 7
move 17 from 7 to 6
move 10 from 6 to 9
move 5 from 4 to 7
move 2 from 3 to 9
move 1 from 2 to 3
move 2 from 7 to 3
move 3 from 9 to 5
move 23 from 9 to 7
move 8 from 9 to 6
move 1 from 9 to 1
move 1 from 5 to 3
move 1 from 8 to 9
move 5 from 6 to 8
move 1 from 9 to 6
move 18 from 7 to 2
move 6 from 7 to 4
move 6 from 4 to 8
move 5 from 7 to 4
move 6 from 6 to 3
move 1 from 4 to 2
move 10 from 2 to 1
move 1 from 2 to 4
move 7 from 1 to 6
move 1 from 7 to 1
move 11 from 6 to 2
move 1 from 6 to 8
move 12 from 3 to 1
move 8 from 1 to 8
move 2 from 5 to 2
move 12 from 8 to 6
move 15 from 2 to 4
move 7 from 4 to 5
move 4 from 5 to 9
move 4 from 9 to 4
move 5 from 4 to 6
move 2 from 5 to 2
move 1 from 2 to 5
move 2 from 5 to 4
move 2 from 1 to 3
move 4 from 1 to 5
move 2 from 8 to 4
move 5 from 2 to 9
move 17 from 6 to 8
move 1 from 3 to 2
move 2 from 5 to 4
move 1 from 3 to 8
move 1 from 1 to 6
move 2 from 5 to 6
move 3 from 9 to 5
move 1 from 5 to 1
move 3 from 1 to 8
move 26 from 8 to 4
move 1 from 5 to 3
move 3 from 2 to 7
move 1 from 5 to 7
move 21 from 4 to 9
move 19 from 4 to 5
move 3 from 4 to 3
move 2 from 7 to 5
move 1 from 8 to 2
move 1 from 6 to 2
move 1 from 8 to 9
move 1 from 6 to 7
move 1 from 2 to 4
move 1 from 4 to 7
move 1 from 2 to 7
move 1 from 7 to 1
move 1 from 1 to 6
move 1 from 3 to 5
move 2 from 6 to 3
move 13 from 5 to 8
move 1 from 4 to 2
move 3 from 5 to 4
move 5 from 5 to 4
move 5 from 8 to 9
move 9 from 9 to 3
move 2 from 7 to 1
move 6 from 4 to 2
move 8 from 9 to 4
move 1 from 2 to 7
move 12 from 9 to 8
move 1 from 4 to 2
move 3 from 7 to 3
move 11 from 8 to 5
move 5 from 8 to 6
move 3 from 6 to 5
move 2 from 4 to 1
move 13 from 5 to 3
move 1 from 1 to 7
move 2 from 1 to 8
move 3 from 4 to 9
move 1 from 1 to 7
move 1 from 2 to 4
move 2 from 7 to 3
move 1 from 5 to 3
move 4 from 4 to 2
move 1 from 4 to 9
move 30 from 3 to 2
move 1 from 9 to 7
move 6 from 8 to 6
move 1 from 7 to 6
move 1 from 5 to 1
move 1 from 3 to 5
move 30 from 2 to 3
move 1 from 1 to 9
move 2 from 9 to 2
move 9 from 6 to 9
move 2 from 2 to 9
move 1 from 5 to 1
move 5 from 9 to 7
move 8 from 2 to 5
move 1 from 1 to 9
move 3 from 9 to 1
move 5 from 3 to 6
move 8 from 5 to 9
move 13 from 3 to 9
move 3 from 1 to 7
move 5 from 7 to 9
move 17 from 9 to 6
move 1 from 7 to 6
move 6 from 3 to 9
move 1 from 2 to 1
move 2 from 7 to 1
move 1 from 2 to 5
move 21 from 9 to 2
move 4 from 3 to 6
move 6 from 6 to 5
move 7 from 5 to 9
move 2 from 3 to 8
move 3 from 1 to 3
move 4 from 6 to 5
move 1 from 8 to 1
move 1 from 8 to 2
move 4 from 5 to 2
move 4 from 9 to 1
move 4 from 3 to 5
move 2 from 1 to 7
move 1 from 7 to 4
move 3 from 9 to 5
move 25 from 2 to 9
move 18 from 9 to 1
move 1 from 4 to 5
move 1 from 3 to 8
move 4 from 5 to 6
move 2 from 9 to 3
move 17 from 1 to 5
move 1 from 2 to 7
move 2 from 3 to 5
move 3 from 1 to 8
move 5 from 9 to 2
move 4 from 8 to 9
move 12 from 5 to 2
move 1 from 1 to 8
move 3 from 9 to 5
move 1 from 8 to 2
move 2 from 7 to 2
move 1 from 9 to 5
move 9 from 5 to 2
move 6 from 6 to 2
move 15 from 6 to 2
move 5 from 5 to 9
move 1 from 5 to 9
move 3 from 9 to 2
move 3 from 9 to 1
move 1 from 1 to 9
move 1 from 9 to 1
move 19 from 2 to 8
move 2 from 1 to 9
move 33 from 2 to 6
move 4 from 6 to 4
move 1 from 2 to 6
move 1 from 9 to 8
move 3 from 4 to 8
move 18 from 8 to 3
move 1 from 4 to 9
move 10 from 3 to 9
move 1 from 1 to 4
move 24 from 6 to 3
move 1 from 4 to 3
move 2 from 8 to 7
move 8 from 9 to 3
move 5 from 6 to 7
move 35 from 3 to 2
move 7 from 7 to 1
move 3 from 1 to 3
move 33 from 2 to 6
move 6 from 3 to 7
move 5 from 7 to 3
move 1 from 1 to 4
move 1 from 7 to 8
move 1 from 4 to 8
move 1 from 3 to 2
move 30 from 6 to 5
move 2 from 1 to 6
move 5 from 8 to 1
move 1 from 9 to 2
move 2 from 6 to 4
move 4 from 1 to 7
move 21 from 5 to 8";
    let instructions: Vec<&str> = instructions.split("\n").collect();

    for instruction in instructions {
        let parsed = sscanf::sscanf!(instruction, "move {} from {} to {}", u32, usize, usize);
        let (count, from, to) = parsed.unwrap();

        println!("{}", instruction);
        p(containers, count, from, to);
        println!("{:?}", containers);
    }
}