# Essence Vault - ETH San Francisco 2024

## Overview

Essence Vault is a multisig wallet on Essential. Due to the declarative nature of this blockchain, the wallet benefits from simplified contract logic that emphasizes what actions need to be performed rather than how they are executed. This results in increased security, as fewer low-level details are exposed, reducing the likelihood of common vulnerabilities like reentrancy attacks. The declarative design also enables faster development and easier auditing, as the rules for fund approvals, signature requirements, and transaction thresholds are expressed clearly in high-level terms. Additionally, the blockchain's built-in optimizations allow Essence Vault to perform efficiently with lower gas fees, making it both secure and cost-effective for managing funds.

## Functionality

# Core Functionalities

1. Create a multisig wallet (give a set of wallet addresses and `m` the minimum number of members that must approve a transaction before it gets executed)
2. Add a member
3. Remove a member
3. Update `m` (`m-of-n` members is needed to approve where `n` is the total number of members and `m` is the minimal approval needed.)
4. Receive fund
5. Send fund (again, this requires `m-of-n` approvals)
6. Propose transactions - proposals for new transactions are created on-chain and require on-chain voting/approvals.
7. Approve transactions - each member can approve transactions on-chain. Once m approvals are reached, the transaction is executed.
8. View wallet state - query wallet info like balance, members, configurations and pending proposals.

# Future Functionalities

1. Revoke approval - a member can revoke their on-chain approval before the transaction has enough votes to be executed.
2. Reject proposal - members can also explicitly reject a proposal.
3. Set time-locked approvals - on-chain time locks where a transaction has to wait till a certain time before it can be executed.
4. Set transaction limits - on-chain limits for different transaction amounts that might automatically adjust approval requirements or allow automatic execution. E.g. larger amount requires a larger `m`.
5. Fund investment - instead of just letting wallet fund sit there, do low/no-risk investment of the fund. E.g. stake a percentage of the fund. (Can apply statistics and some AI to study behaviour of fund's transactions i.e. In/Out to suggest amount to stake and portfolio strategy.)

## Useful links

1. [Quickstart guide](https://essential-contributions.github.io/essential-integration/index.html)
2. [The Book of Pint](https://essential-contributions.github.io/pint/book/)
3. [The Essential Specs](https://essential-contributions.github.io/specs/specs/index.html?ref=blog.essential.builders)
4. [Sample Applications](https://github.com/essential-contributions/essential-integration/tree/main/apps)

