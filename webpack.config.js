/* eslint-disable no-undef */
const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require('webpack');

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: ['babel-loader'],
      },
    ],
  },
  // ...add resolve to .jsx extension
  resolve: {
    extensions: ['*', '.js', '.jsx'],
  },
  devServer: {
    watchFiles: dist,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        path.resolve(__dirname, "static")
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),

    new webpack.HotModuleReplacementPlugin()
  ],
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
};
