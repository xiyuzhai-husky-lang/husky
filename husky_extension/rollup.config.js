import svelte from "rollup-plugin-svelte";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import { terser } from "rollup-plugin-terser";
import typescript from "@rollup/plugin-typescript";
import preprocess from "svelte-preprocess";
import css from "rollup-plugin-css-only";

const production = false;

export default ["entry"].map((name) => {
    return {
        input: "ui/" + name + ".ts",
        output: {
            sourcemap: true,
            format: "iife",
            name: "app",
            file: "out/compiled/" + name + ".js",
        },
        onwarn: function (message, warn) {
            if (/.*io-ts.*/.test(message)) return;
            if (/.*fp-ts.*/.test(message)) return;
            warn(message);
        },
        plugins: [
            svelte({
                compilerOptions: {
                    dev: !production,
                },
                preprocess: preprocess(),
            }),
            css({ output: name + ".css" }),
            typescript({
                tsconfig: "tsconfig.json",
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
