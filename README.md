# wihd.rs v0.1.1

A War in Hex online server written in Rust.

### Issues

* This server is not recommended and not tested, I probably wont be supporting it compared to the erlang version.
* The client must be altered to append newlines to each string sent of the connection. for exmaple conn.send(GAME_NAME+"\n"), wherever a sent is done.

