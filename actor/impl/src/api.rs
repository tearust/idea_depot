use std::str::FromStr;

use tea_sdk::actors::tappstore::{txns::TappstoreTxn};
use tea_sdk::utils::wasm_actor::actors::{
	env::{tappstore_id},
};
use primitive_types::H160;
use tea_sdk::tapp::{DOLLARS, Account, Balance, TokenId};
use crate::types::*;
use tea_sdk::utils::client_wasm_actor::{help, check_auth, request, Result};
use idea_vote_state_actor_codec::{
	NAME,	IdeaQueryRequest, txn::{Idea, Txns},
};

const DAO_RESERVED_ACCOUNT: Account = H160([254u8; 20]);
const TARGET_ACTOR: &'static [u8] = NAME;
pub async fn txn_faucet(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: FaucetRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Start faucet action...");

	let txn = TappstoreTxn::TransferTea {
    token_id: tappstore_id().await?,
    from: DAO_RESERVED_ACCOUNT,
		to: req.address.parse()?,
		amount: DOLLARS * 1000,
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_tappstore_txn(
		&from_actor,
		"faucet_txn",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}

pub async fn create_idea(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: CreateIdeaRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Create idea action...");

	let unit = Balance::from_str_radix(&req.unit, 10)?;
	if unit < DOLLARS {
		return help::result_error("Invalid init contribution".into());
	}
	let txn = Txns::CreateIdea { 
		id: req.id.to_string(),
		title: req.title.to_string(), 
		description: req.description.to_string(), 
		owner: req.address.parse()?, 
		auth_b64: req.auth_b64.to_string(),
		unit,
	};

	request::send_custom_txn(
		&from_actor,
		"create_idea",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn vote_idea(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: VoteIdeaRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Vote idea action...");

	let price = Balance::from_str_radix(&req.price, 10)?;
	if price < DOLLARS {
		return help::result_error("Invalid contribution".into());
	}
	let txn = Txns::VoteIdea {
		id: req.id.to_string(),
		user: req.address.parse()?, 
		auth_b64: req.auth_b64.to_string(),
		price,
	};

	request::send_custom_txn(
		&from_actor,
		"vote_idea",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn query_idea_list(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: QueryIdeaRequest = serde_json::from_slice(&payload)?;
	info!("Start query_idea_list...");

	let uuid: String = req.uuid.to_string();

	let res = request::send_custom_query(
		&from_actor,
		IdeaQueryRequest {
			owner: None,
		},
		TARGET_ACTOR,
	)
	.await?;

	let r: Vec<Idea>  = res.0;
	let x = serde_json::json!({
		"list": r,
	});
	info!("query_idea_list => {:?}", x);
	help::cache_json_with_uuid(&uuid, x).await?;

	help::result_ok()
}

pub async fn init_db(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: InitAppDBRequest = serde_json::from_slice(&payload)?;
	info!("Init DB action...");

	let txn = Txns::Init {};

	request::send_custom_txn(
		&from_actor,
		"complete_task",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn init_token(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: InitAppTokenRequest = serde_json::from_slice(&payload)?;
	info!("Init token action...");

	let txn = TappstoreTxn::GenAesKey {
    token_id: TokenId::from_hex(&req.token_id)?,
	};

	request::send_tappstore_txn(
		&from_actor,
		"faucet_txn",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}

pub async fn set_allowance(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: SetAllowanceRequest = serde_json::from_slice(&payload)?;
	check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("set allowance action... {:?}", req);

	let txn = TappstoreTxn::SetAllowance {
		address: req.address.parse()?,
		token_id: TokenId::from_hex(&req.target_tapp_id_b64)?,
		amount: Balance::from(u128::from_str(&req.amount)?),
	};

	request::send_tappstore_txn(
		&from_actor,
		"set_allowance",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}