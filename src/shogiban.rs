extern crate sdl2;
extern crate shakmaty;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::messagebox::ClickedButton;
use sdl2::messagebox::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::{thread, time};

// use shakmaty::{Board, Chess, File, Move, Position, Rank, Role, Setup, Square};
// use shogai::ai::*;
use shogai::board::*;
use shogai::movement::*;
use shogai::piece::*;
use shogai::position::*;

use std::collections::HashSet;
use std::path::Path;

const SRC_RESERVE_HEIGTH: u32 = 100;
const SCR_WIDTH: u32 = 603;
const SCR_HEIGHT: u32 = 603 + 2 * SRC_RESERVE_HEIGTH;

const SQR_SIZE: u32 = SCR_WIDTH / 9;

pub fn init() -> Result<(), String> {
    // sdl things
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = match video
        .window("Shogi", SCR_WIDTH, SCR_HEIGHT)
        .position_centered()
        .opengl()
        .build()
    {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err),
    };

    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;

    let mut events = context.event_pump()?;

    canvas.set_draw_color(Color::RGB(0xD1, 0x8B, 0x47));
    canvas.clear();

    let texture_creator = canvas.texture_creator();

    // define standard board
    let mut game = Board::new();

    // completely transparent texture
    let nothing = texture_creator.load_texture(Path::new("src/sprites/nothing.png"))?;

    // load white pieces' src/sprites.
    // credits for src/sprites: Wikimedia Commons
    let w_k = texture_creator.load_texture(Path::new("src/sprites/white/k.png"))?;
    let w_r = texture_creator.load_texture(Path::new("src/sprites/white/r.png"))?;
    let w_b = texture_creator.load_texture(Path::new("src/sprites/white/b.png"))?;
    let w_p = texture_creator.load_texture(Path::new("src/sprites/white/p.png"))?;
    let w_n = texture_creator.load_texture(Path::new("src/sprites/white/n.png"))?;
    let w_l = texture_creator.load_texture(Path::new("src/sprites/white/l.png"))?;
    let w_g = texture_creator.load_texture(Path::new("src/sprites/white/g.png"))?;
    let w_s = texture_creator.load_texture(Path::new("src/sprites/white/s.png"))?;

    let w_bp = texture_creator.load_texture(Path::new("src/sprites/white/bp.png"))?;
    let w_rp = texture_creator.load_texture(Path::new("src/sprites/white/rp.png"))?;
    let w_pp = texture_creator.load_texture(Path::new("src/sprites/white/pp.png"))?;
    let w_lp = texture_creator.load_texture(Path::new("src/sprites/white/lp.png"))?;
    let w_np = texture_creator.load_texture(Path::new("src/sprites/white/np.png"))?;
    let w_sp = texture_creator.load_texture(Path::new("src/sprites/white/sp.png"))?;

    // load black pieces' src/sprites.
    let b_k = texture_creator.load_texture(Path::new("src/sprites/black/k.png"))?;
    let b_r = texture_creator.load_texture(Path::new("src/sprites/black/r.png"))?;
    let b_b = texture_creator.load_texture(Path::new("src/sprites/black/b.png"))?;
    let b_p = texture_creator.load_texture(Path::new("src/sprites/black/p.png"))?;
    let b_n = texture_creator.load_texture(Path::new("src/sprites/black/n.png"))?;
    let b_l = texture_creator.load_texture(Path::new("src/sprites/black/l.png"))?;
    let b_g = texture_creator.load_texture(Path::new("src/sprites/black/g.png"))?;
    let b_s = texture_creator.load_texture(Path::new("src/sprites/black/s.png"))?;

    let b_bp = texture_creator.load_texture(Path::new("src/sprites/black/bp.png"))?;
    let b_rp = texture_creator.load_texture(Path::new("src/sprites/black/rp.png"))?;
    let b_pp = texture_creator.load_texture(Path::new("src/sprites/black/pp.png"))?;
    let b_lp = texture_creator.load_texture(Path::new("src/sprites/black/lp.png"))?;
    let b_np = texture_creator.load_texture(Path::new("src/sprites/black/np.png"))?;
    let b_sp = texture_creator.load_texture(Path::new("src/sprites/black/sp.png"))?;

    let piece_to_texture = |piece: &Piece| {
        if piece.promoted {
            match piece.color {
                shogai::piece::Color::White => match piece.piecetype {
                    PieceType::Pawn => &w_pp,
                    PieceType::Bishop => &w_bp,
                    PieceType::Rook => &w_rp,
                    PieceType::Knight => &w_np,
                    PieceType::King => &w_k,
                    PieceType::Gold => &w_g,
                    PieceType::Lance => &w_lp,
                    PieceType::Silver => &w_sp,
                },
                shogai::piece::Color::Black => match piece.piecetype {
                    PieceType::Pawn => &b_pp,
                    PieceType::Bishop => &b_bp,
                    PieceType::Rook => &b_rp,
                    PieceType::Knight => &b_np,
                    PieceType::King => &b_k,
                    PieceType::Gold => &b_g,
                    PieceType::Lance => &b_lp,
                    PieceType::Silver => &b_sp,
                },
            }
        } else {
            match piece.color {
                shogai::piece::Color::White => match piece.piecetype {
                    PieceType::Pawn => &w_p,
                    PieceType::Bishop => &w_b,
                    PieceType::Rook => &w_r,
                    PieceType::Knight => &w_n,
                    PieceType::King => &w_k,
                    PieceType::Gold => &w_g,
                    PieceType::Lance => &w_l,
                    PieceType::Silver => &w_s,
                },
                shogai::piece::Color::Black => match piece.piecetype {
                    PieceType::Pawn => &b_p,
                    PieceType::Bishop => &b_b,
                    PieceType::Rook => &b_r,
                    PieceType::Knight => &b_n,
                    PieceType::King => &b_k,
                    PieceType::Gold => &b_g,
                    PieceType::Lance => &b_l,
                    PieceType::Silver => &b_s,
                },
            }
        }
    };

    // This will parse and draw all pieces currently on the game to the window.
    let draw_pieces = |canvas: &mut Canvas<Window>, game: &Board, hidden: Option<Piece>| {
        for piece in game.iter().filter(|&p| Some(*p) != hidden) {
            //TODO filter "only once" to remove only on exemplary of pieces in reserve
            if let Some(i) = piece.position {
                draw_piece(canvas, piece_to_texture(piece), i);
            } else {
                draw_piece_on_reserve(canvas, piece_to_texture(piece), piece, 0);
                //TODO manage drawing multiple identical pieces
            }
        }
    };

    let get_mouse_position: fn(sdl2::mouse::MouseState) -> Option<Position> = |mouse_state| {
        if mouse_state.y() >= SRC_RESERVE_HEIGTH as i32
            && mouse_state.y() <= SCR_HEIGHT as i32 - SRC_RESERVE_HEIGTH as i32
        {
            return Some(Position(
                (9 - (mouse_state.x() / SQR_SIZE as i32) as u16)
                    + ((mouse_state.y() - SRC_RESERVE_HEIGTH as i32) / SQR_SIZE as i32) as u16 * 9
                    - 1,
            ));
        } else {
            //manage get from reserve
            return None;
        }
    };

    // We need to set this before the render loop to avoid undefined behaviour,
    // so we just set an arbritary texture to this by now.
    let mut curr_texture: &Texture = &nothing;
    let mut hidden = None;

    // arbitrary to avoid undefined behaviour
    let mut prev_click_pos: Option<Position> = None;

    let mut prev_role_click: Option<PieceType> = None;
    let mut curr_role_click: Option<PieceType> = None;

    let mut curr_click_pos: Option<Position> = None;
    let mut prev_mouse_buttons = HashSet::new();

    //main loop start ####################################
    //####################################################
    //###################################################

    'main_loop: loop {
        for event in events.poll_iter() {
            // if esc is pressed, exit main loop
            // (consequently ending the program)
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => false,
            };
        }

        if game.game_over() {
            let who;
            if game.get_turn() {
                who = "second player";
            } else {
                who = "first player";
            }
            let message = [who, &"has won the game!"].join(" ");
            println!("{}", message);
            return show_simple_message_box(
                MessageBoxFlag::empty(),
                &"Game Over",
                &message,
                canvas.window(),
            )
            .map_err(|e| e.to_string());
        }

        let mouse_state = events.mouse_state();
        let curr_mouse_buttons: HashSet<_> = mouse_state.pressed_mouse_buttons().collect();
        canvas.set_draw_color(Color::RGB(0xD1, 0x8B, 0x47));
        canvas.clear();

        draw_shogiban(&mut canvas);
        let mut human_play = || {
            let get_texture = |game: &Board| {
                if let Some(pos) = get_mouse_position(mouse_state) {
                    match game.is_occupied_by(pos) {
                        Some(piece) => piece_to_texture(&piece),
                        None => &nothing,
                    }
                } else {
                    &nothing
                }
            };
            //select in green movable pieces on the board
            if let Some(pos) = prev_click_pos {
                if let Some(selected_piece) = game.is_occupied_by(pos) {
                    if selected_piece.color == game.get_color() {
                        draw_select(pos, &mut canvas);
                    }
                }
            }
            let is_mouse_released = &prev_mouse_buttons - &curr_mouse_buttons;
            prev_mouse_buttons = curr_mouse_buttons.clone();
            prev_role_click = curr_role_click;
            prev_click_pos = curr_click_pos;

            if !is_mouse_released.is_empty() {
                curr_texture = get_texture(&game);
            }

            if !is_mouse_released.is_empty() {
                if let Some(pos) = get_mouse_position(mouse_state) {
                    curr_role_click = match game.is_occupied_by(pos) {
                        None => None,
                        Some(piece) => Some(piece.piecetype),
                    };
                    if let Some(piece) = game.is_occupied_by(pos) {
                        if piece.color == game.get_color() {
                            hidden = game.is_occupied_by(pos);
                        } else {
                            curr_role_click = None;
                        }
                    } else {
                        hidden = None;
                    }
                } else if let Some(piecetype) = get_in_reserve(mouse_state) {
                    //drag n drop from reserve (drop move)
                    if game.iter().any(|p| {
                        p.color == game.get_color()
                            && p.piecetype == piecetype
                            && p.position == None
                    }) {
                        curr_role_click = Some(piecetype);
                        curr_click_pos = None;
                        hidden = None;
                    } else {
                        curr_role_click = None;
                        hidden = None;
                    }
                } else {
                    curr_role_click = None;
                    hidden = None;
                }
                curr_click_pos = get_mouse_position(mouse_state);

                println!("currtype : {:?}", curr_role_click);
                println!("currpos {:?}", curr_click_pos);
                println!("prevtype {:?}", prev_role_click);
                println!("prevpos {:?}", prev_click_pos);

                if let Some(piecetype) = prev_role_click {
                    if let Some(end) = curr_click_pos {
                        let full_mv = Movement {
                            piecetype: piecetype,
                            start: prev_click_pos,
                            end: end,
                            promotion: false,
                            force_capture: false,
                            offer_draw: false,
                            withdraw: false,
                            restart: false,
                        };
                        let full_mv_with_promotion = Movement {
                            promotion: true,
                            ..full_mv
                        };
                        println!("{}{}", full_mv, full_mv_with_promotion);
                        let res1 = game.check_move(&full_mv.to_string()).is_ok();
                        let mut res2 = game.check_move(&full_mv_with_promotion.to_string()).is_ok()
                            && (full_mv_with_promotion.to_string() != full_mv.to_string());
                        if let Some(pos) = prev_click_pos {
                            if let Some(piece) = game.is_occupied_by(pos) {
                                if piece.promoted {
                                    //no need to buzz the player if the piece is already promoted
                                    res2 = false;
                                }
                            }
                        }
                        //^ necessary as to_string 'ing drops with promotion delete the
                        //(impossible) promotion

                        let chosen_move;
                        if res1 && !res2 {
                            chosen_move = full_mv;
                        } else if res2 && !res1 {
                            chosen_move = full_mv_with_promotion;
                        } else if res1 && res2 {
                            println!("{}{}", res1, res2);
                            //ask wether to promote
                            let buttons: Vec<_> = vec![
                                ButtonData {
                                    flags: MessageBoxButtonFlag::RETURNKEY_DEFAULT,
                                    button_id: 1,
                                    text: "Promote",
                                },
                                ButtonData {
                                    flags: MessageBoxButtonFlag::NOTHING,
                                    button_id: 2,
                                    text: "Do not promote",
                                },
                            ];
                            let res: ClickedButton = show_message_box(
                                MessageBoxFlag::empty(),
                                buttons.as_slice(),
                                "",
                                "Do you want to promote the piece ?",
                                canvas.window(),
                                None,
                            )
                            .unwrap();
                            chosen_move = match res {
                                ClickedButton::CloseButton => full_mv_with_promotion,
                                ClickedButton::CustomButton(buttondata) => {
                                    match buttondata.button_id {
                                        1 => full_mv_with_promotion,
                                        _ => full_mv,
                                    }
                                }
                            };
                        } else {
                            chosen_move = full_mv; //to satisfy checker, but is caught later anyway
                        }

                        curr_role_click = None;
                        curr_click_pos = None;
                        let mv = chosen_move.to_string();
                        if game.check_move(&mv).is_ok() {
                            game = game.play_move_unchecked(&mv);
                            curr_texture = &nothing;
                            hidden = None;
                        } else {
                            hidden = None;
                        }
                    }
                }
            }

            //drag effect
            if let Some(_) = curr_role_click {
                if let Some(pos) = curr_click_pos {
                    //manage drag pieces from reserve
                    if let Some(piece) = game.is_occupied_by(pos) {
                        if piece.color == game.get_color() {
                            let _ = canvas.copy(
                                curr_texture,
                                None,
                                Rect::new(
                                    mouse_state.x() as i32 - SQR_SIZE as i32 / 2,
                                    mouse_state.y() as i32 - SQR_SIZE as i32 / 2,
                                    SQR_SIZE,
                                    SQR_SIZE,
                                ),
                            );
                        }
                    }
                }
            }
            if let Some(_) = curr_role_click {
                if let Some(piecetype) = curr_role_click {
                    let drag_piece = Piece {
                        color: game.get_color(),
                        piecetype: piecetype,
                        position: None,
                        promoted: false,
                    };
                    curr_texture = piece_to_texture(&drag_piece);
                    let _ = canvas.copy(
                        curr_texture,
                        None,
                        Rect::new(
                            mouse_state.x() as i32 - SQR_SIZE as i32 / 2,
                            mouse_state.y() as i32 - SQR_SIZE as i32 / 2,
                            SQR_SIZE,
                            SQR_SIZE,
                        ),
                    );
                }
            }
        };

        human_play(); //for now, every turn is human
                      // AI

        // if game.turn() == shakmaty::Color::Black {
        // game = game.to_owned().play(&ai::minimax_root(3, &mut game)).unwrap();
        // }

        // Abandon all hope, ye who enter here.
        // while a mouse button is pressed, it will fall into this conditional
        draw_pieces(&mut canvas, &game, hidden);
        canvas.present();

        // if you don't do this cpu usage will skyrocket to 100%
        events.wait_event_timeout(10);

        events.poll_event();
        //draw_check(&game, &mut canvas);
    }

    Ok(())
}

