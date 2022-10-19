import * as vscode from "vscode";
import type * as lc from "vscode-languageclient/node";
import type { Context } from "./context";

export type Cmd = (...args: any[]) => unknown;
function gotoLocation(ctx: Context): Cmd {
    return async (locationLink: lc.LocationLink) => {
        const client = ctx.client;
        if (client) {
            const uri = client.protocol2CodeConverter.asUri(
                locationLink.targetUri
            );
            let range = client.protocol2CodeConverter.asRange(
                locationLink.targetSelectionRange
            );
            // collapse the range to a cursor position
            range = range.with({ end: range.start });

            await vscode.window.showTextDocument(uri, { selection: range });
        }
    };
}

function analyzerStatus(ctx: Context): Cmd {
    return async () => {
        console.log("todo: analyzer status");
    };
}

function memoryUsage(ctx: Context): Cmd {
    return async () => {
        console.log("todo: memory usage");
    };
}

function notImplemented(ctx: Context): Cmd {
    return async () => {
        console.log("todo: not implemented");
    };
}

export const NAMED_COMMAND_FACTORIES: [string, (ctx: Context) => Cmd][] = [
    ["analyzerStatus", analyzerStatus],
    ["joinLines", notImplemented],
    ["memoryUsage", memoryUsage],
    ["onEnter", notImplemented],
    ["openCorgiToml", notImplemented],
    ["openDocs", notImplemented],
    ["parentModule", notImplemented],
    ["reloadWorkspace", notImplemented],
    ["reload", notImplemented],
    ["run", notImplemented],
    ["serverVersion", notImplemented],
    ["ssr", notImplemented],
    ["syntaxTree", notImplemented],
    ["toggleInlayHints", notImplemented],
    ["viewHir", notImplemented],
    ["viewFileText", notImplemented],
    // Internal commands which are invoked by the server.
    ["gotoLocation", notImplemented],
];
