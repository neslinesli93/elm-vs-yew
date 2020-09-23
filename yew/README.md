# Yew

This project is bootstrapped with [Yew wasm pack template](https://github.com/yewstack/yew-wasm-pack-template).

## About

TODO

## Usage

First of all, install the required dependencies

```
yarn
```

### Serve locally

Launch the app in dev mode and open [http://localhost:8000](http://localhost:8000)

```
yarn run start:dev
```

If you want to have a peek at the production build, use this instead and open [http://localhost:5000](http://localhost:5000)

```
yarn run start:prod
```

### Build

Create the production build inside `dist/` folder

```
yarn run build
```

Create the production build with a custom `publicPath`:

```
YEW_PUBLIC_PATH="/yew/" yarn run build
```

## Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
