fn main() {
    :dep itertools
    :dep plotters = { version = "^0.3.0", default_features = false, features = ["evcxr", "all_series"] }
    :dep splines
    :dep csv
    use itertools::Itertools;
    use splines::Key;
    use std::{
        collections::HashMap,
        fs::File,
        hash::BuildHasher,
        io::{self, BufReader},
    };
    extern crate plotters;
    use plotters::prelude::*;
    
}
