
//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-solo-fresh"), DB CACHE: 128

// Executed Command:
// ./integritee-node
// benchmark
// --chain=integritee-solo-fresh
// --steps=50
// --repeat=20
// --pallet=frame_system
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/frame_system.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for frame_system.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	fn remark(b: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000)).saturating_mul(b.into())
	}
	fn remark_with_event(b: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000)).saturating_mul(b.into())
	}
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		Weight::from_ref_time(2_273_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn set_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_457_000)).saturating_mul(i.into())
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_mul(i.into())
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn kill_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(6_118_000)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(1_007_000)).saturating_mul(i.into())
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_mul(i.into())
	}
	// Storage: Skipped Metadata (r:0 w:0)
	fn kill_prefix(p: u32, ) -> Weight {
		Weight::from_ref_time(0)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(3_208_000)).saturating_mul(p.into())
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_mul(p.into())
	}
}
