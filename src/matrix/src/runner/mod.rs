use std::cell::RefCell;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use ic_cdk::caller;
use ic_cdk_timers::TimerId;
use metapower_framework::log;
use crate::OWNER;

static INITIAL_CANISTER_BALANCE: AtomicU64 = AtomicU64::new(0);
static CYCLES_USED: AtomicU64 = AtomicU64::new(0);
static mut SYSTEM_TICKS: u128 = 0;

thread_local! {
    /// The global vector to keep multiple timer IDs.
    static TIMER_IDS: RefCell<Vec<TimerId>> = const { RefCell::new(Vec::new()) };
}

fn track_cycles_used() {
    // Update the `INITIAL_CANISTER_BALANCE` if needed.
    let current_canister_balance = ic_cdk::api::canister_balance();
    INITIAL_CANISTER_BALANCE.fetch_max(current_canister_balance, Ordering::Relaxed);

    // Store the difference between the initial and the current balance.
    let cycles_used = INITIAL_CANISTER_BALANCE.load(Ordering::Relaxed) - current_canister_balance;
    CYCLES_USED.store(cycles_used, Ordering::Relaxed);
}

#[ic_cdk_macros::init]
fn init(min_interval_secs: u64) {
    unsafe {
        OWNER = caller();
    }   
    start_with_interval_secs(min_interval_secs);
}

#[ic_cdk_macros::query]
fn cycles_used() -> u64 {
    CYCLES_USED.load(Ordering::Relaxed)
}

#[ic_cdk_macros::update]
fn stop() {
    TIMER_IDS.with(|timer_ids| {
        if let Some(timer_id) = timer_ids.borrow_mut().pop() {
            ic_cdk::println!("Timer canister: Stopping timer ID {timer_id:?}...");
            // It's safe to clear non-existent timer IDs.
            ic_cdk_timers::clear_timer(timer_id);
        }
    });
}

#[ic_cdk_macros::update]
fn start_with_interval_secs(secs: u64) {
    let secs = Duration::from_secs(secs);
    ic_cdk::println!("Timer canister: Starting a new timer with {secs:?} interval...");
    // Schedule a new periodic task to increment the counter.
    let timer_id = ic_cdk_timers::set_timer_interval(secs, || MatrixRunner::default().run_loop());
    // Add the timer ID to the global vector.
    TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));

    // To drive an async function to completion inside the timer handler,
    // use `ic_cdk::spawn()`, for example:
    // ic_cdk_timers::set_timer_interval(interval, || ic_cdk::spawn(async_function()));
}

#[derive(Debug)]
pub struct MatrixRunner{
    pub version: String,
}

impl Default for MatrixRunner {
    fn default() -> Self {
        MatrixRunner {
            version: "0.1.0".to_string(),
        }
    }
}

impl MatrixRunner {
    fn increase_tick(&self) {
        unsafe { SYSTEM_TICKS += 1 };
    }
    
    pub fn run_loop(&self) {
        log!("matrix runner new round started.");
        self.increase_tick();
        track_cycles_used();
    }
}

