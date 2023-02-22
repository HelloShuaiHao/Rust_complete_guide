#[derive(Debug, Copy, Clone)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Copy, Clone)]
struct Employee {
    position: Position,
    work_hour: i32,
}

// We don't have an ampersand here so we are not borrowing
// This function takes ownership of employee
// If you are automatically deriving clone and copy is to
// only apply that to structures which are small in size
fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let _me = Employee {
        position: Position::Worker,
        work_hour: 40,
    };
    println!("{:?}",_me);
    println!("My position is: {:?}", _me.position);
    println!("My work_hours is: {:?}", _me.work_hour);

    // clone,copy
    print_employee(_me);
    print_employee(_me);
}

// *Recap
// * #[derive(Debug)] allows us to print
// * #[derive(Debug, Copy, Clone] allows us to make a copy