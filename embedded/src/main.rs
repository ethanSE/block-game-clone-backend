#![no_main]
#![no_std]

use block_game_clone_backend::{piece::PieceName, ts_interop::Action};
use panic_halt as _;

// use block_game_clone_backend::*;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

//memory

use alloc_cortex_m::CortexMHeap;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();
const HEAP_SIZE: usize = 1024 * 40; // in bytes

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    hprintln!("Hello, world!").unwrap();

    let mut a = block_game_clone_backend::game_state::GameState::default();
    a.apply_action(Action::SelectPiece(PieceName::L));
    a.apply_action(Action::PreviewPiece(
        block_game_clone_backend::ts_interop::V3([0.0, 0.0, 0.0].into()),
    ));
    a.apply_action(Action::PlayPreviewedPiece);
    a.apply_action(Action::MakeGreedyAIMove);
    hprintln!("{:?}", a).unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
