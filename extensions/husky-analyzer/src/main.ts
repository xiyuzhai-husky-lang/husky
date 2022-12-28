import type * as vscode from "vscode";
import type * as lc from "vscode-languageclient/node";
import { Config } from "./config";
import { Context } from "./context";

let ctx: Context | undefined;

export interface HuskyAnalyzerExtensionApi {
    client?: lc.LanguageClient;
}

export async function activate(
    ext_ctx: vscode.ExtensionContext
): Promise<HuskyAnalyzerExtensionApi> {
    const config = new Config(ext_ctx);
    ctx = await Context.create(config, ext_ctx, "husky-analyzer-server", {
        kind: "Workspace Folder",
    });

    return {
        client: ctx.client,
    };
}

export function deactivate(): Thenable<void> | undefined {
    if (!ctx) {
        return undefined;
    }
    return ctx.client.stop();
}
