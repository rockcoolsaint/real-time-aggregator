### Setup instructions

- Clone the application
- navigate to the root directory and run the application as follows:
- - cargo run

##### Test the application

Make sure to have installed and launched a redis server locally

On another terminal execute the following commands:
```
- redis-cli publish inputA "hello"
- redis-cli publish inputB "world"
- redis-cli publish inputC "real-time aggregator"
```

Take a look at the application server and the logs should look something similar to below:
```
Received: 'hello' on inputA
Aggregated Result: inputA: 1
Received: 'world' on inputB
Aggregated Result: inputB: 1, inputA: 1
Received: 'rust' on inputC
Aggregated Result: inputB: 1, inputA: 1, inputC: 1
Received: 'rust' on inputC
Aggregated Result: inputB: 1, inputA: 1, inputC: 2
```

#### To run the tests
Execute the following command:
`cargo test -- --nocapture

