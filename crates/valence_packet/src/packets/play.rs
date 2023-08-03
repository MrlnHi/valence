use bitfield_struct::bitfield;
use byteorder::WriteBytesExt;
use glam::{DVec3, IVec3, Vec3};
use valence_core::block_pos::BlockPos;
use valence_core::chunk_pos::ChunkPos;
use valence_core::difficulty::Difficulty;
use valence_core::direction::Direction;
use valence_core::game_mode::GameMode;
use valence_core::hand::Hand;
use valence_core::item::ItemStack;
use valence_core::protocol::byte_angle::ByteAngle;
use valence_core::protocol::global_pos::GlobalPos;
use valence_core::protocol::var_long::VarLong;
use valence_nbt::Compound;

use super::*;

pub mod advancement_tab_c2s;
pub use advancement_tab_c2s::AdvancementTabC2s;
pub mod advancement_update_s2c;
pub use advancement_update_s2c::AdvancementUpdateS2c;
pub mod block_breaking_progress_s2c;
pub use block_breaking_progress_s2c::BlockBreakingProgressS2c;
pub mod block_entity_update_s2c;
pub use block_entity_update_s2c::BlockEntityUpdateS2c;
pub mod block_event_s2c;
pub use block_event_s2c::BlockEventS2c;
pub mod block_update_s2c;
pub use block_update_s2c::BlockUpdateS2c;
pub mod boat_paddle_state_c2s;
pub use boat_paddle_state_c2s::BoatPaddleStateC2s;
pub mod book_update_c2s;
pub use book_update_c2s::BookUpdateC2s;
pub mod boss_bar_s2c;
pub use boss_bar_s2c::BossBarS2c;
pub mod bundle_splitter_s2c;
pub use bundle_splitter_s2c::BundleSplitterS2c;
pub mod button_click_c2s;
pub use button_click_c2s::ButtonClickC2s;
pub mod chat_message_c2s;
pub use chat_message_c2s::ChatMessageC2s;
pub mod chat_message_s2c;
pub use chat_message_s2c::ChatMessageS2c;
pub mod chat_suggestions_s2c;
pub use chat_suggestions_s2c::ChatSuggestionsS2c;
pub mod chunk_biome_data_s2c;
pub use chunk_biome_data_s2c::ChunkBiomeDataS2c;
pub mod chunk_data_s2c;
pub use chunk_data_s2c::ChunkDataS2c;
pub mod chunk_delta_update_s2c;
pub use chunk_delta_update_s2c::ChunkDeltaUpdateS2c;
pub mod chunk_load_distance_s2c;
pub use chunk_load_distance_s2c::ChunkLoadDistanceS2c;
pub mod chunk_render_distance_center_s2c;
pub use chunk_render_distance_center_s2c::ChunkRenderDistanceCenterS2c;
pub mod clear_title_s2c;
pub use clear_title_s2c::ClearTitleS2c;
pub mod click_slot_c2s;
pub use click_slot_c2s::ClickSlotC2s;
pub mod client_command_c2s;
pub use client_command_c2s::ClientCommandC2s;
pub mod client_settings_c2s;
pub use client_settings_c2s::ClientSettingsC2s;
pub mod client_status_c2s;
pub use client_status_c2s::ClientStatusC2s;
pub mod close_handled_screen_c2s;
pub use close_handled_screen_c2s::CloseHandledScreenC2s;
pub mod close_screen_s2c;
pub use close_screen_s2c::CloseScreenS2c;
pub mod command_execution_c2s;
pub use command_execution_c2s::CommandExecutionC2s;
pub mod command_suggestions_s2c;
pub use command_suggestions_s2c::CommandSuggestionsS2c;
pub mod command_tree_s2c;
pub use command_tree_s2c::CommandTreeS2c;
pub mod cooldown_update_s2c;
pub use cooldown_update_s2c::CooldownUpdateS2c;
pub mod craft_failed_response_s2c;
pub use craft_failed_response_s2c::CraftFailedResponseS2c;
pub mod craft_request_c2s;
pub use craft_request_c2s::CraftRequestC2s;
pub mod creative_inventory_action_c2s;
pub use creative_inventory_action_c2s::CreativeInventoryActionC2s;
pub mod custom_payload_c2s;
pub use custom_payload_c2s::CustomPayloadC2s;
pub mod custom_payload_s2c;
pub use custom_payload_s2c::CustomPayloadS2c;
pub mod damage_tilt_s2c;
pub use damage_tilt_s2c::DamageTiltS2c;
pub mod death_message_s2c;
pub use death_message_s2c::DeathMessageS2c;
pub mod difficulty_s2c;
pub use difficulty_s2c::DifficultyS2c;
pub mod disconnect_s2c;
pub use disconnect_s2c::DisconnectS2c;
pub mod end_combat_s2c;
pub use end_combat_s2c::EndCombatS2c;
pub mod enter_combat_s2c;
pub use enter_combat_s2c::EnterCombatS2c;
pub mod entities_destroy_s2c;
pub use entities_destroy_s2c::EntitiesDestroyS2c;
pub mod entity_animation_s2c;
pub use entity_animation_s2c::EntityAnimationS2c;
pub mod entity_attach_s2c;
pub use entity_attach_s2c::EntityAttachS2c;
pub mod entity_attributes_s2c;
pub use entity_attributes_s2c::EntityAttributesS2c;
pub mod entity_damage_s2c;
pub use entity_damage_s2c::EntityDamageS2c;
pub mod entity_equipment_update_s2c;
pub use entity_equipment_update_s2c::EntityEquipmentUpdateS2c;
pub mod entity_passengers_set_s2c;
pub use entity_passengers_set_s2c::EntityPassengersSetS2c;
pub mod entity_position_s2c;
pub use entity_position_s2c::EntityPositionS2c;
pub mod entity_set_head_yaw_s2c;
pub use entity_set_head_yaw_s2c::EntitySetHeadYawS2c;
pub mod entity_spawn_s2c;
pub use entity_spawn_s2c::EntitySpawnS2c;
pub mod entity_status_effect_s2c;
pub use entity_status_effect_s2c::EntityStatusEffectS2c;
pub mod entity_status_s2c;
pub use entity_status_s2c::EntityStatusS2c;
pub mod entity_tracker_update_s2c;
pub use entity_tracker_update_s2c::EntityTrackerUpdateS2c;
pub mod entity_velocity_update_s2c;
pub use entity_velocity_update_s2c::EntityVelocityUpdateS2c;
pub mod experience_bar_update_s2c;
pub use experience_bar_update_s2c::ExperienceBarUpdateS2c;
pub mod experience_orb_spawn_s2c;
pub use experience_orb_spawn_s2c::ExperienceOrbSpawnS2c;
pub mod explosion_s2c;
pub use explosion_s2c::ExplosionS2c;
pub mod features_s2c;
pub use features_s2c::FeaturesS2c;
pub mod full_c2s;
pub use full_c2s::FullC2s;
pub mod game_join_s2c;
pub use game_join_s2c::GameJoinS2c;
pub mod game_message_s2c;
pub use game_message_s2c::GameMessageS2c;
pub mod game_state_change_s2c;
pub use game_state_change_s2c::GameStateChangeS2c;
pub mod hand_swing_c2s;
pub use hand_swing_c2s::HandSwingC2s;
pub mod health_update_s2c;
pub use health_update_s2c::HealthUpdateS2c;
pub mod inventory_s2c;
pub use inventory_s2c::InventoryS2c;
pub mod item_pickup_animation_s2c;
pub use item_pickup_animation_s2c::ItemPickupAnimationS2c;
pub mod jigsaw_generating_c2s;
pub use jigsaw_generating_c2s::JigsawGeneratingC2s;
pub mod keep_alive_c2s;
pub use keep_alive_c2s::KeepAliveC2s;
pub mod keep_alive_s2c;
pub use keep_alive_s2c::KeepAliveS2c;
pub mod light_update_s2c;
pub use light_update_s2c::LightUpdateS2c;
pub mod look_and_on_ground_c2s;
pub use look_and_on_ground_c2s::LookAndOnGroundC2s;
pub mod look_at_s2c;
pub use look_at_s2c::LookAtS2c;
pub mod map_update_s2c;
pub use map_update_s2c::MapUpdateS2c;
pub mod message_acknowledgment_c2s;
pub use message_acknowledgment_c2s::MessageAcknowledgmentC2s;
pub mod move_relative_s2c;
pub use move_relative_s2c::MoveRelativeS2c;
pub mod nbt_query_response_s2c;
pub use nbt_query_response_s2c::NbtQueryResponseS2c;
pub mod on_ground_only_c2s;
pub use on_ground_only_c2s::OnGroundOnlyC2s;
pub mod open_horse_screen_s2c;
pub use open_horse_screen_s2c::OpenHorseScreenS2c;
pub mod open_screen_s2c;
pub use open_screen_s2c::OpenScreenS2c;
pub mod open_written_book_s2c;
pub use open_written_book_s2c::OpenWrittenBookS2c;
pub mod overlay_message_s2c;
pub use overlay_message_s2c::OverlayMessageS2c;
pub mod particle_s2c;
pub use particle_s2c::ParticleS2c;
pub mod pick_from_inventory_c2s;
pub use pick_from_inventory_c2s::PickFromInventoryC2s;
pub mod play_ping_s2c;
pub use play_ping_s2c::PlayPingS2c;
pub mod play_pong_c2s;
pub use play_pong_c2s::PlayPongC2s;
pub mod play_sound_from_entity_s2c;
pub use play_sound_from_entity_s2c::PlaySoundFromEntityS2c;
pub mod play_sound_s2c;
pub use play_sound_s2c::PlaySoundS2c;
pub mod player_abilities_s2c;
pub use player_abilities_s2c::PlayerAbilitiesS2c;
pub mod player_action_c2s;
pub use player_action_c2s::PlayerActionC2s;
pub mod player_action_response_s2c;
pub use player_action_response_s2c::PlayerActionResponseS2c;
pub mod player_input_c2s;
pub use player_input_c2s::PlayerInputC2s;
pub mod player_interact_block_c2s;
pub use player_interact_block_c2s::PlayerInteractBlockC2s;
pub mod player_interact_entity_c2s;
pub use player_interact_entity_c2s::PlayerInteractEntityC2s;
pub mod player_interact_item_c2s;
pub use player_interact_item_c2s::PlayerInteractItemC2s;
pub mod player_list_header_s2c;
pub use player_list_header_s2c::PlayerListHeaderS2c;
pub mod player_list_s2c;
pub use player_list_s2c::PlayerListS2c;
pub mod player_position_look_s2c;
pub use player_position_look_s2c::PlayerPositionLookS2c;
pub mod player_remove_s2c;
pub use player_remove_s2c::PlayerRemoveS2c;
pub mod player_respawn_s2c;
pub use player_respawn_s2c::PlayerRespawnS2c;
pub mod player_session_c2s;
pub use player_session_c2s::PlayerSessionC2s;
pub mod player_spawn_position_s2c;
pub use player_spawn_position_s2c::PlayerSpawnPositionS2c;
pub mod player_spawn_s2c;
pub use player_spawn_s2c::PlayerSpawnS2c;
pub mod position_and_on_ground_c2s;
pub use position_and_on_ground_c2s::PositionAndOnGroundC2s;
pub mod profileless_chat_message_s2c;
pub use profileless_chat_message_s2c::ProfilelessChatMessageS2c;
pub mod query_block_nbt_c2s;
pub use query_block_nbt_c2s::QueryBlockNbtC2s;
pub mod query_entity_nbt_c2s;
pub use query_entity_nbt_c2s::QueryEntityNbtC2s;
pub mod recipe_book_data_c2s;
pub use recipe_book_data_c2s::RecipeBookDataC2s;
pub mod recipe_category_options_c2s;
pub use recipe_category_options_c2s::RecipeCategoryOptionsC2s;
pub mod remove_entity_status_effect_s2c;
pub use remove_entity_status_effect_s2c::RemoveEntityStatusEffectS2c;
pub mod remove_message_s2c;
pub use remove_message_s2c::RemoveMessageS2c;
pub mod rename_item_c2s;
pub use rename_item_c2s::RenameItemC2s;
pub mod request_command_completions_c2s;
pub use request_command_completions_c2s::RequestCommandCompletionsC2s;
pub mod resource_pack_send_s2c;
pub use resource_pack_send_s2c::ResourcePackSendS2c;
pub mod resource_pack_status_c2s;
pub use resource_pack_status_c2s::ResourcePackStatusC2s;
pub mod rotate_s2c;
pub use rotate_s2c::RotateS2c;
pub mod rotate_and_move_relative_s2c;
pub use rotate_and_move_relative_s2c::RotateAndMoveRelativeS2c;
pub mod scoreboard_display_s2c;
pub use scoreboard_display_s2c::ScoreboardDisplayS2c;
pub mod scoreboard_objective_update_s2c;
pub use scoreboard_objective_update_s2c::ScoreboardObjectiveUpdateS2c;
pub mod scoreboard_player_update_s2c;
pub use scoreboard_player_update_s2c::ScoreboardPlayerUpdateS2c;
pub mod screen_handler_property_update_s2c;
pub use screen_handler_property_update_s2c::ScreenHandlerPropertyUpdateS2c;
pub mod screen_handler_slot_update_s2c;
pub use screen_handler_slot_update_s2c::ScreenHandlerSlotUpdateS2c;
pub mod select_advancement_tab_s2c;
pub use select_advancement_tab_s2c::SelectAdvancementTabS2c;
pub mod select_merchant_trade_c2s;
pub use select_merchant_trade_c2s::SelectMerchantTradeC2s;
pub mod server_metadata_s2c;
pub use server_metadata_s2c::ServerMetadataS2c;
pub mod set_camera_entity_s2c;
pub use set_camera_entity_s2c::SetCameraEntityS2c;
pub mod set_trade_offers_s2c;
pub use set_trade_offers_s2c::SetTradeOffersS2c;
pub mod sign_editor_open_s2c;
pub use sign_editor_open_s2c::SignEditorOpenS2c;
pub mod simulation_distance_s2c;
pub use simulation_distance_s2c::SimulationDistanceS2c;
pub mod spectator_teleport_c2s;
pub use spectator_teleport_c2s::SpectatorTeleportC2s;
pub mod statistics_s2c;
pub use statistics_s2c::StatisticsS2c;
pub mod stop_sound_s2c;
pub use stop_sound_s2c::StopSoundS2c;
pub mod subtitle_s2c;
pub use subtitle_s2c::SubtitleS2c;
pub mod synchronize_recipes_s2c;
pub use synchronize_recipes_s2c::SynchronizeRecipesS2c;
pub mod synchronize_tags_s2c;
pub use synchronize_tags_s2c::SynchronizeTagsS2c;
pub mod team_s2c;
pub use team_s2c::TeamS2c;
pub mod teleport_confirm_c2s;
pub use teleport_confirm_c2s::TeleportConfirmC2s;
pub mod title_fade_s2c;
pub use title_fade_s2c::TitleFadeS2c;
pub mod title_s2c;
pub use title_s2c::TitleS2c;
pub mod unload_chunk_s2c;
pub use unload_chunk_s2c::UnloadChunkS2c;
pub mod unlock_recipes_s2c;
pub use unlock_recipes_s2c::UnlockRecipesS2c;
pub mod update_beacon_c2s;
pub use update_beacon_c2s::UpdateBeaconC2s;
pub mod update_command_block_c2s;
pub use update_command_block_c2s::UpdateCommandBlockC2s;
pub mod update_command_block_minecart_c2s;
pub use update_command_block_minecart_c2s::UpdateCommandBlockMinecartC2s;
pub mod update_difficulty_c2s;
pub use update_difficulty_c2s::UpdateDifficultyC2s;
pub mod update_difficulty_lock_c2s;
pub use update_difficulty_lock_c2s::UpdateDifficultyLockC2s;
pub mod update_jigsaw_c2s;
pub use update_jigsaw_c2s::UpdateJigsawC2s;
pub mod update_player_abilities_c2s;
pub use update_player_abilities_c2s::UpdatePlayerAbilitiesC2s;
pub mod update_selected_slot_c2s;
pub use update_selected_slot_c2s::UpdateSelectedSlotC2s;
pub mod update_selected_slot_s2c;
pub use update_selected_slot_s2c::UpdateSelectedSlotS2c;
pub mod update_sign_c2s;
pub use update_sign_c2s::UpdateSignC2s;
pub mod update_structure_block_c2s;
pub use update_structure_block_c2s::UpdateStructureBlockC2s;
pub mod vehicle_move_c2s;
pub use vehicle_move_c2s::VehicleMoveC2s;
pub mod vehicle_move_s2c;
pub use vehicle_move_s2c::VehicleMoveS2c;
pub mod world_border_center_changed_s2c;
pub use world_border_center_changed_s2c::WorldBorderCenterChangedS2c;
pub mod world_border_initialize_s2c;
pub use world_border_initialize_s2c::WorldBorderInitializeS2c;
pub mod world_border_interpolate_size_s2c;
pub use world_border_interpolate_size_s2c::WorldBorderInterpolateSizeS2c;
pub mod world_border_size_changed_s2c;
pub use world_border_size_changed_s2c::WorldBorderSizeChangedS2c;
pub mod world_border_warning_blocks_changed_s2c;
pub use world_border_warning_blocks_changed_s2c::WorldBorderWarningBlocksChangedS2c;
pub mod world_border_warning_time_changed_s2c;
pub use world_border_warning_time_changed_s2c::WorldBorderWarningTimeChangedS2c;
pub mod world_event_s2c;
pub use world_event_s2c::WorldEventS2c;
pub mod world_time_update_s2c;
pub use world_time_update_s2c::WorldTimeUpdateS2c;
