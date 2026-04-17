// use crate::inputs::Spielfeld;

//     #[test]
//     fn x_gewinnt_eine_reihe() {
//         let board = [
//             Spielfeld::X, Spielfeld::X, Spielfeld::X,
//             Spielfeld::Leer, Spielfeld::Leer, Spielfeld::Leer,
//             Spielfeld::Leer, Spielfeld::Leer, Spielfeld::Leer,
//         ];

//         assert_eq!(crate::check_winner::check_winner(&board), Some(true));
//     }

//     #[test]
//     fn o_gewinnt_eine_spallte() {
//         let board: [Spielfeld; 9] = [
//             Spielfeld::O, Spielfeld::Leer, Spielfeld::Leer,
//             Spielfeld::O, Spielfeld::Leer, Spielfeld::Leer,
//             Spielfeld::O, Spielfeld::Leer, Spielfeld::Leer,
//         ];

//         assert_eq!(crate::check_winner::check_winner(&board), Some(false));
//     }

//     #[test]

//     fn x_gewinnt_eine_diagonale() {
//         let board: [Spielfeld; 9] = [
//             Spielfeld::X, Spielfeld::Leer, Spielfeld::Leer,
//             Spielfeld::Leer, Spielfeld::X, Spielfeld::Leer,
//             Spielfeld::Leer, Spielfeld::Leer, Spielfeld::X,
//         ];

//         assert_eq!(crate::check_winner::check_winner(&board), Some(true));
//     }

//     #[test]

//     fn unentschieden_wird_erkannt() {
//         let board: [Spielfeld; 9] = [
//             Spielfeld::X, Spielfeld::O, Spielfeld::X,
//             Spielfeld::O, Spielfeld::X, Spielfeld::O,
//             Spielfeld::O, Spielfeld::X, Spielfeld::X,
//         ];

//         assert_eq!(crate::inputs::is_draw(&board), true);
//     }

//     #[test]
//     fn ungültige_eingabe_wird_abgelehnt() {
//         assert_eq!(crate::inputs::check_input("g", &["q", ""]), false);
//     }

//     #[test]
//     fn belegtes_feld_wird_abgelehnt() {
//         let board: [Spielfeld; 9] = [
//             Spielfeld::X, Spielfeld::Leer, Spielfeld::Leer,
//             Spielfeld::Leer, Spielfeld::Leer, Spielfeld::Leer,
//             Spielfeld::Leer, Spielfeld::Leer, Spielfeld::Leer,
//         ];

//         assert_eq!(crate::inputs::is_field_free(&board, 4), true);
//     }
