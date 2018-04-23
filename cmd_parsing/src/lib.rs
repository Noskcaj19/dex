#[macro_use]
extern crate nom;
use nom::{digit, rest_s};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Endpoint<'a> {
    Fixed(usize),
    Moment(usize),
    Search(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Range<'a> {
    Single(Endpoint<'a>),
    DoubledEnded(Endpoint<'a>, Endpoint<'a>),
    PastToPresent(Endpoint<'a>),
}

named!(pub number(&str) -> usize, map_res!(digit, FromStr::from_str));

named!(fixed(&str) -> Endpoint,
    do_parse!(
        number: number >>
        (Endpoint::Fixed(number))
    )
);

named!(moment(&str) -> Endpoint,
    do_parse!(
        char!('#') >>
        moments: map_res!(digit, FromStr::from_str) >>
        (Endpoint::Moment(moments))
    )
);

named!(search(&str) -> Endpoint,
    do_parse!(
        query: delimited!(
            char!('/'),
            take_until!("/"),
            char!('/')
        ) >>
        (Endpoint::Search(query))
    )
);

named!(pub endpoint(&str) -> Endpoint, alt!(fixed | moment | search));

named!(single(&str) -> Range, do_parse!(
    endpoint: endpoint >>
    (Range::Single(endpoint))
));

named!(double_ended(&str) -> Range, do_parse!(
    left_endpoint: endpoint >>
    char!(',') >>
    right_endpoint: endpoint >>
    (Range::DoubledEnded(left_endpoint, right_endpoint))
));

named!(past_to_present(&str) -> Range, do_parse!(
    endpoint: endpoint >>
    char!(',') >>
    (Range::PastToPresent(endpoint))
));

named!(pub range(&str) -> Range, alt!(double_ended | past_to_present | single));

named!(
    pub command(&str) -> (Option<Range>, &str),
    tuple!(opt!(call!(range)), call!(rest_s))
);

#[cfg(test)]
mod tests {
    use super::command;
    use super::Endpoint::*;
    use super::Range::*;

    #[test]
    fn cmd_no_range() {
        let result = command("d").unwrap().1;

        assert_eq!(result, (None, "d"));
    }

    #[test]
    fn past_to_present_cmd() {
        let result = command("5,d").unwrap().1;

        assert_eq!(result, (Some(PastToPresent(Fixed(5))), "d"));
    }

    #[test]
    fn double_ended_cmd() {
        let result = command("10,5d").unwrap().1;

        assert_eq!(result, (Some(DoubledEnded(Fixed(10), Fixed(5))), "d"));
    }

    #[test]
    fn past_to_present_moment_cmd() {
        let result = command("#5,d").unwrap().1;

        assert_eq!(result, (Some(PastToPresent(Moment(5))), "d"));
    }

    #[test]
    fn invalid_moment() {
        let result = command(",#5d").unwrap().1;

        assert_eq!(result, (None, ",#5d"));
    }

    #[test]
    fn search_cmd() {
        let result = command("/foo/d").unwrap().1;

        assert_eq!(result, (Some(Single(Search("foo"))), "d"));
    }

    #[test]
    fn search_to_present_cmd() {
        let result = command("/bar/,d").unwrap().1;

        assert_eq!(result, (Some(PastToPresent(Search("bar"))), "d"));
    }

    #[test]
    fn double_ended_search_cmd() {
        let result = command("/foo/,/bar/d").unwrap().1;

        assert_eq!(
            result,
            (Some(DoubledEnded(Search("foo"), Search("bar"))), "d")
        );
    }

    #[test]
    fn no_range_cmd() {
        let result = command("d foo bar").unwrap().1;

        assert_eq!(result, (None, "d foo bar"));
    }

    #[test]
    fn no_range_cmd_with_slashes() {
        let result = command("d b/ar/").unwrap().1;

        assert_eq!(result, (None, "d b/ar/"));
    }
}
