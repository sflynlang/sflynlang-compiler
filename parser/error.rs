use crate::{File, Position};

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::term::{
    self,
    termcolor::{ColorChoice, StandardStream},
};

#[derive(Clone, Debug)]
pub enum ErrorType {
    ExpectArguments(usize, usize),
    ExpectToken(String, String),
    ExpectType(String, String),
    Lexical(String),
    NameInUse(String, Position),
    UnknownIdentifier(String),
    UnknownPosition(Position),
    UnknownToken,
}

#[derive(Clone, Debug)]
pub struct Error {
    /// Position object.
    position: Position,

    // Error type object.
    error_type: ErrorType,
}

impl Error {
    /// Create a new error object using a position and an error type.
    pub fn new(position: Position, error_type: ErrorType) -> Self {
        Self {
            position,
            error_type,
        }
    }

    pub fn new_expect_arguments(
        position: Position,
        expect: usize,
        got: usize,
    ) -> Self {
        Self::new(position, ErrorType::ExpectArguments(expect, got))
    }

    pub fn new_expect_token(
        position: Position,
        expect: &str,
        got: &str,
    ) -> Self {
        Self::new(
            position,
            ErrorType::ExpectToken(expect.to_string(), got.to_string()),
        )
    }

    pub fn new_expect_type(
        position: Position,
        expect: &str,
        got: &str,
    ) -> Self {
        Self::new(
            position,
            ErrorType::ExpectType(expect.to_string(), got.to_string()),
        )
    }

    /// Create a new lexical error object using a position and a message.
    pub fn new_lexical(position: Position, message: &str) -> Self {
        Self::new(position, ErrorType::Lexical(message.to_string()))
    }

    pub fn new_unknown_identifier(position: Position, name: String) -> Self {
        Self::new(position, ErrorType::UnknownIdentifier(name))
    }

    pub fn new_unknown_position(position: Position) -> Self {
        Self::new(
            Position::new(0, 0, 1, 1),
            ErrorType::UnknownPosition(position.clone()),
        )
    }

    pub fn new_unknown_token(position: Position) -> Self {
        Self::new(position, ErrorType::UnknownToken)
    }

    /// Get the position object of the error.
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    /// Get the error type object of the error.
    pub fn get_error_type(&self) -> ErrorType {
        self.error_type.clone()
    }

    /// Convert the error to a diagnostic object.
    ///
    /// Read more about the Diagnostic object [clicking here](https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/diagnostic/struct.Diagnostic.html).
    pub fn to_diagnostic(&self) -> Diagnostic<()> {
        match self.get_error_type() {
            // Get the expect arguments error.
            ErrorType::ExpectArguments(expected, got) => Diagnostic::error()
                .with_message("Expected arguments")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message(format!(
                    "Expect `{}` arguments, got `{}` instead.",
                    expected, got
                ))]),

            // Get the expect token error.
            ErrorType::ExpectToken(expected, got) => Diagnostic::error()
                .with_message("Expected token")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message(format!(
                    "Expect `{}`, got `{}` instead.",
                    expected, got
                ))]),

            // Get the expect type token error.
            ErrorType::ExpectType(expected, got) => Diagnostic::error()
                .with_message("Expected data type")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message(format!(
                    "Expect `{}` data type, got `{}` instead.",
                    expected, got
                ))]),

            // Get the lexical error.
            ErrorType::Lexical(message) => Diagnostic::error()
                .with_message("Lexical")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message(message)]),

            // Get the name in use error.
            ErrorType::NameInUse(name, last_position) => Diagnostic::error()
                .with_message("The identifier is already in use")
                .with_labels(vec![
                    Label::primary((), self.get_position().get_range())
                        .with_message(format!(
                            "The `{}` identifier is already in use.",
                            name
                        )),
                    Label::secondary((), last_position.get_range())
                        .with_message(format!(
                            "The `{}` identifier is used here.",
                            name
                        )),
                ]),

            // Get the unknown identifier error.
            ErrorType::UnknownIdentifier(name) => Diagnostic::error()
                .with_message("Unknown identifier")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message(format!(
                    "Cannot find `{}` in this scope.",
                    name
                ))]),

            // Get the unknown position error.
            ErrorType::UnknownPosition(position) => Diagnostic::error()
                .with_message("Unknown position")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message(format!(
                    "Cannot recognize the position at {}.",
                    position
                ))]),

            // Get the unknown token error.
            ErrorType::UnknownToken => Diagnostic::error()
                .with_message("Unknown token")
                .with_labels(vec![Label::primary(
                    (),
                    self.get_position().get_range(),
                )
                .with_message("Cannot recognize this token.")]),
        }
    }

    /// Show the error in the console using term.
    ///
    /// Read more about the term object [clicking here](https://docs.rs/codespan-reporting/0.9.5/codespan_reporting/term/index.html).
    pub fn show(&self, file: &File) {
        if let Err(error) = term::emit(
            &mut StandardStream::stderr(ColorChoice::Always).lock(),
            &term::Config::default(),
            file,
            &self.to_diagnostic(),
        ) {
            println!("Term Error: {:?}", error);
        }
    }
}
