use uuid::Uuid;

struct Task {
    uuid: Uuid,
    name: String,
    description: String
}

pub struct TodoList {
    tasks: Vec<Task>
}

pub fn new() -> TodoList {
    return TodoList { tasks: Vec::new() };
}

impl TodoList {
    /// Creates a new task using given information and append it into its tasks list.
    pub fn new_task(&mut self, name: String, description: String) {
        let task: Task = Task{
            uuid: Uuid::new_v4(),
            name: name,
            description: description,
        };
        self.tasks.push(task);
    }

    /// returns a task identified by the given UUID
    pub fn get_task(&self, uuid: &Uuid) -> Option<&Task> {
        let pos = self.tasks.iter().position(|t| t.uuid.eq(uuid));

        if pos.is_some() {
            return self.tasks.get(pos.unwrap());
        }

        return None;
    }

    /// Removes a task identified by its UUID
    pub fn delete_task(&mut self, uuid: &Uuid) -> Result<usize, String> {
        let pos = self.tasks.iter().position(|t| t.uuid.eq(uuid));
        
        if pos.is_some() {
            let idx: usize = pos.unwrap();
            self.tasks.remove(idx);
            return Ok(idx);
        }

        return Err(String::from("not found"));
    }
}