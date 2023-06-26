use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take_while, take_while1},
    character::complete::multispace0,
    combinator::{map, opt, recognize, value},
    multi::{many0, many1},
    sequence::{delimited, pair},
    IResult,
    Parser,
    error::ParseError,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Module,
    Input,
    Output,
    Reg,
    Wire,
    Identifier(&'a str),
    Number(&'a str),
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Caret,
    LT,
    GT,
    EQ,
    NOT_EQ,
    LT_EQ,
    GT_EQ,
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NonBlockAssign,
    BlockAssign,
    Assign,
    Colon,
    At,
    Begin,
    End,
    If,
    Else,
    Always,
    Posedge,
    Negedge,
    EndModule,
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '\'')(input)
}

fn token_identifier(input: &str) -> IResult<&str, Token> {
    let (input, id) = identifier(input)?;
    let token = match id {
        "module" => Token::Module,
        "input" => Token::Input,
        "output" => Token::Output,
        "reg" => Token::Reg,
        "wire" => Token::Wire,
        "begin" => Token::Begin,
        "end" => Token::End,
        "if" => Token::If,
        "else" => Token::Else,
        "always" => Token::Always,
        "posedge" => Token::Posedge,
        "negedge" => Token::Negedge,
        "endmodule" => Token::EndModule,
        "assign" => Token::Assign,
        _ => {
            if is_number(id) {
                Token::Number(id)
            } else {
                Token::Identifier(id)
            }
        }
    };
    Ok((input, token))
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(16) || c == 'b' || c == 'h' || c == 'd' || c == 'o' || c == '\'')
}



fn token_comma(input: &str) -> IResult<&str, Token> {
    map(tag(","), |_| Token::Comma)(input)
}

fn token_semicolon(input: &str) -> IResult<&str, Token> {
    map(tag(";"), |_| Token::Semicolon)(input)
}

fn token_lparen(input: &str) -> IResult<&str, Token> {
    map(tag("("), |_| Token::LParen)(input)
}

fn token_rparen(input: &str) -> IResult<&str, Token> {
    map(tag(")"), |_| Token::RParen)(input)
}

fn token_lbracket(input: &str) -> IResult<&str, Token> {
    map(tag("["), |_| Token::LBracket)(input)
}

fn token_rbracket(input: &str) -> IResult<&str, Token> {
    map(tag("]"), |_| Token::RBracket)(input)
}

fn token_plus(input: &str) -> IResult<&str, Token> {
    map(tag("+"), |_| Token::Plus)(input)
}

fn token_minus(input: &str) -> IResult<&str, Token> {
    map(tag("-"), |_| Token::Minus)(input)
}

fn token_star(input: &str) -> IResult<&str, Token> {
    map(tag("*"), |_| Token::Star)(input)
}

fn token_slash(input: &str) -> IResult<&str, Token> {
    map(tag("/"), |_| Token::Slash)(input)
}

fn token_percent(input: &str) -> IResult<&str, Token> {
    map(tag("%"), |_| Token::Percent)(input)
}

fn token_ampersand(input: &str) -> IResult<&str, Token> {
    map(tag("&"), |_| Token::Ampersand)(input)
}

fn token_pipe(input: &str) -> IResult<&str, Token> {
    map(tag("|"), |_| Token::Pipe)(input)
}

fn token_caret(input: &str) -> IResult<&str, Token> {
    map(tag("^"), |_| Token::Caret)(input)
}

fn token_lt(input: &str) -> IResult<&str, Token> {
    map(tag("<"), |_| Token::LT)(input)
}

fn token_gt(input: &str) -> IResult<&str, Token> {
    map(tag(">"), |_| Token::GT)(input)
}

fn token_eq(input: &str) -> IResult<&str, Token> {
    map(tag("=="), |_| Token::EQ)(input)
}

fn token_not_eq(input: &str) -> IResult<&str, Token> {
    map(tag("!="), |_| Token::NOT_EQ)(input)
}

fn token_lt_eq(input: &str) -> IResult<&str, Token> {
    map(tag("=<"), |_| Token::LT_EQ)(input)
}

fn token_gt_eq(input: &str) -> IResult<&str, Token> {
    map(tag("=>"), |_| Token::GT_EQ)(input)
}

fn token_and(input: &str) -> IResult<&str, Token> {
    map(tag("&&"), |_| Token::AND)(input)
}

fn token_or(input: &str) -> IResult<&str, Token> {
    map(tag("||"), |_| Token::OR)(input)
}

fn token_lshift(input: &str) -> IResult<&str, Token> {
    map(tag("<<"), |_| Token::LSHIFT)(input)
}

fn token_rshift(input: &str) -> IResult<&str, Token> {
    map(tag(">>"), |_| Token::RSHIFT)(input)
}

fn token_nonblcok_assign(input: &str) -> IResult<&str, Token> {
    map(tag("<="), |_| Token::NonBlockAssign)(input)
}

