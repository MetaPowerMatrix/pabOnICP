use std::cell::RefCell;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Write;
use ic_cdk::caller;
use ic_cdk_timers::TimerId;
use metapower_framework::{log, PatoLocation, AI_MATRIX_DIR, AI_PATO_DIR, TICK, HAVEAREST};

use crate::MapStatus;
use crate::OWNER;

/// Initial canister balance to track the cycles usage.
static INITIAL_CANISTER_BALANCE: AtomicU64 = AtomicU64::new(0);
/// Canister cycles usage tracked in the periodic task.
static CYCLES_USED: AtomicU64 = AtomicU64::new(0);

thread_local! {
    /// The global vector to keep multiple timer IDs.
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
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
        let tickfile = format!("{}/db/tick.txt", AI_MATRIX_DIR);

        if let Ok(content) = fs::read_to_string(tickfile.clone()){
            if let Ok(mut number) = content.trim().parse::<u64>(){
                number += TICK * HAVEAREST;
                if let Ok(mut file) = fs::File::create(tickfile){
                    let _ = write!(file, "{}", number);
                    log!("tick increased to {}", number);
                }
            }
        }
    }
    fn find_pato_dirs(&self) -> Vec<PathBuf> {
        let mut db_dirs = Vec::new();
        let path = Path::new(AI_PATO_DIR);
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    db_dirs.push(path);
                }
            }
        }
        db_dirs
    }
    fn refresh_map(&self) {
        let db_dirs = self.find_pato_dirs();
        for db_dir in db_dirs {
            let location_file_path = db_dir.join("db/location.json");
            // log!("check pato location {:?}", location_file_path);
            let map_st_path = format!("{}/db/map_status.json", AI_MATRIX_DIR);
            if location_file_path.exists() && location_file_path.is_file() {
                if let Ok(file) = fs::File::open(location_file_path){
                    if let Ok(pato_locations) = serde_json::from_reader::<File, Vec<PatoLocation>>(file){
                        if let Some(pato_location) = pato_locations.last(){
                            if let Ok(file) = fs::File::open(&map_st_path){
                                if let Ok(mut map_status) = serde_json::from_reader::<File, Vec<MapStatus>>(file){
                                    let mut found = false;
                                    for map in map_status.iter_mut(){
                                        if map.patos.contains(&pato_location.id){
                                            map.patos.retain(|x| x != &pato_location.id);
                                        }
                                    }
                                    for map in map_status.iter_mut(){
                                        if map.sn == pato_location.tile_sn {
                                            map.patos.push(pato_location.id.clone());
                                            found = true;
                                            break;
                                        }
                                    }
                                    if !found {
                                        map_status.push(MapStatus {
                                            sn: pato_location.tile_sn,
                                            patos: vec![pato_location.id.clone()],
                                        });
                                    }
                                    let map_status_json = serde_json::to_string(&map_status).unwrap();
                                    let map_status_file = Path::new(&map_st_path);
                                    if let Ok(mut file) = fs::File::create(map_status_file){
                                        let _ = write!(file, "{}", map_status_json);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    fn run_battey(&self, id: String, sn: u64) -> std::io::Result<()> {
        Ok(())
    }
    async fn check_battery_life(&self){

        // let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        // let (patos_resp,): (AllPatosResponse,) = match call::call(callee.clone(), "request_all_patos", ()).await {
        //     Ok(response) => response,
        //     Err((code, msg)) => {
        //         println!("request_all_patos失败: {}: {}", code as u8, msg);
        //         return;
        //     }
        // };

        // let patos = patos_resp.pato_sn_id;
        // for pato in patos {
        //     log!("battery life reload for pato: {}", pato.id);
        // }
    }
    
    pub fn run_loop(&self) {
        log!("matrix runner new round started.");
        self.increase_tick();
        self.refresh_map();
        self.check_battery_life();
    }
}

