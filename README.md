# Web Server in Rust

## Part 1 (Single thread server)

- Listening to the TCP Connection
- Writing a Response
- Returning HTML
- Validating the Request
- Refactoring

## Part 2 (Multi thread server)

- Simulating a Slow Request
- Using a Thread Pool
- Thread for Each Request
- Finite Number of Threads
- Compiler Driven Development
- Validating the Number of Threads
- Creating Space to Store the Threads
- Worker Struct
- Sending Requests via Channels
- Implementing the execute Method

## Part 3 (Graceful Shutdown and cleanup)

- Implementing Drop Trait on ThreadPool
- Signaling to the Threads

#### Commands employed

- cargo new server
- cd server
- cargo run
- touch index.html
- touch 404.html
