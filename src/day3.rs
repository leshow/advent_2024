use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    combinator::{map, value},
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone)]
enum Either<A, B> {
    Left(A),
    Right(B),
}

pub fn run(input: &str) -> u32 {
    let input = input.as_bytes();
    let (_, sum) = parse_one(input).unwrap();
    sum
}

fn parse_mul(input: &[u8]) -> IResult<&[u8], u32> {
    map(
        delimited(
            tag("mul("),
            separated_pair(complete::u32, tag(","), complete::u32),
            tag(")"),
        ),
        |(a, b)| a * b,
    )(input)
}

fn parse_one(input: &[u8]) -> IResult<&[u8], u32> {
    map(many1(alt((parse_mul, value(0, take(1u8))))), |v| {
        v.into_iter().sum()
    })(input)
}

fn parse_two(input: &[u8]) -> IResult<&[u8], Either<u32, bool>> {
    alt((
        map(parse_mul, Either::Left),
        value(Either::Right(true), tag("do()")),
        value(Either::Right(false), tag("don't()")),
        value(Either::Left(0), take(1u8)),
    ))(input)
}

pub fn run_2(input: &str) -> u32 {
    let mut input = input.as_bytes();
    let mut acc = 0;
    let mut doit = true;

    while let Ok((rest, result)) = parse_two(input) {
        input = rest;
        match result {
            Either::Left(n) if doit => acc += n,
            Either::Right(d) => doit = d,
            _ => {}
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../data/day3/a.txt")), 156_388_521);
    }
    #[test]
    fn test_2() {
        assert_eq!(run_2(include_str!("../data/day3/a.txt")), 75920122);
    }
}
