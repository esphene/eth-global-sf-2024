use anyhow::bail;
use essential_rest_client::EssentialClient;
use essential_types::{
    convert::word_4_from_u8_32,
    solution::{Mutation, Solution, SolutionData},
    ContentAddress, PredicateAddress, Word,
};
use essential_wallet::Wallet;

pint_abi::gen_from_file! {
    abi: "../contract/out/debug/multisig-wallet-abi.json",
    contract: "../contract/out/debug/multisig-wallet.json",
}

#[derive(Debug, Clone)]
pub struct Addresses {
    pub multisig_wallet: ContentAddress,
    pub propose_transaction: PredicateAddress,
    pub approve_transaction: PredicateAddress,
    pub propose_add_member: PredicateAddress,
    pub approve_add_member: PredicateAddress,
    pub propose_remove_member: PredicateAddress,
    pub approve_remove_member: PredicateAddress,
    pub propose_update_min_approvals: PredicateAddress,
    pub approve_update_min_approvals: PredicateAddress,
}

pub struct App {
    client: EssentialClient,
    wallet: Wallet,
    addresses: Addresses,
}

/// Core Methods
impl App {
    /// Creates a new instance of the multisig wallet app
    pub fn new(
        server_address: String,
        addresses: Addresses,
        wallet: Wallet,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            client: EssentialClient::new(server_address)?,
            addresses,
            wallet,
        })
    }

    /// Propose a transaction to send `amount` to `recipient`.
    pub async fn propose_transaction(
        &mut self,
        proposer_name: &str,
        recipient_name: &str,
        amount: Word,
    ) -> anyhow::Result<()> {
        let proposer = self.get_hashed_key(proposer_name)?;
        let recipient = self.get_hashed_key(recipient_name)?;

        // Craft and submit the proposal
        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.propose_transaction.clone(),
                decision_variables: ProposeTransaction::Vars {
                    proposer,
                    recipient,
                    amount,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Approve a proposed transaction
    pub async fn approve_transaction(
        &mut self,
        approver_name: &str,
        proposal_id: Word,
    ) -> anyhow::Result<()> {
        let approver = self.get_hashed_key(approver_name)?;

        // Craft and submit the approval
        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.approve_transaction.clone(),
                decision_variables: ApproveTransaction::Vars {
                    approver,
                    proposal_id,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Propose adding a new member
    pub async fn propose_add_member(
        &mut self,
        proposer_name: &str,
        new_member_name: &str,
    ) -> anyhow::Result<()> {
        let proposer = self.get_hashed_key(proposer_name)?;
        let new_member = self.get_hashed_key(new_member_name)?;

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.propose_add_member.clone(),
                decision_variables: ProposeAddMember::Vars {
                    proposer,
                    new_member,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Approve adding a new member
    pub async fn approve_add_member(
        &mut self,
        approver_name: &str,
        proposal_id: Word,
    ) -> anyhow::Result<()> {
        let approver = self.get_hashed_key(approver_name)?;

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.approve_add_member.clone(),
                decision_variables: ApproveAddMember::Vars {
                    approver,
                    proposal_id,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Propose removing a member
    pub async fn propose_remove_member(
        &mut self,
        proposer_name: &str,
        member_to_remove_name: &str,
    ) -> anyhow::Result<()> {
        let proposer = self.get_hashed_key(proposer_name)?;
        let member_to_remove = self.get_hashed_key(member_to_remove_name)?;

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.propose_remove_member.clone(),
                decision_variables: ProposeRemoveMember::Vars {
                    proposer,
                    member_to_remove,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Approve removing a member
    pub async fn approve_remove_member(
        &mut self,
        approver_name: &str,
        proposal_id: Word,
    ) -> anyhow::Result<()> {
        let approver = self.get_hashed_key(approver_name)?;

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.approve_remove_member.clone(),
                decision_variables: ApproveRemoveMember::Vars {
                    approver,
                    proposal_id,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Propose updating the minimum approvals
    pub async fn propose_update_min_approvals(
        &mut self,
        proposer_name: &str,
        new_min_approvals: Word,
    ) -> anyhow::Result<()> {
        let proposer = self.get_hashed_key(proposer_name)?;

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.propose_update_min_approvals.clone(),
                decision_variables: ProposeUpdateMinApprovals::Vars {
                    proposer,
                    new_min_approvals,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }

    /// Approve updating the minimum approvals
    pub async fn approve_update_min_approvals(
        &mut self,
        approver_name: &str,
        proposal_id: Word,
    ) -> anyhow::Result<()> {
        let approver = self.get_hashed_key(approver_name)?;

        let solution = Solution {
            data: vec![SolutionData {
                predicate_to_solve: self.addresses.approve_update_min_approvals.clone(),
                decision_variables: ApproveUpdateMinApprovals::Vars {
                    approver,
                    proposal_id,
                }
                .into(),
                transient_data: Default::default(),
                state_mutations: Default::default(),
            }],
        };

        self.client.submit_solution(solution).await?;
        Ok(())
    }
}

/// Utility Methods
impl App {
    /// Given an account name, produce the hash of its public key
    fn get_hashed_key(&mut self, account_name: &str) -> anyhow::Result<[Word; 4]> {
        let public_key = self.wallet.get_public_key(account_name)?;
        let essential_signer::PublicKey::Secp256k1(public_key) = public_key else {
            anyhow::bail!("Invalid public key")
        };
        let encoded = essential_sign::encode::public_key(&public_key);
        Ok(word_4_from_u8_32(essential_hash::hash_words(&encoded)))
    }
}
