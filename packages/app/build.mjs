import { build } from "esbuild";
import { copy } from "esbuild-plugin-copy"
import { wasmLoader } from 'esbuild-plugin-wasm';
import { resolve } from "path";

build({
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
                {
                    from: resolve("pkg", "app_bg.wasm"),
                    to: resolve("dist")
                },
            ]
        })
    ]
}).catch(console.error)
