import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";
import * as lsp_ext from "./lsp_ext";
import { DebuggerSingleton } from "./DebuggerPanel";

let client: lc.LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(
        vscode.commands.registerCommand("husky_lang_server.sayHello", () => {
            DebuggerSingleton.createOrShow(context.extensionUri);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand("husky_lang_server.refresh", () => {
            DebuggerSingleton.kill();
            DebuggerSingleton.createOrShow(context.extensionUri);
            vscode.commands.executeCommand(
                "workbench.action.webview.openDeveloperTools"
            );
        })
    );

    const run: lc.Executable = {
        command: "husky_lang_server",
    };

    const serverOptions: lc.ServerOptions = {
        run,
        debug: run,
    };

    const traceOutputChannel = vscode.window.createOutputChannel(
        "Husky Language Server Trace"
    );

    const clientOptions: lc.LanguageClientOptions = {
        documentSelector: [{ scheme: "file", language: "husky" }],
        diagnosticCollectionName: "huskyc",
        traceOutputChannel,
        middleware: {
            async provideHover(
                document: vscode.TextDocument,
                position: vscode.Position,
                token: vscode.CancellationToken,
                _next: lc.ProvideHoverSignature
            ) {
                const editor = vscode.window.activeTextEditor;
                const positionOrRange = editor?.selection?.contains(position)
                    ? client.code2ProtocolConverter.asRange(editor.selection)
                    : client.code2ProtocolConverter.asPosition(position);
                return client
                    .sendRequest(
                        lsp_ext.hover,
                        {
                            textDocument:
                                client.code2ProtocolConverter.asTextDocumentIdentifier(
                                    document
                                ),
                            position: positionOrRange,
                        },
                        token
                    )
                    .then(
                        (result) => {
                            const hover =
                                client.protocol2CodeConverter.asHover(result);
                            if (hover) {
                                const actions = (<any>result).actions;
                                if (actions) {
                                    hover.contents.push(
                                        renderHoverActions(actions)
                                    );
                                }
                            }
                            return hover;
                        },
                        (error) => {
                            client.handleFailedRequest(
                                lc.HoverRequest.type,
                                token,
                                error
                            );
                            return Promise.resolve(null);
                        }
                    );
            },
        },
    };

    const client = new lc.LanguageClient(
        "husky_lang_server",
        "Husky Language Server",
        serverOptions,
        clientOptions
    );

    // To turn on all proposed features use: client.registerProposedFeatures();
    client.registerFeature(new ExperimentalFeatures());

    client.start();
}

function renderCommand(cmd: lsp_ext.CommandLink) {
    return `[${cmd.title}](command:${cmd.command}?${encodeURIComponent(
        JSON.stringify(cmd.arguments)
    )} '${cmd.tooltip}')`;
}

function renderHoverActions(
    actions: lsp_ext.CommandLinkGroup[]
): vscode.MarkdownString {
    const text = actions
        .map(
            (group) =>
                (group.title ? group.title + " " : "") +
                group.commands.map(renderCommand).join(" | ")
        )
        .join("___");

    const result = new vscode.MarkdownString(text);
    result.isTrusted = true;
    return result;
}

class ExperimentalFeatures implements lc.StaticFeature {
    fillClientCapabilities(capabilities: lc.ClientCapabilities): void {
        const caps: any = capabilities.experimental ?? {};
        caps.snippetTextEdit = true;
        caps.codeActionGroup = true;
        caps.hoverActions = true;
        caps.serverStatusNotification = true;
        caps.commands = {
            commands: [
                "husky-lang-server.runSingle",
                "husky-lang-server.debugSingle",
                "husky-lang-server.showReferences",
                "husky-lang-server.gotoLocation",
                "editor.action.triggerParameterHints",
            ],
        };
        capabilities.experimental = caps;
    }
    initialize(
        _capabilities: lc.ServerCapabilities<any>,
        _documentSelector: lc.DocumentSelector | undefined
    ): void {}
    dispose(): void {}
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
