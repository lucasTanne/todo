mod todo_list;

fn main() {
    let mut todo_list: todo_list::TodoList = todo_list::TodoList::new();

    todo_list.add_task("Task one", "The first task should be to test this Todo list ");

    todo_list.add_list("home");
    todo_list.add_task_to_list("home", "Task two", "Refactor my brain using rust !");
    // todo_list.add_task_to_list("taf", "Task three", "Err task");

    todo_list.print_todo_list();
}
