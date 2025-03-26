use crate::{ cmd::CommandError, HGet, HGetAll, HSet, RespMap };

use super::{ extract_args, validate_command, CommandExceutor, RespArray, RespFrame, RESP_OK };

impl CommandExceutor for HGet {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        match backend.hget(&self.key, &self.field) {
            Some(val) => val,
            None => RespFrame::Null(crate::RespNull),
        }
    }
}

impl CommandExceutor for HSet {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        backend.hset(self.key, self.field, self.value);
        RESP_OK.clone()
    }
}

impl CommandExceutor for HGetAll {
    fn execute(self, backend: &crate::Backend) -> RespFrame {
        let hmap = backend.hmap.get(&self.key);
        match hmap {
            Some(hmap) => {
                let mut map = RespMap::new();
                for v in hmap.iter() {
                    let key = v.key().to_owned();
                    map.insert(key, v.value().clone());
                }
                map.into()
            }
            None => RespArray::new([]).into(),
        }
    }
}

impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) =>
                Ok(HGet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key or field".to_string())),
        }
    }
}

impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) =>
                Ok(HSet {
                    key: String::from_utf8(key.0)?,
                    field: String::from_utf8(field.0)?,
                    val: value,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key, field or value".to_string())),
        }
    }
}

impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;

        let mut args = extract_args(value, 1)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) =>
                Ok(HGetAll {
                    key: String::from_utf8(key.0)?,
                }),

            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RespDecode;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_hget_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let result: HGet = frame.try_into()?;

        assert_eq!(result.key, "map");
        assert_eq!(result.field, "hello");
        Ok(())
    }

    #[test]
    fn test_hset_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n");

        let frame = RespArray::decode(&mut buf)?;

        let result: HSet = frame.try_into()?;
        assert_eq!(result.key, "map");
        assert_eq!(result.field, "hello");
        assert_eq!(result.val, RespFrame::BulkString(b"world".into()));
        Ok(())
    }

    #[test]
    fn test_hgetall_from_resp_array() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$7\r\nhgetall\r\n$3\r\nmap\r\n");

        let frame = RespArray::decode(&mut buf)?;
        let result: HGetAll = frame.try_into()?;

        assert_eq!(result.key, "map");
        Ok(())
    }

    #[test]
    fn test_hset_hget_hgetall_commands() -> Result<()> {
        // Test HSet
        let backend = crate::Backend::new();
        let cmd = HSet {
            key: "map".to_string(),
            field: "hello".to_string(),
            val: RespFrame::BulkString(b"world".into()),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RESP_OK.clone());

        // HSet, Then Test HGet and HGetAll
        let cmd = HSet {
            key: "map".to_string(),
            field: "hello".to_string(),
            val: RespFrame::BulkString(b"world".into()),
        };
        cmd.execute(&backend);
        let cmd = HGet {
            key: "map".to_string(),
            field: "hello".to_string(),
        };
        let result = cmd.execute(&backend);
        assert_eq!(result, RespFrame::BulkString(b"world".into()));

        let cmd = HGetAll {
            key: "map".to_string(),
        };
        let result = cmd.execute(&backend);
        let mut expected = RespMap::new();
        expected.insert("hello".to_string(), RespFrame::BulkString(b"world".into()));

        assert_eq!(result, expected.into());

        Ok(())
    }
}
