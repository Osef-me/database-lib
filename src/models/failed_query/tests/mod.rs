//! Test modules for the `FailedQuery` model.
//! 
//! This module contains comprehensive tests for the `FailedQuery` model,
//! including validation tests and model behavior tests.
//! 
//! # Test Categories
//! 
//! ## Validation Tests (`validation/`)
//! Tests that verify the validation rules of the `FailedQuery` model:
//! - Hash format validation (alphanumeric characters only)
//! - Hash length validation (1-255 characters)
//! - Edge cases and error conditions
//! 
//! ## Model Tests (`model_tests.rs`)
//! Tests that verify the basic functionality and behavior of the `FailedQuery` struct:
//! - Model creation and field access
//! - Edge cases for different field values
//! - Timestamp handling
//! 
//! # Running Tests
//! 
//! To run all failed query tests:
//! ```bash
//! cargo test models::failed_query::tests
//! ```
//! 
//! To run only validation tests:
//! ```bash
//! cargo test models::failed_query::tests::validation
//! ```
//! 
//! To run only model tests:
//! ```bash
//! cargo test models::failed_query::tests::model_tests
//! ```

pub mod validation;
pub mod model_tests;