//-----------------------------------------------------------------------------------
//

fn get_side(mouse_state: sdl2::mouse::MouseState) -> Option<shogai::piece::Color> {
    if mouse_state.y() <= SRC_RESERVE_HEIGTH as i32 {
        return Some(shogai::piece::Color::White);
    }
    if mouse_state.y() >= SCR_HEIGHT as i32 - SRC_RESERVE_HEIGTH as i32 {
        return Some(shogai::piece::Color::Black);
    }
    return None;
}

fn get_in_reserve(mouse_state: sdl2::mouse::MouseState) -> Option<PieceType> {
    if mouse_state.y() >= SRC_RESERVE_HEIGTH as i32
        && mouse_state.y() <= SCR_HEIGHT as i32 - SRC_RESERVE_HEIGTH as i32
    {
        return None;
    } else {
        // in reserve
        match mouse_state.x() * 7 / SCR_WIDTH as i32 {
            0 => Some(PieceType::Pawn),
            1 => Some(PieceType::Knight),
            2 => Some(PieceType::Lance),
            3 => Some(PieceType::Rook),
            4 => Some(PieceType::Bishop),
            5 => Some(PieceType::Gold),
            6 => Some(PieceType::Silver),
            _ => None,
        }
    }
}

fn draw_piece(canvas: &mut Canvas<Window>, texture: &Texture, i: Position) {
    canvas
        .copy(
            texture,
            None,
            Rect::new(
                ((9 - (i.0 as u32 % 9) - 1) * SQR_SIZE) as i32,
                (i.0 as u32 / 9 * SQR_SIZE) as i32 + SRC_RESERVE_HEIGTH as i32,
                SQR_SIZE,
                SQR_SIZE,
            ),
        )
        .unwrap();
}

