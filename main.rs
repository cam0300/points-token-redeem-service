use std::collections::HashMap;

struct User {
    points: u32,
}

#[derive(Debug, Clone)]
struct Token {
    claim_on_revenues: u32,
}

fn award_points(users: &mut HashMap<String, User>, username: &str, points: u32) {
    let user = users.get_mut(username).unwrap();
    user.points += points;
}

fn exchange_points_for_revenue_token(
    users: &mut HashMap<String, User>,
    tokens: &mut Vec<Token>,
    username: &str,
    exchange_rate: u32,
) -> Result<Token, &'static str> {
    let user = users.get_mut(username).ok_or("User not found")?;
    if user.points >= exchange_rate {
        user.points -= exchange_rate;
        let token = Token { claim_on_revenues: exchange_rate };
        tokens.push(token.clone());
        Ok(token)
    } else {
        Err("Not enough points")
    }
}

fn main() {
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
}
