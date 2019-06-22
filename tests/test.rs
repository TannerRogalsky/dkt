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
}