<div align="center">

  <h1><code>WRAPPED-SM-FOR-WEB</code></h1>

  <strong>
  A Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a> 
  and <a href="https://github.com/rustwasm/wasm-pack-template.git">wasm-pack-template</a> to wrap 
  <a href="https://github.com/bytesboy/Yet-Another-Rust-Implementation-Of-SM-Algorithms.git">
  Yarism</a>.
  </strong>


</div>

## About

This project is designed to compile
<a href="https://github.com/bytesboy/Yet-Another-Rust-Implementation-Of-SM-Algorithms.git"> Yarism </a>
into WebAssembly.


## 🚴 Usage


### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

> [`wasm-pack build command usage`](https://rustwasm.github.io/docs/wasm-pack/commands/build.html) 

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Relative Projects

* [`wasm-pack`](https://github.com/rustwasm/wasm-pack)
* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
* [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
