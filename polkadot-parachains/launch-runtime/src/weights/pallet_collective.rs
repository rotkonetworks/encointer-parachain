
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-26, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("launch-rococo-fresh"), DB CACHE: 128

// Executed Command:
// target/release/encointer-collator
// benchmark
// --chain=launch-rococo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/launch-runtime/src/weights/pallet_collective.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Collective Members (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:0)
	// Storage: Collective Voting (r:100 w:100)
	// Storage: Collective Prime (r:0 w:1)
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		Weight::from_parts(0, 0)
			// Standard Error: 69_000
			.saturating_add(Weight::from_parts(15_485_000_u64, 0).saturating_mul(m.into()))
			// Standard Error: 69_000
			.saturating_add(Weight::from_parts(20_494_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Collective Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_parts(20_626_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(3_000_u64, 0).saturating_mul(b.into()))
			// Standard Error: 1_000
			.saturating_add(Weight::from_parts(74_000_u64, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_parts(24_295_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(3_000_u64, 0).saturating_mul(b.into()))
			// Standard Error: 1_000
			.saturating_add(Weight::from_parts(149_000_u64, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalCount (r:1 w:1)
	// Storage: Collective Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_parts(36_933_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(10_000_u64, 0).saturating_mul(b.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(86_000_u64, 0).saturating_mul(m.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(360_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
		Weight::from_parts(41_498_000, 0)
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(183_000_u64, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_parts(45_184_000, 0)
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(153_000_u64, 0).saturating_mul(m.into()))
			// Standard Error: 2_000
			.saturating_add(Weight::from_parts(322_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_parts(55_539_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(9_000_u64, 0).saturating_mul(b.into()))
			// Standard Error: 4_000
			.saturating_add(Weight::from_parts(143_000_u64, 0).saturating_mul(m.into()))
			// Standard Error: 4_000
			.saturating_add(Weight::from_parts(365_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Prime (r:1 w:0)
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_parts(48_452_000, 0)
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(166_000_u64, 0).saturating_mul(m.into()))
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(348_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Collective Voting (r:1 w:1)
	// Storage: Collective Members (r:1 w:0)
	// Storage: Collective Prime (r:1 w:0)
	// Storage: Collective ProposalOf (r:1 w:1)
	// Storage: Collective Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_parts(60_654_000, 0)
			// Standard Error: 0
			.saturating_add(Weight::from_parts(7_000_u64, 0).saturating_mul(b.into()))
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(190_000_u64, 0).saturating_mul(m.into()))
			// Standard Error: 3_000
			.saturating_add(Weight::from_parts(336_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Collective Proposals (r:1 w:1)
	// Storage: Collective Voting (r:0 w:1)
	// Storage: Collective ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_parts(26_137_000, 0)
			// Standard Error: 11_000
			.saturating_add(Weight::from_parts(544_000_u64, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
