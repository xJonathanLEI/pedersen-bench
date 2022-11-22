import * as path from "path";

const config = {
  entry: "./js-bench.ts",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: "ts-loader",
        options: {
          allowTsInNodeModules: true,
        },
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "js-bench.js",
    path: path.resolve(__dirname, "build"),
  },
  target: "node",
  mode: "production",
  performance: {
    hints: false,
  },
};

export default config;
