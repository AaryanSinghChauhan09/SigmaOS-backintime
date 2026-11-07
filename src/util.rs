//! backintime model — Linux backup tool.

#[derive(Debug, Clone)]
pub struct Backup {
    pub id: String,
    pub name: String,
    pub date: String,
    pub size: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub backups: Vec<Backup>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { backups: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create(&mut self, name: &str, date: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.backups.push(Backup { id: id.clone(), name: name.into(), date: date.into(), size: 0, status: "completed".into() });
        id
    }

    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(pos) = self.backups.iter().position(|b| b.id == id) {
            self.backups.remove(pos);
            true
        } else { false }
    }

    pub fn list(&self) -> &[Backup] { &self.backups }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn backup_create_delete() {
        let mut b = Model::new();
        let id = b.create("daily", "2024-01-01");
        assert_eq!(b.list().len(), 1);
        assert!(b.delete(&id));
    }
}
