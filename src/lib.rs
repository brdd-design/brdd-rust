use serde::{Deserialize, Serialize};

/// Standardized error object for BRDD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrddError {
    pub code: String,
    pub message: String,
}

/// A subset of the context that allows adding errors and checking validity.
pub trait ValidationContext {
    fn add_error(&mut self, code: String, message: String);
    fn is_valid(&self) -> bool;
    fn get_errors(&self) -> &[BrddError];
}

/// The central state object passed around and returned by UseCases.
pub trait ExecutionContext<T>: ValidationContext {
    fn add_effect(&mut self, code: String);
    fn add_setter(&mut self, code: String);
    fn set_data(&mut self, data: T);
    
    fn get_data(&self) -> Option<&T>;
    fn get_effects(&self) -> &[String];
    fn get_setters(&self) -> &[String];
    fn get_status(&self) -> u16;
}

/// The default implementation of the ExecutionContext.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultExecutionContext<T> {
    pub data: Option<T>,
    pub errors: Vec<BrddError>,
    pub setters: Vec<String>,
    pub effects: Vec<String>,
    pub status: u16,
}

impl<T> DefaultExecutionContext<T> {
    /// Creates a new execution context with default successful status.
    pub fn new(initial_data: Option<T>) -> Self {
        Self {
            data: initial_data,
            errors: Vec::new(),
            setters: Vec::new(),
            effects: Vec::new(),
            status: 200,
        }
    }
}

impl<T> ValidationContext for DefaultExecutionContext<T> {
    fn add_error(&mut self, code: String, message: String) {
        self.errors.push(BrddError { code, message });
        self.status = 400; // Default error status
    }

    fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    fn get_errors(&self) -> &[BrddError] {
        &self.errors
    }
}

impl<T> ExecutionContext<T> for DefaultExecutionContext<T> {
    fn add_effect(&mut self, code: String) {
        self.effects.push(code);
    }

    fn add_setter(&mut self, code: String) {
        self.setters.push(code);
    }

    fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    fn get_effects(&self) -> &[String] {
        &self.effects
    }

    fn get_setters(&self) -> &[String] {
        &self.setters
    }

    fn get_status(&self) -> u16 {
        self.status
    }
}

/// Protocol for services dedicated to pure business logic validation.
pub trait ValidateService<I> {
    fn validate(&self, ctx: &mut dyn ValidationContext, input: &I);
}

/// Protocol for services that fetch additional data needed for the UseCase.
pub trait EnrichService<I, E, T> {
    fn enrich(&self, ctx: &mut dyn ExecutionContext<T>, input: &I) -> Result<E, BrddError>;
}

/// Protocol for external adapters (APIs, DBs) to perform side-effects.
pub trait ClientService<I, T> {
    fn execute(&self, ctx: &mut dyn ExecutionContext<T>, input: &I);
}

/// The orchestrator.
pub trait UseCase<I, O> {
    fn execute(&self, input: I) -> DefaultExecutionContext<O>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let ctx = DefaultExecutionContext::new(Some(1));
        assert_eq!(ctx.get_data(), Some(&1));
        assert!(ctx.is_valid());
        assert_eq!(ctx.get_status(), 200);
        assert!(ctx.get_errors().is_empty());
        assert!(ctx.get_effects().is_empty());
        assert!(ctx.get_setters().is_empty());
    }

    #[test]
    fn test_add_error() {
        let mut ctx: DefaultExecutionContext<i32> = DefaultExecutionContext::new(None);
        ctx.add_error("R001".to_string(), "Invalid".to_string());
        
        assert!(!ctx.is_valid());
        assert_eq!(ctx.get_status(), 400);
        assert_eq!(ctx.get_errors().len(), 1);
        assert_eq!(ctx.get_errors()[0].code, "R001");
    }

    #[test]
    fn test_add_effect_and_setter() {
        let mut ctx: DefaultExecutionContext<i32> = DefaultExecutionContext::new(None);
        ctx.add_effect("E001".to_string());
        ctx.add_setter("S001".to_string());
        
        assert_eq!(ctx.get_effects().len(), 1);
        assert_eq!(ctx.get_effects()[0], "E001");
        assert_eq!(ctx.get_setters().len(), 1);
        assert_eq!(ctx.get_setters()[0], "S001");
    }

    #[test]
    fn test_set_data() {
        let mut ctx: DefaultExecutionContext<i32> = DefaultExecutionContext::new(None);
        ctx.set_data(2);
        assert_eq!(ctx.get_data(), Some(&2));
    }
}

pub use brdd_macros::{brdd_use_case, brdd_rule};

