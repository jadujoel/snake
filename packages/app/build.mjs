import { build } from "esbuild";
import { copy } from "esbuild-plugin-copy"
import { wasmLoader } from 'esbuild-plugin-wasm';
import { resolve } from "path";

let args = process.argv.slice(2)
let isWatch = args.includes("--watch")

build({
    watch: isWatch,
    entryPoints: [resolve("src", "app.ts")],
    outdir: resolve("dist"),
    bundle: true,
    minify: true,
    sourcemap: true,
    treeShaking: true,
    platform: "browser",
    target: "esnext",
    format: "esm",
    plugins: [
        wasmLoader({
            mode: "defered"
        }),
        copy({
            assets: [{
                    from: resolve("src", "index.html"),
                    to: resolve("dist")
                },
                // {
                //     from: resolve("src", "app.css"),
                //     to: resolve("dist", "app.css")
                // },
                {
                    from: resolve("pkg", "app_bg.wasm"),
                    to: resolve("dist")
                },
            ]
        })
    ]
}).catch(console.error)
