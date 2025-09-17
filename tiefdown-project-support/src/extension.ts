import { spawn } from "node:child_process";
import * as vscode from "vscode";

type TiefdownProject = {
  projectUri: vscode.Uri;
  manifestUri: vscode.Uri;
};

interface ProjectQuickPickItem extends vscode.QuickPickItem {
  project: TiefdownProject;
}

let outputChannel: vscode.OutputChannel | undefined;

function describeProject(project: TiefdownProject): string {
  const workspaceFolder = vscode.workspace.getWorkspaceFolder(project.projectUri);
  if (!workspaceFolder) {
    return project.projectUri.fsPath;
  }
  const relativePath = vscode.workspace.asRelativePath(project.projectUri, false);
  return relativePath.length > 0 ? `${workspaceFolder.name}/${relativePath}` : workspaceFolder.name;
}

function isUri(value: unknown): value is vscode.Uri {
  return value instanceof vscode.Uri;
}

function getOutputChannel(): vscode.OutputChannel {
  if (!outputChannel) {
    outputChannel = vscode.window.createOutputChannel("Tiefdown Converter");
  }
  return outputChannel;
}

async function findProjectsWithManifest(): Promise<TiefdownProject[]> {
  const manifestUris = await vscode.workspace.findFiles("**/manifest.toml");
  return manifestUris.map((manifestUri) => ({
    manifestUri,
    projectUri: vscode.Uri.joinPath(manifestUri, ".."),
  }));
}

async function getProjectFromFolderUri(folderUri: vscode.Uri): Promise<TiefdownProject | undefined> {
  try {
    const folderStat = await vscode.workspace.fs.stat(folderUri);
    if ((folderStat.type & vscode.FileType.Directory) === 0) {
      return undefined;
    }
  } catch (error) {
    if (error instanceof vscode.FileSystemError && error.code === "FileNotFound") {
      return undefined;
    }
    throw error;
  }

  const manifestUri = vscode.Uri.joinPath(folderUri, "manifest.toml");
  try {
    await vscode.workspace.fs.stat(manifestUri);
    return { manifestUri, projectUri: folderUri };
  } catch (error) {
    if (error instanceof vscode.FileSystemError && error.code === "FileNotFound") {
      return undefined;
    }
    throw error;
  }
}

async function pickProject(projects: TiefdownProject[]): Promise<TiefdownProject | undefined> {
  if (projects.length === 0) {
    return undefined;
  }

  if (projects.length === 1) {
    return projects[0];
  }

  const items: ProjectQuickPickItem[] = projects.map((project) => {
    const workspaceFolder = vscode.workspace.getWorkspaceFolder(project.projectUri);
    const relativePath = vscode.workspace.asRelativePath(project.projectUri, false);
    const fallbackLabel = project.projectUri.path.split("/").filter(Boolean).pop() ?? project.projectUri.toString();
    const label = workspaceFolder?.name ?? (relativePath.length > 0 ? relativePath : fallbackLabel);
    const description = relativePath.length > 0 ? relativePath : undefined;
    return {
      label,
      description,
      project,
    };
  });

  const selection = await vscode.window.showQuickPick(items, {
    placeHolder: "Select a Tiefdown project to convert",
  });

  return selection?.project;
}

async function listTemplates(project: TiefdownProject): Promise<string[]> {
  return new Promise((resolve, reject) => {
    const child = spawn("tiefdownconverter", ["project", "list-templates"], {
      cwd: project.projectUri.fsPath,
    });

    let stdout = "";
    let stderr = "";

    child.stdout?.on("data", (chunk: Buffer) => {
      stdout += chunk.toString();
    });

    child.stderr?.on("data", (chunk: Buffer) => {
      stderr += chunk.toString();
    });

    child.on("error", (err: NodeJS.ErrnoException) => {
      if (err.code === "ENOENT") {
        vscode.window.showErrorMessage("tiefdownconverter CLI not found in PATH. Install it to convert Tiefdown projects.");
      }
      reject(err);
    });

    child.on("close", (code) => {
      if (code !== 0) {
        const message = stderr || stdout || "Failed to list templates for this project.";
        reject(new Error(message));
        return;
      }

      const lines = stdout.split(/\r?\n/);
      const templates = lines
        .map((line) => line.trimEnd())
        .filter((line) => line.length > 0 && !line.startsWith(" ") && line.endsWith(":"))
        .map((line) => line.slice(0, -1));
      resolve(templates);
    });
  });
}

