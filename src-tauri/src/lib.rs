use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, mpsc, Arc},
};

use notify::{
    event::{DataChange, ModifyKind},
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use tauri::{Emitter, Runtime};
use tokio::{
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver},
        Mutex,
    },
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

#[derive(Debug)]
struct TaskState {
    token: CancellationToken,
    handle: JoinHandle<()>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub log_listener: Arc<Mutex<Option<TaskState>>>,
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, UnboundedReceiver<notify::Result<Event>>)>
{
    let (tx, rx) = unbounded_channel();

    let watcher = RecommendedWatcher::new(
        move |res| {
            tauri::async_runtime::block_on(async {
                tx.send(res).unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

fn handle_message<R: Runtime>(
    app: tauri::AppHandle<R>,
    msg: Result<Event, notify::Error>,
    position: &mut u64,
    path: &Path,
) {
    match msg {
        Ok(event)
            if matches!(
                event.kind,
                EventKind::Modify(ModifyKind::Data(DataChange::Content))
            ) =>
        {
            let mut f = File::open(path).unwrap();
            f.seek(SeekFrom::Start(*position)).unwrap();

            let mut new_log_content = String::new();
            f.read_to_string(&mut new_log_content).unwrap();

            // Update position for the next read
            *position = f.stream_position().unwrap();

            if new_log_content.is_empty() {
                return;
            }

            for line in new_log_content.lines() {
                todo!("Filter actual logs for 'high damage'")
            }
        }
        Ok(_) => (),
        Err(err) => panic!("encountered an error: {err}"),
    }
}

async fn listener_loop<R: Runtime>(
    app: tauri::AppHandle<R>,
    path: PathBuf,
    cancellation_token: CancellationToken,
) {
    let (mut watcher, mut rx) = async_watcher().unwrap();

    let mut position = File::open(&path)
        .unwrap_or_else(|_| panic!("Couldn't open file {}", path.display()))
        .seek(SeekFrom::End(0))
        .unwrap();

    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    loop {
        tokio::select! {
            _ = cancellation_token.cancelled() => break,
            msg = rx.recv() => handle_message(app.clone(), msg.unwrap(), &mut position, &path),

        }
    }
}

#[cfg(target_os = "linux")]
pub const EE_LOG: &str = "~/.local/share/Steam/steamapps/compatdata/230410/pfx/drive_c/users/steamuser/Local Settings/Application Data/Warframe/";

#[cfg(target_os = "windows")]
pub const EE_LOG: &str = r#"%localappdata%\Warframe\EE.log"#;

#[tauri::command]
async fn start<R: Runtime>(
    app: tauri::AppHandle<R>,
    state: tauri::State<'_, AppState>,
    file_path: Option<String>,
) -> Result<(), String> {
    let path = PathBuf::from(file_path.unwrap_or(EE_LOG.to_owned()));

    let mut listener_handle = state.log_listener.lock().await;

    let token = CancellationToken::new();

    let handle = tokio::spawn(listener_loop(app, path, token.clone()));

    *listener_handle = Some(TaskState { token, handle });

    Ok(())
}

pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            log_listener: Arc::new(Mutex::new(None)),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
