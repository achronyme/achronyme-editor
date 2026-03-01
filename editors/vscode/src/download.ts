import * as os from "os";
import * as fs from "fs";
import * as path from "path";
import * as https from "https";
import { execFile } from "child_process";
import { workspace, window, ProgressLocation, CancellationToken } from "vscode";

const GITHUB_REPO = "achronyme/achronyme";
const RELEASES_API = `https://api.github.com/repos/${GITHUB_REPO}/releases/latest`;
const INSTALL_DIR = path.join(os.homedir(), ".achronyme", "bin");
const VERSION_FILE = path.join(INSTALL_DIR, ".ach-version");

function getArtifactName(): string | null {
  const platform = os.platform();
  const arch = os.arch();

  if (platform === "linux" && arch === "x64") return "achronyme-linux-x86_64";
  if (platform === "darwin" && arch === "x64") return "achronyme-macos-x86_64";
  if (platform === "darwin" && arch === "arm64") return "achronyme-macos-aarch64";
  if (platform === "win32" && arch === "x64") return "achronyme-windows-x86_64.exe";

  return null;
}

function getBinaryName(): string {
  return os.platform() === "win32" ? "ach.exe" : "ach";
}

function getAchBinaryPath(): string | null {
  // 1. User-configured path
  const config = workspace.getConfiguration("achronyme");
  const configured = config.get<string>("executablePath");
  if (configured && fs.existsSync(configured)) return configured;

  // 2. ~/.achronyme/bin/ach
  const installed = path.join(INSTALL_DIR, getBinaryName());
  if (fs.existsSync(installed)) return installed;

  return null;
}

function findOnPath(name: string): Promise<string | null> {
  const cmd = os.platform() === "win32" ? "where" : "which";
  return new Promise((resolve) => {
    execFile(cmd, [name], (err, stdout) => {
      if (err || !stdout.trim()) {
        resolve(null);
      } else {
        resolve(stdout.trim().split("\n")[0]);
      }
    });
  });
}

function httpsGetJson<T>(url: string): Promise<T> {
  return new Promise((resolve, reject) => {
    const req = https.get(url, { headers: { "User-Agent": "achronyme-vscode" } }, (res) => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        const location = res.headers.location;
        if (location) {
          httpsGetJson<T>(location).then(resolve, reject);
          return;
        }
      }
      if (res.statusCode !== 200) {
        reject(new Error(`HTTP ${res.statusCode}`));
        return;
      }
      let data = "";
      res.on("data", (chunk: Buffer) => { data += chunk.toString(); });
      res.on("end", () => {
        try { resolve(JSON.parse(data) as T); }
        catch (e) { reject(e); }
      });
    });
    req.on("error", reject);
  });
}

function downloadFile(
  url: string,
  dest: string,
  onProgress: (pct: number) => void,
  token: CancellationToken,
): Promise<void> {
  return new Promise((resolve, reject) => {
    if (token.isCancellationRequested) { reject(new Error("Cancelled")); return; }

    const req = https.get(url, { headers: { "User-Agent": "achronyme-vscode" } }, (res) => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        const location = res.headers.location;
        if (location) {
          downloadFile(location, dest, onProgress, token).then(resolve, reject);
          return;
        }
      }
      if (res.statusCode !== 200) {
        reject(new Error(`HTTP ${res.statusCode}`));
        return;
      }

      const total = parseInt(res.headers["content-length"] || "0", 10);
      let received = 0;

      fs.mkdirSync(path.dirname(dest), { recursive: true });
      const file = fs.createWriteStream(dest);

      res.on("data", (chunk: Buffer) => {
        received += chunk.length;
        if (total > 0) onProgress(Math.round((received / total) * 100));
      });
      res.pipe(file);
      file.on("finish", () => { file.close(() => resolve()); });
      file.on("error", reject);
    });

    req.on("error", reject);

    token.onCancellationRequested(() => {
      req.destroy();
      reject(new Error("Cancelled"));
    });
  });
}

interface GitHubAsset {
  name: string;
  browser_download_url: string;
}

interface GitHubRelease {
  tag_name: string;
  assets: GitHubAsset[];
}

async function downloadAchBinary(): Promise<string> {
  const artifactName = getArtifactName();
  if (!artifactName) {
    throw new Error(`Unsupported platform: ${os.platform()}/${os.arch()}`);
  }

  const release = await httpsGetJson<GitHubRelease>(RELEASES_API);
  const asset = release.assets.find((a) => a.name === artifactName);
  if (!asset) {
    throw new Error(
      `No binary found for ${artifactName} in release ${release.tag_name}`,
    );
  }

  const dest = path.join(INSTALL_DIR, getBinaryName());

  await window.withProgress(
    {
      location: ProgressLocation.Notification,
      title: `Downloading Achronyme CLI ${release.tag_name}...`,
      cancellable: true,
    },
    async (progress, token) => {
      await downloadFile(asset.browser_download_url, dest, (pct) => {
        progress.report({ increment: 0, message: `${pct}%` });
      }, token);
    },
  );

  // chmod +x on non-Windows
  if (os.platform() !== "win32") {
    fs.chmodSync(dest, 0o755);
  }

  // Save version
  fs.writeFileSync(VERSION_FILE, release.tag_name, "utf-8");

  window.showInformationMessage(
    `Achronyme CLI ${release.tag_name} installed to ${dest}`,
  );

  return dest;
}

async function checkForUpdate(): Promise<void> {
  if (!fs.existsSync(VERSION_FILE)) return;

  const currentVersion = fs.readFileSync(VERSION_FILE, "utf-8").trim();

  let release: GitHubRelease;
  try {
    release = await httpsGetJson<GitHubRelease>(RELEASES_API);
  } catch {
    return; // Silent fail — no internet, rate limit, etc.
  }

  if (release.tag_name === currentVersion) return;

  const choice = await window.showInformationMessage(
    `Achronyme CLI update available: ${release.tag_name} (current: ${currentVersion}). Update?`,
    "Update",
    "Not Now",
  );

  if (choice === "Update") {
    try {
      await downloadAchBinary();
    } catch (err) {
      window.showErrorMessage(`Failed to update Achronyme CLI: ${err}`);
    }
  }
}

export async function ensureAchBinary(): Promise<void> {
  // 1. Check configured path or installed binary
  const existing = getAchBinaryPath();
  if (existing) {
    // Already installed — check for updates in background
    checkForUpdate().catch(() => {});
    return;
  }

  // 2. Check PATH
  const onPath = await findOnPath("ach");
  if (onPath) return;

  // 3. Prompt user
  const choice = await window.showInformationMessage(
    "Achronyme CLI (ach) not found. Download from GitHub?",
    "Download",
    "Configure Path",
    "Not Now",
  );

  if (choice === "Download") {
    try {
      await downloadAchBinary();
    } catch (err) {
      window.showErrorMessage(`Failed to download Achronyme CLI: ${err}`);
    }
  } else if (choice === "Configure Path") {
    await workspace
      .getConfiguration("achronyme")
      .update("executablePath", "", true);
    // Open settings filtered to our config
    await import("vscode").then((vscode) =>
      vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "achronyme.executablePath",
      ),
    );
  }
}
