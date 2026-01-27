use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)] // This is required for most Soroban errors
pub enum Error {
    InvalidInput = 1,
    AlreadyExists = 2,
    NotFound = 3,
    InvalidTotalPercentage = 4,
    EmptyMembers = 5,
    DuplicateMember = 6,
    NotAuthorized = 7,
    InsufficientBalance = 8,
    InvalidAmount = 9,
}
