mod tictactoe {
    use dkt::*;

    #[test]
    fn new() {
        let t = TicTacToe::new();
        println!("{:?}", t);
    }

    #[test]
    fn get() {
        let t = TicTacToe::new();
        assert!(t.get(0, 0).is_none());
    }

    #[test]
    fn set() {
        let element = TTTElements::X;
        let row_index = 0;
        let col_index = 0;
        let mut t = TicTacToe::new();
        t.set(row_index, col_index, element);
        assert_eq!(t.get(row_index, col_index), Some(element));
        assert_eq!(
            t.set(row_index, col_index, element),
            Err("Already set".to_string())
        );
    }

    #[test]
    fn is_game_over() {
        let mut t = TicTacToe::new();
        assert_eq!(t.is_game_over(), false);
        for y in 0..3 {
            for x in 0..3 {
                t.set(x, y, TTTElements::X);
            }
        }
        assert_eq!(t.is_game_over(), true);
    }
}
