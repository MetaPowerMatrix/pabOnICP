use anyhow::{anyhow, Error};
use candid::Principal;
use ic_stable_structures::memory_manager::MemoryManager as MM;
use ic_stable_structures::{DefaultMemoryImpl, RestrictedMemory};
use stable_fs::fs::{FdStat, FileSystem, OpenFlags};
use stable_fs::storage::stable::StableStorage;
use std::cell::RefCell;
use std::fmt::Write;
use crate::{
    CreateRequest, CreateResonse, LoginRequest,
    CALLEE,
};
use ic_cdk::api::call::call;
use metapower_framework::dao::sqlite::MetapowerSqlite3;
use metapower_framework::{get_now_secs, PatoInfo, SimpleResponse, MAX_SAVE_BYTES};

type RM = RestrictedMemory<DefaultMemoryImpl>;

const METADATA_PAGES: u64 = 512;

thread_local! {
    static MEMORY_MANAGER: RefCell<MM<RM>> = RefCell::new(
        MM::init(RM::new(DefaultMemoryImpl::default(), 16..METADATA_PAGES))
        );

    static FS: RefCell<FileSystem> = {
        MEMORY_MANAGER.with(|m| {
            let memory_manager = m.borrow();
            let storage = StableStorage::new_with_memory_manager(&memory_manager, 200..210u8);
            RefCell::new(
                FileSystem::new(Box::new(storage)).unwrap()
            )
        })
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

    pub fn save_session_assets(&self, id: String, session: String, file_name: String, data: [u8; MAX_SAVE_BYTES]) 
        -> Result<(), Error>{
        let chat_session_message_file = format!(
            "ai/gen/{}/{}/{}",
            id, session, file_name
        );
        if let Err(e) = FS.with(|fs|{
            let fd = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &chat_session_message_file, 
                FdStat::default(), OpenFlags::CREATE|OpenFlags::TRUNCATE, get_now_secs()).unwrap();
            if let Err(e) = fs.borrow_mut().write(fd, &data){
                return Err(anyhow!("{:?}", e));
            }
            Ok(())
        }){
            return Err(anyhow!("{:?}", e));
        }

        Ok(())
    }

    pub fn get_session_assets(&self, id: String, session: String, file_name: String) -> Result<[u8; MAX_SAVE_BYTES], Error>
    {
        let chat_session_message_file = format!(
            "ai/gen/{}/{}/{}",
            id, session, file_name
        );
        let mut data = [0; MAX_SAVE_BYTES];
        if let Err(e) = FS.with(|fs|{
            let fd = fs.borrow_mut().open_or_create(fs.borrow().root_fd(), &chat_session_message_file, 
            FdStat::default(),OpenFlags::empty(), get_now_secs()).unwrap();
            if let Err(e) = fs.borrow_mut().read(fd, &mut data) { return Err(anyhow!("{:?}", e)) }
            Ok(())
        }){
            return Err(anyhow!("{:?}", e));
        }

        Ok(data)
    }

}
