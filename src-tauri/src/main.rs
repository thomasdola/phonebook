// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod db;
mod models;
mod schema;

extern crate diesel;
extern crate diesel_migrations;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

use crate::controller::{fetch, parse, wipe};

fn main() {
    let connection = &mut db::establish_connection();

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error on migrating");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse, fetch, wipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
