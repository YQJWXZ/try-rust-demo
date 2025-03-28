#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespFrame;

    #[test]
    fn test_map_encode() {
        let mut map = RespMap::new();
        map.insert("hello".to_string(), BulkString::new("world".to_string()).into());
        map.insert("foo".to_string(), (-123456.789).into());

        let frame: RespFrame = map.into();

        assert_eq!(
            // String::from_utf8_lossy(&frame.encode()),
            frame.encode(),
            b"%2\r\n+foo\r\n,-123456.789\r\n+hello\r\n$5\r\nworld\r\n"
        )
    }

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = RespSet::new([
            RespArray::new([(1234).into(), true.into()]).into(),
            BulkString::new("world".to_string()).into(),
        ]).into();
        assert_eq!(frame.encode(), b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n")
    }
}
