import * as lc from "vscode-languageclient/node";

export interface ServerStatusParams {
    health: "ok" | "warning" | "error";
    quiescent: boolean;
    message?: string;
}

export const serverStatus = new lc.NotificationType<ServerStatusParams>(
    "experimental/serverStatus"
);

export interface CommandLink extends lc.Command {
    /**
     * A tooltip for the command, when represented in the UI.
     */
    tooltip?: string;
}

export interface CommandLinkGroup {
    title?: string;
    commands: CommandLink[];
}
