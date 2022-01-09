import type { Trace, FigureProps } from "./types";

const simpleImage1 = { data: [], originalWidth: 28, originalHeight: 28 };

const traces: { [idx: number]: Trace } = {
    0: {
        idx: 0,
        parent: null,
        tokens: [
            { type: "keyword", value: "let", spaces_before: 0 },
            { type: "ident", value: "a" },
            { type: "special", value: "=" },
            { type: "scope", value: "f" },
            { type: "special", value: "(", spaces_before: 0 },
            { type: "ident", value: "c", spaces_before: 0 },
            { type: "special", value: "+" },
            { type: "ident", value: "b" },
            { type: "special", value: ")", spaces_before: 0 },
            { type: "fade", value: "=" },
            { type: "fade", value: "a plot" },
        ],
    },
    1: {
        idx: 1,
        parent: 0,
        tokens: [
            { type: "scope", value: "f", spaces_before: 4 },
            { type: "special", value: "(", spaces_before: 0 },
            { type: "ident", value: "c", spaces_before: 0 },
            { type: "special", value: "+" },
            { type: "ident", value: "b" },
            { type: "special", value: ")", spaces_before: 0 },
            { type: "fade", value: "=" },
            { type: "fade", value: "an arrow" },
        ],
    },
    2: {
        idx: 2,
        parent: 0,
        tokens: [
            { type: "ident", value: "c", spaces_before: 4 },
            { type: "fade", value: "=" },
            { type: "fade", value: "arrows" },
        ],
    },
    3: {
        idx: 3,
        parent: 0,
        tokens: [
            { type: "ident", value: "b", spaces_before: 4 },
            { type: "fade", value: "=" },
            { type: "fade", value: "an image" },
        ],
    },
};

const figures: readonly FigureProps[] = [
    {
        type: "Plot2d",
        plot_kind: "Scatter",
        groups: [
            {
                points: [
                    { x: 0, y: 0.2 },
                    { x: 2.2, y: -1.0 },
                    { x: 3.2, y: -1.0 },
                    { x: 4.2, y: -2.0 },
                    { x: 2.2, y: -1.5 },
                    { x: 3.2, y: -1.2 },
                    { x: 3.2, y: -1.2 },
                ],
                color: "red",
            },
            {
                points: [
                    { x: 0.1, y: 0.2 },
                    { x: 1.2, y: -1.0 },
                ],
                color: "yellow",
            },
        ],
        xrange: [-5, 5],
        yrange: [-5, 5],
    },
    {
        type: "Graphics2d",
        image: null,
        shape_groups: [
            {
                shapes: [
                    {
                        shape_kind: "Arrow",
                        from: { x: 5, y: 6 },
                        to: { x: 16, y: 15 },
                    },
                ],
                lineWidth: 0.15,
                color: "yellow",
            },
        ],
        xrange: [0, 28],
        yrange: [0, 28],
    },
    {
        type: "Graphics2d",
        image: null,
        shape_groups: [
            {
                shapes: [
                    {
                        shape_kind: "Arrow",
                        from: { x: 5, y: 6 },
                        to: { x: 16, y: 15 },
                    },
                ],
                lineWidth: 0.15,
                color: "yellow",
            },
            {
                shapes: [
                    {
                        shape_kind: "Arrow",
                        from: { x: 16, y: 6 },
                        to: { x: 5, y: 15 },
                    },
                ],
                lineWidth: 0.15,
                color: "red",
            },
        ],
        xrange: [0, 28],
        yrange: [0, 28],
    },
    {
        type: "Graphics2d",
        image: simpleImage1,
        shape_groups: [
            {
                shapes: [
                    {
                        shape_kind: "Arrow",
                        from: { x: 5, y: 6 },
                        to: { x: 16, y: 15 },
                    },
                ],
                lineWidth: 0.15,
                color: "yellow",
            },
            {
                shapes: [
                    {
                        shape_kind: "Arrow",
                        from: { x: 16, y: 6 },
                        to: { x: 5, y: 15 },
                    },
                ],
                lineWidth: 0.15,
                color: "red",
            },
        ],
        xrange: [0, 28],
        yrange: [0, 28],
    },
];

const children: any = { 0: [1, 2, 3], 1: [], 2: [], 3: [] };
const expansions: boolean[] = [true, false];
let active_trace: number | null = null;

function getTrace(version: number, idx: number): Trace {
    return traces[idx];
}

function getExpansions(version: number): readonly boolean[] {
    return [...expansions];
}

function didToggleExpansion(version: number, idx: number) {
    expansions[idx] = !expansions[idx];
}

function getChildren(version: number, idx: number): number[] {
    return children[idx];
}

function getFigure(version: number, idx: number): FigureProps {
    let figure = figures[idx];
    if (figure !== null && figure.type === "Graphics2d") {
        console.log("figures[idx].image", figure.image);
    }
    return figures[idx];
}

function getActiveTrace(version: number) {
    return active_trace;
}

function didActivate(version: number, idx: number) {
    active_trace = idx;
}

export default {
    getTrace,
    getExpansions,
    didToggleExpansion,
    getChildren,
    getFigure,
    getActiveTrace,
    didActivate,
};
