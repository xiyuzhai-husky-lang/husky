import { decode_string } from "src/decode/decode";

export type Color = "red" | "yellow" | "green" | "blue";

export function decode_color(data: unknown): Color {
    switch (decode_string(data)) {
        case "red":
            return "red";
        case "yellow":
            return "yellow";
        case "green":
            return "green";
        case "blue":
            return "blue";
        default:
            throw new Error("TODO");
    }
}
