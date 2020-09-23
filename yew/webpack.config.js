const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const rimraf = require("rimraf");

const htmlPath = path.resolve(__dirname, "static/index.html");
const distPath = path.resolve(__dirname, "dist");
rimraf.sync(distPath);

// See https://webpack.js.org/guides/public-path/
const YEW_PUBLIC_PATH = process.env.YEW_PUBLIC_PATH || "/";

module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === "production",
      port: 8000,
    },
    entry: "./bootstrap.js",
    output: {
      path: distPath,
      filename: "yew.[contenthash:8].js",
      webassemblyModuleFilename: "yew.wasm",
      publicPath: YEW_PUBLIC_PATH,
    },
    module: {
      rules: [
        {
          test: /\.css$/i,
          use: [MiniCssExtractPlugin.loader, "css-loader"],
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        inject: true,
        template: htmlPath,
        minify: {
          removeComments: true,
          collapseWhitespace: true,
          removeRedundantAttributes: true,
          useShortDoctype: true,
          removeEmptyAttributes: true,
          removeStyleLinkTypeAttributes: true,
          keepClosingSlash: true,
          minifyJS: true,
          minifyCSS: true,
          minifyURLs: true,
        },
      }),
      new MiniCssExtractPlugin({
        filename: "yew.css",
      }),
      new CopyWebpackPlugin([{ from: "./static", to: distPath }]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      }),
    ],
    watch: argv.mode !== "production",
  };
};
