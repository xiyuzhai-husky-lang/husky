import * as lc from "vscode-languageclient/node";
import type { Workspace } from "./context";
import * as vscode from "vscode";
import * as lsp_ext from "./ext/lsp_ext";
import * as render from "./render";

export async function createClient(
  serverPath: string,
  workspace: Workspace
): Promise<lc.LanguageClient> {
  const run: lc.Executable = {
    command: serverPath,
  };

  const server_options: lc.ServerOptions = {
    run,
    debug: run,
  };

  const trace_output_channel = vscode.window.createOutputChannel("Husky Language Server Trace");

  const client_options: lc.LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "husky" }],
    diagnosticCollectionName: "huskyc",
    traceOutputChannel: trace_output_channel,
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
              textDocument: client.code2ProtocolConverter.asTextDocumentIdentifier(document),
              position: positionOrRange,
            },
            token
          )
          .then(
            (result) => {
              const hover = client.protocol2CodeConverter.asHover(result);
              if (hover) {
                const actions = (<any>result).actions;
                if (actions) {
                  hover.contents.push(render.renderHoverActions(actions));
                }
              }
              return hover;
            },
            (error) => {
              client.handleFailedRequest(lc.HoverRequest.type, token, error);
              return Promise.resolve(null);
            }
          );
      },
    },
  };

  const client = new lc.LanguageClient(
    serverPath,
    "Husky Language Server",
    server_options,
    client_options
  );

  // To turn on all proposed features use: client.registerProposedFeatures();
  client.registerFeature(new ExperimentalFeatures());
  return client;
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
        "husky-analyzer-server.runSingle",
        "husky-analyzer-server.debugSingle",
        "husky-analyzer-server.showReferences",
        "husky-analyzer-server.gotoLocation",
        "editor.action.triggerParameterHints",
      ],
    };
    console.log("capabilities = ", caps);
    capabilities.experimental = caps;
  }
  initialize(
    _capabilities: lc.ServerCapabilities<any>,
    _documentSelector: lc.DocumentSelector | undefined
  ): void {}
  dispose(): void {}
}
