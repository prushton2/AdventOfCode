use std::{collections::HashMap, fs};
use std::hash::Hash;
use std::ops::{Add, Mul};
static CACHE_SIZE: i32 = 5;

struct Cache {
    cache: HashMap<u64, HashMap<u64, u64>>,
    cache_hits: u32,
    cache_misses: u32,
}

impl Cache {
    pub fn new() -> Self {
        return Cache {
            cache: [].into(),
            cache_hits: 0,
            cache_misses: 0,
        };
    }

    pub fn get(&mut self, key: u64) -> Option<&HashMap<u64, u64>> {
        match self.cache.get(&key) {
            Some(n) => {
                self.cache_hits += 1;
                return Some(n);
            }
            None => {
                self.cache_misses += 1;
                return None;
            }
        }
    }

    pub fn update(&mut self, key: u64, value: HashMap<u64, u64>) {
        self.cache.insert(key, value);
    }

    pub fn print_cache(&self) {
        println!("Cache:\n    Hits: {}\n  Misses: {}\n     PCT: {}\n", self.cache_hits, self.cache_misses, (100.0 * self.cache_hits as f64)/(self.cache_hits as f64 + self.cache_misses as f64))
    }
}

fn main() {
    let file = fs::read_to_string("./src/input.txt").expect("Error reading input");
    let mut stones: HashMap<u64, u64> = [].into();
    let mut cache = Cache::new();
    let mut sum: u64 = 0;

    // println!("Solving for {}", i);
    // sum += solve(i.parse::<u64>().unwrap(), 0, &mut cache);
    for i in file.split(' ') {
        stones.insert_or_sum(i.parse::<u64>().unwrap(), 1);
    }

    for _ in 0..(75/CACHE_SIZE) {
        let mut new_stones: HashMap<u64, u64> = [].into();

        for (stone, count) in stones.into_iter() {

            let result = match cache.get(stone) {
                Some(n) => n,
                None => {
                    cache.update(stone, calculate_steps(CACHE_SIZE, stone));
                    cache.get(stone).unwrap()
                }
            };

            new_stones.merge(result, count);
        }

        stones = new_stones;
    }

    for (_, count) in stones.into_iter() {
        sum += count;
    }
    cache.print_cache();
    println!("{}", sum);
}

pub trait HashMap2<K, V> {
    fn insert_or_sum(&mut self, k: K, v: V);
    fn merge(&mut self, other: &HashMap<K, V>, multiplier: V);
}

impl<K, V> HashMap2<K, V> for HashMap<K, V> where 
    K: Eq + Hash + Clone,
    V: Add<Output = V> + Clone + Mul<Output = V>
{
    fn insert_or_sum(&mut self, k: K, v: V) {
        match self.insert(k.clone(), v.clone()) {
            Some(n) => {self.insert(k, v+n);},
            None => {}
        }
    }

    fn merge(&mut self, other: &HashMap<K, V>, multiplier: V) {
        for (k, v) in other.iter() {
            self.insert_or_sum(k.clone(), v.clone() * multiplier.clone());
        }
    }
}

fn calculate_steps(steps: i32, num: u64) -> HashMap<u64, u64> {
    let mut stones: HashMap<u64, u64> = [(num, 1)].into();

    for _ in 0..steps {
        let mut new_stones: HashMap<u64, u64> = [].into();
        for (stone, count) in stones.into_iter() {
            
            if stone == 0 {
                new_stones.insert_or_sum(1, count);
                continue;
            }

            if stone.ilog(10) % 2 == 1 {
                let chars = 1 + stone.ilog(10) as usize;
                let string = format!("{}", stone);

                new_stones.insert_or_sum(string[0..chars / 2].parse::<u64>().unwrap(), count);
                new_stones.insert_or_sum(string[chars/2..chars].parse::<u64>().unwrap(), count);

                continue;
            }

            new_stones.insert_or_sum(stone * 2024, count);
        }
        stones = new_stones;
    }
    return stones;
}
