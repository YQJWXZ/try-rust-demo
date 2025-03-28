#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespFrame;

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = RespSet::new([
            RespArray::new([(1234).into(), true.into()]).into(),
            BulkString::new("world".to_string()).into(),
        ]).into();
        assert_eq!(frame.encode(), b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n")
    }
}
