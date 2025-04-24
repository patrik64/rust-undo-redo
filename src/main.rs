#[derive(PartialEq)]
enum ActionType {
    Mark,
    Insert,
    Remove,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Country {
    name: String,
    continent: String,
    population: usize,
}

struct UndoAction {
    action_type: ActionType,
    country: Option<Country>,
}

struct UndoHistory {
    actions: Vec<UndoAction>,
    current_action: usize,
    max_action: usize,
}

impl UndoHistory {
    fn new() -> Self {
        let mut uh = UndoHistory {
            actions: Vec::new(),
            current_action: 0,
            max_action: 0,
        };
        uh.append_mark_action();
        uh.current_action -= 1;
        uh
    }

    fn append_mark_action(&mut self) {
        self.current_action += 1;
        let undo_action = UndoAction {
            action_type: ActionType::Mark,
            country: None,
        };
        self.actions.push(undo_action);
    }

    fn append_action(&mut self, action_type: ActionType, country: Option<Country>) {
        self.current_action = self.max_action + 1;
        let undo_action = UndoAction {
            action_type,
            country,
        };
        self.actions.push(undo_action);

        self.append_mark_action();
        self.max_action = self.current_action;
    }

    fn can_undo(&self) -> bool {
        self.current_action > 0 && self.max_action > 0
    }

    fn can_redo(&self) -> bool {
        self.max_action > self.current_action
    }

    fn print(&self) {
        println!("##### Print Undo History #####");
        println!("Current Action: {}", self.current_action);
        println!("Max Action: {}", self.max_action);
        for (index, action) in self.actions.iter().enumerate() {
            println!("idx = {:?}", index);
            match action.action_type {
                ActionType::Mark => println!("Mark"),
                ActionType::Insert => println!("Insert"),
                ActionType::Remove => println!("Remove"),
            }
            if let Some(c) = &action.country {
                println!("Country name = {:?}", c.name);
                println!("Country continent = {:?}", c.continent);
                println!("Country population = {:?}", c.population);
            }
        }
        println!("###############");
    }
}

struct CountriesWithUndoRedo {
    countries: Vec<Country>,
    undo_history: UndoHistory,
}

impl CountriesWithUndoRedo {
    fn new() -> Self {
        let countries = Vec::new();
        let undo_history = UndoHistory::new();
        Self {
            countries,
            undo_history,
        }
    }

    fn add_country(&mut self, country: Country) {
        self.undo_history
            .append_action(ActionType::Insert, Some(country.clone()));
        self.countries.push(country);
    }

    fn remove_country(&mut self) {
        if self.countries.len() > 0 {
            let c = self.countries.pop();
            self.undo_history.append_action(ActionType::Remove, c);
        }
    }

    fn undo(&mut self) {
        if self.undo_history.can_undo() {
            let index = self.undo_history.current_action;
            let mut at = self.undo_history.actions.get(index);

            while at.unwrap().action_type == ActionType::Mark {
                self.undo_history.current_action -= 1;
                let index = self.undo_history.current_action;
                at = self.undo_history.actions.get(index);
            }

            let action = self
                .undo_history
                .actions
                .get(self.undo_history.current_action)
                .unwrap();

            if action.action_type == ActionType::Insert {
                println!("undo - Undoing Insert");
                self.countries.pop();
            } else {
                println!("undo - Undoing Remove");
                let c = action.country.clone().unwrap();
                self.countries.push(c);
            }
            self.undo_history.current_action -= 1;
        } else {
            println!("Noting to Undo!!!");
        }
    }

    fn redo(&mut self) {
        if self.undo_history.can_redo() {
            let index = self.undo_history.current_action;
            let mut at = self.undo_history.actions.get(index).unwrap();

            while at.action_type == ActionType::Mark {
                self.undo_history.current_action += 1;
                let index = self.undo_history.current_action;
                at = self.undo_history.actions.get(index).unwrap();
            }

            let action = self
                .undo_history
                .actions
                .get(self.undo_history.current_action)
                .unwrap();

            if action.action_type == ActionType::Insert {
                println!("redo - Redoing Insert");
                let c = action.country.clone().unwrap();
                self.countries.push(c);
            } else {
                println!("redo - Redoing Remove");
                self.countries.pop();
            }
            self.undo_history.current_action += 1;
        } else {
            println!("Noting to Redo!!!");
        }
    }

    fn print_countries(&self) {
        println!("print countries ------------");
        for c in &self.countries {
            println!("{:?}", c);
        }
        println!("------------------------");
    }
}

fn main() {
    let c1 = Country {
        name: "Mexico".to_string(),
        continent: "North America".to_string(),
        population: 130_000_000,
    };
    let c2 = Country {
        name: "Canada".to_string(),
        continent: "North America".to_string(),
        population: 40_000_000,
    };
    let c3 = Country {
        name: "Austria".to_string(),
        continent: "Europe".to_string(),
        population: 8_000_000,
    };
    let c4 = Country {
        name: "China".to_string(),
        continent: "Asia".to_string(),
        population: 1_400_000_000,
    };
    let c5 = Country {
        name: "Argentina".to_string(),
        continent: "South America".to_string(),
        population: 89_000_000,
    };

    let mut cu = CountriesWithUndoRedo::new();
    cu.add_country(c1);
    cu.remove_country();
    cu.add_country(c2);
    cu.add_country(c3);
    cu.add_country(c4);
    cu.add_country(c5);

    cu.undo();
    cu.redo();

    cu.print_countries();

    cu.undo_history.print();
}
