use thiserror::Error;

#[derive(Error, Debug)]
pub enum KovaError {
    #[error("{message}")]
    LexerError { message: String },

    #[error("Kova parsing error:\n\n{message}")]
    ParserError { message: String },

    #[error("Kova analysis error:\n\n{message}")]
    AnalysisError { message: String },

    #[error("Kova compile error:\n\n{message}")]
    CompileError { message: String },
}