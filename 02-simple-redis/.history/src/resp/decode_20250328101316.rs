#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use bytes::{ BufMut, BytesMut };

    #[test]

    #[test]
    fn test_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");
        let frame = RespArray::decode(&mut buf)?;
        let array = RespArray::new([b"set".into(), b"hello".into()]);
        assert_eq!(frame, array);

        Ok(())
    }

    #[test]
    fn test_map_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"%2\r\n+hello\r\n$5\r\nworld\r\n+foo\r\n$3\r\nbar\r\n");
        let frame = RespMap::decode(&mut buf)?;

        let mut map = RespMap::new();
        map.insert("hello".to_string(), b"world".into());
        map.insert("foo".to_string(), b"bar".into());
        assert_eq!(frame, map);

        Ok(())
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");
        let frame = RespNullArray::decode(&mut buf)?;
        assert_eq!(frame, RespNullArray);

        Ok(())
    }

    #[test]
    fn test_set_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"~2\r\n$3\r\nset\r\n$5\r\nhello\r\n");
        let frame = RespSet::decode(&mut buf)?;
        let set = RespSet::new([b"set".into(), b"hello".into()]);
        assert_eq!(frame, set);
        Ok(())
    }

    #[test]
    fn test_calc_array_length() -> Result<()> {
        let buf = b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n";
        let (end, len) = parse_length(buf, "*")?;
        let total_len = calc_total_length(buf, end, len, "*")?;
        assert_eq!(total_len, buf.len());

        let buf = b"*2\r\n$3\r\nset\r\n";
        let (end, len) = parse_length(buf, "*")?;
        let ret = calc_total_length(buf, end, len, "*");
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);
        Ok(())
    }
}
