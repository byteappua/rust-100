use std::collections::HashMap;

struct Company {
    // TODO: Define data structure to store employees by department
    // Hint: HashMap<String, Vec<String>>
    data: HashMap<String, Vec<String>>, // Just giving a hint or let them define?
    // Let's hide it to make them think, or provide the struct and let them impl methods.
}

impl Company {
    fn new() -> Self {
        Company {
            data: HashMap::new(),
        }
    }

    fn add_employee(&mut self, employee: &str, department: &str) {
        // TODO: Implement
    }

    fn list_department(&self, department: &str) -> Vec<String> {
        // TODO: Return list of employees in department (sorted alphabetically)
        vec![]
    }

    fn list_all(&self) -> Vec<String> {
        // TODO: Return all employees formatted like "Dept: Name"
        // Or just print them? The exercise says "List".
        // Let's return a list of strings for verification.
        vec![]
    }
}

fn main() {
    let mut company = Company::new();

    company.add_employee("Sally", "Engineering");
    company.add_employee("Amir", "Engineering");
    company.add_employee("Bill", "Sales");

    let eng = company.list_department("Engineering");
    println!("Engineering: {:?}", eng);
    assert_eq!(eng, vec!["Amir", "Sally"]); // Should be sorted

    let sales = company.list_department("Sales");
    println!("Sales: {:?}", sales);
    assert_eq!(sales, vec!["Bill"]);
}
