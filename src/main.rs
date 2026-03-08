use std::io;

mod todo_list;
use todo_list::TodoList;

fn main() {
    let mut todo_list: TodoList = todo_list::new();

    let mut name: String = String::new();
    println!("Name of the task: ");
    io::stdin().read_line(&mut name).expect("Could not read name from stdin");

    let mut description: String = String::new();
    println!("Description of the task: ");
    io::stdin().read_line(&mut description).expect("Could nto read description from stdin");

    todo_list.new_task(name, description);

}
