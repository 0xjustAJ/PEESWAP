use soroban_sdk::{contract, contractimpl, log, token, Address, Env, IntoVal};

 /* adding another functionality to log the swap details */
 log!(&env, "swap", (a, b, token_a, token_b, amount_a, min_b_for_a, amount_b, min_a_for_b));    

/* adding another functionality to log the transfer details */
 log!(&env, "transfer", (from, to, max_spend_amount, transfer_amount));


 