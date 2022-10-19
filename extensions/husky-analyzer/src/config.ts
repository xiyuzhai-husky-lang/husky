export const server_executable = "husky-analyzer-server";
import * as vscode from "vscode";
import { log } from "./util";

export class Config {
    readonly extensionId = "husky-lang.husky-analyzer";
    readonly rootSection = "husky-analyzer";
    readonly globalStorageUri: vscode.Uri;
    readonly package: {
        version: string;
        releaseTag: string | null;
        enableProposedApi: boolean | undefined;
    } = vscode.extensions.getExtension(this.extensionId)!.packageJSON;
    private readonly requiresWorkspaceReloadOpts = [
        "serverPath",
        "server",
        // FIXME: This shouldn't be here, changing this setting should reload
        // `continueCommentsOnNewline` behavior without restart
        "typing",
    ].map((opt) => `${this.rootSection}.${opt}`);
    private readonly requiresReloadOpts = [
        "cargo",
        "procMacro",
        "files",
        "lens", // works as lens.*
    ]
        .map((opt) => `${this.rootSection}.${opt}`)
        .concat(this.requiresWorkspaceReloadOpts);

    constructor(ext_ctx: vscode.ExtensionContext) {
        this.globalStorageUri = ext_ctx.globalStorageUri;
        vscode.workspace.onDidChangeConfiguration(
            this.onDidChangeConfiguration,
            this,
            ext_ctx.subscriptions
        );
        this.refreshLogging();
    }

    private refreshLogging() {
        log.setEnabled(this.traceExtension);
        log.info("Extension version:", this.package.version);

        const cfg = Object.entries(this.cfg).filter(
            ([_, val]) => !(val instanceof Function)
        );
        log.info("Using configuration", Object.fromEntries(cfg));
    }

    // We don't do runtime config validation here for simplicity. More on stackoverflow:
    // https://stackoverflow.com/questions/60135780/what-is-the-best-way-to-type-check-the-configuration-for-vscode-extension

    private get cfg(): vscode.WorkspaceConfiguration {
        return vscode.workspace.getConfiguration(this.rootSection);
    }

    private get<T>(path: string): T {
        return this.cfg.get<T>(path)!;
    }

    get traceExtension() {
        return this.get<boolean>("trace.extension");
    }

    get restartServerOnConfigChange() {
        return this.get<boolean>("restartServerOnConfigChange");
    }

    private async onDidChangeConfiguration(
        event: vscode.ConfigurationChangeEvent
    ) {
        this.refreshLogging();

        const requiresReloadOpt = this.requiresReloadOpts.find((opt) =>
            event.affectsConfiguration(opt)
        );

        if (!requiresReloadOpt) return;

        const requiresWorkspaceReloadOpt =
            this.requiresWorkspaceReloadOpts.find((opt) =>
                event.affectsConfiguration(opt)
            );

        if (!requiresWorkspaceReloadOpt && this.restartServerOnConfigChange) {
            await vscode.commands.executeCommand("husky-analyzer.reload");
            return;
        }

        const message = requiresWorkspaceReloadOpt
            ? `Changing "${requiresWorkspaceReloadOpt}" requires a window reload`
            : `Changing "${requiresReloadOpt}" requires a reload`;
        const userResponse = await vscode.window.showInformationMessage(
            message,
            "Reload now"
        );

        if (userResponse === "Reload now") {
            const command = requiresWorkspaceReloadOpt
                ? "workbench.action.reloadWindow"
                : "husky-analyzer.reload";
            if (userResponse === "Reload now") {
                await vscode.commands.executeCommand(command);
            }
        }
    }
}