async function listProfiles(project: TiefdownProject): Promise<string[]> {
  return new Promise((resolve, reject) => {
    const child = spawn("tiefdownconverter", ["project", "profiles", "list"], {
      cwd: project.projectUri.fsPath,
    });

    let stdout = "";
    let stderr = "";

    child.stdout?.on("data", (chunk: Buffer) => {
      stdout += chunk.toString();
    });

    child.stderr?.on("data", (chunk: Buffer) => {
      stderr += chunk.toString();
    });

    child.on("error", (err: NodeJS.ErrnoException) => {
      if (err.code === "ENOENT") {
        vscode.window.showErrorMessage("tiefdownconverter CLI not found in PATH. Install it to convert Tiefdown projects.");
      }
      reject(err);
    });

    child.on("close", (code) => {
      if (code !== 0) {
        const message = stderr || stdout || "Failed to list templates for this project.";
        reject(new Error(message));
        return;
      }

      const lines = stdout.split(/\r?\n/);
      const templates = lines.map((line) => line.trimEnd()).filter((line) => line.length > 0 && !line.startsWith(" "));
      resolve(templates);
    });
  });
}

async function pickTemplates(project: TiefdownProject, templates: string[]): Promise<string[] | undefined> {
  try {
    const selection = await vscode.window.showQuickPick(
      templates.map((template) => ({
        label: template,
      })),
      {
        canPickMany: true,
        placeHolder: "Select templates to convert (leave empty to convert all templates)",
      }
    );

    if (selection === undefined) {
      return undefined;
    }

    return selection.map((item) => item.label);
  } catch (error) {
    const message = error instanceof Error ? error.message : "Failed to list Tiefdown templates.";
    const channel = getOutputChannel();
    channel.appendLine(`Template discovery failed for ${describeProject(project)}: ${message}`);
    if (error instanceof Error && error.stack) {
      channel.appendLine(error.stack);
    }
    vscode.window.showErrorMessage(message);
    return undefined;
  }
}

async function pickProfiles(project: TiefdownProject, availableProfiles: string[]): Promise<vscode.QuickPickItem | undefined> {
  try {
    const selection = await vscode.window.showQuickPick(
      availableProfiles.map((profile) => ({
        label: profile,
      })),
      {
        canPickMany: false,
        placeHolder: "Select profile to convert (leave empty to convert all profiles)",
      }
    );
    return selection;
  } catch (error) {
    const message = error instanceof Error ? error.message : "Failed to list Tiefdown profiles.";
    const channel = getOutputChannel();
    channel.appendLine(`Profile discovery failed for ${describeProject(project)}: ${message}`);
    if (error instanceof Error && error.stack) {
      channel.appendLine(error.stack);
    }
    vscode.window.showErrorMessage(message);
    return undefined;
  }
}

async function runConvert(project: TiefdownProject, templates: string[] | undefined, profile: string | undefined): Promise<void> {
  const channel = getOutputChannel();
  channel.show(true);
  const args = ["convert"];
  let commandSummary = "tiefdownconverter convert";
  if (templates && templates.length > 0) {
    args.push("--templates", ...templates);
    commandSummary = `tiefdownconverter convert --templates ${templates.join(", ")}`;
  }
  if (profile) {
    args.push("--profile", profile);
    commandSummary += ` --profile ${profile}`;
  }
  channel.appendLine(`Running "${commandSummary}" in ${project.projectUri.fsPath}`);

  await new Promise<void>((resolve, reject) => {
    const child = spawn("tiefdownconverter", args, {
      cwd: project.projectUri.fsPath,
    });

    child.stdout?.on("data", (chunk: Buffer) => {
      channel.append(chunk.toString());
    });

    child.stderr?.on("data", (chunk: Buffer) => {
      channel.append(chunk.toString());
    });

    child.on("error", (err: NodeJS.ErrnoException) => {
      if (err.code === "ENOENT") {
        vscode.window.showErrorMessage("tiefdownconverter CLI not found in PATH. Install it to convert Tiefdown projects.");
      } else {
        vscode.window.showErrorMessage(`Failed to start tiefdownconverter: ${err.message}`);
      }
      reject(err);
    });

    child.on("close", (code) => {
      if (code === 0) {
        vscode.window.showInformationMessage(`Tiefdown conversion finished for ${describeProject(project)}.`);
        resolve();
        return;
      }

      vscode.window.showErrorMessage("tiefdownconverter convert exited with an error. Check the Tiefdown Converter output for details.");
      reject(new Error(`tiefdownconverter convert exited with code ${code ?? "unknown"}`));
    });
  });
}

