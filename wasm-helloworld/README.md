# WASM example

Build docker image:
```
docker buildx build --platform wasi/wasm32 -t binblee/wasm-helloworld:0.0.1 .
docker run --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 binblee/wasm-helloworld:0.0.1
```

Run with wasmedge:

```
docker pull wasmedge/slim:0.11.2
docker run -it --rm -v $PWD:/app wasmedge/slim:0.11.2 wasmedge wasm-helloworld.wasm
```

https://wasmedge.org/book/en/quick_start/use_docker.html