fn token_block_assign(input: &str) -> IResult<&str, Token> {
    map(tag("="), |_| Token::BlockAssign)(input)
}

fn token_colon(input: &str) -> IResult<&str, Token> {
    map(tag(":"), |_| Token::Colon)(input)
}

fn token_at(input: &str) -> IResult<&str, Token> {
    map(tag("@"), |_| Token::At)(input)
}

fn token_begin(input: &str) -> IResult<&str, Token> {
    map(tag("begin"), |_| Token::Begin)(input)
}

fn token_end(input: &str) -> IResult<&str, Token> {
    map(tag("end"), |_| Token::End)(input)
}

fn token_if(input: &str) -> IResult<&str, Token> {
    map(tag("if"), |_| Token::If)(input)
}

fn token_else(input: &str) -> IResult<&str, Token> {
    map(tag("else"), |_| Token::Else)(input)
}

fn token_always(input: &str) -> IResult<&str, Token> {
    map(tag("always"), |_| Token::Always)(input)
}

fn token_posedge(input: &str) -> IResult<&str, Token> {
    map(tag("posedge"), |_| Token::Posedge)(input)
}

fn token_negedge(input: &str) -> IResult<&str, Token> {
    map(tag("negedge"), |_| Token::Negedge)(input)
}

fn whitespace<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Parser<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((
        alt((token_identifier,
        token_comma,
        token_semicolon,
        token_lparen,
        token_rparen,
        token_lbracket,
        token_rbracket,
        token_plus,
        token_minus,
        token_star)),
        alt((token_slash,
        token_percent,
        token_ampersand,
        token_pipe,
        token_caret,
        token_eq,
        token_nonblcok_assign,
        token_block_assign,
        token_lt,
        token_gt,
        token_not_eq,
        token_lt_eq)),
        alt((token_gt_eq,
        token_and,
        token_or,
        token_lshift,
        token_rshift,
        token_colon,
        token_at,
        token_begin,
        token_end,
        token_if,
        token_else,
        token_always,
        token_posedge,
        token_negedge)),
    ))(input)
}

fn tokens(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, tokens) = many0(whitespace(token))(input)?;
    Ok((input, tokens))
}

/*fn main() {
    let input = "module test(input clk, input [7:0] data_in, output [7:0] data_out);
                  reg [7:0] count;
                  always @(posedge clk) begin
                      if (count == 8'hff) begin
                          count <= 8'h00;
                      end else begin
                          count <= count + 1;
                      end
                  end
                  assign data_out = count + data_in;
                  endmodule";
    let result = tokens(input).unwrap().1;
    println!("{:?}", result);
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let input = "module test(input clk, input [7:0] data_in, output [7:0] data_out);
                      reg [7:0] count;
                      always @(posedge clk) begin
                          if (count == 8'hff) begin
                              count <= 8'h00;
                          end else begin
                              count <= count + 1;
                          end
                      end
                      assign data_out = count + data_in;
                      endmodule";
        let expected = vec![
            Token::Module,
            Token::Identifier("test"),
            Token::LParen,
            Token::Input,
            Token::Identifier("clk"),
            Token::Comma,
            Token::Input,
            Token::LBracket,
            Token::Number("7"),
            Token::Colon,
            Token::Number("0"),
            Token::RBracket,
            Token::Identifier("data_in"),
            Token::Comma,
            Token::Output,
            Token::LBracket,
            Token::Number("7"),
            Token::Colon,
            Token::Number("0"),
            Token::RBracket,
            Token::Identifier("data_out"),
            Token::RParen,
            Token::Semicolon,
            Token::Reg,
            Token::LBracket,
            Token::Number("7"),
            Token::Colon,
            Token::Number("0"),
            Token::RBracket,
            Token::Identifier("count"),
            Token::Semicolon,
            Token::Always,
            Token::At,
            Token::LParen,
            Token::Posedge,
            Token::Identifier("clk"),
            Token::RParen,
            Token::Begin,
            Token::If,
            Token::LParen,
            Token::Identifier("count"),
            Token::EQ,
            Token::Number("8'hff"),
            Token::RParen,
            Token::Begin,
            Token::Identifier("count"),
            Token::NonBlockAssign,
            Token::Number("8'h00"),
            Token::Semicolon,
            Token::End,
            Token::Else,
            Token::Begin,
            Token::Identifier("count"),
            Token::NonBlockAssign,
            Token::Identifier("count"),
            Token::Plus,
            Token::Number("1"),
            Token::Semicolon,
            Token::End,
            Token::End,
            Token::Assign,
            Token::Identifier("data_out"),
            Token::BlockAssign,
            Token::Identifier("count"),
            Token::Plus,
            Token::Identifier("data_in"),
            Token::Semicolon,
            Token::EndModule,
        ];
        let result = tokens(input).unwrap().1;
        assert_eq!(result, expected);
    }
}