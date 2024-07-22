use self::prelude::*;
use super::prelude::*;
use crate::prelude::*;

mod prelude {
    pub(super) use super::*;
    pub(super) use ::mdbook::preprocess::{Preprocessor, PreprocessorContext};

    pub(super) use std::io::Read;
}

mod naive;

pub use naive::*;

pub fn fetch_context<R: Read>(reader: R) -> Result<(PreprocessorContext, Book)> {
    ::mdbook::preprocess::CmdPreprocessor::parse_input(reader)
}
