use super::todo::Todo;

pub struct TodoList {
    pub entities: Vec<Todo>    
}

pub struct TodoListIter<'a> {
    todos: &'a Vec<Todo>,
    idx: usize,
}

impl<'a> Iterator for TodoListIter<'a> {
    type Item = &'a Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.todos.len() {
            let idx = self.idx;
            self.idx += 1;

        return Some(&self.todos[idx]);
        } else {
            return None;
        }
    }
}

impl<'a> TodoListIter<'a> {
   fn new(todo_list: &'a TodoList) -> Self {
        return TodoListIter {
            todos: &todo_list.entities,
            idx: 0
        }
   } 
}

impl<'a> IntoIterator for &'a TodoList {
    type Item = &'a Todo;

    type IntoIter = TodoListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return TodoListIter::new(&self);
    }
}
