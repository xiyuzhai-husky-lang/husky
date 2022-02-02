import type { Trace, FigureProps, ImageProps } from "./types";

const simpleImage1: ImageProps = {
    data: [],
    original_width: 28,
    original_height: 28,
};

const traces: { [idx: number]: Trace } = {
    0: {
        id: 0,
        parent: null,
        tokens: [
            { kind: "keyword", value: "let", spaces_before: 0 },
            { kind: "ident", value: "a" },
            { kind: "special", value: "=" },
            { kind: "scope", value: "f" },
            { kind: "special", value: "(", spaces_before: 0 },
            { kind: "ident", value: "c", spaces_before: 0 },
            { kind: "special", value: "+" },
            { kind: "ident", value: "b" },
            { kind: "special", value: ")", spaces_before: 0 },
            { kind: "fade", value: "=" },
            { kind: "fade", value: "a plot" },
        ],
    },
    1: {
        id: 1,
        parent: 0,
        tokens: [
            { kind: "scope", value: "f", spaces_before: 4 },
            { kind: "special", value: "(", spaces_before: 0 },
            { kind: "ident", value: "c", spaces_before: 0 },
            { kind: "special", value: "+" },
            { kind: "ident", value: "b" },
            { kind: "special", value: ")", spaces_before: 0 },
            { kind: "fade", value: "=" },
            { kind: "fade", value: "an arrow" },
        ],
    },
    2: {
        id: 2,
        parent: 0,
        tokens: [
            { kind: "ident", value: "c", spaces_before: 4 },
            { kind: "fade", value: "=" },
            { kind: "fade", value: "arrows" },
        ],
    },
    3: {
        id: 3,
        parent: 0,
        tokens: [
            { kind: "ident", value: "b", spaces_before: 4 },
            { kind: "fade", value: "=" },
            { kind: "fade", value: "an image" },
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
                        type: "Arrow",
                        from: { x: 5, y: 6 },
                        to: { x: 16, y: 15 },
                    },
                ],
                line_width: 0.15,
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
                        type: "Arrow",
                        from: { x: 5, y: 6 },
                        to: { x: 16, y: 15 },
                    },
                ],
                line_width: 0.15,
                color: "yellow",
            },
            {
                shapes: [
                    {
                        type: "Arrow",
                        from: { x: 16, y: 6 },
                        to: { x: 5, y: 15 },
                    },
                ],
                line_width: 0.15,
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
                        type: "Arrow",
                        from: { x: 5, y: 6 },
                        to: { x: 16, y: 15 },
                    },
                ],
                line_width: 0.15,
                color: "yellow",
            },
            {
                shapes: [
                    {
                        type: "Arrow",
                        from: { x: 16, y: 6 },
                        to: { x: 5, y: 15 },
                    },
                ],
                line_width: 0.15,
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