async function updateManifestContext(): Promise<void> {
  const projects = await findProjectsWithManifest();
  vscode.commands.executeCommand("setContext", "tiefdownProjectSupport.hasManifest", projects.length > 0);
}

export async function activate(context: vscode.ExtensionContext) {
  console.log("tiefdown-project-support extension activated");

  await updateManifestContext();

  const manifestWatcher = vscode.workspace.createFileSystemWatcher("**/manifest.toml");
  context.subscriptions.push(
    manifestWatcher,
    manifestWatcher.onDidCreate(() => {
      void updateManifestContext();
    }),
    manifestWatcher.onDidDelete(() => {
      void updateManifestContext();
    }),
    manifestWatcher.onDidChange(() => {
      void updateManifestContext();
    })
  );

  const disposable = vscode.commands.registerCommand("tiefdown-project-support.convertProject", async (resourceUri?: vscode.Uri) => {
    let project: TiefdownProject | undefined;
    if (resourceUri && isUri(resourceUri)) {
      project = await getProjectFromFolderUri(resourceUri);
      if (!project) {
        vscode.window.showErrorMessage("No manifest.toml found in the selected folder.");
        return;
      }
    } else {
      const projects = await findProjectsWithManifest();
      if (projects.length === 0) {
        vscode.window.showErrorMessage("No manifest.toml found in the current workspace.");
        return;
      }

      project = await pickProject(projects);
      if (!project) {
        return;
      }
    }

    let availableProfiles: string[] = await listProfiles(project);
    let availableTemplates: string[] = await listTemplates(project);

    const conversionModes = [{ label: "Convert all templates", mode: "all" }];

    if (availableTemplates.length > 0) {
      conversionModes.push({ label: "Select templates to convert", mode: "templates" });
    }

    if (availableProfiles.length > 0) {
      conversionModes.push({ label: "Select profiles to convert", mode: "profiles" });
    }

    const choseConversionMode = await vscode.window.showQuickPick(conversionModes, {
      placeHolder: "Select conversion mode",
    });

    if (!choseConversionMode) {
      return;
    }

    if (choseConversionMode.mode === "profiles") {
      const selectedProfiles = await pickProfiles(project, availableProfiles);
      if (selectedProfiles === undefined) {
        return;
      }

      const profilesToConvert = selectedProfiles ? selectedProfiles.label : undefined;

      await runConvert(project, undefined, profilesToConvert).catch((error) => {
        console.error("tiefdownconverter convert failed", error);
      });
      return;
    }

    if (choseConversionMode.mode === "templates") {
      const selectedTemplates = await pickTemplates(project, availableTemplates);
      if (selectedTemplates === undefined) {
        return;
      }

      await runConvert(project, selectedTemplates, undefined).catch((error) => {
        console.error("tiefdownconverter convert failed", error);
      });
      return;
    }

    await runConvert(project, undefined, undefined).catch((error) => {
      console.error("tiefdownconverter convert failed", error);
    });
  });

  context.subscriptions.push(disposable);

  context.subscriptions.push(
    vscode.workspace.onDidChangeWorkspaceFolders(() => {
      void updateManifestContext();
    })
  );
}

export function deactivate() {}
