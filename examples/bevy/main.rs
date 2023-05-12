extern crate incrustmental;

extern crate bevy;
extern crate bevy_egui;

mod resources;
mod systems;

use std::{path::Path, time::Duration};

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use incrustmental::{
    incremental::State,
    prelude::{
        load, Automation, AutomationKind, Badge, Objective, Perk, PerkKind, Product,
        ProductConditionKind, ProductMaterial, Quantity, Relation, RelationKind,
    },
    timer::Timer,
    types::{ProductId, ProductMaterialId},
};
use resources::StateRes;
use systems::{draw, end_screen, handle_input, main_menu, update, AppState};

const LEMONADE: ProductId = 0;
const SHOP: ProductMaterialId = 0;
const LEMON: ProductMaterialId = 1;

#[allow(dead_code)]
fn procedural_state() -> State {
    State::new(
        0f64,
        Objective::new(vec![
            Quantity::Money(10000f64).into(),
            Quantity::Product(LEMONADE, 500, Some(ProductConditionKind::Produced)).into(),
            Quantity::Product(LEMONADE, 200, Some(ProductConditionKind::Sold)).into(),
        ]),
        vec![
            ProductMaterial::new(4, "Shop".to_string(), 1f64, 1.07f64, true),
            ProductMaterial::new(100, "Lemon".to_string(), 0.01f64, 1.02f64, true),
            ProductMaterial::new(0, "Sugar".to_string(), 0.1f64, 1.07f64, false),
        ],
        vec![Product::new(
            "Lemonade".to_string(),
            Some(1f64),
            vec![
                Relation::new(RelationKind::ManufacturedBy, Quantity::Material(SHOP, 1)),
                Relation::new(RelationKind::Consumes, Quantity::Material(LEMON, 2)),
                Relation::new(RelationKind::SoldBy, Quantity::Material(SHOP, 1)),
            ],
            vec![0],
            vec![],
            true,
        )],
        vec![
            Badge::new(
                "King of the lemonade trade".to_string(),
                "Produced 10 lemonades!".to_string(),
                vec![Quantity::Product(LEMONADE, 10, Some(ProductConditionKind::Produced)).into()],
            ),
            Badge::new(
                "Lemonade emperor".to_string(),
                "Produced 200 lemonades!".to_string(),
                vec![Quantity::Product(LEMONADE, 200, Some(ProductConditionKind::Produced)).into()],
            ),
        ],
        vec![Perk::new(
            "Lemonficcient".to_string(),
            "Each lemon produces 10 times more lemonade".to_string(),
            vec![Quantity::Product(LEMONADE, 10, Some(ProductConditionKind::Sold)).into()],
            vec![Quantity::Money(10.), Quantity::Material(LEMON, 10)],
            (Quantity::Product(LEMONADE, 10, None), PerkKind::Multiply),
        )],
        vec![
            Automation::new(
                "Lemonade Machine".to_string(),
                AutomationKind::Build(LEMONADE),
                Some(Timer::new(Duration::from_secs(1))),
                vec![Quantity::Product(LEMONADE, 100, Some(ProductConditionKind::Sold)).into()],
                vec![Quantity::Material(SHOP, 10)],
            ),
            Automation::new(
                "Lemonade fetch-boy".to_string(),
                AutomationKind::Buy(LEMON),
                Some(Timer::new(Duration::from_secs(1))),
                vec![Quantity::Product(LEMONADE, 200, Some(ProductConditionKind::Produced)).into()],
                vec![Quantity::Money(1000.)],
            ),
        ],
    )
}

fn main() {
    let state = load(Path::new("res/lemonstand.yml")).unwrap();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "IncRustMental".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EguiPlugin)
        .add_state::<AppState>()
        .insert_resource(StateRes(state))
        .add_system(main_menu.in_set(OnUpdate(AppState::MainMenu)))
        .add_systems(
            (handle_input.before(update), update, draw.after(update))
                .in_set(OnUpdate(AppState::Game)),
        )
        .add_system(end_screen.in_set(OnUpdate(AppState::EndGame)))
        .run();
}
