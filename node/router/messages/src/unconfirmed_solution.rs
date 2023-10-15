// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkOS library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

use snarkvm::{
    ledger::narwhal::Data,
    prelude::{FromBytes, ToBytes},
};

use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnconfirmedSolution<N: Network> {
    pub puzzle_commitment: PuzzleCommitment<N>,
    pub solution: Data<ProverSolution<N>>,
}

impl<N: Network> MessageTrait for UnconfirmedSolution<N> {
    /// Returns the message name.
    #[inline]
    fn name(&self) -> Cow<'static, str> {
        "UnconfirmedSolution".into()
    }
}

impl<N: Network> ToBytes for UnconfirmedSolution<N> {
    fn write_le<W: io::Write>(&self, mut writer: W) -> io::Result<()> {
        self.puzzle_commitment.write_le(&mut writer)?;
        self.solution.write_le(&mut writer)
    }
}

impl<N: Network> FromBytes for UnconfirmedSolution<N> {
    fn read_le<R: io::Read>(mut reader: R) -> io::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self { puzzle_commitment: PuzzleCommitment::read_le(&mut reader)?, solution: Data::read_le(reader)? })
    }
}
