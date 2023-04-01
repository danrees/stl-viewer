use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    id: Option<Thing>,
    path: String,
}

#[tauri::command]
pub async fn save_library(
    name: &str,
    path: &str,
    db: State<'_, Surreal<Db>>,
) -> Result<Library, String> {
    println!("Calling save library with: {} {}", name, path);
    let l: Library = db
        .create(("library", name))
        .content(Library {
            id: None,
            path: path.into(),
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(l)
}

#[tauri::command]
pub async fn list_libraries(db: State<'_, Surreal<Db>>) -> Result<Vec<Library>, String> {
    let l: Vec<Library> = db.select("library").await.map_err(|e| e.to_string())?;
    println!("Found: {:?}", l);
    Ok(l)
}

#[tauri::command]
pub async fn delete_library(id: (&str, &str), db: State<'_, Surreal<Db>>) -> Result<(), String> {
    println!("{:?}", id);
    let l: Option<Library> = db.delete(id).await.map_err(|e| e.to_string())?;
    Ok(())
}
