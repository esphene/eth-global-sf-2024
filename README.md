# Essence Vault - ETH San Francisco 2024

## Overview

Essence Vault is a multisig wallet on Essential. Due to the declarative nature of this blockchain, the wallet benefits from simplified contract logic that emphasizes what actions need to be performed rather than how they are executed. This results in increased security, as fewer low-level details are exposed, reducing the likelihood of common vulnerabilities like reentrancy attacks. The declarative design also enables faster development and easier auditing, as the rules for fund approvals, signature requirements, and transaction thresholds are expressed clearly in high-level terms. Additionally, the blockchain's built-in optimizations allow Essence Vault to perform efficiently with lower gas fees, making it both secure and cost-effective for managing funds.

## Functionality

### Core Functionalities

1. Create a multisig wallet (give a set of wallet addresses and `m` the minimum number of members that must approve a transaction before it gets executed)
2. Add a member
3. Remove a member
3. Update `m` (`m-of-n` members is needed to approve where `n` is the total number of members and `m` is the minimal approval needed.)
4. Receive fund
5. Send fund (again, this requires `m-of-n` approvals)
6. Propose transactions - proposals for new transactions are created on-chain and require on-chain voting/approvals.
7. Approve transactions - each member can approve transactions on-chain. Once m approvals are reached, the transaction is executed.
8. View wallet state - query wallet info like balance, members, configurations and pending proposals.

### Future Functionalities

1. Revoke approval - a member can revoke their on-chain approval before the transaction has enough votes to be executed.
2. Reject proposal - members can also explicitly reject a proposal.
3. Set time-locked approvals - on-chain time locks where a transaction has to wait till a certain time before it can be executed.
4. Set transaction limits - on-chain limits for different transaction amounts that might automatically adjust approval requirements or allow automatic execution. E.g. larger amount requires a larger `m`.
5. Fund investment - instead of just letting wallet fund sit there, do low/no-risk investment of the fund. E.g. stake a percentage of the fund. (Can apply statistics and some AI to study behaviour of fund's transactions i.e. In/Out to suggest amount to stake and portfolio strategy.)

## Interesting Use Case 

An interesting use case to further explore in the future is a Hierarchical Multisig Wallets for DAO, Corporation, NPO, VC Funds etc.

Essence Vault can be used in a hierarchical structure, where each member of a multisig wallet is itself another multisig wallet. This allows for a cascading approval process that mirrors the corporate hierarchy, ensuring that decisions flow from the bottom up, with final approvals coming from the top-level treasury.

Top-Level Corporate Treasury Wallet:

At the highest level, the organization’s core treasury multisig wallet could have members like the Finance Department Wallet, Operations Department Wallet, and Executive Committee Wallet, each represented as a separate multisig wallet. To approve a high-value transaction, each department’s wallet must first approve it.
Mid-Level (Departmental) Multisig Wallets:

Each department manages its own multisig wallet, with multiple signatories (e.g., CFO, VP of Finance for the Finance Department) approving departmental expenses. These wallets then represent members in the top-level corporate treasury wallet.
Lower-Level (Team or Regional) Multisig Wallets:

Teams or regions can manage smaller budgets using their own multisig wallets, which then forward larger approval requests to their respective departments for final approval.
Chained Approvals:

Transactions start at the lower levels, cascading upwards for approval at each layer of the hierarchy. This ensures thorough scrutiny of large transactions, while smaller ones are handled efficiently within their respective departments.

Some parts of Pint contract probably can be redesigned to be optimized for this kind use case. Maybe implement additional contracts with additional predicates or constraints (but potentially no storage as it will apply those rules to the same storage of the original parent vault).


## Useful links

1. [Quickstart guide](https://essential-contributions.github.io/essential-integration/index.html)
2. [The Book of Pint](https://essential-contributions.github.io/pint/book/)
3. [The Essential Specs](https://essential-contributions.github.io/specs/specs/index.html?ref=blog.essential.builders)
4. [Sample Applications](https://github.com/essential-contributions/essential-integration/tree/main/apps)

