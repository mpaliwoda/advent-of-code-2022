use std::str::FromStr;

pub struct Range {
    start: usize,
    finish: usize,
}

impl Range {
    pub fn either_fully_contains_another(left: &Range, right: &Range) -> bool {
        let (lower, upper) = Range::sort_pair(left, right);
        return lower.start <= upper.start && lower.finish >= upper.finish;
    }

    pub fn either_partially_contains_another(left: &Range, right: &Range) -> bool {
        let (lower, upper) = Range::sort_pair(left, right);
        return lower.finish >= upper.start;
    }

    fn sort_pair<'a>(left: &'a Range, right: &'a Range) -> (&'a Range, &'a Range) {
        if left.start < right.start || left.start == right.start && left.finish >= right.finish {
            return (left, right);
        }

        return (right, left);
    }
}

#[derive(Debug, Clone)]
pub struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, ParseRangeError> {
        match s.split_once("-") {
            Some((start, finish)) => match (start.parse::<usize>(), finish.parse::<usize>()) {
                (Ok(start), Ok(finish)) => Ok(Range { start, finish }),
                _ => Err(ParseRangeError {}),
            },
            None => Err(ParseRangeError {}),
        }
    }
}
