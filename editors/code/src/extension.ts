import * as vscode from "vscode";
import { svgminiPath } from "svgmini";
import { spawn } from "child_process";

export function activate(context: vscode.ExtensionContext) {
  let disposable = vscode.commands.registerTextEditorCommand(
    "svgmini.minifyInFile",
    (editor, _edit) => {
      const config = vscode.workspace.getConfiguration();
      const shouldReplaceFillConfig = config.get("svgmini.replaceFill");
      const shouldReplaceFill =
        typeof shouldReplaceFillConfig === "boolean"
          ? shouldReplaceFillConfig
          : false;

      let path = editor.document.fileName;

      const args = [path, shouldReplaceFill ? "--replace-fill" : ""].filter(
        arg => arg !== ""
      );

      let svgminiProc = spawn(svgminiPath, args);

      svgminiProc.stdout.on(
        "data",
        data =>
          data &&
          data.toString() !== "" &&
          console.log("svgmini stdout:\n", data.toString())
      );

      svgminiProc.stderr.on("data", data => {
        if (data && data.toString() !== "") {
          console.log("svgmini stderr:\n", data.toString());
          vscode.window.showErrorMessage(`SVGMini error: ${data.toString()}`);
        }
      });
    }
  );

  context.subscriptions.push(disposable);
}

export function deactivate() {}
