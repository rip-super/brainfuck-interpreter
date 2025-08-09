use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("script.bf")?);

    let mut tape = vec![0u8; 30_000];
    let mut pointer = 0;

    let code: Vec<char> = reader
        .lines()
        .map_while(Result::ok)
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut jump_table = HashMap::new();
    let mut stack = Vec::new();

    for (i, &c) in code.iter().enumerate() {
        if c == '[' {
            stack.push(i);
        } else if c == ']' {
            if let Some(start) = stack.pop() {
                jump_table.insert(start, i);
                jump_table.insert(i, start);
            } else {
                return Err("Unmatched closing bracket".into());
            }
        }
    }

    if !stack.is_empty() {
        return Err("Unmatched opening bracket".into());
    }

    let mut pos = 0;

    while pos < code.len() {
        match code[pos] {
            '>' => {
                pointer += 1;
                if pointer >= tape.len() {
                    pointer = tape.len() - 1;
                }
            }
            '<' => {
                pointer = pointer.saturating_sub(1);
            }
            '+' => {
                tape[pointer] = tape[pointer].wrapping_add(1);
            }
            '-' => {
                tape[pointer] = tape[pointer].wrapping_sub(1);
            }
            '.' => {
                print!("{}", tape[pointer] as char)
            }
            ',' => {
                let mut input = String::new();
                tape[pointer] = {
                    io::stdin().read_line(&mut input)?;
                    input.trim().chars().next().map_or(0, |c| c as u8)
                };
            }
            '[' => {
                if tape[pointer] == 0 {
                    pos = *jump_table.get(&pos).unwrap();
                }
            }
            ']' => {
                if tape[pointer] != 0 {
                    pos = *jump_table.get(&pos).unwrap();
                }
            }
            _ => {}
        }

        pos += 1;
    }

    Ok(())
}
