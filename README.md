# Async tcp echo server

## Usage

window1
```
> cargo run -q                                                                                                                                           [master]
2020-03-22T17:18:07.255683+09:00   INFO async_echo: bind 127.0.0.1:6666
2020-03-22T17:18:14.405723+09:00   INFO async_echo: accept 127.0.0.1:50850
2020-03-22T17:18:16.053616+09:00  TRACE async_echo: read 7 bytes
2020-03-22T17:18:16.053813+09:00  TRACE async_echo: write Hello
```

window2
```
telnet localhost 6666                                                                                                                                       [master]
Trying ::1...
telnet: connect to address ::1: Connection refused
Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
Hello
Hello
^]
```
