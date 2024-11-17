use anyhow::{anyhow, Error};
use candid::Principal;
use ic_stable_structures::DefaultMemoryImpl;
use stable_fs::fs::{Fd, FdStat, FileSize, FileSystem, OpenFlags};
use stable_fs::storage::stable::StableStorage;
use stable_fs::storage::types::Node;
use std::cell::RefCell;
use std::fmt::Write;
use std::str::FromStr;
use crate::{
    CreateRequest, CreateResonse, LoginRequest,
    CALLEE,
};
use ic_cdk::api::call::call;
use metapower_framework::dao::sqlite::MetapowerSqlite3;
use metapower_framework::{PatoInfo, SimpleResponse};

thread_local! {
    static FS: RefCell<FileSystem> = {
        let memory = DefaultMemoryImpl::default();
        let storage = StableStorage::new(memory);
        RefCell::new(FileSystem::new(Box::new(storage)).unwrap())
    };
}

#[derive(Debug, Default)]
pub struct MetaPowerMatrixControllerService {}

impl MetaPowerMatrixControllerService {
    fn create_pato_db(&self) -> Result<(), Error> {
        let message_table = "CREATE TABLE IF NOT EXISTS chat_message (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            session TEXT NOT NULL,
            sender TEXT NOT NULL,
            receiver TEXT NOT NULL,
            message TEXT NOT NULL
        )";

        MetapowerSqlite3::new().create_table(message_table.to_owned())?;

