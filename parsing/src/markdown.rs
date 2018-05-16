/*
Italics         	    *italics* or _italics_
Bold	                **bold**
Underline italics	    __*underline italics*__
Underline bold  	    __**underline bold**__
Bold Italics	        ***bold italics***
underline bold italics	__***underline bold italics***__
Underline	            __underline__
Strikethrough	         ~~Strikethrough~~
*/

use nom::anychar;
use nom::types::CompleteStr;

// TODO: Support nested styles
#[derive(Debug, PartialEq)]
pub enum Style {
    Text(String),
    Code(String),
    Bold(String),
    Italic(String),
    BoldItalics(String),
    Underline(String),
    UnderlineBold(String),
    UnderlineItalics(String),
    UnderlineBoldItalics(String),
    Strikethrough(String),
}

use self::Style::*;

named!(code<CompleteStr, Style>,
    map!(
        delimited!(tag!("`"), take_until!("`"), tag!("`")),
        |text| Code(text.0.to_owned())
    )
);

named!(star_italic<CompleteStr, Style>,
    map!(
        delimited!(tag!("*"), take_until!("*"), tag!("*")),
        |text| Italic(text.0.to_owned())
    )
);

named!(underscore_italic<CompleteStr, Style>,
    map!(
        delimited!(tag!("_"), take_until!("_"), tag!("_")),
        |text| Italic(text.0.to_owned())
    )
);

named!(italic<CompleteStr, Style>,
    alt_complete!(star_italic | underscore_italic)
);

named!(bold<CompleteStr, Style>,
    map!(
        delimited!(tag!("**"), take_until!("**"), tag!("**")),
        |text| Bold(text.0.to_owned())
    )
);

named!(underline<CompleteStr, Style>,
    map!(
        delimited!(tag!("__"), take_until!("__"), tag!("__")),
        |text| Underline(text.0.to_owned())
    )
);

named!(strikethrough<CompleteStr, Style>,
    map!(
        delimited!(tag!("~~"), take_until!("~~"), tag!("~~")),
        |text| Strikethrough(text.0.to_owned())
    )
);

named!(styled<CompleteStr, Style>, alt!(bold | underline | italic | strikethrough | code));

named!(maybe_style<CompleteStr, Style>,
    alt!(
        styled |
        map!(
            many_till!(call!(anychar), alt!(recognize!(peek!(styled)) | eof!())),
            |chars| Text(chars.0.iter().collect())
        )
    )
);

named!(text<CompleteStr, Vec<Style>>,
    many0!(maybe_style)
);

pub fn parse_msg(msg: &str) -> Option<Vec<Style>> {
    match text(CompleteStr(msg)) {
        Ok((_, msg)) => Some(msg),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! style {
        ($style:ident($text:tt)) => {
            $style($text.to_owned())
        };
    }

    macro_rules! parse {
        ($parser:ident($str:tt)) => {
            $parser(CompleteStr($str)).unwrap().1
        };
    }

    #[test]
    fn italic_underline() {
        assert_eq!(parse!(italic("_italic_")), style!(Italic("italic")));
    }

    #[test]
    fn italic_star() {
        assert_eq!(parse!(italic("*italic*")), style!(Italic("italic")));
    }

    #[test]
    fn bold_test() {
        assert_eq!(parse!(bold("**bold**")), style!(Bold("bold")));
    }

    #[test]
    fn underline_test() {
        assert_eq!(
            parse!(underline("__underline__")),
            style!(Underline("underline")),
        );
    }

    #[test]
    fn strikethrough_test() {
        assert_eq!(
            parse!(strikethrough("~~strikethrough~~")),
            style!(Strikethrough("strikethrough")),
        );
    }

    #[test]
    fn variety() {
        assert_eq!(
            parse!(text(
                "_italic_ **bold** *italic* __underline__ ~~strikethrough~~"
            )),
            [
                style!(Italic("italic")),
                style!(Text(" ")),
                style!(Bold("bold")),
                style!(Text(" ")),
                style!(Italic("italic")),
                style!(Text(" ")),
                style!(Underline("underline")),
                style!(Text(" ")),
                style!(Strikethrough("strikethrough")),
            ]
        );
    }
}
