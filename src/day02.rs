use std::num::ParseIntError;

/**
 */
pub fn solve(path: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string(path)?;
    let lines = file.lines();

    let parsed_lines: Vec<Game> = lines
        .map(|l| parse_line(l))
        .filter_map(|res| match res {
            Ok((_, game)) => Some(game),
            _ => unreachable!("invalid input"),
        })
        .collect();

    let s1: u32 = parsed_lines
        .iter()
        .map(|game| {
            if game
                .sets
                .iter()
                .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
            {
                game.game_id
            } else {
                0
            }
        })
        .sum();

    let s2: u32 = parsed_lines
        .iter()
        .map(|game| {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for set in &game.sets {
                red = red.max(set.red);
                green = green.max(set.green);
                blue = blue.max(set.blue);
            }
            red * green * blue
        })
        .sum();

    Ok((s1.to_string(), s2.to_string()))
}

#[derive(Debug)]
struct Set {
    pub(crate) blue: u32,
    pub(crate) red: u32,
    pub(crate) green: u32,
}

#[derive(Debug)]
enum Cube {
    Blue(u32),
    Red(u32),
    Green(u32),
}

#[derive(Debug)]
struct Game {
    pub(crate) game_id: u32,
    pub(crate) sets: Vec<Set>,
}

fn parse_line(input: &str) -> nom::IResult<&str, Game, nom::error::Error<&str>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::map_res;
    use nom::multi::separated_list1;
    use nom::sequence::preceded;
    use nom::sequence::{separated_pair, terminated};

    map_res(
        separated_pair(
            map_res(preceded(tag("Game "), digit1), |s: &str| {
                Ok::<u32, ParseIntError>(s.parse::<u32>()?)
            }),
            tag(": "),
            separated_list1(
                tag("; "),
                map_res(
                    separated_list1(
                        tag(", "),
                        alt((
                            map_res(terminated(digit1, tag(" blue")), |s: &str| {
                                Ok::<Cube, ParseIntError>(Cube::Blue(s.parse::<u32>()?))
                            }),
                            map_res(terminated(digit1, tag(" red")), |s: &str| {
                                Ok::<Cube, ParseIntError>(Cube::Red(s.parse::<u32>()?))
                            }),
                            map_res(terminated(digit1, tag(" green")), |s: &str| {
                                Ok::<Cube, ParseIntError>(Cube::Green(s.parse::<u32>()?))
                            }),
                        )),
                    ),
                    |cubes| {
                        let mut red = 0;
                        let mut green = 0;
                        let mut blue = 0;

                        for cube in cubes {
                            match cube {
                                Cube::Blue(n) => blue += n,
                                Cube::Red(n) => red += n,
                                Cube::Green(n) => green += n,
                            }
                        }

                        Ok::<Set, ParseIntError>(Set { red, green, blue })
                    },
                ),
            ),
        ),
        |(a, b)| {
            Ok::<Game, ParseIntError>(Game {
                game_id: a,
                sets: b,
            })
        },
    )(input)
}

#[cfg(test)]
mod test {
    #[test]
    fn evaluate_1() {
        let expected = 2286;
        let actual = super::solve("testdata/day02.txt").unwrap().1;
        assert_eq!(expected.to_string(), actual);
    }
}
