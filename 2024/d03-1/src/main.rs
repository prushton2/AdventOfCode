use std::fs;
#[derive(PartialEq, Debug)]
// what character we are at
enum MulState {
    Searching,
    M,
    U,
    L,
    OpenParen,
    Num1(i32),
    Comma(i32),
    Num2(i32, i32),
    CloseParen(i32, i32)
}
#[derive(Debug)]
enum DoState {
    Searching,
    D,
    O,
    N,
    Apostrophe,
    T,
    OpenParen(bool),
    CloseParen(bool)
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading file");

    let mut mul_state = MulState::Searching;
    let mut do_state = DoState::Searching;

    let mut enabled = true;
    let mut sum = 0;

    for char in file.chars() {
        parse_state(&mut mul_state, &mut do_state, char);

        println!("{} {} {:?} {:?}", char, enabled, mul_state, do_state);

        match do_state {
            DoState::CloseParen(b) => enabled = b,
            _ => {}
        }

        if !enabled {
            continue
        }

        match mul_state {
            MulState::CloseParen(a, b) => {
                sum += a * b;
                mul_state = MulState::Searching;
            }
            _ => {}
        }
    }

    println!("{}", sum)
}

fn parse_state(mul_state: &mut MulState, do_state: &mut DoState, char: char) {
    match mul_state {
        MulState::Searching if char == 'm' => *mul_state = MulState::M,
        MulState::M if char == 'u' => *mul_state = MulState::U,
        MulState::U if char == 'l' => *mul_state = MulState::L,
        MulState::L if char == '(' => *mul_state = MulState::OpenParen,

        MulState::OpenParen if char.is_numeric() => *mul_state = MulState::Num1(char.to_digit(10).unwrap() as i32),
        MulState::Num1(a) if char.is_numeric() => *mul_state = MulState::Num1(*a*10 + (char.to_digit(10).unwrap() as i32)),
        MulState::Num1(a) if char == ',' => *mul_state = MulState::Comma(*a),
        
        MulState::Comma(a) if char.is_numeric() => *mul_state = MulState::Num2(*a, char.to_digit(10).unwrap() as i32),
        MulState::Num2(a, b) if char.is_numeric() => *mul_state = MulState::Num2(*a, *b*10 + (char.to_digit(10).unwrap() as i32)),
        MulState::Num2(a, b) if char == ')' => *mul_state = MulState::CloseParen(*a, *b),

        _ => *mul_state = MulState::Searching
    }
    match do_state {
        DoState::Searching if char == 'd' => *do_state = DoState::D,
        DoState::D if char == 'o' => *do_state = DoState::O,

        DoState::O if char == 'n' => *do_state = DoState::N,
        DoState::O if char == '(' => *do_state = DoState::OpenParen(true),

        DoState::N if char == '\'' => *do_state = DoState::Apostrophe,
        DoState::Apostrophe if char == 't' => *do_state = DoState::T,
        DoState::T if char == '(' => *do_state = DoState::OpenParen(false),

        DoState::OpenParen(a) if char == ')' => *do_state = DoState::CloseParen(*a),

        _ => *do_state = DoState::Searching
    }
}
