/// .
pub(crate) 

use raylib::prelude::*;
use raylib::ffi::{Rectangle, CheckCollisionCircleRec};

struct Player {

    rect: Rectangle,
    score: i32,
    name: String,
    direction: f32,
    ai: AI

}

struct AI {
    prevy: f32
}

struct Projectile {

    pos: Vector2,
    direction: Vector2

}

struct Game {
    player_speed: f32,
    game_speed: f32,
    start_dir: f32
}

static mut GAME: Game = Game {
    player_speed: 5.0,
    game_speed: 3.0,
    start_dir: 1.0
};

fn bPlayer(name: String, x: f32) -> Player {

    Player {
        rect: Rectangle { x: (x), y: (10.0), width: (10.0), height: (50.0) },
        score: 0,
        name: name,
        direction: 0f32,
        ai: AI {
            prevy: 0f32
        }
    }

}

fn bProjectile() -> Projectile {
    
    Projectile {

        pos: Vector2 { x: 720.0/2.0, y: 720.0/2.0 },
        direction: Vector2 { x: 1.0, y: 0f32 }

    }

}

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(720, 720)
        .title("Pong")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);
        
    let mut player1 = bPlayer(String::from("Player 1"), 700.0);
    let mut player2 = bPlayer(String::from("Player 2"), 10.0);

    let mut players = (player1, player2);

    let mut projectile = bProjectile();

    while !rl.window_should_close() {run_loop(&mut rl, &thread, &mut players, &mut projectile);}

}

fn run_loop(rl : &mut RaylibHandle, thread: &RaylibThread, players: &mut (Player, Player), projectile: &mut Projectile) {

    poll_events(rl, players);
    update_projectile(projectile);
    
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::BLACK);

    unsafe {
        check_player_colision(projectile, &players.0);
        check_player_colision(projectile, &players.1);
        check_wall_colision(projectile, players);
    }

    draw_player(&mut d, &players.0);
    draw_projectile(&mut d, projectile);
    draw_player(&mut d, &players.1);

    // impossibleAi(&mut players.1, projectile);
   
}

fn update_projectile(projectile: &mut Projectile) {
    unsafe {
        projectile.pos.x += GAME.game_speed * projectile.direction.x;
        projectile.pos.y += GAME.game_speed * projectile.direction.y;    
    }

}

fn reset(projectile: &mut Projectile) {

    unsafe {
        GAME.player_speed = 5.0;
        GAME.game_speed = 3.0;
        GAME.start_dir = -GAME.start_dir;
        projectile.direction.x = GAME.start_dir;
    }

    projectile.pos.x = 720f32/2f32;
    projectile.pos.y = 720f32/2f32;
 
    projectile.direction.y = 0f32;

}

unsafe fn check_player_colision(projectile: &mut Projectile, player: &Player) {
    if ffi::CheckCollisionCircleRec(projectile.pos.into(), 5.0, player.rect) {
        projectile.direction.x = -projectile.direction.x;
        projectile.direction.y = player.direction;
        GAME.game_speed += 0.1;
        GAME.player_speed += 0.05;
    } 
}

unsafe fn check_wall_colision(projectile: &mut Projectile, players: &mut (Player, Player)) {

    if CheckCollisionCircleRec(projectile.pos.into(), 5.0, Rectangle { x: 0f32, y: 0f32, width: 720f32, height: 1f32 }) {
        projectile.direction.y = -projectile.direction.y;
    }
    
    if CheckCollisionCircleRec(projectile.pos.into(), 5.0, Rectangle { x: 0f32, y: 720f32, width: 720f32, height: 1f32 }) {
        projectile.direction.y = -projectile.direction.y;
    }

    if CheckCollisionCircleRec(projectile.pos.into(), 5.0, Rectangle { x: 0f32, y: 0f32, width: 1f32, height: 720f32 }) {
        reset(projectile);
        players.0.score += 1;
    }

    if CheckCollisionCircleRec(projectile.pos.into(), 5.0, Rectangle { x: 720f32, y: 0f32, width: 1f32, height: 720f32 }) {
        reset(projectile);
        players.1.score += 1;
    }
}

fn impossibleAi(player: &mut Player, projectile: &mut Projectile) {

    player.direction = clamp_float((projectile.pos.y - 20f32) - player.ai.prevy, -1.0, 1.0);

    player.rect.y = projectile.pos.y - 20f32;
    player.ai.prevy = projectile.pos.y - 20f32;
} 

fn draw_projectile(d: &mut RaylibDrawHandle, projectile: &mut Projectile) {

    d.draw_circle_v(projectile.pos, 5.0, Color::WHITE);

}

fn draw_player(d: &mut RaylibDrawHandle, player: &Player) {
    let playerscore = &player.score;
    let strin = format!("{}", playerscore);
    d.draw_text(&strin, player.rect.x as i32, 10, 20, Color::WHITE);
    d.draw_rectangle_rec(player.rect,  Color::WHITE);
}

fn clamp_float(num: f32, min: f32, max :f32) -> f32 {

    if num > max {
        return max;
    }

    if num < min {
        return min;
    }

    return num;

}

fn poll_events(rl: &mut RaylibHandle, players: &mut (Player, Player)) {
    
    unsafe {

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            players.0.rect.y = clamp_float(players.0.rect.y + GAME.player_speed, 10.0, 660.0);
            players.0.direction = 1f32;
        }
    
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            players.0.rect.y = clamp_float(players.0.rect.y - GAME.player_speed, 10.0, 660.0);
            players.0.direction = -1f32;
        }
    
        if rl.is_key_down(KeyboardKey::KEY_S) {
            players.1.rect.y = clamp_float(players.1.rect.y + GAME.player_speed, 10.0, 660.0);
            players.1.direction = 1f32;
        }
    
        if rl.is_key_down(KeyboardKey::KEY_W) {
            players.1.rect.y = clamp_float(players.1.rect.y - GAME.player_speed, 10.0, 660.0);
            players.1.direction = -1f32;
        }
    
    }

    
}