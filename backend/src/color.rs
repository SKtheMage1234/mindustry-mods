#![allow(dead_code)]
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while_m_n},
    character::complete::{alpha1, char, none_of},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

type PResult<'a, E> = IResult<&'a str, E>;

/// Token representing color of the following text.
#[derive(Debug, PartialEq)]
pub enum ColorTag<'a> {
    /// Empty square brackets, picking the color before the current one.
    LastColor,

    /// Parsed hex-rgb(a) string.
    HexColor { r: u8, g: u8, b: u8, a: u8 },

    /// Parsed named string.
    Named(&'a str),
}

impl<'a> ColorTag<'a> {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::HexColor { r, g, b, a }
    }

    pub fn named(name: &'a str) -> Self {
        name.into()
    }
}

impl<'a> From<&'a str> for ColorTag<'a> {
    fn from(x: &'a str) -> Self {
        Self::Named(x)
    }
}

impl From<[u8; 4]> for ColorTag<'_> {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Self::HexColor { r, g, b, a }
    }
}

impl From<[u8; 3]> for ColorTag<'_> {
    fn from([r, g, b]: [u8; 3]) -> Self {
        let a = 0;
        Self::HexColor { r, g, b, a }
    }
}

#[derive(Debug, PartialEq)]
pub struct Text<'a> {
    pub color: ColorTag<'a>,
    pub text: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(text: &'a str) -> Self {
        let color = ColorTag::LastColor;
        Self { text, color }
    }

    pub fn with_color<T: Into<ColorTag<'a>>>(self, color: T) -> Self {
        let color = color.into();
        Self { color, ..self }
    }
}

#[derive(Debug, PartialEq)]
pub struct Markup<'a> {
    /// Default color.
    color: ColorTag<'a>,

    texts: Vec<Text<'a>>,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> PResult<u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> PResult<ColorTag> {
    let (input, _) = tag("#")(input)?;
    let (input, (r, g, b)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
    let (input, a) = opt(hex_primary)(input)?;
    let a = a.unwrap_or(0);
    Ok((input, ColorTag::new(r, g, b, a)))
}

fn named_color(input: &str) -> PResult<ColorTag> {
    let (input, color) = alpha1(input)?;
    Ok((input, ColorTag::Named(color)))
}

fn last_color(input: &str) -> PResult<ColorTag> {
    Ok((input, ColorTag::LastColor))
}

fn color(input: &str) -> PResult<ColorTag> {
    let color_parser = alt((hex_color, named_color, last_color));
    Ok(delimited(char('['), color_parser, char(']'))(input)?)
}

fn text_color(input: &str) -> PResult<Text<'_>> {
    let (input, color) = opt(color)(input)?;
    let (input, text) = is_not("[")(input)?;
    let color = color.unwrap_or(ColorTag::LastColor);
    Ok((input, Text { color, text }))
}

fn text_colors(input: &str) -> PResult<Vec<Text<'_>>> {
    Ok(many0(text_color)(input)?)
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok(("", ColorTag::new(47, 20, 223, 0)))
    );
}

#[test]
fn parse_color_lower_case() {
    assert_eq!(
        hex_color("#2f14df"),
        Ok(("", ColorTag::new(47, 20, 223, 0)))
    );
}

#[test]
fn parse_color_mixed_case() {
    assert_eq!(
        hex_color("#2F14df"),
        Ok(("", ColorTag::new(47, 20, 223, 0)))
    );
}

#[test]
fn parse_named_color() {
    assert_eq!(named_color("red"), Ok(("", ColorTag::Named("red"))));
}

#[test]
fn parse_color_alpha() {
    assert_eq!(
        hex_color("#2F14DF05"),
        Ok(("", ColorTag::new(47, 20, 223, 5)))
    );
}

#[test]
fn parse_one_colored_text() {
    assert_eq!(
        text_color("[#01020304]text"),
        Ok(("", Text::new("text").with_color([1, 2, 3, 4])))
    );
}

#[test]
fn parse_one_named_color_text() {
    assert_eq!(
        text_color("[red]text"),
        Ok(("", Text::new("text").with_color("red")))
    );
}

#[test]
fn parse_many_colored_text() {
    assert_eq!(
        text_colors("[#01020304]texta[#04030201]textb"),
        Ok((
            "",
            vec![
                Text::new("texta").with_color([1, 2, 3, 4]),
                Text::new("textb").with_color([4, 3, 2, 1]),
            ]
        ))
    );
}

#[test]
fn parse_no_leading_color() {
    assert_eq!(
        text_colors("texta[#04030201]textb"),
        Ok((
            "",
            vec![
                Text::new("texta"),
                Text::new("textb").with_color([4, 3, 2, 1]),
            ]
        ))
    );
}

#[test]
fn parse_no_leading_named_color() {
    assert_eq!(
        text_colors("texta[red]textb"),
        Ok((
            "",
            vec![Text::new("texta"), Text::new("textb").with_color("red"),]
        ))
    );
}

#[test]
fn parse_last_color() {
    assert_eq!(
        text_colors("[#010203]texta[]textb"),
        Ok((
            "",
            vec![Text::new("texta").with_color([1, 2, 3]), Text::new("textb"),]
        ))
    );
}

#[test]
fn parse_last_color_alone() {
    assert_eq!(color("[]"), Ok(("", ColorTag::LastColor)));
}

#[test]
fn parse_last_color_named() {
    assert_eq!(
        text_colors("[red]texta[]textb"),
        Ok((
            "",
            vec![Text::new("texta").with_color("red"), Text::new("textb"),]
        ))
    );
}

// #[test]
fn parse_escaped() {
    assert_eq!(
        text_colors("[[red]texta[]textb"),
        Ok((
            "",
            vec![Text::new("texta").with_color("red"), Text::new("textb"),]
        ))
    );
}
