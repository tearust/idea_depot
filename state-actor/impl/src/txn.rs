use crate::{
    account,
    error::{Result},
    sql::{sql_init, create_idea, vote_idea},
    utils::{check_account, decode_auth_key, my_token_id},
};
use log::info;
use prost::Message;
use idea_vote_state_actor_codec::txn::{Txns};
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid},
    actors::tokenstate::{SqlBeginTransactionRequest, NAME},
    actorx::{ActorId},
    serialize,
    utils::wasm_actor::actors::statemachine::{query_state_tsid, CommitContext, CommitContextList},
    vmh::message::{encode_protobuf, structs_proto::tokenstate},
};

pub(crate) async fn txn_exec(tsid: Tsid, txn: &Txns) -> Result<()> {
    info!("begin of process transaction for idea vote state actor  => {txn}");

    let base: Tsid = query_state_tsid().await?;
    let ctx = serialize(&TokenContext::new_slim(tsid, base, my_token_id().await?))?;
    let commit_list = match txn {
        Txns::Init {} => {
            sql_init(tsid).await?;
            CommitContextList {
                ctx_list: vec![CommitContext::ctx_receipting(ctx, txn.to_string())],
                ..Default::default()
            }
        }        
        Txns::CreateIdea {
            id,
            title,
            description,
            owner,
            auth_b64,
            unit,
        } => {
            check_account(auth_b64, *owner).await?;
            let (tappstore_ctx, ctx) = account::deposit_for_idea(tsid, base, *owner, *unit, ctx).await?;
            let glue_ctx = new_gluedb_context().await?;
            create_idea(
                tsid,
                id.to_string(),
                title.to_string(),
                description.to_string(),
                *owner,
                *unit,
            )
            .await?;
            CommitContextList {
                ctx_list: vec![
                    CommitContext::new(
                        ctx,
                        glue_ctx,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    ),
                    CommitContext::ctx_receipting(tappstore_ctx, txn.to_string()),
                ],
                ..Default::default()
            }
        }
        Txns::VoteIdea { id, user, auth_b64, price } => {
            check_account(auth_b64, *user).await?;
            let (tappstore_ctx, ctx) = account::deposit_for_idea(tsid, base, *user, *price, ctx).await?;

            let glue_ctx = new_gluedb_context().await?;
            vote_idea(tsid, id.to_string(), *user, *price).await?;

            CommitContextList {
                ctx_list: vec![
                    CommitContext::new(
                        ctx,
                        glue_ctx,
                        None,
                        None,
                        decode_auth_key(auth_b64)?,
                        txn.to_string(),
                    ),
                    CommitContext::ctx_receipting(tappstore_ctx, txn.to_string()),
                ],
                ..Default::default()
            }
        }
    };

    commit_list.commit(base, tsid).await?;
    info!("transaction {txn} committed successfully");

    Ok(())
}

async fn new_gluedb_context() -> Result<Option<tokenstate::GluedbTransactionContext>> {
    let buf = ActorId::Static(NAME).call(
        SqlBeginTransactionRequest(encode_protobuf(tokenstate::BeginTransactionRequest {
            token_id: serialize(&my_token_id().await?)?,
        })?),
    )
    .await?;
    let res = tokenstate::BeginTransactionResponse::decode(buf.0.as_slice())?;
    Ok(res.context)
}
