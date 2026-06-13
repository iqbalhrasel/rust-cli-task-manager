use rusqlite::{Connection, params};

use crate::model::Task;

pub struct TaskService {
    pub conn: Connection,
}
impl TaskService {
    pub fn new(conn: Connection) -> Self {
        return Self { conn };
    }

    pub fn add(&self, desc: &str) -> Result<(), String> {
        self.conn
            .execute(r#"INSERT INTO tasks (description) VALUES (?1)"#, [desc])
            .map_err(|e| format!("db error: {:?}", e))?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Task>, String> {
        let mut stmt = self
            .conn
            .prepare(r#"SELECT id, description, done FROM tasks"#)
            .map_err(|e| format!("db error: {:?}", e))?;

        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get("id")?,
                    description: row.get("description")?,
                    done: row.get("done")?,
                })
            })
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string());

        return tasks;
    }

    pub fn done_task(&self, id: i64, done: bool) -> Result<(), String> {
        let rows = self
            .conn
            .execute(
                "UPDATE tasks SET done = ?1 WHERE id = ?2",
                params![done, id],
            )
            .map_err(|e| format!("db error: {}", e))?;

        if rows == 0 {
            return Err(format!("task {} not found", id));
        }

        Ok(())
    }

    pub fn delete(&self, id: i64) -> Result<(), String> {
        let rows = self
            .conn
            .execute("DELETE FROM tasks WHERE id = ?1", [id])
            .map_err(|e| format!("db error: {}", e))?;

        if rows == 0 {
            return Err(format!("task {} not found", id));
        }
        Ok(())
    }

    pub fn update_desc(&self, id: i64, desc: &str) -> Result<(), String> {
        let rows = self
            .conn
            .execute(
                "UPDATE tasks SET description = ?1 WHERE id = ?2",
                params![desc, id],
            )
            .map_err(|e| format!("db error: {}", e))?;

        if rows == 0 {
            return Err(format!("task {} not found", id));
        }

        Ok(())
    }
}
