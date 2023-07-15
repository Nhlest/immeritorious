use std::collections::HashMap;
use easy_imgui::cgmath::{InnerSpace, MetricSpace};
use easy_imgui::Vector2;
use crate::game::card::Card;

pub struct GameState {
  pub cards: Vec<Card>,
  pub focus: String,
}

impl GameState {
  pub fn new() -> Self {
    Self {
      cards: vec![],
      focus: "".to_owned()
    }
  }
}