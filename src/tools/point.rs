use std::ops::{Add, AddAssign};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const DOWN: Point = Point { x: 0, y: 1 };
    pub const UP: Point = Point { x: 0, y: -1 };

    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    /// Parses a Point neperated by a seperator
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Point::parse_seperated("4,-5", ",").unwrap(), Point::new(4, -5));
    /// ```
    pub fn parse_seperated(input: &str, seperator: &str) -> Result<Point, String> {
        let Some((x, y)) = input.split_once(seperator) else {
            return Err(format!("{}: seperator [{}] not found", input, seperator));
        };

        Ok(Point {
            x: x.parse::<isize>().map_err(|e| format!("{}: {}", x, e))?,
            y: y.parse::<isize>().map_err(|e| format!("{}: {}", y, e))?,
        })
    }

    // TODO test
    pub fn is_within(&self, lower: &Point, upper: &Point) -> bool {
        (self.x >= lower.x && self.y >= lower.y && self.x <= upper.x && self.y <= upper.y)
            || (self.x >= upper.x && self.y >= upper.y && self.x <= lower.x && self.y <= lower.y)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("4,-5" , ",", Point::new(4,-5))]
    #[case("4 -5" , " ", Point::new(4,-5))]
    #[case("4--5" , "-", Point::new(4,-5))]
    #[case("-4,-5" , ",", Point::new(-4,-5))]
    #[case("0,0", ",", Point::new(0, 0))]
    #[case("1,1", ",", Point::new(1, 1))]
    #[case("1---1", "---", Point::new(1, 1))]
    fn it_can_parse_sepearted(#[case] input: &str, #[case] seperator: &str, #[case] result: Point) {
        assert_eq!(Point::parse_seperated(input, seperator).unwrap(), result)
    }

    #[rstest]
    #[case("4,-5", " ", "4,-5: seperator [ ] not found")]
    #[case("4,-a5", ",", "-a5: invalid digit found in string")]
    #[case("b4,-a5", ",", "b4: invalid digit found in string")]
    fn it_returns_an(#[case] input: &str, #[case] seperator: &str, #[case] result: String) {
        assert_eq!(Point::parse_seperated(input, seperator), Err(result.into()))
    }
}
