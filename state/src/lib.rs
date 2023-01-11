#![no_std]

use demo_ping_io::*;
use gmeta::{metawasm, Metadata};
use gstd::prelude::*;

#[metawasm]
pub trait Metawasm {
    type State = <DemoPingMetadata as Metadata>::State;

    fn get_first_message(state: Self::State) -> String {
        state.first().clone().expect("Message log is empty!")
    }

    fn get_last_message(state: Self::State) -> String {
        state.last().clone().expect("Message log is empty!")
    }

    fn get_messages_len(state: Self::State) -> usize {
        state.len()
    }

    fn get_message(index: usize, state: Self::State) -> String {
        state.get(index).clone().expect("Invalid index!")
    }
}
