use serde::Deserialize;
use crate::errors::{ValidationError};

#[derive(Deserialize)]
pub struct CreateTaskForm {
    pub description: String,
}

impl CreateTaskForm {
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = vec![];

        if self.description.is_empty() {
            errors.push(ValidationError {
                field: "description".to_string(),
                message: "Description is required".to_string(),
            });
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task_form_validate() {
        let form = CreateTaskForm { description: "".to_string() };
        let errors = form.validate();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "description");
        assert_eq!(errors[0].message, "Description is required");
    }

    #[test]
    fn test_create_task_form_validate_with_description() {
        let form = CreateTaskForm { description: "Do the laundry".to_string() };
        let errors = form.validate();

        assert_eq!(errors.len(), 0);
    }
}