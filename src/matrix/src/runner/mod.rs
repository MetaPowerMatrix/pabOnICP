use std::time::SystemTime;
use std::env;
use std::fs;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::process::Command;
use metapower_framework::service::metapowermatrix_agent_mod::agent_grpc::{
    meta_power_matrix_agent_svc_client::MetaPowerMatrixAgentSvcClient,
    EmptyRequest
};
use metapower_framework::{log, PatoLocation, AGENT_GRPC_REST_SERVER, AI_MATRIX_DIR, AI_PATO_DIR, TICK, HAVEAREST};
use sysinfo::System;
use tokio::time::sleep;
use crate::MapStatus;

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
        let battery_life = "/data/bin/metapower_battery";
        //         -i {} -s {}", id, sn);
        unsafe {
            Command::new(battery_life)
                .arg("-i")
                .arg(id)
                .arg("-s")
                .arg(sn.to_string())
                .pre_exec(|| {
                    // This makes the child process the leader of a new session
                    libc::setsid();
                    Ok(())
                })
                .spawn()?;
        }

        log!("Started process in detached mode.");
        Ok(())
    }
    async fn check_battery_life(&self){
        let mut sys = System::new_all();
        // Update all information of our system struct.
        sys.refresh_all();

        let mut cmds  = vec![];

        for (pid, process) in sys.processes() {
            let name = process.name();
            if name.contains("metapower") {
                // log!("PID: {} - Name: {}", pid, name);
                match process.cmd() {
                    // Command line as a Vec<String>
                    cmd if !cmd.is_empty() => {
                        // log!("Command line: {}", cmd.join(" "));
                        cmds.push(cmd);
                    }
                    _ => { log!("Command line not available"); }
                }
            }
        }

        if let Ok(mut client) = MetaPowerMatrixAgentSvcClient::connect(AGENT_GRPC_REST_SERVER).await {
            let request = EmptyRequest{};
            if let Ok(patos_resp) = client.request_for_all_patos(request).await{
                let patos = patos_resp.get_ref().pato_sn_id.clone();
                for pato in patos {
                    let is_running = cmds.iter().any(|c| (*c).contains(&pato.id));
                    if !is_running {
                        if let Err(e) = self.run_battey(pato.id.clone(), pato.sn.parse::<u64>().unwrap_or_default()){
                            println!("run_battey error: {}", e);
                        }
                        log!("battery life reload for pato: {}", pato.id);
                        sleep(std::time::Duration::from_secs(TICK * HAVEAREST)).await;
                    }
                    sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    }
    pub async fn run_loop(&self) {
        loop {
            log!("matrix runner new round started.");
            self.increase_tick();
            self.refresh_map();
            self.check_battery_life().await;
    
            sleep(std::time::Duration::from_secs(TICK)).await;
        }
    }
    
}

