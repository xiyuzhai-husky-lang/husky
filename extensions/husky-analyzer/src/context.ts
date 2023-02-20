import type { Config } from "./config";
import type * as lc from "vscode-languageclient/node";
import * as vscode from "vscode";
import { createClient } from "./client";
import * as lsp_ext from "./lsp_ext";
import { NAMED_COMMAND_FACTORIES, type Cmd } from "./commands";
import { log } from "./util";

export type Workspace =
    | {
          kind: "Workspace Folder";
      }
    | {
          kind: "Detached Files";
          files: vscode.TextDocument[];
      };

export class Context {
    private constructor(
        readonly config: Config,
        private readonly extCtx: vscode.ExtensionContext,
        readonly client: lc.LanguageClient,
        readonly serverPath: string,
        readonly statusBar: vscode.StatusBarItem
    ) {}

    static async create(
        config: Config,
        ext_ctx: vscode.ExtensionContext,
        server_path: string,
        workspace: Workspace
    ): Promise<Context> {
        const client = await createClient(server_path, workspace);
        const statusBar = Context.createStatusBar(ext_ctx);
        const ctx = new Context(
            config,
            ext_ctx,
            client,
            server_path,
            statusBar
        );
        ctx.init(client);
        return ctx;
    }

    private static createStatusBar(
        ext_ctx: vscode.ExtensionContext
    ): vscode.StatusBarItem {
        const status_bar = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left
        );
        ext_ctx.subscriptions.push(status_bar);
        status_bar.text = "husky-analyzer";
        status_bar.tooltip = "ready";
        status_bar.command = "husky-analyzer.analyzerStatus";
        status_bar.show();
        return status_bar;
    }

    async init(client: lc.LanguageClient) {
        log.info("Starting language client");
        this.initCommands();
        await this.initClient(client);
    }

    private async initClient(client: lc.LanguageClient) {
        await client.start();
        // this.pushCleanup();
        // await client.onReady();
        client.onNotification(lsp_ext.serverStatus, (params) =>
            this.setServerStatus(params)
        );
    }

    private initCommands() {
        for (const [name, factory] of NAMED_COMMAND_FACTORIES) {
            this.registerCommand(name, factory);
        }
    }

    private registerCommand(name: string, factory: (ctx: Context) => Cmd) {
        const fullName = `husky-analyzer.${name}`;
        const cmd = factory(this);
        const d = vscode.commands.registerCommand(fullName, cmd);
        this.pushCleanup(d);
    }

    setServerStatus(status: lsp_ext.ServerStatusParams) {
        let icon = "";
        const statusBar = this.statusBar;
        switch (status.health) {
            case "ok":
                statusBar.tooltip = status.message ?? "Ready";
                statusBar.command = undefined;
                statusBar.color = undefined;
                statusBar.backgroundColor = undefined;
                break;
            case "warning":
                statusBar.tooltip =
                    (status.message ? status.message + "\n" : "") +
                    "Click to reload.";

                statusBar.command = "husky-analyzer.reloadWorkspace";
                statusBar.color = new vscode.ThemeColor(
                    "statusBarItem.warningForeground"
                );
                statusBar.backgroundColor = new vscode.ThemeColor(
                    "statusBarItem.warningBackground"
                );
                icon = "$(warning) ";
                break;
            case "error":
                statusBar.tooltip =
                    (status.message ? status.message + "\n" : "") +
                    "Click to reload.";

                statusBar.command = "husky-analyzer.reloadWorkspace";
                statusBar.color = new vscode.ThemeColor(
                    "statusBarItem.errorForeground"
                );
                statusBar.backgroundColor = new vscode.ThemeColor(
                    "statusBarItem.errorBackground"
                );
                icon = "$(error) ";
                break;
        }
        if (!status.quiescent) icon = "$(sync~spin) ";
        statusBar.text = `${icon}husky-analyzer`;
    }

    pushCleanup(d: Disposable) {
        this.extCtx.subscriptions.push(d);
    }
}

export interface Disposable {
    dispose(): void;
}
