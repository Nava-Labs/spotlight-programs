# Solana Escrow Program

## Overview

This Solana program facilitates token escrow between projects and influencers. It ensures secure payments where SOL is locked in the program until the conditions of the request are met. The program includes two main functionalities: `request` and `claim`.

## Core Functions

### 1. Request
The `request` function allows projects to engage with influencers by:
- Paying the required amount of SOL upfront, which will be locked in the escrow until the conditions are met.

### 2. Claim
The `claim` function serves two purposes:
- If the influencer fulfills the task to the project's satisfaction, the influencer can claim the locked SOL.
- If the task is not fulfilled or the project is dissatisfied, the project can reclaim their SOL.

## Workflow

1. **Project Request**: 
   - A project submits a request and locks SOL in the escrow via the `request` function.
   
2. **Task Completion**:
   - The influencer completes the task according to the project's requirements.
   
3. **Approval or Rejection**:
   - If the project approves the influencer's work, the influencer claims the locked SOL.
   - If the project is not satisfied with the task, they can reclaim the SOL.
