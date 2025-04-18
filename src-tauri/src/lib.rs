// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::{Duration, Local, NaiveDate};
use once_cell::sync::OnceCell;
use rusqlite::Connection;
use rusqlite::{Result, Row};
use serde::Serialize;
use std::cmp;
use std::fs;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};

#[derive(Serialize)]
pub struct Task {
    id: i32,
    name: String,
    url: String,
    level: i32,
    due_date: NaiveDate,
}

static DB: OnceCell<Mutex<Connection>> = OnceCell::new();

fn map_row_to_task(row: &Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: row.get(0)?,
        name: row.get(1)?,
        url: row.get(2)?,
        level: row.get(3)?,
        due_date: row.get::<_, String>(4)?.parse::<NaiveDate>().unwrap(),
    })
}

fn init_db(db_path: &str) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            url TEXT NOT NULL,
            level INTEGER NOT NULL,
            due_date TEXT NOT NULL)",
        (),
    )?;
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM tasks", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO tasks (name, url, level, due_date) VALUES (?1, ?2, ?3, ?4)",
            ("Add a task", "http://example.com", 0, "2025-04-08"),
        )?;
    }
    Ok(conn)
}

#[tauri::command]
fn get_all() -> Result<Vec<Task>, String> {
    let conn = DB.get().expect("DB not initialized").lock().unwrap();
    let mut stmt = match conn
        .prepare("SELECT id, name, url, level, due_date FROM tasks ORDER BY due_date DESC")
    {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };
    let task_iter = match stmt.query_map([], |row| map_row_to_task(row)) {
        Ok(iter) => iter,
        Err(e) => return Err(format!("Failed to execute query: {}", e)),
    };

    let tasks: Vec<Task> = match task_iter.collect::<Result<Vec<Task>, rusqlite::Error>>() {
        Ok(tasks) => tasks,
        Err(e) => return Err(format!("Failed to collect tasks: {}", e)),
    };
    Ok(tasks)
}

#[tauri::command]
fn get_due() -> Result<Vec<Task>, String> {
    let conn = DB.get().expect("DB not initialized").lock().unwrap();
    let mut stmt = match conn.prepare("SELECT id, name, url, level, due_date FROM tasks WHERE due_date <= ?1 ORDER BY due_date DESC") {
        Ok(stmt) => stmt,
        Err(e) => return Err(format!("Failed to prepare statement: {}", e)),
    };
    let today = Local::now().format("%Y-%m-%d").to_string();
    let task_iter = match stmt.query_map([&today], |row| map_row_to_task(row)) {
        Ok(iter) => iter,
        Err(e) => return Err(format!("Failed to execute query: {}", e)),
    };
    let tasks: Vec<Task> = match task_iter.collect::<Result<Vec<Task>, rusqlite::Error>>() {
        Ok(tasks) => tasks,
        Err(e) => return Err(format!("Failed to collect due tasks: {}", e)),
    };
    Ok(tasks)
}

#[tauri::command]
fn create_task(name: &str, url: &str) {
    let conn = DB.get().expect("DB not initialized").lock().unwrap();
    let today = Local::now().format("%Y-%m-%d").to_string();

    let _ = conn.execute(
        "INSERT INTO tasks (name, url, level, due_date) VALUES (?1, ?2, ?3, ?4)",
        (name, url, 0, today),
    );
}

#[tauri::command]
fn delete_task(id: i32) {
    let conn = DB.get().expect("DB not initialized").lock().unwrap();
    let _ = conn.execute("DELETE FROM tasks WHERE id=?1", (id,));
}

// TODO: update task (name/url)

#[tauri::command]
fn rate_task(id: i32, rating: i32) {
    let conn = DB.get().expect("DB not initialized").lock().unwrap();
    let mut task: Task = conn
        .query_row(
            "SELECT id, name, url, level, due_date from tasks WHERE id=?1",
            [id],
            map_row_to_task,
        )
        .expect("hihi");
    let interval: i64;
    match rating {
        // TODO: need more work
        1 => {
            // INFO: adjusted from 2 to 1.9 => 2 is too aggressive
            interval = 1.9_f32.powi(task.level).floor() as i64;
            task.level += 1
        }
        2 => {
            //INFO: chose 1.3 over 1.5
            interval = 1.3_f32.powi(task.level).floor() as i64;
            task.level += 1
        }
        3 => {
            //INFO: reduce level -> boost learning instead of reset
            task.level -= cmp::min(task.level, 3);
            interval = 1;
        }
        // WARN: should have error
        _ => interval = 0,
    }
    task.due_date = (Local::now() + Duration::days(interval as i64))
        .naive_local()
        .into();
    let _ = conn.execute(
        "UPDATE tasks SET level = ?1, due_date=?2 WHERE id=?3",
        (task.level, task.due_date.format("%Y-%m-%d").to_string(), id),
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let db_dir = format!("{home_dir}/.memobot/");
    let _ = fs::create_dir_all(db_dir);
    let db_path = format!("{home_dir}/.memobot/db.sqlite");
    let db_conn = init_db(&db_path).expect("Failed to initialize database");
    DB.set(Mutex::new(db_conn)).expect("DB already initialized");

    // TODO: system tray
    tauri::Builder::default()
        .setup(|app| {
            let quit = MenuItem::with_id(app, "quit", "Quit Memobot", true, None::<&str>)?;
            let hide = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit, &hide, &show])?;

            let _tray = TrayIconBuilder::new()
                // need .menu() to be visible, even if menu empty
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "hide" => {
                        // use tauri::Manager -> need it to use get_window
                        let window = app.get_window("main").unwrap();
                        let _ = window.hide();
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        let _ = window.show();
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app);
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // prevent app close when close window => keep systemtray available
                window.hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all,
            get_due,
            create_task,
            delete_task,
            rate_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
