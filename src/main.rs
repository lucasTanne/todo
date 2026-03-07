use std::io;
use uuid::Uuid;

struct Task {
    uuid: Uuid,
    name: String,
    description: String
}

struct TodoList {
    tasks: Vec<Task>
}

impl TodoList {
    /// Creates a new task using given information and append it into its tasks list.
    fn new_task(&mut self, name: String, description: String) {
        let task: Task = Task{
            uuid: Uuid::new_v4(),
            name: name,
            description: description,
        };
        self.tasks.push(task);
    }

    /// returns a task identified by the given UUID
    fn get_task(&self, uuid: &Uuid) -> Option<&Task> {
        let pos = self.tasks.iter().position(|t| t.uuid.eq(uuid));

        if pos.is_some() {
            return self.tasks.get(pos.unwrap());
        }

        return None;
    }

    /// Removes a task identified by its UUID
    fn delete_task(&mut self, uuid: &Uuid) -> Result<usize, String> {
        let pos = self.tasks.iter().position(|t| t.uuid.eq(uuid));
        
        if pos.is_some() {
            let idx: usize = pos.unwrap();
            self.tasks.remove(idx);
            return Ok(idx);
        }

        return Err(String::from("not found"));
    }
}

fn main() {
    let mut todo_list: TodoList = TodoList{ 
        tasks: Vec::new()
    };

    let mut name: String = String::new();
    println!("Name of the task: ");
    io::stdin().read_line(&mut name).expect("Could not read name from stdin");

    let mut description: String = String::new();
    println!("Description of the task: ");
    io::stdin().read_line(&mut description).expect("Could nto read description from stdin");

    todo_list.new_task(name, description);

    println!("Task len: {}", todo_list.tasks.len());

}
