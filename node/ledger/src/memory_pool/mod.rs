// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

mod solutions;
mod transactions;

use snarkvm::prelude::{Network, ProverSolution, PuzzleCommitment, Transaction};

use anyhow::{bail, Result};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MemoryPool<N: Network> {
    /// The pool of unconfirmed transactions.
    unconfirmed_transactions: HashMap<N::TransactionID, Transaction<N>>,
    /// The pool of unconfirmed solutions.
    unconfirmed_solutions: HashMap<PuzzleCommitment<N>, ProverSolution<N>>,
}

impl<N: Network> Default for MemoryPool<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N: Network> MemoryPool<N> {
    /// Initializes a new instance of a memory pool.
    pub fn new() -> Self {
        Self { unconfirmed_transactions: Default::default(), unconfirmed_solutions: Default::default() }
    }
}
