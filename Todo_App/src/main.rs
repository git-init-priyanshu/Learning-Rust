use std::io;

fn updating_todo(todo_list: &mut Vec<String>){
    println!("Input your Todo number to be updated");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num: usize = input.trim().parse().expect("Invalid Input");

    println!("Input you new todo");
    print!("\n");
    let mut todo = String::new();
    io::stdin().read_line(&mut todo).expect("Failed to read inpukt");
    todo_list[num-1] = todo;
}
fn delete_todo(todo_list: &mut Vec<String>){
    println!("Input your Todo number to be deleted");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num: usize = input.trim().parse().expect("Invalid Input");

    todo_list.remove(num-1);
}
fn print_todos(todo_list: &Vec<String>){
    let mut index = 0;
    println!("Your Todos are:");
    for todo in todo_list{
        index += 1;
        println!("{} -> {}",index, todo);
    }
}
fn add_todo(todo_list: &mut Vec<String>){
    print!("Input your Todo: ");
    print!("\n");
    let mut todo = String::new();
    io::stdin().read_line(&mut todo).expect("Failed to read inpukt");

    todo_list.push(todo);
}
fn main() {
    println!("My ToDo Application in Rust\n");

    let mut todo_list: Vec<String> = Vec::new(); // Creating a dynamic array of String

    print!("Adding todo\n");

    add_todo(&mut todo_list);

    println!("Printing Todo list");
    print_todos(&todo_list);
    
    print!("Adding more todo");

    add_todo(&mut todo_list);

    println!("Printing Todo list");
    print_todos(&todo_list);

    print!("Adding even more todo");

    add_todo(&mut todo_list);

    println!("Printing Todo list");
    print_todos(&todo_list);

    println!("Deleting Todos");
    delete_todo(&mut todo_list);
    print_todos(&todo_list);

    println!("Updating Todos");
    updating_todo(&mut todo_list);
    print_todos(&todo_list);
}