        Ok(())
    }
    async fn prepare_pato_db(&self, id: String, name: String) -> Result<i64, String> {
        let mut sn = 0;
        let callee = CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());

        let (resp,): (SimpleResponse,) = match call(
            callee,
            "request_population_registration",
            (name, id.clone()),
        )
        .await
        {
            Ok(response) => response,
            Err((code, msg)) => return Err(format!("pato注册失败: {}: {}", code as u8, msg)),
        };
        if resp.success {
            if let Ok(last_sn) = resp.message.parse::<i64>() {
                sn = last_sn;
            }
        }

        let bonus_register = 100.0_f32;
        let (_resp,): (SimpleResponse,) =
            match call(callee, "request_airdrop", (bonus_register, id)).await {
                Ok(response) => response,
                Err((code, msg)) => return Err(format!("pato空投失败: {}: {}", code as u8, msg)),
            };

        Ok(sn)
    }

    pub fn request_pato_login(
        &self,
        _request: LoginRequest,
    ) -> std::result::Result<(), Error> {
        Ok(())
    }

    pub async fn request_create_pato(
        &self,
        request: CreateRequest,
    ) -> std::result::Result<CreateResonse, Error> {
        let (bytes,): (Vec<u8>,) =
            ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ())
                .await
                .unwrap_or_default();
        // let pato_id = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        let pato_id = bytes.iter().fold("".to_string(), |mut acc, a| {
            write!(acc, "{:02x}", a).unwrap_or_default();
            acc
        });

        if let Err(e) = self.create_pato_db() {
            return Err(anyhow!("message数据库创建失败: {}", e));
        }

        match self
            .prepare_pato_db(pato_id.clone(), request.name.clone())
            .await
        {
            Ok(last_sn) => {
                println!("pato {} sn {}", pato_id, last_sn);
            }
            Err(e) => {
                return Err(anyhow!("agent数据库创建失败: {}", e));
            }
        }

        let response = CreateResonse {
            id: pato_id.clone(),
        };

        Ok(response)
    }
    pub async fn request_hot_ai(&self) -> std::result::Result<Vec<PatoInfo>, String> {
        let callee = CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());

        let (patos_resp,): (Vec<PatoInfo>,) = match call(callee, "request_all_patos", ()).await {
            Ok(response) => response,
            Err((code, msg)) => return Err(format!("request_hot_ai: {}: {}", code as u8, msg)),
        };

        Ok(patos_resp)
    }
    pub fn save_session_assets(&self, id: String, session: String, file_name: String, data: Vec<u8>)
        -> Result<FileSize, Error>{
        let dirs = format!("ai/gen/{}/{}", id, session);
        let root_fd = FS.with(|fs| fs.borrow_mut().root_fd());
        let dir = self.open_dir(root_fd, dirs)?;
        let mut size = 0;

        match FS.with(|fs|{
            match fs.borrow_mut().open_or_create(dir, &file_name, 
                FdStat::default(), OpenFlags::CREATE|OpenFlags::TRUNCATE, 0){
                Ok(fd) => Ok(fd),
                Err(e) => Err(anyhow!("{:?}", e)),
            }
        }){
            Ok(fd) => {
                if let Err(e) = FS.with(|fs|{
                    match fs.borrow_mut().write(fd, &data) { 
                        Ok(s) => size = s,
                        Err(e) => return Err(anyhow!("{:?}", e)) 
                    };
                    Ok(())
                }){
                    return Err(anyhow!("{:?}", e));
                }
            }
            Err(e) => return Err(anyhow!("{:?}", e)),
        }

        Ok(size)
    }

    pub fn get_session_assets(&self, id: String, session: String, file_name: String) -> Result<(Vec<u8>, u64), Error>
    {
        let dirs = format!("ai/gen/{}/{}", id, session);
        let root_fd = FS.with(|fs| fs.borrow_mut().root_fd());
        let dir = self.open_dir(root_fd, dirs)?;
        let mut size = 0;

        let mut data: Vec<u8> = vec![];
        match FS.with(|fs|{
            match fs.borrow_mut().open_or_create(dir, &file_name, 
            FdStat::default(),OpenFlags::empty(), 0){
                Ok(fd) => Ok(fd),
                Err(e) => Err(anyhow!("{:?}", e)),
            }
        }){
            Ok(fd) => {
                if let Err(e) = FS.with(|fs|{
                    let mut fhandle = fs.borrow_mut();
                    let meta = fhandle.metadata(fd).unwrap();
                    let mut buf  = vec![0; meta.size as usize];
                    match fhandle.read(fd, &mut buf) { 
                        Ok(s) => {
                            size = s;
                            data = buf;
                        }
                        Err(e) => return Err(anyhow!("{:?}", e)) 
                    };
                    Ok(())
                }){
                    return Err(anyhow!("{:?}", e));
                }
            }
            Err(e) => return Err(anyhow!("{:?}", e)),
        }

        Ok((data, size))
    }

    pub fn list_files(&self, path: String) -> Result<Vec<String>, Error> {
        let mut res = vec![];
        let mut root_fd = FS.with(|fs| fs.borrow_mut().root_fd());
        if path != *"/"{
            root_fd = self.open_dir(root_fd, path)?;
        }
        // let fd = FS.with(|fs|{
        //     let fd = fs.borrow_mut().open_or_create(root_fd, path, FdStat::default(), OpenFlags::DIRECTORY, 0).unwrap();
        //     fd
        // });
        let mut entry_index = FS.with(|fs|{
            let meta = fs.borrow_mut().metadata(root_fd).unwrap();
            meta.first_dir_entry
        });

        while let Some(index) = entry_index {
            let entry = FS.with(|fs|{
                fs.borrow_mut().get_direntry(root_fd, index).unwrap()
            });

            let filename_str: &str =
                std::str::from_utf8(&entry.name.bytes[0..(entry.name.length as usize)]).unwrap();

            let st = String::from_str(filename_str).unwrap();

            res.push(st);

            entry_index = entry.next_entry;
        }

        Ok(res)
    }
    fn create_dir(&self, root_fd: u32, dirs: String) -> Result<u32, Error>{
        let path = dirs.split("/").collect::<Vec<&str>>();
        let mut last_dir = root_fd;

        for dir_name in path {
            match FS.with(|fs| {
                match fs.borrow_mut().create_dir(last_dir, dir_name, FdStat::default(), 0){
                    Ok(dir) => Ok(dir),
                    Err(e) => Err(anyhow!("{:?}", e)),
                }
            }){
                Ok(dir) => last_dir = dir,
                Err(e) => return Err(anyhow!("{:?}", e)),
            }
        }

        Ok(last_dir)
    }
    fn open_dir(&self, root_fd: u32, dirs: String) -> Result<u32, Error>{
        let path = dirs.split("/").collect::<Vec<&str>>();
        let mut last_dir = root_fd;

        for dir_name in path {
            match FS.with(|fs| {
                match fs.borrow_mut().open_or_create(last_dir,dir_name,FdStat::default(),OpenFlags::CREATE|OpenFlags::DIRECTORY,0){
                    Ok(dir) => Ok(dir),
                    Err(e) => Err(anyhow!("{:?}", e)),
                }
            }){
                Ok(dir) => last_dir = dir,
                Err(e) => return Err(anyhow!("{:?}", e)),
            }
        }

        Ok(last_dir)
    }

}
