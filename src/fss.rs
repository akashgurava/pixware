use std::{fs::File, path::Path};

use walkdir::{DirEntry, Error as WlkError, WalkDir};

use crate::error::Error;
