pub type Cursor<'a> = std::iter::Peekable<std::str::Chars<'a>>;

pub fn int(input: &mut Cursor) -> Option<usize> {
    let str: String = input.clone().take_while(|c| c.is_ascii_digit()).collect();

    match str.parse::<usize>() {
        Ok(num) => {
            for _ in 0..str.len() {
                input.next();
            }
            Some(num)
        }
        Err(_) => None,
    }
}

pub fn expect(input: &mut Cursor, expected: &str) -> Option<()> {
    if expected == input.clone().take(expected.len()).collect::<String>() {
        for _ in 0..expected.len() {
            input.next();
        }
        Some(())
    } else {
        None
    }
}

pub fn list<T>(input: &mut Cursor, parser: fn(input: &mut Cursor) -> Option<T>) -> Vec<T> {
    let mut list: Vec<T> = vec![];

    loop {
        let mut cursor: Cursor = input.clone();
        if let Some(item) = parser(&mut cursor) {
            list.push(item);
            *input = cursor;
        } else {
            break;
        }
    }

    list
}

pub fn optional<T>(input: &mut Cursor, parser: fn(input: &mut Cursor) -> Option<T>) -> Option<T> {
    let mut cursor = input.clone();

    let res = parser(&mut cursor);
    if res.is_some() {
        *input = cursor;
    }
    res
}
