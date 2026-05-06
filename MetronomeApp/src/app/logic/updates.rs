use std::{
    fs::File,
    io,
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    thread,
};

use directories::UserDirs;
use semver::Version;
use serde::Deserialize;

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/UnbrokenHunter/Metronome-App/releases/latest";

const USER_AGENT: &str = "MetronomeApp";

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub installer_url: Option<String>,
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Idle,
    Downloading,
    Downloaded(PathBuf),
    Failed(String),
}

pub struct UpdateRuntime {
    check_started: bool,
    dismissed: bool,
    info: Option<UpdateInfo>,
    download_status: DownloadStatus,
    receiver: Option<Receiver<UpdateMessage>>,
}

impl Default for UpdateRuntime {
    fn default() -> Self {
        Self {
            check_started: false,
            dismissed: false,
            info: None,
            download_status: DownloadStatus::Idle,
            receiver: None,
        }
    }
}

enum UpdateMessage {
    CheckFinished(Result<Option<UpdateInfo>, String>),
    DownloadFinished(Result<PathBuf, String>),
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

pub fn start_update_check(runtime: &mut UpdateRuntime) {
    if runtime.check_started {
        return;
    }

    runtime.check_started = true;

    let (sender, receiver) = mpsc::channel();
    runtime.receiver = Some(receiver);

    thread::spawn(move || {
        let result = check_for_update();
        let _ = sender.send(UpdateMessage::CheckFinished(result));
    });
}

pub fn receive_update_messages(runtime: &mut UpdateRuntime) {
    let Some(receiver) = &runtime.receiver else {
        return;
    };

    while let Ok(message) = receiver.try_recv() {
        match message {
            UpdateMessage::CheckFinished(result) => match result {
                Ok(Some(info)) => {
                    runtime.info = Some(info);
                    runtime.download_status = DownloadStatus::Idle;
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("Update check failed: {error}");
                }
            },

            UpdateMessage::DownloadFinished(result) => match result {
                Ok(path) => {
                    runtime.download_status = DownloadStatus::Downloaded(path);
                }
                Err(error) => {
                    runtime.download_status = DownloadStatus::Failed(error);
                }
            },
        }
    }
}

pub fn draw_update_popup(ctx: &egui::Context, runtime: &mut UpdateRuntime) {
    if runtime.dismissed {
        return;
    }

    let Some(update) = runtime.info.clone() else {
        return;
    };

    enum Action {
        Download,
        OpenReleasePage,
        OpenDownloadedFile(PathBuf),
        Dismiss,
    }

    let mut action = None;

    egui::Window::new("Update available")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(format!(
                "A new version of MetronomeApp is available.\n\nCurrent: {}\nLatest: {}",
                update.current_version, update.latest_version
            ));

            ui.add_space(8.0);

            match runtime.download_status.clone() {
                DownloadStatus::Idle => {
                    ui.horizontal(|ui| {
                        if update.installer_url.is_some()
                            && ui.button("Download installer").clicked()
                        {
                            action = Some(Action::Download);
                        }

                        if ui.button("Open release page").clicked() {
                            action = Some(Action::OpenReleasePage);
                        }

                        if ui.button("Remind me later").clicked() {
                            action = Some(Action::Dismiss);
                        }
                    });
                }

                DownloadStatus::Downloading => {
                    ui.label("Downloading installer...");
                }

                DownloadStatus::Downloaded(path) => {
                    ui.label(format!("Downloaded to:\n{}", path.display()));

                    ui.horizontal(|ui| {
                        if ui.button("Open installer").clicked() {
                            action = Some(Action::OpenDownloadedFile(path.clone()));
                        }

                        if ui.button("Close").clicked() {
                            action = Some(Action::Dismiss);
                        }
                    });
                }

                DownloadStatus::Failed(error) => {
                    ui.label(format!("Download failed:\n{error}"));

                    ui.horizontal(|ui| {
                        if ui.button("Open release page").clicked() {
                            action = Some(Action::OpenReleasePage);
                        }

                        if ui.button("Close").clicked() {
                            action = Some(Action::Dismiss);
                        }
                    });
                }
            }
        });

    match action {
        Some(Action::Download) => start_installer_download(runtime),
        Some(Action::OpenReleasePage) => {
            let _ = open::that(update.release_url);
            runtime.dismissed = true;
        }
        Some(Action::OpenDownloadedFile(path)) => {
            let _ = open::that(path);
            runtime.dismissed = true;
        }
        Some(Action::Dismiss) => {
            runtime.dismissed = true;
        }
        None => {}
    }
}

fn start_installer_download(runtime: &mut UpdateRuntime) {
    let Some(update) = runtime.info.clone() else {
        return;
    };

    if matches!(runtime.download_status, DownloadStatus::Downloading) {
        return;
    }

    runtime.download_status = DownloadStatus::Downloading;

    let (sender, receiver) = mpsc::channel();
    runtime.receiver = Some(receiver);

    thread::spawn(move || {
        let result = download_installer(&update);
        let _ = sender.send(UpdateMessage::DownloadFinished(result));
    });
}

fn check_for_update() -> Result<Option<UpdateInfo>, String> {
    let current = Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|error| format!("Invalid current app version: {error}"))?;

    let release: GithubRelease = ureq::get(LATEST_RELEASE_URL)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|error| format!("Failed to reach GitHub releases: {error}"))?
        .body_mut()
        .read_json()
        .map_err(|error| format!("Failed to parse GitHub release response: {error}"))?;

    let latest_text = release.tag_name.trim_start_matches('v');

    let latest = Version::parse(latest_text)
        .map_err(|error| format!("Invalid GitHub release version: {error}"))?;

    if latest <= current {
        return Ok(None);
    }

    let installer_url = release
        .assets
        .iter()
        .find(|asset| asset.name == installer_asset_name())
        .map(|asset| asset.browser_download_url.clone());

    Ok(Some(UpdateInfo {
        current_version: current.to_string(),
        latest_version: latest.to_string(),
        release_url: release.html_url,
        installer_url,
    }))
}

fn download_installer(update: &UpdateInfo) -> Result<PathBuf, String> {
    let installer_url = update
        .installer_url
        .as_ref()
        .ok_or_else(|| "No installer asset was found for this operating system.".to_string())?;

    let output_path = downloads_dir().join(installer_asset_name());

    let mut response = ureq::get(installer_url)
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|error| format!("Failed to download installer: {error}"))?;

    let mut reader = response.body_mut().as_reader();

    let mut file = File::create(&output_path)
        .map_err(|error| format!("Failed to create installer file: {error}"))?;

    io::copy(&mut reader, &mut file)
        .map_err(|error| format!("Failed to save installer: {error}"))?;

    Ok(output_path)
}

fn downloads_dir() -> PathBuf {
    UserDirs::new()
        .and_then(|dirs| dirs.download_dir().map(PathBuf::from))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn installer_asset_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "MetronomeApp-windows-installer.msi"
    }

    #[cfg(target_os = "macos")]
    {
        "MetronomeApp-macos.zip"
    }

    #[cfg(target_os = "linux")]
    {
        "MetronomeApp-linux.tar.gz"
    }
}
