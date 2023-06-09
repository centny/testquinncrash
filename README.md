### start server

```.sh
go build -o testserver -v .
./testserver
```

### start test client

- open project by Android Studio
- run app
- click `start`
- then click `close`

### logcat

```.log
2023-06-09 09:00:02.445  6419-6419  quincrash               com.example.quinncrash               D  rustls::client::hs: No cached session for DnsName("test.loc")
2023-06-09 09:00:02.446  6419-6419  quincrash               com.example.quinncrash               D  rustls::client::hs: Not resuming any session
2023-06-09 09:00:02.447  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.455  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.456  6419-6540  quincrash               com.example.quinncrash               D  rustls::client::hs: Using ciphersuite TLS13_AES_128_GCM_SHA256
2023-06-09 09:00:02.456  6419-6540  quincrash               com.example.quinncrash               D  rustls::client::tls13: Not resuming
2023-06-09 09:00:02.457  6419-6540  quincrash               com.example.quinncrash               D  rustls::client::tls13: TLS1.3 encrypted extensions: [TransportParameters([67, 3, 6, 5, 186, 149, 185, 92, 150, 5, 4, 128, 8, 0, 0, 6, 4, 128, 8, 0, 0, 7, 4, 128, 8, 0, 0, 4, 4, 128, 12, 0, 0, 8, 2, 64, 100, 9, 2, 64, 100, 1, 4, 128, 0, 117, 48, 3, 2, 69, 172, 11, 1, 26, 12, 0, 2, 16, 114, 177, 167, 219, 145, 161, 240, 240, 213, 232, 64, 189, 162, 191, 136, 131, 0, 20, 42, 143, 201, 213, 198, 210, 80, 93, 53, 43, 96, 4, 237, 62, 38, 71, 155, 59, 0, 132, 14, 1, 4, 15, 4, 238, 57, 10, 148, 32, 2, 68, 176])]
2023-06-09 09:00:02.458  6419-6540  quincrash               com.example.quinncrash               D  rustls::client::hs: ALPN protocol is None
2023-06-09 09:00:02.460  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.461  6419-6540  quincrash               com.example.quinncrash               I  quinncrash: read_loop is starting
2023-06-09 09:00:02.465  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.467  6419-6539  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:02.467  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.471  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.499  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:02.528  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:03.572  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:03.575  6419-6539  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:03.577  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:04.494  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:04.496  6419-6539  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:05.518  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:05.520  6419-6539  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:06.546  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:06.549  6419-6539  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:07.529  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:07.531  6419-6540  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:08.479  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:08.480  6419-6540  quincrash               com.example.quinncrash               I  quinncrash: receive 8 bytes
2023-06-09 09:00:08.795  6419-6419  quincrash               com.example.quinncrash               I  quinncrash: conn is shutdown
2023-06-09 09:00:08.796  6419-6539  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
2023-06-09 09:00:08.799  6419-6540  quincrash               com.example.quinncrash               D  quinn::connection: drive; id=0
```
