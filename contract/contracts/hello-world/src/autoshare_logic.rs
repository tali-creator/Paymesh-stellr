use crate::base::errors::Error;
use crate::base::events::{emit_autoshare_created, emit_autoshare_updated};
use crate::base::types::{AutoShareDetails, GroupMember};
use soroban_sdk::{contracttype, Address, BytesN, Env, String, Vec};

#[contracttype]
pub enum DataKey {
    AutoShare(BytesN<32>),
    AllGroups,
}

// Helper to validate members
fn validate_members(members: &Vec<GroupMember>) -> Result<(), Error> {
    if members.is_empty() {
        return Err(Error::EmptyMembers);
    }

    let mut total_percentage: u32 = 0;
    for member in members.iter() {
        total_percentage += member.percentage;
    }

    if total_percentage != 100 {
        return Err(Error::InvalidTotalPercentage);
    }

    // Check for duplicates
    // O(N^2) is acceptable here as member lists are expected to be small
    for (i, member1) in members.iter().enumerate() {
        for (j, member2) in members.iter().enumerate() {
            if i != j && member1.address == member2.address {
                return Err(Error::DuplicateMember);
            }
        }
    }

    Ok(())
}

pub fn create_autoshare(
    env: Env,
    id: BytesN<32>,
    name: String,
    creator: Address,
    members: Vec<GroupMember>,
) -> Result<(), Error> {
    let key = DataKey::AutoShare(id.clone());

    // Check if it already exists to prevent overwriting
    if env.storage().persistent().has(&key) {
        return Err(Error::AlreadyExists);
    }

    // Validate members
    validate_members(&members)?;

    let details = AutoShareDetails {
        id: id.clone(),
        name,
        creator: creator.clone(),
        members,
    };

    // Store the details in persistent storage
    env.storage().persistent().set(&key, &details);

    // Add to all groups list
    let all_groups_key = DataKey::AllGroups;
    let mut all_groups: Vec<BytesN<32>> = env
        .storage()
        .persistent()
        .get(&all_groups_key)
        .unwrap_or(Vec::new(&env));
    all_groups.push_back(id.clone());
    env.storage().persistent().set(&all_groups_key, &all_groups);

    // Emit event
    emit_autoshare_created(&env, id, creator);
    Ok(())
}

pub fn update_members(
    env: Env,
    id: BytesN<32>,
    caller: Address,
    new_members: Vec<GroupMember>,
) -> Result<(), Error> {
    let key = DataKey::AutoShare(id.clone());
    let mut details: AutoShareDetails = env
        .storage()
        .persistent()
        .get(&key)
        .ok_or(Error::NotFound)?;

    // Authorization check
    if caller != details.creator {
        return Err(Error::NotAuthorized);
    }

    // Validate new members
    validate_members(&new_members)?;

    // Update members
    details.members = new_members;
    env.storage().persistent().set(&key, &details);

    // Emit event
    emit_autoshare_updated(&env, id, caller);
    Ok(())
}

pub fn get_autoshare(env: Env, id: BytesN<32>) -> Result<AutoShareDetails, Error> {
    let key = DataKey::AutoShare(id);
    env.storage().persistent().get(&key).ok_or(Error::NotFound)
}

pub fn get_all_groups(env: Env) -> Vec<AutoShareDetails> {
    let all_groups_key = DataKey::AllGroups;
    let group_ids: Vec<BytesN<32>> = env
        .storage()
        .persistent()
        .get(&all_groups_key)
        .unwrap_or(Vec::new(&env));

    let mut result: Vec<AutoShareDetails> = Vec::new(&env);
    for id in group_ids.iter() {
        if let Ok(details) = get_autoshare(env.clone(), id) {
            result.push_back(details);
        }
    }
    result
}

pub fn get_groups_by_creator(env: Env, creator: Address) -> Vec<AutoShareDetails> {
    let all_groups = get_all_groups(env.clone());
    let mut result: Vec<AutoShareDetails> = Vec::new(&env);

    for group in all_groups.iter() {
        if group.creator == creator {
            result.push_back(group);
        }
    }
    result
}

pub fn is_group_member(env: Env, id: BytesN<32>, address: Address) -> Result<bool, Error> {
    let details = get_autoshare(env, id)?;

    for member in details.members.iter() {
        if member.address == address {
            return Ok(true);
        }
    }
    Ok(false)
}
