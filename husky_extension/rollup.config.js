import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import { terser } from "rollup-plugin-terser";
import alias from "@rollup/plugin-alias";
import typescript from "@rollup/plugin-typescript";
import preprocess from "svelte-preprocess";
import css from "rollup-plugin-css-only";

const aliases = alias({
    resolve: [".svelte", ".js"], //optional, by default this will just look for .js files or folders
    entries: [
        { find: "components", replacement: "src/components" },
        { find: "metadata", replacement: "src/metadata" },
        { find: "util", replacement: "src/util" },
        { find: "server", replacement: "src/server" },
    ],
});

// const production = !process.env.ROLLUP_WATCH;
const production = false;

export default ["Debugger"].map((name) => {
    return {
        input: "src/webviews/" + name + ".ts",
        output: {
            sourcemap: true,
            format: "iife",
            name: "app",
            file: "out/compiled/" + name + ".js",
        },
        plugins: [
            aliases,
            svelte({
                compilerOptions: {
                    dev: !production,
                },
                preprocess: preprocess(),
            }),
            css({ output: name + ".css" }),
            typescript({
                tsconfig: "svelte_tsconfig.json",
                sourceMap: true,
                inlineSources: !production,
            }),
            resolve({
                browser: true,
                dedupe: ["svelte"],
            }),
            commonjs(),
            production && terser(),
        ],
        watch: {
            clearScreen: false,
        },
    };
});
