// -Command
// SET key val  "set hello world" =>  "*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
// GET key      "get hello"       =>  "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
// HSET key field val  "hset map hello world" => "*4\r\n$4\r\nhset\r\n$3\r\nmap\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
// HGET key field      "hget map hello" => "*3\r\n$4\r\nhget\r\n$3\r\nmap\r\n$5\r\nhello\r\n"
// HGETALL key          "hgetall hello" => "*2\r\n$7\r\nhgetall\r\n$5\r\nhello\r\n"

pub enum Command {
    Get,
    Set,
    Hget,
    Hset,
}
