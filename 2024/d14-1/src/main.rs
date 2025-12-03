use std::fs;
static GRID_SIZE: (i64, i64) = (101, 103);
// static GRID_SIZE: (i64, i64) = (11, 7);
struct Robot {
    pub pos: (i64, i64),
    pub v: (i64, i64)
}
fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut robots: Vec<Robot> = {
        let mut v: Vec<Robot> = vec![];
        for line in file.split('\n') {
            let binding = line.chars().filter(|c| !(c.is_alphabetic() || c == &'=')).collect::<String>();
            // println!("{binding}");
            let mut temp = binding.split(&[',', ' ']).map(|c| c.parse::<i64>().unwrap());
            v.push(Robot { 
                pos: (
                    temp.next().unwrap(),
                    temp.next().unwrap(),
                ), 
                v: (
                    temp.next().unwrap(),
                    temp.next().unwrap(),
                ) 
            });
        }
        v
    };

    
    let sim_time = 6531;
    let mut grid: [[u8; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize] = [[0; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize];

    for i in 0..robots.len() {
        let robot = match robots.get_mut(i) {
            Some(n) => n,
            None => continue
        };

        robot.pos = (
            robot.pos.0 + robot.v.0 * sim_time,
            robot.pos.1 + robot.v.1 * sim_time
        );

        robot.pos = (
            modulo(robot.pos.0, GRID_SIZE.0),
            modulo(robot.pos.1, GRID_SIZE.1)
        );

        // println!("{:?}", robot.pos);

        grid[robot.pos.1 as usize][robot.pos.0 as usize] += 1;
    }
    println!("Sim time {}", sim_time);
    for y in 0..GRID_SIZE.1 as usize {
        for x in 0..GRID_SIZE.0 as usize {
            match grid[y][x] {
                0 => {print!(".");}
                d => {print!("{}", d)}
            }
        }
        println!("");
    }
    
    let mut bots_per_quadrant: (u64, u64, u64, u64) = (0, 0, 0, 0);

    for robot in robots {

        match robot.pos {
            (x, y) if x > GRID_SIZE.0/2 && y < GRID_SIZE.1/2 => {bots_per_quadrant.0 += 1}
            (x, y) if x < GRID_SIZE.0/2 && y < GRID_SIZE.1/2 => {bots_per_quadrant.1 += 1}
            (x, y) if x < GRID_SIZE.0/2 && y > GRID_SIZE.1/2 => {bots_per_quadrant.2 += 1}
            (x, y) if x > GRID_SIZE.0/2 && y > GRID_SIZE.1/2 => {bots_per_quadrant.3 += 1}
            _ => {}
        }
    }

    println!("{:?}", bots_per_quadrant);
    println!("{}", bots_per_quadrant.0 * bots_per_quadrant.1 * bots_per_quadrant.2 * bots_per_quadrant.3);

}

fn modulo(a: i64, b: i64) -> i64 {
    let mut a = a;
    while a < 0 {
        a += b;
    }

    a %= b;
    return a;
}