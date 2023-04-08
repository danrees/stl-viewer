use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use surrealdb::{engine::local::Db, sql::Thing, Surreal};
use tauri::State;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Library {
    id: Option<Thing>,
    name: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    id: Option<Thing>,
    name: String,
    extension: String,
    pub path: String,
    tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    id: Option<Thing>,
    value: String,
}

#[tauri::command]
pub async fn save_library(
    name: &str,
    path: &str,
    db: State<'_, Surreal<Db>>,
) -> Result<Library, String> {
    let l: Library = db
        .create("library")
        .content(Library {
            id: None,
            name: name.into(),
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
    db.delete(id).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn scan_library(
    id: (&str, &str),
    extension: &str,
    db: State<'_, Surreal<Db>>,
) -> Result<(), String> {
    let library: Library = db.select(id).await.map_err(|e| e.to_string())?;

    for entry in WalkDir::new(&library.path)
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| {
            if let Some(ext) = f.path().extension() {
                return ext == extension;
            }
            return false;
        })
    {
        let s = Sha256::new();

        let name = entry.file_name().to_string_lossy().into_owned();
        let hashed_name = s.chain_update(&name).finalize();
        let hashed_name_string = base16ct::lower::encode_string(&hashed_name);

        let tags = entry
            .path()
            .parent()
            .unwrap_or(&entry.path())
            .strip_prefix(&library.path)
            .unwrap_or(&entry.path())
            .components()
            .into_iter()
            .map(|entry| Tag {
                id: None,
                value: entry.as_os_str().to_string_lossy().into_owned(),
            })
            .collect::<Vec<Tag>>();
        let mut tags_to_save = Vec::new();
        for tag in tags.iter() {
            let t: Tag = db
                .update(("tag", tag.value.clone()))
                .content(tag)
                .await
                .unwrap();
            tags_to_save.push(t);
        }
        // let updated_tags: Vec<Future<Tag>> = tags
        //     .into_iter()
        //     .map(|tag| db.update(("tag", tag.value)).content(tag))
        //     .collect();

        let to_save = File {
            id: Some(Thing::from(("3dfile", hashed_name_string.as_str()))),
            extension: entry
                .path()
                .extension()
                .unwrap_or(&OsString::from("none"))
                .to_string_lossy()
                .into_owned(),
            name: name,
            path: entry.path().as_os_str().to_string_lossy().into_owned(),
            tags: tags_to_save,
        };

        let f: Option<File> = db
            .update(("3dfile", &hashed_name_string))
            .content(to_save)
            .await
            .map_err(|e| e.to_string())
            .unwrap();
        println!("Updated: {:?}", f);
    }

    Ok(())
}

#[tauri::command]
pub async fn list_files(db: State<'_, Surreal<Db>>) -> Result<Vec<File>, String> {
    let files: Vec<File> = db.select("3dfile").await.map_err(|e| e.to_string())?;
    println!("Found {} files", files.len());
    Ok(files)
}

#[tauri::command]
pub async fn get_tags(db: State<'_, Surreal<Db>>) -> Result<Vec<Tag>, String> {
    let tags = db.select("tag").await.map_err(|e| e.to_string())?;
    Ok(tags)
}
