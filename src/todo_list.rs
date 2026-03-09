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

#[cfg(test)]
mod test {
    use uuid::Uuid;
    use crate::todo_list::{Task, new};
    use super::TodoList;

    #[test]
    fn test_new_task() {
        let mut _todo_list: TodoList = new();
        _todo_list.new_task(String::from("test"), String::from("desc test"));

        assert_eq!(_todo_list.tasks.len(), 1);
    }

    #[test]
    fn test_get_task() {
        let _task_uuid = Uuid::new_v4();
        let mut _todo_list: TodoList = new();
        _todo_list.tasks.push(Task{
            uuid: _task_uuid,
            name: String::from("test"),
            description: String::from("test desc"),
        });

        let _res: Option<&Task> = _todo_list.get_task(&_task_uuid);
        assert!(_res.is_some());
        assert_eq!(_res.unwrap().name, String::from("test"));
        assert_eq!(_res.unwrap().description, String::from("test desc"));

        let _fake_uuid = Uuid::new_v4();
        assert!(_todo_list.get_task(&_fake_uuid).is_none());
    }

    #[test]
    fn test_remove_task() {
        let _task_uuid = Uuid::new_v4();
        let mut _todo_list: TodoList = new();
        _todo_list.tasks.push(Task{
            uuid: _task_uuid,
            name: String::from("test"),
            description: String::from("test desc"),
        });
        assert_eq!(_todo_list.tasks.len(), 1);

        let _res = _todo_list.delete_task(&_task_uuid);
        assert_eq!(_res.unwrap(), 0);
        assert_eq!(_todo_list.tasks.len(), 0);
    }
}