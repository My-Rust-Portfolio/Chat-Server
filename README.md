# Chat-Server

Multithreade chat server using Tokio, PostgreSQL, sqlx and Docker.
NOTE: Telnet needs to be installed to be able to connect to the server.

To build: 
cd rust-chat-server
docker compose up --build

Connect to client:
telnet localhost 9000

