#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespFrame;

    #[test]
    fn test_boolean_encode() {
        let frame: RespFrame = true.into();
        assert_eq!(frame.encode(), b"#t\r\n");

        let frame: RespFrame = false.into();
        assert_eq!(frame.encode(), b"#f\r\n");
    }

    #[test]
    fn test_double_encode() {
        let frame: RespFrame = (123.456).into();
        assert_eq!(frame.encode(), b",+123.456\r\n");

        let frame: RespFrame = (-123.456).into();
        assert_eq!(frame.encode(), b",-123.456\r\n");

        let frame: RespFrame = (1.23456e8).into();
        assert_eq!(frame.encode(), b",+1.23456e8\r\n");

        let frame: RespFrame = (-1.23456e-9).into();
        assert_eq!(frame.encode(), b",-1.23456e-9\r\n");
    }

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
