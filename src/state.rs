use libsql::{Connection, Database};

use std::sync::{Arc, Mutex};

use crate::components::counter::Counter;

#[derive(Clone)]
pub(crate) struct AppState {
    pub db: Arc<Mutex<Database>>,
}

impl AppState {
    pub(crate) async fn get_counters(&mut self) -> Vec<Counter> {
        let conn = self.db.lock().unwrap().connect().unwrap();
        let mut stmt = conn
            .prepare(include_str!("../sql/5_select_all_counters.sql"))
            .await
            .unwrap();

        let mut output = vec![];
        let mut rows = stmt.query(()).await.unwrap();

        while let Ok(Some(row)) = rows.next().await {
            output.push(libsql::de::from_row::<Counter>(&row).unwrap());
        }

        output
    }

    pub(crate) async fn create_counter(&mut self) -> Counter {
        let conn = self.db.lock().unwrap().connect().unwrap();

        let mut stmt = conn
            .prepare(include_str!("../sql/4_create_counter.sql"))
            .await
            .unwrap();
        let mut rows = stmt.query(()).await.unwrap();
        let row = rows.next().await.unwrap().unwrap();
        let id = row.get(0).unwrap();

        Self::internal_get_counter_value(&conn, id).await
    }

    pub(crate) async fn increment_counter(&mut self, id: u32) -> Counter {
        let conn = self.db.lock().unwrap().connect().unwrap();

        let mut stmt = conn
            .prepare(include_str!("../sql/1_increment_counter.sql"))
            .await
            .unwrap();
        stmt.execute([id]).await.unwrap();

        Self::internal_get_counter_value(&conn, id).await
    }

    pub(crate) async fn reset_counter(&mut self, id: u32) -> Counter {
        let conn = self.db.lock().unwrap().connect().unwrap();

        let mut stmt = conn
            .prepare(include_str!("../sql/2_set_counter.sql"))
            .await
            .unwrap();
        stmt.execute([0, id]).await.unwrap();

        Self::internal_get_counter_value(&conn, id).await
    }

    pub(crate) async fn get_counter_value(&mut self) -> Counter {
        let id = 1u32;
        let conn = self.db.lock().unwrap().connect().unwrap();
        Self::internal_get_counter_value(&conn, id).await
    }

    async fn internal_get_counter_value(conn: &Connection, id: u32) -> Counter {
        let mut stmt = conn
            .prepare(include_str!("../sql/3_select_counter_value.sql"))
            .await
            .unwrap();
        let row = stmt.query_row([id]).await.unwrap();
        libsql::de::from_row(&row).expect("deserialize row to counter")
    }
}