fn draw_piece_on_reserve(
    canvas: &mut Canvas<Window>,
    texture: &Texture,
    piece: &Piece,
    count: isize,
) {
    let x = match piece.piecetype {
        PieceType::Pawn => 0,
        PieceType::Knight => 1,
        PieceType::Lance => 2,
        PieceType::Rook => 3,
        PieceType::Bishop => 4,
        PieceType::Gold => 5,
        PieceType::Silver => 6,
        PieceType::King => panic!("King was found in reserve, what kind of shit is this?"),
    };
    let spacing_multiplier = 10; //pixels per identical pieces
    let y: isize;
    if piece.color == shogai::piece::Color::White {
        y = (count + 1) * spacing_multiplier;
    } else {
        y = SCR_HEIGHT as isize - SQR_SIZE as isize - (count + 1) * spacing_multiplier;
    }
    canvas
        .copy(
            texture,
            None,
            Rect::new((x * SCR_WIDTH / 7) as i32, y as i32, SQR_SIZE, SQR_SIZE),
        )
        .unwrap();
}

fn draw_shogiban(canvas: &mut Canvas<Window>) {
    draw_grid(canvas);

    canvas.set_draw_color(Color::RGB(0x75, 0x48, 0x3B));
    let _ = canvas.fill_rect(Rect::new(0, 0, SCR_WIDTH, SRC_RESERVE_HEIGTH));
    let _ = canvas.fill_rect(Rect::new(
        0,
        SCR_HEIGHT as i32 - SRC_RESERVE_HEIGTH as i32,
        SCR_WIDTH,
        SRC_RESERVE_HEIGTH,
    ));
}

