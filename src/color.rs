use std::sync::atomic::AtomicU8;

// 格式为 [R, G, B, A]
const BG: [u8; 4] = [40, 44, 52, 255]; // #282c34
const FG: [u8; 4] = [171, 178, 191, 255]; // #abb2bf
const RED: [u8; 4] = [224, 108, 117, 255]; // #e06c75
const ORANGE: [u8; 4] = [209, 154, 102, 255]; // #d19a66
const YELLOW: [u8; 4] = [229, 192, 123, 255]; // #e5c07b
const GREEN: [u8; 4] = [152, 195, 121, 255]; // #98c379
const CYAN: [u8; 4] = [86, 182, 194, 255]; // #56b6c2
const BLUE: [u8; 4] = [97, 175, 239, 255]; // #61afef
const PURPLE: [u8; 4] = [198, 120, 221, 255]; // #c678dd
const WHITE: [u8; 4] = [171, 178, 191, 255]; // #abb2bf
const BLACK: [u8; 4] = [40, 44, 52, 255]; // #282c34
const GRAY: [u8; 4] = [92, 99, 112, 255]; // #5c6370

// Color palette
const COLOR_PALETTE: [[u8; 4]; 8] = [RED, ORANGE, YELLOW, GREEN, CYAN, BLUE, PURPLE, GRAY];

pub fn get_color(index: usize) -> [u8; 4] {
  COLOR_PALETTE[index as usize]
}
pub fn get_bg() -> [u8; 4] {
  BG
}
pub fn get_fg() -> [u8; 4] {
  FG
}
