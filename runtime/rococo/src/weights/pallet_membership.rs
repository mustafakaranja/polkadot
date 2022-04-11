// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_membership`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_membership
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/pallet_membership.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn add_member(m: u32, ) -> Weight {
		(12_232_000 as Weight)
			// Standard Error: 0
			.saturating_add((59_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn remove_member(m: u32, ) -> Weight {
		(14_143_000 as Weight)
			// Standard Error: 0
			.saturating_add((46_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn swap_member(m: u32, ) -> Weight {
		(14_218_000 as Weight)
			// Standard Error: 0
			.saturating_add((55_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:0)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn reset_member(m: u32, ) -> Weight {
		(13_830_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((186_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Membership Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Membership Prime (r:1 w:1)
	// Storage: Collective Members (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn change_key(m: u32, ) -> Weight {
		(14_772_000 as Weight)
			// Standard Error: 0
			.saturating_add((51_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Membership Members (r:1 w:0)
	// Storage: Membership Prime (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn set_prime(m: u32, ) -> Weight {
		(3_651_000 as Weight)
			// Standard Error: 0
			.saturating_add((29_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Membership Prime (r:0 w:1)
	// Storage: Collective Prime (r:0 w:1)
	fn clear_prime(_m: u32, ) -> Weight {
		(1_014_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
