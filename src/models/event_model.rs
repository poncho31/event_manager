use std::fmt;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use crate::schema::schema::events; // Import des schémas
use serde::{Serialize, Deserialize};
use crate::utils::builder::page_builder::list::{IntoList, ListItem};
use crate::utils::builder::page_builder::table::IntoTable;

#[derive(Queryable, Serialize, Debug, Clone)]
pub struct Event {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

impl Event {
    fn to_key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("ID".to_string(), self.id.map_or_else(|| "-".to_string(), |v| v.to_string())),
            ("Titre".to_string(), self.title.clone()),
            (
                "Description".to_string(),
                self.description.clone().unwrap_or_else(|| "Aucune description".to_string()),
            ),
            ("Date".to_string(), self.date.format("%Y-%m-%d").to_string()),
        ]
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).unwrap_or_else(|e| format!("Erreur : {}", e))
        )
    }
}

impl IntoTable for Event {
    fn headers() -> Vec<String> {
        vec!["ID".to_string(), "Nom de l'événement".to_string(), "Description".to_string(), "Date".to_string()]
    }

    fn to_row(&self) -> Vec<String> {
        self.to_key_value_pairs().into_iter().map(|(_, v)| v).collect()
    }
}

impl IntoList for Event {
    fn to_list_item(&self) -> ListItem {
        ListItem {
            data: self.to_key_value_pairs(),
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct NewEventData {
    pub title: String,
    pub description: Option<String>,
    pub date: String,
    pub user_id: i32,
}

impl NewEventData {
    pub fn new(&self) -> NewEvent {
        NewEvent {
            title: &self.title,
            description: self.description.as_deref(),
            date: Self::parse_date(&self.date),
            user_id: self.user_id,
        }
    }

    fn parse_date(date: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").expect("Format de date invalide")
    }
}
