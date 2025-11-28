use super::{structs::*, *};

fn predictable_game() -> Game {
    Game {
        deck: Deck::init(),
        ..Game::default()
    }
}

#[test]
fn primitive_game_play() {
    let mut game = predictable_game();
    game.init();
    eprintln!("{game}");
    assert_eq!(State::DealerJack, game.current_state());
    game.player.make_bet(500).unwrap();
    game.player.pay_out(game.current_state());
    assert_eq!(500, game.player.wealth);
    assert_eq!(0, game.player.get_bet());
    game.play_again();
    assert_eq!(State::WaitingBet, game.current_state());
    assert_eq!(None, game.player.make_bet(1000));
    game.player.make_bet(500).unwrap();
    assert_eq!(0, game.player.wealth);
    game.init();
    eprintln!("{game}");
    assert_eq!(State::Ongoing, game.current_state());
    game.update_state(MoveAction::Stand);
    assert_eq!(State::DealerWin, game.current_state());
    assert_eq!(0, game.player.get_bet());
    assert_eq!(0, game.player.wealth);
    eprintln!("{game}");
}
