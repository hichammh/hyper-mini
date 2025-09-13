use axum::{
    routing::{get, post},
    Router,
    extract::State,
    Json,
};
use serde::{Serialize, Deserialize};
use std::{sync::{Arc, Mutex}, collections::HashMap};
use uuid::Uuid;

use crate::vm;

#[derive(Clone, Default)]
pub struct VmState {
    pub inner: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Deserialize)]
pub struct StartReq { pub mem_mb: u64 }

#[derive(Serialize)]
pub struct StartRes { pub id: String, pub status: String }

async fn start_vm(State(st): State<VmState>, Json(req): Json<StartReq>) -> Json<StartRes> {
    let id = Uuid::new_v4().to_string();
    // Call VM backend (dummy or KVM)
    let status = vm::start_vm(id.clone(), req.mem_mb).unwrap_or("error".into());
    st.inner.lock().unwrap().insert(id.clone(), status.clone());
    Json(StartRes { id, status })
}

async fn list_vms(State(st): State<VmState>) -> Json<Vec<(String, String)>> {
    Json(st.inner.lock().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect())
}

async fn stop_vm() -> &'static str { "ok" }

pub fn router() -> Router {
    let st = VmState::default();
    Router::new()
        .route("/v1/vm/start", post(start_vm))
        .route("/v1/vm/stop/:id", post(stop_vm))
        .route("/v1/vm", get(list_vms))
        .with_state(st)
}
