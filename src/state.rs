use libsql::{Connection, Database};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub(crate) struct AppState {
    pub db: Arc<Mutex<Database>>,
}

impl AppState {
    pub(crate) async fn increment_counter(&mut self) -> u16 {
        let id = 1u32;
        let conn = self.db.lock().unwrap().connect().unwrap();

        let mut stmt = conn
            .prepare("UPDATE counter SET value = value + 1 WHERE ID = ?1")
            .await
            .unwrap();
        stmt.execute([id]).await.unwrap();

        Self::internal_get_counter_value(&conn, id).await
    }

    pub(crate) async fn reset_counter(&mut self) -> u16 {
        let id = 1u32;
        let conn = self.db.lock().unwrap().connect().unwrap();

        let mut stmt = conn
            .prepare("UPDATE counter SET value = 0 WHERE ID = ?1")
            .await
            .unwrap();
        stmt.execute([id]).await.unwrap();

        Self::internal_get_counter_value(&conn, id).await
    }

    pub(crate) async fn get_counter_value(&mut self) -> u16 {
        let id = 1u32;
        let conn = self.db.lock().unwrap().connect().unwrap();
        Self::internal_get_counter_value(&conn, id).await
    }

    async fn internal_get_counter_value(conn: &Connection, id: u32) -> u16 {
        let mut stmt = conn
            .prepare("SELECT value FROM counter WHERE id = ?1 LIMIT 1")
            .await
            .unwrap();
        let mut rows = stmt.query([id]).await.unwrap();
        let row = rows.next().await.unwrap().unwrap();

        match row.get_value(0).unwrap() {
            libsql::Value::Integer(i) => i as u16,
            _ => 0,
        }
    }
}
