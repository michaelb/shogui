extern crate sdl2;
extern crate shakmaty;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::messagebox::{show_message_box, show_simple_message_box, MessageBoxFlag};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::{thread, time};

// use shakmaty::{Board, Chess, File, Move, Position, Rank, Role, Setup, Square};
use shogai::ai::*;
use shogai::board::*;
use shogai::movement::*;
use shogai::piece::*;
use shogai::position::*;

use std::collections::HashSet;
use std::path::Path;

use crate::emscripten_file;

const SRC_RESERVE_HEIGTH: u32 = 150;
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
            if let Some(i) = piece.position {
                draw_piece(canvas, &game, piece_to_texture(piece), i);
            }
            //else draw them on the reserve
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
        // AI

        // if game.turn() == shakmaty::Color::Black {
        // game = game.to_owned().play(&ai::minimax_root(3, &mut game)).unwrap();
        // }

        // Abandon all hope, ye who enter here.
        // while a mouse button is pressed, it will fall into this conditional
        let get_texture = |game: &Board| {
            // TODO: use filter here
            // match is more readable than if let.
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
                hidden = game.is_occupied_by(pos);
            } else {
                curr_role_click = None; //manage reserve TODO
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
                        promotion: false, //TODO manage promotion
                        force_capture: false,
                        offer_draw: false,
                        withdraw: false,
                        restart: false,
                    };
                    let mv = full_mv.to_string();
                    if game.check_move(&mv).is_ok() {
                        game = game.play_move_unchecked(&mv);
                        curr_texture = &nothing;
                        hidden = None;
                    }
                }
            }
        }

        if let Some(_) = curr_role_click {
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
        draw_pieces(&mut canvas, &game, hidden);
        canvas.present();
        // if you don't do this cpu usage will skyrocket to 100%
        //
        events.wait_event_timeout(10);

        events.poll_event();
        //draw_check(&game, &mut canvas);
    }

    Ok(())
}

//-----------------------------------------------------------------------------------

fn draw_piece(canvas: &mut Canvas<Window>, game: &Board, texture: &Texture, i: Position) {
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
// TODO: make this actually work as expected

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
