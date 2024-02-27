use std::io;


#[derive(Debug)]
struct TODO {
    todo_list: Vec<String>,
    amount: u64,
}

impl TODO{
    fn create()-> Self{
        TODO{
            todo_list: vec![],
            amount: 0,
        }
    }


    fn add(&mut self, inputs: &Vec<&str>){
        if inputs.len() == 1 {
            println!("No Arguments given");
            return;
        }

        // Adding Something to the List
        println!("Adding....");
        let arg_concat = inputs[1..].join(" ");

        // println!("Value of Arg_Concat: {}", arg_concat);
        self.todo_list.push(arg_concat);

        self.amount = self.todo_list.len() as u64;
    }

    fn list(&self){
        // println!("{:?}", self.todo_list);
        println!("Listing all currently added TODO's ");
        if self.amount == 0 {
        println!("Current Amount is: {}, Cant list anything", self.amount);
        }

        for i in 0..self.todo_list.len() {
            println!("index: {}, Content: {}", i, self.todo_list.get(i).unwrap());
        }
    }

    fn remove(&mut self, args: &Vec<&str>){
        if self.amount == 0 {
            println!("Cannot Remove, which doesnt exist.");
            return;
        }

        if args.get(1).is_some() {
            println!("{:?}", args.get(1));

            let is_parsable:Option<usize> = args[1].parse().ok();
            match is_parsable {
                Some(id) => {
                    if !(id >= self.todo_list.len()) || (!id == self.todo_list.len()){
                        // println!("ID{} LEN{}", id, self.todo_list.len()); // DEBUG only
                        self.todo_list.remove(id);
                        self.amount -= 1;
                    } else { println!("Cannot Remove, because ID is higher or in the minus") }
                }
                None => {}
            }
            // println!("{:?}", is_parsable); // DEBUG only
        }
    }

}


fn main() {
    // Functionality
    /*
    Upon Creating a todo_list, the user can view these again.
    */
    //TODO_LIST 1
    let mut todo_list = TODO::create();


    println!("Welcome to the Todo Application!");

//LOOP until input is either CTRL C or Exit E
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");

        let binding = String::from(input);
        let args:Vec<&str> = binding.trim().split_terminator(" ").collect();
        println!("Arguments Given: {:?}", args);

        let args_0 = String::from(args[0]).to_lowercase();
        if args_0.starts_with("add") {
            todo_list.add(&args);
        }

        if args_0.starts_with("list") {
            todo_list.list();
        }

        if args_0.starts_with("remove") | args_0.starts_with("r") {
            todo_list.remove(&args);
        }

        if args_0.starts_with("exit") | args_0.starts_with("e")  {
            println!("Exiting Program");
            break;
        }
    }








}
