use indexmap::IndexMap;
use std::{env, error::Error, fs, time, u8};

#[derive(serde::Deserialize)]
struct Bible {
    books: Vec<Book>,
    translation: String,
}

#[derive(serde::Deserialize)]
struct Book {
    chapters: Vec<Chapter>,
    name: String,
    nr: usize,
}

#[derive(serde::Deserialize)]
struct Chapter {
    name: String,
    verses: Vec<Verse>,
}

#[derive(serde::Deserialize)]
struct Selection {
    translation: String,
    book: usize,
    chapter: usize,
}

#[derive(serde::Deserialize)]
struct Translation {
    abbreviation: String,
    distribution_about: String,
    distribution_license: String,
    lang: String,
    language: String,
    sha: String,
    translation: String,
}

#[derive(serde::Deserialize)]
struct Verse {
    name: String,
    verse: usize,
    text: String,
}

pub fn initialize() -> Result<(), Box<dyn Error>> {
    let dir = get_data_dir();
    fs::create_dir_all(&dir)?;
    let index = format!("{}/translations.json", &dir);
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
        save_index()?;
    }
    save_selection(None, None, None)?;
    let selection = get_selection().unwrap();
    download_file(&selection.0, false)
}

pub fn save_selection(
    translation: Option<&str>,
    book: Option<usize>,
    chapter: Option<usize>,
) -> Result<(), Box<dyn Error>> {
    let file = format!("{}/selection.json", get_data_dir());
    let previous = get_selection().unwrap_or(default_selection());
    let new_translation: &str = translation.unwrap_or(&previous.0);
    let new_book: usize = book.unwrap_or(previous.1);
    let new_chapter: usize = chapter.unwrap_or(previous.2);
    let data = serde_json::json!({
        "translation": new_translation,
        "book": new_book,
        "chapter": new_chapter
    });
    fs::write(&file, data.to_string())?;
    Ok(())
}

fn default_selection() -> (String, usize, usize) {
    ("schlachter".to_string(), 0, 0)
}

pub fn turn_chapter(direction: bool, amount: Option<usize>) -> Result<(), Box<dyn Error>> {
    let previous = get_selection().unwrap_or(default_selection());
    let count = get_count(&previous.0, Some(previous.1)).unwrap_or((1, Some(1)));
    let amount = amount.unwrap_or(1);
    let new_selection = if !direction && previous.2 < amount {
        let new_book: usize = (previous.1 as isize - 1)
            .clamp(0, (count.0 - 1) as isize)
            .try_into()
            .unwrap();
        let new_chapter = if previous.1 == 0 {
            0
        } else {
            let count = get_count(&previous.0, Some(new_book)).unwrap_or((1, Some(1)));
            previous.2 + count.1.unwrap() - amount
        };
        (new_book, new_chapter)
    } else if direction && previous.2 >= count.1.unwrap() - amount {
        let new_book = (previous.1 + 1).clamp(0, count.0 - 1);
        let new_chapter = if previous.1 >= count.0 - 1 {
            previous.2
        } else {
            amount + previous.2 - count.1.unwrap()
        };
        (new_book, new_chapter)
    } else {
        if direction {
            (previous.1, previous.2 + amount)
        } else {
            (previous.1, previous.2 - amount)
        }
    };
    save_selection(None, Some(new_selection.0), Some(new_selection.1))
}

pub fn turn_book(direction: bool) -> Result<(), Box<dyn Error>> {
    let previous = get_selection().unwrap_or(default_selection());
    let count = get_count(&previous.0, Some(previous.1)).unwrap_or((1, Some(1)));
    let new_book = if !direction && previous.1 <= 0 {
        0
    } else if direction && previous.1 >= count.0 - 1 {
        previous.1
    } else {
        if direction {
            previous.1 + 1
        } else {
            previous.1 - 1
        }
    };
    let new_count = get_count(&previous.0, Some(new_book)).unwrap_or((1, Some(1)));
    let new_chapter = previous.2.clamp(0, new_count.1.unwrap() - 1);
    save_selection(None, Some(new_book), Some(new_chapter))
}

