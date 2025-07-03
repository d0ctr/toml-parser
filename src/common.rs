pub fn skip_whitespaces<T: Iterator<Item = char>>(iter: &mut T) -> Option<(usize,char)> {
    let mut pos = 0;
    while let Some(c) = iter.next() {
        if !c.is_whitespace() {
            return Some((pos,c));
        }
        pos += 1
    }

    None
}