use std::{io};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("입력 실패");

    let mut counter : [u32; 26] = [0; 26];
    input = input.trim().to_uppercase();

    for _char in input.chars()
    {
        let char_diff = (_char as u32) - 65;
        counter[char_diff as usize] += 1;
    }

    let mut max_count = 0;
    let mut max_element = 0;
    for idx in 0..counter.len()
    {
        if max_count < counter[idx]
        {
            max_count = counter[idx];
            max_element = idx;
        }
    }

    let mut mat : i32 = 0;
    for idx in (max_element+1)..counter.len()
    {
        if max_count <= counter[idx]
        {
            mat = 1;
            break;
        }
    }

    match mat {
        1 => {
            println!("?");
        },
        _ => {
            println!("{}", ((max_element+65) as u8) as char)
        }
    };
}