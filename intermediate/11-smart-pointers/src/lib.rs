use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Category {
    name: String,
    visits: RefCell<u32>,
}

impl Category {
    #[must_use]
    pub fn shared(name: impl Into<String>) -> Rc<Self> {
        Rc::new(Self {
            name: name.into(),
            visits: RefCell::new(0),
        })
    }

    pub fn visit(&self) {
        *self.visits.borrow_mut() += 1;
    }

    #[must_use]
    pub fn summary(&self) -> String {
        format!("{}: {} visita(s)", self.name, self.visits.borrow())
    }
}

#[must_use]
pub fn shared_owner_count(category: &Rc<Category>) -> usize {
    Rc::strong_count(category)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::{Category, shared_owner_count};

    #[test]
    fn shared_owners_observe_interior_changes() {
        let category = Category::shared("libros");
        let menu_entry = Rc::clone(&category);
        menu_entry.visit();
        assert_eq!(category.summary(), "libros: 1 visita(s)");
        assert_eq!(shared_owner_count(&category), 2);
    }
}
