use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Block;

    #[wasm_bindgen(static_method_of = Block, js_name = indep_hash)]
    pub fn indep_hash() -> String;

    #[wasm_bindgen(static_method_of = Block, js_name = height)]
    pub fn height() -> i32;

    #[wasm_bindgen(static_method_of = Block, js_name = timestamp)]
    pub fn timestamp() -> i32;

    #[wasm_bindgen(js_name = rust_fetch)]
    pub async fn fetch(s: &str) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Contract;

    #[wasm_bindgen(static_method_of = Contract, js_name = contractId)]
    pub fn id() -> String;

    #[wasm_bindgen(static_method_of = Contract, js_name = contractOwner)]
    pub fn owner() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Transaction;

    #[wasm_bindgen(static_method_of = Transaction, js_name = id)]
    pub fn id() -> String;

    #[wasm_bindgen(static_method_of = Transaction, js_name = owner)]
    pub fn owner() -> String;

    #[wasm_bindgen(static_method_of = Transaction, js_name = target)]
    pub fn target() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type KV;

    #[wasm_bindgen(catch, static_method_of = KV, js_name = kvGet)]
    pub async fn get(key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, static_method_of = KV, js_name = kvPut)]
    pub async fn put(key: &str, value: JsValue) -> Result<(), JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type SmartWeave;

    #[wasm_bindgen(catch, static_method_of = SmartWeave, js_name = readContractState)]
    pub async fn read_contract_state(contract_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, static_method_of = SmartWeave, js_name = viewContractState)]
    pub async fn view_contract_state(contract_id: &str, input: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, static_method_of = SmartWeave, js_name = write)]
    pub async fn write(contract_id: &str, input: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(static_method_of = SmartWeave, js_name = refreshState)]
    pub async fn refresh_state();

    #[wasm_bindgen(static_method_of = SmartWeave, js_name = caller)]
    pub fn caller() -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    pub type Vrf;

    #[wasm_bindgen(static_method_of = Vrf, js_name = value)]
    pub fn value() -> String;

    #[wasm_bindgen(static_method_of = Vrf, js_name = randomInt)]
    pub fn randomInt(max_value: i32) -> i32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}