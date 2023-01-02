pub type ParserResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type FallibleParser<T> = fn(input: &mut Cursor) -> ParserResult<T>;
pub type Cursor<'a> = std::iter::Peekable<std::str::Chars<'a>>;

struct ParserError {
    message: String,
}

impl std::fmt::Debug for ParserError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.message)
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(&self.message)
    }
}

impl std::error::Error for ParserError {}


pub fn singed_int(cursor: &mut Cursor) -> ParserResult<isize> {
    let mut possible_cursor = cursor.clone();
    let mut str = String::new();

    if let Some(c) = possible_cursor.peek() {
        if *c == '-' {
            str.push('-');
            possible_cursor.next();
        }
    }

    for c in possible_cursor.clone().take_while(|c| c.is_ascii_digit()) {
        str.push(c);
    }

    match str.parse::<isize>() {
        Ok(num) => {
            for _ in 0..str.len() {
                cursor.next();
            }
            Ok(num)
        }
        Err(_) => {
            let message = if let Some(c) = possible_cursor.peek() {
                format!("Expect digit, found {c}")
            } else {
                "Unexpected end of input".to_string()
            };
            Err(Box::new(ParserError { message }))
        },
    }
}

pub fn int(cursor: &mut Cursor) -> ParserResult<usize> {
    let str: String = cursor.clone().take_while(|c| c.is_ascii_digit()).collect();

    match str.parse::<usize>() {
        Ok(num) => {
            for _ in 0..str.len() {
                cursor.next();
            }
            Ok(num)
        }
        Err(_) => {
            let message = if let Some(c) =  cursor.peek() {
                format!("Expect digit, found {c}")
            } else {
                "Unexpected end of input".to_string()
            };
            Err(Box::new(ParserError { message }))
        },
    }
}

pub fn expect(cursor: &mut Cursor, expected: &str) -> ParserResult<()> {
    let found = cursor.clone().take(expected.len()).collect::<String>();
    if expected == found {
        for _ in 0..expected.len() {
            cursor.next();
        }
        Ok(())
    } else {
        let message = format!("expected \"{expected}\" found \"{found}\"",);
        Err(Box::new(ParserError { message }))
    }
}

pub fn list<T>(input: &mut Cursor, parser: FallibleParser<T>) -> Vec<T> {
    let mut list: Vec<T> = vec![];

    loop {
        let mut cursor: Cursor = input.clone();
        if let Ok(item) = parser(&mut cursor) {
            list.push(item);
            *input = cursor;
        } else {
            break;
        }
    }

    list
}

pub fn optional<T>(input: &mut Cursor, parser: FallibleParser<T>) -> ParserResult<T> {
    let mut cursor = input.clone();

    let res = parser(&mut cursor);
    if res.is_ok() {
        *input = cursor;
    }
    res
}
