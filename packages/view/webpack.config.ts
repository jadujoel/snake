import copyWebpackPlugin from "copy-webpack-plugin";
import { resolve } from "path";
import type { Configuration } from "webpack";

const target = 'dist' as const
const config : Configuration = {
    entry: {
        "index": resolve("src", "index.ts")
    },
    output: {
        filename: '[name].js',
        path: resolve(target),
    },
    resolve: {
        extensions: ['.ts'],
        alias: { src: resolve("src") }
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'ts-loader',
                exclude: /node_modules/,
            },
        ]
    },
    // optimization: {
    //     minimize: true
    // },
    mode: "development",
    plugins: [
        new copyWebpackPlugin({
            patterns: [{
                from: resolve("src", "index.html"),
                to: resolve(target, "index.html"),
            },
            {
                from: resolve("src", "style.css"),
                to: resolve(target, "style.css"),
            },
        ]
        })
    ],
    devtool: "source-map",
    optimization: {
        minimize: true
    }
}
export default config;
