use std::error::Error;
use std::{env, fs, time, u8};

#[derive(serde::Deserialize)]
struct Chapter {
    name: String,
}

#[derive(serde::Deserialize)]
struct Selection {
    translation: String,
    book: usize,
    chapter: usize,
}

#[derive(serde::Deserialize)]
struct Verse {
    verse: usize,
    text: String,
}

pub fn initialize() -> Result<(), Box<dyn Error>> {
    let index = format!("{}/translations.json", get_data_dir());
    if !fs::exists(&index)?
        || fs::metadata(&index)
            .unwrap()
            .modified()
            .unwrap()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            > 86400
    {
        let _ = save_index();
    }
    let _ = save_selection(&None, &None, &None);
    let selection = get_selection().unwrap();
    if !fs::exists(format!("{}/{}.json", get_data_dir(), &selection.0))? {
        let _ = download_file(&selection.0);
    }
    Ok(())
}

fn save_selection(
    translation: &Option<&str>,
    book: &Option<usize>,
    chapter: &Option<usize>,
) -> Result<(), Box<dyn Error>> {
    let file = format!("{}/selection.json", get_data_dir());
    let mut data = serde_json::json!({
        "translation": "schlachter",
        "book": 0,
        "chapter": 0
    });
    if fs::exists(&file)? {
        data = serde_json::from_str(&file)?;
        fs::remove_file(&file)?;
    }
    if translation.is_some() {
        let value: serde_json::Value = serde_json::from_str(&translation.unwrap()).unwrap();
        data["translation"] = value;
    }
    if book.is_some() {
        let value: serde_json::Value = book.unwrap().into();
        data["book"] = value;
    }
    if chapter.is_some() {
        let value: serde_json::Value = chapter.unwrap().into();
        data["chapter"] = value;
    }
    fs::write(&file, data.to_string())?;
    Ok(())
}

pub fn get_selection() -> Result<(String, usize, usize), Box<dyn Error>> {
    let file = fs::read_to_string(format!("{}/selection.json", get_data_dir()))?;
    let data: Selection = serde_json::from_str(&file)?;
    Ok((data.translation, data.book, data.chapter))
}

fn get_data_dir() -> String {
    format!(
        "{}/.local/share/Bible4TUI",
        env::home_dir().unwrap().display()
    )
}

fn save_index() -> Result<(), Box<dyn Error>> {
    let _ = download_file("translations");
    Ok(())
}

fn download_file(abbrev: &str) -> Result<(), Box<dyn Error>> {
    let turl = format!("https://api.getbible.net/v2/{}.json", &abbrev);
    let translation = reqwest::blocking::get(turl)?.bytes()?;
    fs::write(format!("{}/{}.json", get_data_dir(), &abbrev), &translation)?;
    Ok(())
}

fn check_update(abbrev: &str) -> Result<bool, Box<dyn Error>> {
    if !fs::exists(format!("{}/{}.json", get_data_dir(), &abbrev))? {
        return Ok(true);
    };
    let checksums = get_checksum(&abbrev, true)?;
    Ok(checksums.0 != checksums.1)
}
fn get_checksum(translation: &str, index: bool) -> Result<(String, String), Box<dyn Error>> {
    let data_dir = get_data_dir();
    let translation_checksum = sha256::try_digest(format!("{}/{}.json", data_dir, translation))
        .unwrap()
        .to_string();
    let index_checksum = if index {
        let file = fs::read_to_string(format!("{}/translations.json", data_dir)).unwrap();
        let data: serde_json::Value = serde_json::from_str(&file)?;
        data[translation]["sha"].to_string()
    } else {
        String::new()
    };
    Ok((translation_checksum, index_checksum))
}

pub fn get_chapter(
    translation: &str,
    book: usize,
    chapter: usize,
) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    let file: String = fs::read_to_string(format!("{}/{}.json", get_data_dir(), translation))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let verses: Vec<Verse> =
        serde_json::from_value(json["books"][book]["chapters"][chapter]["verses"].clone())?;
    println!("{}", json["books"][book]["chapters"][chapter]["name"]);
    for verse in verses {
        text.push_str(format!("{} {}\n", verse.verse, verse.text).as_str());
    }
    Ok(text)
}
