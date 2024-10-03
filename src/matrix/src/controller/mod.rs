use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::vec;
use std::fmt::Write;
use anyhow::Error;
use candid::Principal;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableCell, DefaultMemoryImpl, RestrictedMemory, StableBTreeMap, StableLog, Storable};
use ic_stable_structures::memory_manager::{
    MemoryId,
    MemoryManager as MM,
    VirtualMemory,
  };
  
use crate::{
    CreateRequest, CreateResonse, EmptyResponse, HotAi, HotAiResponse,
    HotTopicResponse, Knowledge, LoginRequest, SharedKnowledgesResponse, 
    CALLEE,
};
use ic_cdk::api::call::call;
use metapower_framework::dao::sqlite::MetapowerSqlite3;
use metapower_framework::{
    log, AirdropRequest, AllPatosResponse, EmptyRequest, NameResponse, SimpleResponse
};

type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;

const KNOWLEDGES_MEM_ID: MemoryId = MemoryId::new(0);
const LOG_NAME_INDX_MEM_ID: MemoryId = MemoryId::new(1);
const LOG_NAME_DATA_MEM_ID: MemoryId = MemoryId::new(2);
const SUMMARY_MEM_ID: MemoryId = MemoryId::new(3);
const LOG_SESSION_INDX_MEM_ID: MemoryId = MemoryId::new(4);
const LOG_SESSION_DATA_MEM_ID: MemoryId = MemoryId::new(5);
const LOG_TOPICS_INDX_MEM_ID: MemoryId = MemoryId::new(6);
const LOG_TOPICS_DATA_MEM_ID: MemoryId = MemoryId::new(7);
const METADATA_PAGES: u64 = 16;

#[derive(Default)]
struct Cbor<T>(pub T) where T: serde::Serialize + serde::de::DeserializeOwned;

impl<T> std::ops::Deref for Cbor<T> where T: serde::Serialize + serde::de::DeserializeOwned
{
  type Target = T;

  fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> Storable for Cbor<T> where T: serde::Serialize + serde::de::DeserializeOwned
{

    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self.0, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(ciborium::de::from_reader(bytes.as_ref()).unwrap())
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MM<RM>> = RefCell::new(
        MM::init(RM::new(DefaultMemoryImpl::default(), METADATA_PAGES..u64::MAX))
        );

    static SESSIONS: RefCell<StableLog<String, VM, VM>> =
        MEMORY_MANAGER.with(|mm| {
          RefCell::new(StableLog::init(
            mm.borrow().get(LOG_SESSION_INDX_MEM_ID),
            mm.borrow().get(LOG_SESSION_DATA_MEM_ID),
          ).expect("failed to initialize the session record"))
        });
    static TOPICS: RefCell<StableLog<String, VM, VM>> =
        MEMORY_MANAGER.with(|mm| {
          RefCell::new(StableLog::init(
            mm.borrow().get(LOG_TOPICS_INDX_MEM_ID),
            mm.borrow().get(LOG_TOPICS_DATA_MEM_ID),
          ).expect("failed to initialize the session record"))
        });
    static KNOWLEDGES: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
          RefCell::new(StableBTreeMap::init(mm.borrow().get(KNOWLEDGES_MEM_ID)))
        });        
    static SUMMARY: RefCell<StableBTreeMap<String, String, VM>> =
        MEMORY_MANAGER.with(|mm| {
          RefCell::new(StableBTreeMap::init(mm.borrow().get(SUMMARY_MEM_ID)))
        });        
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
            message TEXT NOT NULL,
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
    fn get_pato_shared_books(&self, id: String) -> Vec<String> {
        let mut books: Vec<String> = vec![];
        KNOWLEDGES.with(|item| 
            for (_, v) in item.borrow().iter(){
                books.push(v)
            }
        );

        books
    }

    pub fn request_pato_login(
        &self,
        _request: LoginRequest,
    ) -> std::result::Result<EmptyResponse, Error> {
        let response = EmptyResponse {};

        Ok(response)
    }

    pub async fn request_create_pato(
        &self,
        request: CreateRequest,
    ) -> std::result::Result<CreateResonse, Error> {
        let mut create_pato_success = true;

        let (bytes,): (Vec<u8>,) = ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ()).await.unwrap_or_default();
        // let pato_id = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();
        let pato_id = bytes.iter().fold("".to_string(), |mut acc, a| { write!(acc, "{:02x}", a).unwrap_or_default(); acc});

        if let Err(e) = self.create_pato_db() {
            println!("pato数据库创建失败: {}", e);
            create_pato_success = false;
        }

        match self
            .prepare_pato_db(pato_id.clone(), request.name.clone())
            .await
        {
            Ok(last_sn) => {
                println!("pato {} sn {}", pato_id, last_sn);
            }
            Err(e) => {
                create_pato_success = false;
                ic_cdk::trap(&e);
            }
        }

        let response = if create_pato_success {
            CreateResonse {
                id: pato_id.clone(),
            }
        } else {
            CreateResonse { id: "".to_string() }
        };

        Ok(response)
    }
    pub async fn request_hot_ai(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<HotAiResponse, String> {
        let mut all_ais: Vec<HotAi> = vec![];
        let callee = CALLEE.with(|callee| *callee.borrow().as_ref().unwrap());

        let (patos_resp,): (AllPatosResponse,) =
            match call(callee, "request_all_patos", ()).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    return Err(format!("request_all_patos失败: {}: {}", code as u8, msg))
                }
            };
        let patos = patos_resp.pato_sn_id;
        let ids: Vec<String> = patos.iter().map(|p| p.id.to_owned()).collect();

        let (resp,): (NameResponse,) =
            match call(callee, "request_pato_name_and_pro", (ids,)).await {
                Ok(name_pro_resp) => name_pro_resp,
                Err((code, msg)) => {
                    return Err(format!(
                        "request_pato_name_and_pro: {}: {}",
                        code as u8, msg
                    ))
                }
            };
        for pato in patos {
            for name_pro in resp.name_pros.iter() {
                if name_pro.id == pato.id {
                    let info = HotAi {
                        id: pato.id.clone(),
                        name: name_pro.name.clone(),
                        talks: 1,
                        pros: name_pro.pros.join(","),
                    };
                    all_ais.push(info);
                }
            }
        }

        all_ais.sort_by(|a, b| b.talks.cmp(&a.talks));
        log!("all sorted ais {:?}", all_ais);
        let hot_ais = if all_ais.len() > 10 {
            let _ = all_ais.split_off(5);
            all_ais
        } else {
            all_ais
        };
        log!("hot_ais {:?}", hot_ais);
        let response = HotAiResponse { sheniu: hot_ais };
        Ok(response)
    }

    pub async fn request_shared_knowledges(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<SharedKnowledgesResponse, String> {
        let mut knowledges: Vec<Knowledge> = vec![];

        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (patos_resp,): (AllPatosResponse,) =
            match call(callee, "request_all_patos", ()).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    return Err(format!("request_all_patos失败: {}: {}", code as u8, msg))
                }
            };
        let patos = patos_resp.pato_sn_id;
        for pato in patos {
            let books = self.get_pato_shared_books(pato.id.clone());
            for book in books {
                if !book.is_empty() && book.contains('#') {
                    let pair = book
                        .split('#')
                        .map(|b| b.to_owned())
                        .collect::<Vec<String>>();
                    let summary = SUMMARY.with(|v| v.borrow().get(&pair[1]).unwrap_or_default());
                    if !summary.is_empty() {
                        let resp = Knowledge {
                            sig: pair[1].clone(),
                            title: pair[0].clone(),
                            owner: pato.id.clone(),
                            summary,
                        };
                        knowledges.push(resp);
                    }
                }
            }
        }

        let response = SharedKnowledgesResponse { books: knowledges };
        Ok(response)
    }

    pub async fn request_hot_topics(
        &self,
        _request: EmptyRequest,
    ) -> std::result::Result<HotTopicResponse, String> {
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (patos_resp,): (AllPatosResponse,) =
            match call(callee, "request_all_patos", ()).await {
                Ok(response) => response,
                Err((code, msg)) => {
                    return Err(format!("request_all_patos失败: {}: {}", code as u8, msg))
                }
            };

        let mut all_events = vec![];
        TOPICS.with(|v|{
            for event in v.borrow().iter(){
                all_events.push(event);
            }
        });
        
        let mut set = HashSet::new();
        let mut result = Vec::new();
        for item in all_events {
            if set.insert(item.clone()) {
                result.push(item.clone());
            }
        }

        let response = HotTopicResponse { topics: result };
        Ok(response)
    }
}
