use rusqlite::Connection;

pub struct ToDo {
    pub id: i64,
    pub description: String,
    pub done: bool,
    pub create_date: String,
}

pub struct LazyToDo {
    connection: Connection,
}

pub trait ToDoTrait {
    fn get_all(&self, checked: bool) -> Result<Vec<ToDo>, String>;

    fn insert(&self, description: String) -> Result<(), String>;

    fn mark_as_done(&self, id: i64) -> Result<(), String>;
}

impl LazyToDo {
    pub(crate) fn new() -> Result<Self, String> {
        let home_path = match dirs::home_dir() {
            Some(path) => {
                let str_path = match path.to_str() {
                    Some(str_p) => str_p,
                    None => {
                        return Err(String::from("Failed to create: could not get home path"));
                    }
                };
                str_path.to_owned()
            }
            None => return Err(String::from("Failed to create: could not get home path"))
        };

        let db_path = format!("{}/lazytodo.db", home_path);

        return match Connection::open(db_path) {
            Ok(connection) => {
                let todo = LazyToDo { connection };
                Ok(todo)
            }
            Err(_) => Err(String::from("Failed to create: could not open database"))
        };
    }
}

impl ToDoTrait for LazyToDo {
    fn get_all(&self, checked: bool) -> Result<Vec<ToDo>, String> {
        let query = if checked {
            format!("SELECT * FROM todo WHERE done == {}", 1)
        } else {
            format!("SELECT * FROM todo WHERE done == {}", 0)
        };

        let mut statement = match self.connection.prepare(query.as_str()) {
            Ok(statement) => statement,
            Err(_) => return Err(String::from("Could not get entries"))
        };

        let mut rows = match statement.query([]) {
            Ok(rows) => rows,
            Err(_) => return Err(String::from("Could not get entries"))
        };

        let mut entries: Vec<ToDo> = Vec::new();

        while let Ok(Some(r)) = rows.next() {
            let id: i64 = match r.get(0) {
                Ok(id) => id,
                Err(_) => return Err(String::from("Could not get entries"))
            };
            let description: String = match r.get(1) {
                Ok(description) => description,
                Err(_) => return Err(String::from("Could not get entries"))
            };
            let done: bool = match r.get(2) {
                Ok(done) => done,
                Err(_) => return Err(String::from("Could not get entries"))
            };
            let create_date: String = match r.get(3) {
                Ok(create) => create,
                Err(_) => return Err(String::from("Could not get entries"))
            };

            let entry = ToDo {
                id,
                description,
                done,
                create_date,
            };

            entries.push(entry);
        }

        return Ok(entries);
    }

    fn insert(&self, description: String) -> Result<(), String> {
        let query = "INSERT INTO todo(description) VALUES(?)";
        let result = match self.connection.execute(query, [description]) {
            Ok(count) => count,
            Err(_) => return Err(String::from("Could not insert new entry"))
        };

        return if result == 1 {
            Ok(())
        } else {
            Err(String::from("Could not insert new entry: unknown error"))
        };
    }

    fn mark_as_done(&self, id: i64) -> Result<(), String> {
        let query = "UPDATE todo SET done = ? WHERE id = ?";
        let result = match self.connection.execute(query, [1, id]) {
            Ok(count) => count,
            Err(_) => return Err(format!("Could not mark entry {} as done", id))
        };

        return if result == 1 {
            Ok(())
        } else {
            Err(format!("Could not mark entry {} as done", id))
        };
    }
}