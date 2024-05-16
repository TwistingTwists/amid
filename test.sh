grpcurl -plaintext -d '{"name": "Alice"}' -proto proto/hello.proto -import-path proto/ 127.0.0.1:50051 hello.Greeter/SayHello