// from: https://www.libsdl.org/tmp/SDL/test/testdrawchessboard.c
// adapted for shogi
fn draw_grid(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0xFF, 0xCE, 0x9E));
    let mut row = 0;

    while row < 9 {
        let mut x = row % 2;

        for _ in (row % 2)..(5 + (row % 2)) {
            let rect = Rect::new(
                x * SQR_SIZE as i32,
                row * SQR_SIZE as i32 + SRC_RESERVE_HEIGTH as i32,
                SQR_SIZE,
                SQR_SIZE,
            );
            x += 2;

            let _ = canvas.fill_rect(rect);
        }

        row += 1;
    }
}

//----------------------------------------------------------------

fn draw_select(p: Position, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(5, 150, 5));
    let x = (8 - p.0 % 9) * SQR_SIZE as u16;
    let y = p.0 / 9 * SQR_SIZE as u16 + SRC_RESERVE_HEIGTH as u16;
    let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, SQR_SIZE, SQR_SIZE));
}

fn draw_error(x: i32, y: i32, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 5, 5));
    let _ = canvas.fill_rect(Rect::new(x, y, SQR_SIZE, SQR_SIZE));
    thread::sleep(time::Duration::from_millis(100));
}
//
// fn draw_check(game: &Board, canvas: &mut Canvas<Window>) {
//     let pieces = game.board().pieces();
//     let mut white_king_pos: Square = Square::new(0);
//     let mut black_king_pos: Square = Square::new(0);
//
//     for i in 0..pieces.len() {
//         pieces
//             .to_owned()
//             .nth(i)
//             .filter(|piece| piece.1.role == Role::King)
//             .map(|piece| {
//                 if piece.1.color == shakmaty::Color::White {
//                     white_king_pos = piece.to_owned().0;
//                 } else {
//                     black_king_pos = piece.to_owned().0;
//                 }
//             });
//     }
//
//     if game.is_check() {
//         let x: i32;
//         let y: i32;
//
//         if game.turn() == shakmaty::Color::White {
//             x = ((white_king_pos.file().char() as u32 - 'a' as u32) * SQR_SIZE) as i32;
//             y = ((white_king_pos.rank().flip_vertical().char() as u32 - '1' as u32) * SQR_SIZE)
//                 as i32;
//         } else {
//             x = ((black_king_pos.file().char() as u32 - 'a' as u32) * SQR_SIZE) as i32;
//             y = ((black_king_pos.rank().flip_vertical().char() as u32 - '1' as u32) * SQR_SIZE)
//                 as i32;
//         }
//
//         canvas.set_draw_color(Color::RGB(255, 5, 5));
//         let _ = canvas.fill_rect(Rect::new(x, y, SQR_SIZE, SQR_SIZE));
//     }
// }
