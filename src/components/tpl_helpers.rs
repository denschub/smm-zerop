use crate::entities::smm2_level::Smm2Level;

pub fn clear_condition_text(id: i64, magnitude: Option<i64>) -> String {
    Smm2Level::clear_condition_text(id, magnitude)
}

pub fn formatted_level_id(raw_id: &str) -> String {
    Smm2Level::formatted_level_id(raw_id)
}

pub fn ms_to_minsecs(ms: i64) -> String {
    let seconds = (ms as f64 / 1000.0).ceil();
    if seconds > 60.0 {
        let mins = (seconds / 60.0).floor();
        let remaining_seconds = seconds % 60.0;
        format!("{mins}m {remaining_seconds}s")
    } else {
        format!("{seconds}s")
    }
}

pub fn tag_list(tags: Vec<String>) -> String {
    tags.iter()
        .map(|tag| tag_name(tag))
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn tag_name(raw_name: &str) -> String {
    minijinja::filters::title(raw_name.replace("_", " ").into())
}
