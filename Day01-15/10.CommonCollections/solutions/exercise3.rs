use std::collections::HashMap;

struct Company {
    data: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Self {
        Company {
            data: HashMap::new(),
        }
    }

    fn add_employee(&mut self, employee: &str, department: &str) {
        let employees = self.data.entry(String::from(department)).or_insert(Vec::new());
        employees.push(String::from(employee));
    }

    fn list_department(&self, department: &str) -> Vec<String> {
        match self.data.get(department) {
            Some(employees) => {
                let mut sorted = employees.clone();
                sorted.sort();
                sorted
            },
            None => Vec::new(),
        }
    }
}

fn main() {
    let mut company = Company::new();

    company.add_employee("Sally", "Engineering");
    company.add_employee("Amir", "Engineering");
    company.add_employee("Bill", "Sales");

    let eng = company.list_department("Engineering");
    println!("Engineering: {:?}", eng);
    assert_eq!(eng, vec!["Amir", "Sally"]);

    let sales = company.list_department("Sales");
    println!("Sales: {:?}", sales);
    assert_eq!(sales, vec!["Bill"]);

    println!("All systems go!");
}
