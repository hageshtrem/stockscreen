# Building
```
protoc \
    --include_imports \
    --include_source_info \
    --proto_path=./emitent-service/proto/ \
    --descriptor_set_out=./gateway/proto.pb \
    ./emitent-service/proto/emitent.proto
```