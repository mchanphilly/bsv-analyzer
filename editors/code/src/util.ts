import * as vscode from "vscode";
import { strict as nativeAssert } from "assert";
import { exec, type ExecOptions } from "child_process";
import { inspect } from "util";
import type { CargoRunnableArgs, ShellRunnableArgs } from "./lsp_ext";

export function assert(condition: boolean, explanation: string): asserts condition {
    try {
        nativeAssert(condition, explanation);
    } catch (err) {
        log.error(`Assertion failed:`, explanation);
        throw err;
    }
}

export type Env = {
    [name: string]: string;
};

class Log {
    private readonly output = vscode.window.createOutputChannel("BSV Analyzer Client", {
        log: true,
    });

    trace(...messages: [unknown, ...unknown[]]): void {
        this.output.trace(this.stringify(messages));
    }

    debug(...messages: [unknown, ...unknown[]]): void {
        this.output.debug(this.stringify(messages));
    }

    info(...messages: [unknown, ...unknown[]]): void {
        this.output.info(this.stringify(messages));
    }

    warn(...messages: [unknown, ...unknown[]]): void {
        this.output.warn(this.stringify(messages));
    }

    error(...messages: [unknown, ...unknown[]]): void {
        this.output.error(this.stringify(messages));
        this.output.show(true);
    }

    private stringify(messages: unknown[]): string {
        return messages
            .map((message) => {
                if (typeof message === "string") {
                    return message;
                }
                if (message instanceof Error) {
                    return message.stack || message.message;
                }
                return inspect(message, { depth: 6, colors: false });
            })
            .join(" ");
    }
}

export const log = new Log();

export function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export type SupportedLanguageId = "bluespec" | "minispec";
export type LanguageDocument<T extends SupportedLanguageId> = vscode.TextDocument & { languageId: T };
export type SupportedLanguageDocument = LanguageDocument<SupportedLanguageId>;
export type LanguageEditor<T extends SupportedLanguageId> = vscode.TextEditor & { document: LanguageDocument<T> };
export type SupportedLanguageEditor = LanguageEditor<SupportedLanguageId>;

export function isSupportedLanguageDocument(document: vscode.TextDocument): document is SupportedLanguageDocument {
    // Prevent corrupted text (particularly via inlay hints) in diff views
    // by allowing only `file` schemes
    // unfortunately extensions that use diff views not always set this
    // to something different than 'file' (see ongoing bug: #4608)
    return (document.languageId === "bluespec" || document.languageId === "minispec") 
           && document.uri.scheme === "file";
}

export function isSupportedEditor(editor: vscode.TextEditor): editor is SupportedLanguageEditor {
    return isSupportedLanguageDocument(editor.document);
}

export function isCargoTomlDocument(document: vscode.TextDocument): document is SupportedLanguageDocument {
    // ideally `document.languageId` should be 'toml' but user maybe not have toml extension installed
    // return document.uri.scheme === "file" && document.fileName.endsWith("Cargo.toml");
    return false;
}

export function isCargoRunnableArgs(
    args: CargoRunnableArgs | ShellRunnableArgs,
): args is CargoRunnableArgs {
    return (args as CargoRunnableArgs).executableArgs !== undefined;
}

export function isDocumentInWorkspace(document: SupportedLanguageDocument): boolean {
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders) {
        return false;
    }
    for (const folder of workspaceFolders) {
        if (document.uri.fsPath.startsWith(folder.uri.fsPath)) {
            return true;
        }
    }
    return false;
}

/** Sets ['when'](https://code.visualstudio.com/docs/getstarted/keybindings#_when-clause-contexts) clause contexts */
export function setContextValue(key: string, value: any): Thenable<void> {
    return vscode.commands.executeCommand("setContext", key, value);
}

/**
 * Returns a higher-order function that caches the results of invoking the
 * underlying async function.
 */
export function memoizeAsync<Ret, TThis, Param extends string>(
    func: (this: TThis, arg: Param) => Promise<Ret>,
) {
    const cache = new Map<string, Ret>();

    return async function (this: TThis, arg: Param) {
        const cached = cache.get(arg);
        if (cached) return cached;

        const result = await func.call(this, arg);
        cache.set(arg, result);

        return result;
    };
}

/** Awaitable wrapper around `child_process.exec` */
export function execute(command: string, options: ExecOptions): Promise<string> {
    log.info(`running command: ${command}`);
    return new Promise((resolve, reject) => {
        exec(command, options, (err, stdout, stderr) => {
            if (err) {
                log.error("error:", err);
                reject(err);
                return;
            }

            if (stderr) {
                reject(new Error(stderr));
                return;
            }

            resolve(stdout.trimEnd());
        });
    });
}

export class LazyOutputChannel implements vscode.OutputChannel {
    constructor(name: string) {
        this.name = name;
    }

    name: string;
    _channel: vscode.OutputChannel | undefined;

    get channel(): vscode.OutputChannel {
        if (!this._channel) {
            this._channel = vscode.window.createOutputChannel(this.name);
        }
        return this._channel;
    }

    append(value: string): void {
        this.channel.append(value);
    }
    appendLine(value: string): void {
        this.channel.appendLine(value);
    }
    replace(value: string): void {
        this.channel.replace(value);
    }
    clear(): void {
        if (this._channel) {
            this._channel.clear();
        }
    }
    show(preserveFocus?: boolean): void;
    show(column?: vscode.ViewColumn, preserveFocus?: boolean): void;
    show(column?: any, preserveFocus?: any): void {
        this.channel.show(column, preserveFocus);
    }
    hide(): void {
        if (this._channel) {
            this._channel.hide();
        }
    }
    dispose(): void {
        if (this._channel) {
            this._channel.dispose();
        }
    }
}

export type NotNull<T> = T extends null ? never : T;

export type Nullable<T> = T | null;

function isNotNull<T>(input: Nullable<T>): input is NotNull<T> {
    return input !== null;
}

function expectNotNull<T>(input: Nullable<T>, msg: string): NotNull<T> {
    if (isNotNull(input)) {
        return input;
    }

    throw new TypeError(msg);
}

export function unwrapNullable<T>(input: Nullable<T>): NotNull<T> {
    return expectNotNull(input, `unwrapping \`null\``);
}
export type NotUndefined<T> = T extends undefined ? never : T;

export type Undefinable<T> = T | undefined;

function isNotUndefined<T>(input: Undefinable<T>): input is NotUndefined<T> {
    return input !== undefined;
}

export function expectNotUndefined<T>(input: Undefinable<T>, msg: string): NotUndefined<T> {
    if (isNotUndefined(input)) {
        return input;
    }

    throw new TypeError(msg);
}

export function unwrapUndefinable<T>(input: Undefinable<T>): NotUndefined<T> {
    return expectNotUndefined(input, `unwrapping \`undefined\``);
}
