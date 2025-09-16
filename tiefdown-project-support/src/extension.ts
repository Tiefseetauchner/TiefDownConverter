import { spawn } from 'node:child_process';
import * as vscode from 'vscode';

type TiefdownProject = {
	folder: vscode.WorkspaceFolder;
	manifestUri: vscode.Uri;
};

interface ProjectQuickPickItem extends vscode.QuickPickItem {
	project: TiefdownProject;
}

let outputChannel: vscode.OutputChannel | undefined;

function getOutputChannel(): vscode.OutputChannel {
	if (!outputChannel) {
		outputChannel = vscode.window.createOutputChannel('Tiefdown Converter');
	}
	return outputChannel;
}

async function findProjectsWithManifest(): Promise<TiefdownProject[]> {
	const folders = vscode.workspace.workspaceFolders;
	if (!folders) {
		return [];
	}

	const projects: TiefdownProject[] = [];
	for (const folder of folders) {
		const manifestUri = vscode.Uri.joinPath(folder.uri, 'manifest.toml');
		try {
			await vscode.workspace.fs.stat(manifestUri);
			projects.push({ folder, manifestUri });
		} catch (error) {
			if (error instanceof vscode.FileSystemError && error.code === 'FileNotFound') {
				continue;
			}
			console.error(`Failed to access ${manifestUri.fsPath}`, error);
		}
	}

	return projects;
}

async function pickProject(projects: TiefdownProject[]): Promise<TiefdownProject | undefined> {
	if (projects.length === 0) {
		return undefined;
	}

	if (projects.length === 1) {
		return projects[0];
	}

	const items: ProjectQuickPickItem[] = projects.map((project) => ({
		label: project.folder.name,
		description: vscode.workspace.asRelativePath(project.manifestUri, false),
		project,
	}));

	const selection = await vscode.window.showQuickPick(items, {
		placeHolder: 'Select a Tiefdown project to convert',
	});

	return selection?.project;
}

async function runConvert(project: TiefdownProject): Promise<void> {
	const channel = getOutputChannel();
	channel.show(true);
	channel.appendLine(`Running "tiefdownconverter convert" in ${project.folder.uri.fsPath}`);

	await new Promise<void>((resolve, reject) => {
		const child = spawn('tiefdownconverter', ['convert'], {
			cwd: project.folder.uri.fsPath,
		});

		child.stdout?.on('data', (chunk: Buffer) => {
			channel.append(chunk.toString());
		});

		child.stderr?.on('data', (chunk: Buffer) => {
			channel.append(chunk.toString());
		});

		child.on('error', (err: NodeJS.ErrnoException) => {
			if (err.code === 'ENOENT') {
				vscode.window.showErrorMessage('tiefdownconverter CLI not found in PATH. Install it to convert Tiefdown projects.');
			} else {
				vscode.window.showErrorMessage(`Failed to start tiefdownconverter: ${err.message}`);
			}
			reject(err);
		});

		child.on('close', (code) => {
			if (code === 0) {
				vscode.window.showInformationMessage(`Tiefdown conversion finished for ${project.folder.name}.`);
				resolve();
				return;
			}

			vscode.window.showErrorMessage('tiefdownconverter convert exited with an error. Check the Tiefdown Converter output for details.');
			reject(new Error(`tiefdownconverter convert exited with code ${code ?? 'unknown'}`));
		});
	});
}

async function updateManifestContext(): Promise<void> {
	const projects = await findProjectsWithManifest();
	vscode.commands.executeCommand('setContext', 'tiefdownProjectSupport.hasManifest', projects.length > 0);
}

export async function activate(context: vscode.ExtensionContext) {
	console.log('tiefdown-project-support extension activated');

	await updateManifestContext();

	const manifestWatcher = vscode.workspace.createFileSystemWatcher('**/manifest.toml');
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

	const disposable = vscode.commands.registerCommand('tiefdown-project-support.convertProject', async () => {
		const projects = await findProjectsWithManifest();
		if (projects.length === 0) {
			vscode.window.showErrorMessage('No manifest.toml found in the current workspace.');
			return;
		}

		const project = await pickProject(projects);
		if (!project) {
			return;
		}

		await runConvert(project).catch((error) => {
			console.error('tiefdownconverter convert failed', error);
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
