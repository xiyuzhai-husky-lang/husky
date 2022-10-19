import type * as lsp_ext from "./lsp_ext";
import * as vscode from "vscode";

export function renderHoverActions(
    actions: lsp_ext.CommandLinkGroup[]
): vscode.MarkdownString {
    const text = actions
        .map(
            (group) =>
                (group.title ? group.title + " " : "") +
                group.commands.map(renderCommand).join(" | ")
        )
        .join("\n___\n");

    const result = new vscode.MarkdownString(text);
    result.isTrusted = true;
    return result;
}

function renderCommand(cmd: lsp_ext.CommandLink): string {
    return `[${cmd.title}](command:${cmd.command}?${encodeURIComponent(
        JSON.stringify(cmd.arguments)
    )} '${cmd.tooltip}')`;
}
