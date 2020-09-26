<p align="center">
  <h2 align="center">
    Elm vs Yew
    <br>
  </h2>
  <p align="center">
    <a href="https://tommasopifferi.com/experiments/elm-vs-yew/">Demo</a> &bull;
    <a href="https://tommasopifferi.com/posts/elm-vs-yew/">Article</a>
  </p>
</p>

<br>

## About

This is a demo showcasing the rendering performance of two frontend frameworks, [Elm](https://elm-lang.org/) and [Yew](https://yew.rs/).

You can either run the whole demo, or run each single app separately. Please note that in order to run the Yew app, you need a browser that supports WASM.

## Usage

First of all, install the required dependencies

```
yarn
cd elm && yarn && cd -
cd yew && yarn && cd -
```

For the yew app, you need `wasm-pack` and you need to add `wasm32-unknown-unknown` to your rust toolchain:

```
rustup target add wasm32-unknown-unknown
```

### Serve locally

Bundle the two apps and serve them on a single page, available at [http://localhost:10000](http://localhost:10000)

```
./make.sh serve
```

### Build

Create the production build inside `dist/` folder

```
./make.sh build
```

Create the production build with a custom `publicPath` for the Yew app:

```
YEW_PUBLIC_PATH="/custom/yew/" ./make.sh build
```
