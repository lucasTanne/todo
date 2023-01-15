mod todo_list {
    use core::panic;
    use std::{collections::HashMap};

    pub struct TodoList {
        id: u32,
        lists: HashMap<String, List>,
        tasks: Vec<Task>
    }
    impl TodoList {
        pub fn new() -> Self{
            Self { id: 0, lists: HashMap::new(), tasks: Vec::new() }
        }

        pub fn print_todo_list(self) {
            for (_, list) in self.lists {
                list.print_list();
            }

            println!();
            for task in self.tasks {
                task.print_task();
            }
        }

        pub fn add_task(&mut self, name: &str, description: &str) {
            let task: Task = Task::new(name, description);
            self.tasks.push(task);
        }

        pub fn add_list(&mut self, name: &str) {
            let list: List = List::new(name);
            self.lists.insert(name.to_string(), list);
        }

        pub fn add_task_to_list(&mut self, name_list: &str, task_name: &str, task_description: &str) {
            let task: Task = Task::new(task_name, task_description);

            let list = self.lists.get_mut(name_list);
            match list {
                Some(x) => x.add_task(task),
                None => panic!("The list doesn't exist!")
            }
        }
    }

    struct List {
        id: u32,
        name: String,
        tasks: Vec<Task>,
        done: bool
    }
    impl List {
        fn new(name: &str) -> Self{
            Self { id: 0, name: name.to_string(), tasks: Vec::new(), done: false }
        }

        fn print_list(self) {
            let mut done: &str;
    
            for task in self.tasks {
                done = if self.done {
                    "x"
                } else {
                    " "
                };
                println!("[{}] {}", done, self.name);
                task.print_task();
            }
        }
        
        fn add_task(&mut self, task: Task) {
            self.tasks.push(task);
        }
    }

    struct Task {
        id: u32,
        name: String,
        description: String,
        done: bool
    }
    impl Task {
        fn new(name: &str, description: &str) -> Self {
            Self { id: 0, name: name.to_string(), description: description.to_string(), done: false }
        }

        fn print_task(self) {
            let done: &str = if self.done {
                "x"
            } else {
                " "
            };
            println!("\t[{}] {}", done, self.name);
        }

        fn print_description(self) {
            println!("Task: {}\n\n{}", self.name, self.description);
        }
    }
}

fn main() {
    let mut todo_list: todo_list::TodoList = todo_list::TodoList::new();

    todo_list.add_task("Task one", "The first task should be to test this Todo list ");

    todo_list.add_list("home");
    todo_list.add_task_to_list("home", "Task two", "Refactor my brain using rust !");
    // todo_list.add_task_to_list("taf", "Task three", "Err task");

    todo_list.print_todo_list();
}
