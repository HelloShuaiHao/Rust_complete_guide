// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
#[derive(Debug)]
enum EmployeeType {
    MaintenanceCrews,
    MarketingDepartmentEmployees,
    Managers,

    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}
enum status {
    Active,
    Terminated,
}
// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    position: EmployeeType,
    is_employed: status,
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn permission_check(employee: &Employee) -> Result<EmployeeType,String> {
    match employee.is_employed {
        status::Terminated => return Err("Not employed".to_owned()), // early return
        _ => (), // Doing nothing
    }
    match employee.position {
        EmployeeType::MaintenanceCrews => Ok(EmployeeType::MaintenanceCrews),
        EmployeeType::Managers => Ok(EmployeeType::Managers),
        EmployeeType::MarketingDepartmentEmployees => Ok(EmployeeType::MarketingDepartmentEmployees),
        _ => Err("No permission".to_owned()),
    }
}
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_info(staff: &Employee) -> Result<(),String>{
    let info = permission_check(staff)?;
    // if it's failed , it will be directly terminated and the err will be returned
    println!("Your position is {:?}", info);
    Ok(())
}

fn main() {
    let employee = Employee {
        position: EmployeeType::MaintenanceCrews,
        is_employed: status::Active,
    };
    match print_info(&employee) {
        Err(e) => println!("{:?}",e),
        _ => (),
    }
}