pub fn turn_translation(direction: bool) -> Result<(), Box<dyn Error>> {
    let previous = get_selection().unwrap_or(default_selection());
    let translations = get_translation_map().unwrap();
    let index = translations
        .iter()
        .position(|item| item.1.abbreviation == previous.0)
        .unwrap_or(0);
    let new_index = if direction {
        index + 1
    } else {
        (index as isize - 1) as usize
    }
    .clamp(0, translations.len() - 1);
    let new_translation = translations.get_index(new_index).unwrap().0;
    download_file(new_translation, false)?;
    let previous = get_selection().unwrap_or(default_selection());
    let count = get_count(new_translation, Some(previous.1)).unwrap_or((1, Some(1)));
    save_selection(
        Some(new_translation),
        Some(previous.1.clamp(0, count.0 - 1)),
        Some(previous.2.clamp(0, count.1.unwrap() - 1)),
    )
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
    download_file("translations", true)
}

fn download_file(abbrev: &str, force: bool) -> Result<(), Box<dyn Error>> {
    if force || check_update(abbrev)? {
        let turl = format!("https://api.getbible.net/v2/{}.json", &abbrev);
        let translation = reqwest::blocking::get(turl)?.bytes()?;
        fs::write(format!("{}/{}.json", get_data_dir(), &abbrev), &translation)?;
    }
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
    let file = fs::read_to_string(format!("{}/{}.json", get_data_dir(), translation))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let verses: Vec<Verse> =
        serde_json::from_value(json["books"][book]["chapters"][chapter]["verses"].clone())?;
    for verse in verses {
        text.push_str(format!("{} {}\n", verse.verse, verse.text).as_str());
    }
    Ok(text)
}

pub fn get_count(
    translation: &str,
    book: Option<usize>,
) -> Result<(usize, Option<usize>), Box<dyn Error>> {
    let file = fs::read_to_string(format!("{}/{}.json", get_data_dir(), translation))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let books: Vec<Book> = serde_json::from_value(json["books"].clone())?;
    let chapter_count = if book.is_some() {
        let chapters: Vec<Chapter> =
            serde_json::from_value(json["books"][book.unwrap()]["chapters"].clone())?;
        Some(chapters.len())
    } else {
        None
    };
    Ok((books.len(), chapter_count))
}

pub fn get_book_list(translation: &str, selected: usize) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    let file = fs::read_to_string(format!("{}/{}.json", get_data_dir(), translation))?;
    let json: serde_json::Value = serde_json::from_str(&file)?;
    let books: Vec<Book> = serde_json::from_value(json["books"].clone())?;
    for book in books {
        text.push_str(
            format!(
                "{}{}\n",
                if book.nr - 1 == selected { ">" } else { "" },
                &book.name
            )
            .as_str(),
        );
    }
    Ok(text)
}

pub fn get_chapter_list(
    translation: &str,
    book: usize,
    selected: usize,
) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    let count = get_count(translation, Some(book)).unwrap();
    for i in 1..count.1.unwrap() + 1 {
        if i - 1 == selected {
            text.push_str(format!(">{:>3}  ", i).as_str());
        } else {
            text.push_str(format!("{:>4}  ", i).as_str());
        }
        if i % 3 == 0 {
            text = text.trim_end().to_string();
            text.push_str("\n")
        };
    }
    Ok(text)
}

pub fn get_translation_list(selected: &str) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    let translations = get_translation_map().unwrap();
    for item in translations {
        text.push_str(
            format!(
                "{}{}\n",
                if item.1.abbreviation == selected {
                    ">"
                } else {
                    ""
                },
                &item.1.abbreviation
            )
            .as_str(),
        );
    }
    Ok(text)
}

fn get_translation_map() -> Result<IndexMap<String, Translation>, Box<dyn Error>> {
    let file = fs::read_to_string(format!("{}/translations.json", get_data_dir()))?;
    let translations: IndexMap<String, Translation> = serde_json::from_str(&file)?;
    Ok(translations)
}
