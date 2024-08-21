use std::borrow::{Borrow, BorrowMut, Cow};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Deref;
use std::time::SystemTime;
use std::{env, io};
use std::path::{Path, PathBuf};
use std::vec;
use anyhow::Error;
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
use metapower_framework::{
    dao::sqlite::MetapowerSqlite3, AI_PATO_DIR,
};
use metapower_framework::{
    get_past_date_str, log, AirdropRequest, AllPatosResponse, EmptyRequest, NameResponse, SimpleResponse
};
use uuid::Uuid;

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
const PERSONA_PAGES: u64 = 1;

#[derive(Default)]
struct Cbor<T>(pub T)
where T: serde::Serialize + serde::de::DeserializeOwned;

impl<T> std::ops::Deref for Cbor<T>
where T: serde::Serialize + serde::de::DeserializeOwned
{
  type Target = T;

  fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> Storable for Cbor<T>
where T: serde::Serialize + serde::de::DeserializeOwned
{
  fn to_bytes(&self) -> Cow<[u8]> {
    let mut buf = vec![];
    ciborium::ser::into_writer(&self.0, &mut buf).unwrap();
    Cow::Owned(buf)
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Self(ciborium::de::from_reader(bytes.as_ref()).unwrap())
  }
  
  const BOUND: ic_stable_structures::storable::Bound;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MM<RM>> = RefCell::new(
        MM::init(RM::new(DefaultMemoryImpl::default(), METADATA_PAGES..u64::MAX))
        );
    static PERSONA: RefCell<StableCell<Cbor<String>, RM>> =
        RefCell::new(StableCell::init(
            RM::new(DefaultMemoryImpl::default(), 0..PERSONA_PAGES),
            Cbor::default(),
        ).expect("failed to initialize the metadata cell")
        );          
    static DEFAULT_PERSONA:  RefCell<StableCell<Cbor<String>, RM>> =
        RefCell::new(StableCell::init(
            RM::new(DefaultMemoryImpl::default(), 0..PERSONA_PAGES),
            Cbor::default(),
        ).expect("failed to initialize the metadata cell")
        );  

    static PATO_NAME: RefCell<StableLog<Cbor<String>, VM, VM>> =
        MEMORY_MANAGER.with(|mm| {
          RefCell::new(StableLog::init(
            mm.borrow().get(LOG_NAME_INDX_MEM_ID),
            mm.borrow().get(LOG_NAME_DATA_MEM_ID),
          ).expect("failed to initialize the name record"))
        });
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
            place TEXT NOT NULL,
            sender TEXT NOT NULL,
            receiver TEXT NOT NULL,
            question TEXT NOT NULL,
            answer TEXT NOT NULL
        )";
        let pray_table = "CREATE TABLE IF NOT EXISTS pray_message (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            sender TEXT NOT NULL,
            content TEXT NOT NULL
        )";

        MetapowerSqlite3 {
            db_name: "population_database".to_string(),
        }
        .create_table(message_table.to_owned())?
        .create_table(pray_table.to_owned())?;

        Ok(())
    }
    fn update_name_file(&self, _: String, name: String) -> Result<(), Error> {
        PATO_NAME.with(|v| v.borrow_mut().append(&Cbor(name)));
        Ok(())
    }
    async fn prepare_pato_db(&self, id: String, name: String) -> Result<i64, String> {
        let mut sn = 0;
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

        let (resp,): (SimpleResponse,) = match call(
            callee.clone(),
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

        let request = AirdropRequest { id, amount: 100.0 };
        let (_resp,): (SimpleResponse,) =
            match call(callee.clone(), "request_airdrop", (request,)).await {
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
    fn count_pato_sessions(&self, id: String, count_days: u64) -> i64 {
        let mut session_count = 0;
        for i in 0..count_days {
            let date_string = get_past_date_str(i);
            let session_dir = format!("{}/{}/db/{}", AI_PATO_DIR, id, date_string);
            log!("searching {}", session_dir);
            let path = Path::new(&session_dir);
            if path.exists() && path.is_dir() {
                for entry in fs::read_dir(path).unwrap().flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        session_count += 1;
                    }
                }
            }
        }
        session_count
    }
    fn create_pato_iss(&self, id: String, name: String, default_person: String) -> Result<(), Error> {
        PERSONA.with(|v|{
            v.borrow_mut().set(Cbor(default_person));
        });

        Ok(())
    }
    pub fn get_all_categories(&self) -> Vec<&str> {
        let activity_categories = self.get_event_category();
        let mut all_categories = vec![];
        for (_, v) in activity_categories.iter() {
            all_categories.extend(v.clone());
        }
        all_categories
    }
    pub fn get_event_category(&self) -> HashMap<&'static str, Vec<&'static str>> {
        HashMap::from([
            ("eat", vec!["cafe"]),
            ("work", vec!["office building", "library"]),
            ("learn", vec!["library", "school"]),
            ("commuting", vec!["airport"]),
            ("entertainment", vec!["museum"]),
            ("shopping", vec!["shopping mall"]),
        ])
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

        let pato_id = Uuid::new_v4().to_string();

        if let Err(e) = self.create_pato_db() {
            log!("pato数据库创建失败: {}", e);
            create_pato_success = false;
        }
        let mut sn = 0;
        match self
            .prepare_pato_db(pato_id.clone(), request.name.clone())
            .await
        {
            Ok(last_sn) => {
                sn = last_sn;
            }
            Err(e) => {
                log!("pato注册失败: {}", e);
                create_pato_success = false;
            }
        }
        if let Err(e) = self.update_name_file(pato_id.clone(), request.name.clone()) {
            log!("pato名称文件更新失败: {}", e);
            create_pato_success = false;
        }

        DEFAULT_PERSONA.with(|v| {
            let tmp = v.borrow();
            let persona =  tmp.get();
            let _ = self.create_pato_iss(pato_id.clone(), request.name.clone(), persona.deref().to_string());
        });


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
        let callee = CALLEE.with(|callee| callee.borrow().as_ref().unwrap().clone());

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
                    let sessions = self.count_pato_sessions(pato.id.clone(), 3);
                    let info = HotAi {
                        id: pato.id.clone(),
                        name: name_pro.name.clone(),
                        talks: sessions as i32,
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
