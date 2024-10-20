use essential_app_utils::{compile::compile_pint_project, local_server::setup_server};
use essential_types::contract::Contract;
use essential_types::PredicateAddress;
use multisig_wallet_app::{Addresses, App};

// Private key for the member
const MEMBER_PRIV_KEY: &str = "..REPLACE-WITH-PRIVATE-KEY..";

#[tokio::test]
async fn test_multisig_wallet() {
    let (server_address, _server) = setup_server().await.unwrap();

    let mut wallet = essential_wallet::Wallet::temp().unwrap();

    // Setup an account for the deployer
    let deployer = "deployer".to_string();
    wallet
        .new_key_pair(&deployer, essential_wallet::Scheme::Secp256k1)
        .ok();

    // Setup accounts for the multisig members
    let alice = "alice";
    let bob = "bob";
    let charlie = "charlie";
    
    // Set up alice using the private key `MEMBER_PRIV_KEY`
    wallet
        .insert_key(
            alice,
            essential_signer::Key::Secp256k1(
                essential_signer::secp256k1::SecretKey::from_slice(
                    &hex::decode(MEMBER_PRIV_KEY).unwrap(),
                )
                .unwrap(),
            ),
        )
        .unwrap();

    // Set up bob and charlie as members
    wallet
        .new_key_pair(bob, essential_wallet::Scheme::Secp256k1)
        .ok();
    wallet
        .new_key_pair(charlie, essential_wallet::Scheme::Secp256k1)
        .ok();

    // Compile the Pint contract and return the contract object
    let multisig_contract: Contract =
        compile_pint_project(concat!(env!("CARGO_MANIFEST_DIR"), "/../contract").into())
            .await
            .unwrap();

    // These are the addresses we need for the contract's predicates
    use essential_hash::{content_addr, contract_addr};
    let multisig_address = contract_addr::from_contract(&multisig_contract);
    let addresses = Addresses {
        multisig_wallet: multisig_address.clone(),
        propose_transaction: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[0]),
        },
        approve_transaction: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[1]),
        },
        propose_add_member: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[2]),
        },
        approve_add_member: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[3]),
        },
        propose_remove_member: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[4]),
        },
        approve_remove_member: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[5]),
        },
        propose_update_min_approvals: PredicateAddress {
            contract: multisig_address.clone(),
            predicate: content_addr(&multisig_contract.predicates[6]),
        },
        approve_update_min_approvals: PredicateAddress {
            contract: multisig_address,
            predicate: content_addr(&multisig_contract.predicates[7]),
        },
    };

    // Sign and deploy the multisig wallet contract at `server_address`. The deployer is `deployer`.
    essential_deploy_contract::sign_and_deploy(
        server_address.clone(),
        &deployer,
        &mut wallet,
        multisig_contract,
    )
    .await
    .unwrap();

    // This is a new instance of the multisig wallet app
    let mut multisig_wallet = App::new(server_address.clone(), addresses, wallet).unwrap();

    // Step 1: Propose a transaction to send funds from the wallet to a recipient
    let recipient = "recipient";
    multisig_wallet
        .wallet
        .new_key_pair(recipient, essential_wallet::Scheme::Secp256k1)
        .ok();
    let transaction_amount = 500;
    
    // Propose a transaction from alice (who is a member)
    multisig_wallet
        .propose_transaction(alice, recipient, transaction_amount)
        .await
        .unwrap();

    // Step 2: Approve the transaction by the required number of members (m-of-n)
    let proposal_id = 1;
    multisig_wallet
        .approve_transaction(bob, proposal_id)
        .await
        .unwrap();
    
    // Wait for the solution to be processed
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Step 3: Check that the recipient received the funds
    let recipient_balance = multisig_wallet.balance(recipient).await.unwrap();
    assert_eq!(recipient_balance, transaction_amount);

    // Step 4: Propose and approve adding a new member (charlie)
    multisig_wallet
        .propose_add_member(alice, charlie)
        .await
        .unwrap();
    multisig_wallet
        .approve_add_member(bob, proposal_id + 1) // Assuming next proposal ID is 2
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Step 5: Propose and approve removing a member (alice)
    multisig_wallet
        .propose_remove_member(bob, alice)
        .await
        .unwrap();
    multisig_wallet
        .approve_remove_member(charlie, proposal_id + 2) // Assuming next proposal ID is 3
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Final Step: Check balances to ensure consistency
    let recipient_balance = multisig_wallet.balance(recipient).await.unwrap();
    assert_eq!(recipient_balance, transaction_amount);
}
