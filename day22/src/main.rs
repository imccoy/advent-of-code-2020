use std::io::{self, BufRead};
use std::collections::HashMap;

fn play_round(deck1: &mut Vec<usize>, deck2: &mut Vec<usize>) {
    let mut player1_card = deck1.remove(0);
    let mut player2_card = deck2.remove(0);
    if (player1_card > player2_card) {
        deck1.push(player1_card);
        deck1.push(player2_card);
    } else {
        deck2.push(player2_card);
        deck2.push(player1_card);
    }
}


fn score(deck: Vec<usize>) -> u64 {
    return deck.iter().rev().enumerate().map(|(n, card)| ((n+1) * card) as u64).sum();
}

fn main() {
    let mut current_player_number = 0;
    let mut players_cards : HashMap<usize, Vec<usize>> = HashMap::new();
    for wrapped_line in io::stdin().lock().lines() {
        let line = wrapped_line.unwrap();
        if line.starts_with("Player ") {
            current_player_number += 1;
        } else if line.len() != 0 {
            let card = line.parse::<usize>().unwrap();
            players_cards.entry(current_player_number)
                .or_insert(Vec::new())
                .push(card);
        }
    }
    dbg!(&players_cards);
    let deck1 = players_cards.get(&1).unwrap().clone();
    let deck2 = players_cards.get(&2).unwrap().clone();
    play_combat(deck1.clone(), deck2.clone());
    let mut games = 0;
    play_recursive_combat(deck1.clone(), deck2.clone(), &mut games);
}

fn play_combat(mut deck1 : Vec<usize>, mut deck2 : Vec<usize>) {
    let mut round = 0;
    while deck1.len() != 0 && deck2.len() != 0 {
        round += 1;
        play_round(&mut deck1, &mut deck2);
    }
    dbg!(round, score(deck1), score(deck2));
}

#[derive(Debug)]
enum RoundResult { Player1Round, Player2Round }

fn play_recursive_combat(mut deck1 : Vec<usize>, mut deck2 : Vec<usize>, num_games: &mut usize) -> RoundResult {
    let mut already_seen : HashMap<(Vec<usize>, Vec<usize>), ()> = HashMap::new();
    let mut round = 0;
    *num_games += 1;
    let game_number = *num_games;
    while deck1.len() != 0 && deck2.len() != 0 {
        if already_seen.insert((deck1.clone(), deck2.clone()), ()).is_some() {
            return RoundResult::Player1Round;
        }
        let mut player1_card = deck1.remove(0);
        let mut player2_card = deck2.remove(0);
        let winner = if player1_card <= deck1.len() && player2_card <= deck2.len() {
            play_recursive_combat(deck1[0..player1_card].to_vec(), deck2[0..player2_card].to_vec(), num_games)
        } else {
            if player1_card > player2_card { RoundResult::Player1Round } else { RoundResult::Player2Round }
        };
        round += 1;
        match winner {
            RoundResult::Player1Round => {
                deck1.push(player1_card);
                deck1.push(player2_card);
            },
            RoundResult::Player2Round => {
                deck2.push(player2_card);
                deck2.push(player1_card);
            },
        }
    }
    
    if game_number == 1 {
        dbg!(score(deck1.clone()), score(deck2.clone()));
    }
    if deck1.len() != 0 {
        RoundResult::Player1Round
    } else {
        RoundResult::Player2Round
    }
}
