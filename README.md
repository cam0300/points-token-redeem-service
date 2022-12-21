# points-token-redeem-service

Points and Revenue Tokens
This Rust program allows users to earn points and exchange them for tokens that have a claim on future revenues. The program includes the following functions:


award_points: 
  Awards points to a user.
exchange_points_for_revenue_token: 
  Exchanges points for a revenue token. The user must have enough points to exchange, and the exchange rate is specified as an argument.
issue_revenue_tokens: 
  Automatically issues revenue tokens to users who have earned enough points.
  
  
Structs
The program defines the following structs:
User: Represents a user with a number of points.
Token: Represents a token with a claim on future revenues.


Example:

use std::collections::HashMap;

let mut users = HashMap::new();
users.insert(
    "Alice".to_string(),
    User { points: 100 },
);
users.insert(
    "Bob".to_string(),
    User { points: 50 },
);

let mut tokens = Vec::new();

award_points(&mut users, "Alice", 50);
award_points(&mut users, "Bob", 25);

let alice_token = exchange_points_for_revenue_token(
    &mut users,
    &mut tokens,
    "Alice",
    150,
);
let bob_token = exchange_points_for_revenue_token(
    &mut users,
    &mut tokens,
    "Bob",
    75,
);

println!("Alice has a token: {:?}", alice_token);
println!("Bob has a token: {:?}", bob_token);

issue_revenue_tokens(&mut users, &mut tokens, 100);


This code will award points to Alice and Bob, then attempt to exchange points for revenue tokens. Alice will not have enough points to exchange, so her exchange will fail. Bob will have enough points, so his exchange will succeed and he will receive a revenue token. Finally, the issue_revenue_tokens function will automatically issue revenue tokens to any users who have earned enough points.
