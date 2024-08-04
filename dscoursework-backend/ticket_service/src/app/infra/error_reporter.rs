pub enum ReporterError {
    Internal
}

pub trait ErrorReporter {
    fn report() -> Result<(), ReporterError>;
}
