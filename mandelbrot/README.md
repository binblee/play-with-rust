# Mandelbort

single thread:
```
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20  4.42s user 0.01s system 89% cpu 4.964 total
time target/debug/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
target/debug/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20  117.53s user 0.17s system 99% cpu 1:57.90 total
```

multiple threads:
```
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20  4.71s user 0.02s system 337% cpu 1.398 total
```
