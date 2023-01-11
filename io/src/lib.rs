#![no_std]

use gmeta::{InOut, Metadata};
use gstd::prelude::*;

pub struct DemoPingMetadata;

impl Metadata for DemoPingMetadata {
    type Init = ();
    type Handle = InOut<Vec<u8>, Vec<u8>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Vec<String>;
}
