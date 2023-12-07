use uuid::Uuid;

pub struct Task {
    pub id: String,
    pub name: String,
    pub done: bool
}

pub struct Todo {
    pub tasks: Vec<Task>
}

impl Task {
    pub fn new(name: String, done: bool) -> Self {
        Task {
            id: Uuid::new_v4().to_string(),
            name,
            done
        }
    }
}

impl Todo {
    pub fn new() -> Self {
        Todo {
            tasks: Vec::new()
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn remove_task(&mut self, id: String) -> Result<Task, String> {
        let mut iterator = self.tasks.iter();

        let task_index = iterator.position(|t| t.id == id);

        match task_index {
            Some(index) => {
                let removed_task = self.tasks.remove(index);

                return Ok(removed_task);
            },
            None => {
                return Err(String::from("Task not found"));
            }
        } 
    }

    pub fn get_by_id(&self, id: String) -> Result<&Task, String> {
        let mut iterator = self.tasks.iter();

        let task = iterator.find(|t| t.id == id);

        match task {
            Some(task) => {
                return Ok(&task);
            }
            None => {
                return Err(String::from("Task not found"));
            }
        }
    }
}