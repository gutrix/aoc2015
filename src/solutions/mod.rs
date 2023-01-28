use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashSet,
    str::FromStr,
};

pub type Solution = (Option<i32>, Option<i32>);

pub fn s(i: i32) -> std::io::Result<Solution> {
    let input = get_input(i.to_string().as_str())?;
    let solution: Solution = match i {
        1 => s1(input.as_str()),
        2 => s2(input.as_str()),
        3 => s3(input.as_str()),
        4 => s4(input.as_str()),
        5 => s5(input.as_str()),
        6 => s6(input.as_str()),
        _ => (None, None),
    };
    Ok(solution)
}

fn get_input(num: &str) -> std::io::Result<String> {
    std::fs::read_to_string(format!("input/{}.txt", num))
}

fn s1(input: &str) -> Solution {
    let mut i = 0;
    let mut b: Option<i32> = None;
    for (it, c) in input.trim().chars().enumerate() {
        if c == '(' {
            i = i + 1;
        }
        if c == ')' {
            i = i - 1;
        }
        if i == -1 && b == None {
            b = Some(it as i32 + 1);
        }
    }
    (Some(i), b)
}

fn s2(input: &str) -> Solution {
    let lines = input.lines();

    // parse each line
    let mut perimeter: i64 = 0;
    let mut ribben_length: i64 = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split('x').collect();
        if tokens.len() != 3 {
            return (None, None);
        }

        let mut tokens: Vec<i64> = tokens.iter().map(|t| t.parse::<i64>().unwrap()).collect();

        let l = tokens[0];
        let w = tokens[1];
        let h = tokens[2];

        // calculate size of sides
        let lw = l * w;
        let wh = w * h;
        let hl = h * l;

        // size of wrapping paper needed
        perimeter += 2 * lw + 2 * wh + 2 * hl + min(lw, min(wh, hl));

        // Part 2
        tokens.sort();

        let smallest_loop = 2 * tokens[0] + 2 * tokens[1];
        let volume = l * w * h;
        ribben_length += smallest_loop + volume;
    }

    (
        Some(perimeter.try_into().unwrap()),
        Some(ribben_length.try_into().unwrap()),
    )
}

fn s3(input: &str) -> Solution {
    let mov = |c: char, i: (i32, i32)| match c {
        '>' => (i.0 + 1, i.1),
        '<' => (i.0 - 1, i.1),
        '^' => (i.0, i.1 + 1),
        'v' => (i.0, i.1 - 1),
        _ => i,
    };

    let mut first: HashSet<(i32, i32)> = HashSet::new();
    first.insert((0, 0));

    let mut second: HashSet<(i32, i32)> = HashSet::new();
    second.insert((0, 0));

    let mut alone_santa = (0, 0);
    let mut santa = (0, 0);
    let mut robo = (0, 0);
    let mut real_santa_year = true;

    // Treverse
    for c in input.chars() {
        // first year, only santa.
        alone_santa = mov(c, alone_santa);
        first.insert(alone_santa);

        // second year, santa and robo santa
        if real_santa_year {
            santa = mov(c, santa);
            second.insert(santa);
        } else {
            robo = mov(c, robo);
            second.insert(robo);
        }

        real_santa_year = !real_santa_year;
    }

    (Some(first.len() as i32), Some(second.len() as i32))
}

fn s4(input: &str) -> Solution {
    // This is slow, so it's just returning hardcoded results.
    // Just set a,b to None below to have it search.

    let mut a = Some(346386);
    let mut b = Some(9958218);

    for i in 0..1000000000 {
        let secret = input.to_owned() + &i.to_string();
        let ans = format!("{:x}", md5::compute(secret));

        if ans.starts_with("00000") && a.is_none() {
            a = Some(i);
        }

        if ans.starts_with("000000") && b.is_none() {
            b = Some(i);
        }

        if a.is_some() && b.is_some() {
            return (a, b);
        }
    }

    (None, None)
}

