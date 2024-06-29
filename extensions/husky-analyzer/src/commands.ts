import * as vscode from "vscode";
import type * as lc from "vscode-languageclient/node";
import type { Context } from "./context";

export type Cmd = (...args: any[]) => unknown;
function gotoLocation(ctx: Context): Cmd {
  return async (locationLink: lc.LocationLink) => {
    const client = ctx.client;
    if (client) {
      const uri = client.protocol2CodeConverter.asUri(locationLink.targetUri);
      let range = client.protocol2CodeConverter.asRange(locationLink.targetSelectionRange);
      // collapse the range to a cursor position
      range = range.with({ end: range.start });

      await vscode.window.showTextDocument(uri, { selection: range });
    }
  };
}

function notImplemented(ctx: Context): Cmd {
  return async () => {
    console.error("todo: not implemented");
  };
}

// note: make it coherent with those ("commands", "menus") in package.json
export const NAMED_COMMAND_FACTORIES: [string, (ctx: Context) => Cmd][] = [
  ["deps", notImplemented],
];
