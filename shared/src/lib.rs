#![allow(clippy::all)]

use base_engine::BaseEngine;
use compute_engine::ComputeEngine;

#[cfg(test)]
mod tests;

pub fn entrypoint() {
    let compute_engine = ComputeEngine::new();

    ComputeEngine::print_api_information(compute_engine.get_instance(), log::Level::Info);
}