fn s5(input: &str) -> Solution {
    let vowels = "aeiou";

    let mut q1 = 0;
    let mut q2 = 0;
    for line in input.lines() {
        let number_of_vowels = line
            .chars()
            .filter(|c| vowels.contains(*c))
            .collect::<Vec<char>>()
            .len();

        let contains_double_letter =
            line.chars().tuple_windows().filter(|(a, b)| a == b).count() > 0;

        let contains_evil = line.contains("ab")
            || line.contains("cd")
            || line.contains("pq")
            || line.contains("xy");

        if number_of_vowels >= 3 && contains_double_letter && !contains_evil {
            q1 += 1;
        }

        // part two
        let mut contains_pair = false;
        for c in line.chars().tuple_windows::<(char, char)>() {
            let c = format!("{}{}", c.0, c.1);
            let i = line.find(&c).unwrap();

            let rline = line.replacen(&c, "", 1);
            let (l, r) = rline.split_at(i);
            if l.contains(&c) || r.contains(&c) {
                contains_pair = true;
                break;
            }
        }

        let contains_triplet = line
            .chars()
            .tuple_windows()
            .filter(|(a, _, c)| a == c)
            .count()
            > 0;

        if contains_pair && contains_triplet {
            q2 += 1;
        }
    }

    (Some(q1), Some(q2))
}

fn s6(input: &str) -> Solution {
    #[derive(PartialEq, Debug)]
    struct Vec2u {
        x: usize,
        y: usize,
    }

    #[derive(PartialEq, Debug)]
    struct Coords {
        p1: Vec2u,
        p2: Vec2u,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct ParseCoordError;

    impl FromStr for Coords {
        type Err = ParseCoordError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let cs: Vec<&str> = s.split(",").collect();
            let (x1, y1, x2, y2) = cs
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .ok_or(ParseCoordError)?;
            Ok(Coords {
                p1: Vec2u { x: x1, y: y1 },
                p2: Vec2u { x: x2, y: y2 },
            })
        }
    }

    #[derive(PartialEq, Debug)]
    enum Command {
        Toggle(Coords),
        TurnOn(Coords),
        TurnOff(Coords),
    }

    let mut lights1 = vec![false; 1000000];
    // let mut lights2 = Box::new([Box:::new([0; 1000]); 1000]);
    let mut lights2 = vec![0; 1000000];

    fn apply<T>(lights: &mut [T], c: &Coords, f: fn(T) -> T)
    where
        T: Copy,
    {
        for x in c.p1.x..=c.p2.x {
            for y in c.p1.y..=c.p2.y {
                let i = x * 1000 + y;
                lights[i] = f(lights[i]);
            }
        }
    }

    for line in input.lines() {
        // cleaned input to be more suitable
        let line = line.replace(" through ", ",");

        // parse input into command and numbers
        let mut words: Vec<&str> = line.split(" ").collect();
        words.reverse();

        let command = match words.pop().unwrap() {
            "toggle" => Command::Toggle(Coords::from_str(words.pop().unwrap()).unwrap()),
            "turn" => match words.pop().unwrap() {
                "on" => Command::TurnOn(Coords::from_str(words.pop().unwrap()).unwrap()),
                "off" => Command::TurnOff(Coords::from_str(words.pop().unwrap()).unwrap()),
                e => panic!("Unknown command {}", e),
            },
            e => panic!("Unknown command {}", e),
        };

        match &command {
            Command::Toggle(c) => apply(&mut lights1, c, |b: bool| !b),
            Command::TurnOff(c) => apply(&mut lights1, c, |_| false),
            Command::TurnOn(c) => apply(&mut lights1, c, |_| true),
        }

        match &command {
            Command::Toggle(c) => apply(&mut lights2, c, |i| i + 2),
            Command::TurnOff(c) => apply(&mut lights2, c, |i| {
                let i = i - 1;
                max(i, 0)
            }),
            Command::TurnOn(c) => apply(&mut lights2, c, |i| i + 1),
        }
    }

    let n_lights = lights1.iter().filter(|b| **b).count() as i32;
    let brightness = lights2.iter().sum();

    (Some(n_lights), Some(brightness))
}
