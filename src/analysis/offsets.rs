use std::collections::BTreeMap;

use anyhow::Result;

use log::debug;

use memflow::prelude::v1::*;

use pelite::pe64::{Pe, PeView, Rva};

pub type OffsetMap = BTreeMap<String, BTreeMap<String, Rva>>;

mod client {
    use super::*;

    pub fn offsets(view: PeView<'_>) -> BTreeMap<String, Rva> {
        let mut map = BTreeMap::new();

        // Add static offsets with updated values
        map.insert("dwCSGOInput".to_string(), 0x1D2BE10);
        map.insert("dwEntityList".to_string(), 0x1CBE4A0);
        map.insert("dwGameRules".to_string(), 0x1D1D2E0);
        map.insert("dwGlobalVars".to_string(), 0x1AE92E8);
        map.insert("dwGlowManager".to_string(), 0x1D1D308);
        map.insert("dwPlantedC4".to_string(), 0x1D26E00);
        map.insert("dwViewMatrix".to_string(), 0x1D21800);
        map.insert("dwViewRender".to_string(), 0x1D22440);

        // Previously provided static offsets
        map.insert("dwLocalPlayerPawn".to_string(), 0x1AF4A20);
        map.insert("dwSensitivity".to_string(), 0x1A6A9D8);
        map.insert("dwSensitivity_sensitivity".to_string(), 0x40);
        map.insert("dwViewAngles".to_string(), 0x1A78650);

        // Convert previously outdated patterns to static offsets
        map.insert("dwGameEntitySystem".to_string(), 0x1E49280);
        map.insert("dwGameEntitySystem_highestEntityIndex".to_string(), 0x1520);
        map.insert("dwLocalPlayerController".to_string(), 0x1C092D8);
        map.insert("dwPrediction".to_string(), 0x1D1F420);
        map.insert("dwWeaponC4".to_string(), 0x1E01A70);

        // Add field offsets
        map.insert("m_hPlayerPawn".to_string(), 0x6B4);
        map.insert("m_iHealth".to_string(), 0x34C);
        map.insert("m_iTeamNum".to_string(), 0x3EB);
        map.insert("m_Glow".to_string(), 0xC00);
        map.insert("m_iGlowType".to_string(), 0x30);
        map.insert("m_glowColorOverride".to_string(), 0x40);
        map.insert("m_bGlowing".to_string(), 0x51);
        map.insert("m_vOldOrigin".to_string(), 0x15B0);
        map.insert("m_pGameSceneNode".to_string(), 0x330);
        map.insert("m_modelState".to_string(), 0x170);
        map.insert("m_iIDEntIndex".to_string(), 0x1458);
        map.insert("m_designerName".to_string(), 0x20);
        map.insert("m_pEntity".to_string(), 0x10);
        map.insert("m_flFlashDuration".to_string(), 0x140C);
        map.insert("m_flFlashBangTime".to_string(), 0x13F8);
        map.insert("m_flFlashOverlayAlpha".to_string(), 0x1400);
        map.insert("m_sSanitizedPlayerName".to_string(), 0x778);
        map.insert("m_hPawn".to_string(), 0x62C);
        map.insert("m_fFlags".to_string(), 0x3F8);
        map.insert("m_iShotsFired".to_string(), 0x23FC);
        map.insert("m_aimPunchCache".to_string(), 0x15A8);
        map.insert("m_pClippingWeapon".to_string(), 0x1620);
        map.insert("m_AttributeManager".to_string(), 0x1148);
        map.insert("m_Item".to_string(), 0x50);
        map.insert("m_iItemDefinitionIndex".to_string(), 0x1BA);
        map.insert("m_bIsScoped".to_string(), 0x23E8);
        map.insert("jump".to_string(), 0x1850DF0);

        for (name, value) in &map {
            debug!(
                "found offset: {} at {:#X} (client.dll + {:#X})",
                name,
                *value as u64 + view.optional_header().ImageBase,
                value
            );
        }

        map
    }
}

mod engine2 {
    use super::*;

    pub fn offsets(view: PeView<'_>) -> BTreeMap<String, Rva> {
        let mut map = BTreeMap::new();

        // Add all static offsets for engine2
        map.insert("dwBuildNumber".to_string(), 0x5F7014);
        map.insert("dwNetworkGameClient".to_string(), 0x5F6FB8);
        map.insert("dwNetworkGameClient_clientTickCount".to_string(), 0x174);
        map.insert("dwNetworkGameClient_deltaTick".to_string(), 0x178);
        map.insert("dwNetworkGameClient_isBackgroundMap".to_string(), 0x278);
        map.insert("dwNetworkGameClient_localPlayer".to_string(), 0x2A8);
        map.insert("dwNetworkGameClient_maxClients".to_string(), 0x270);
        map.insert("dwNetworkGameClient_serverTickCount".to_string(), 0x170);
        map.insert("dwNetworkGameClient_signOnState".to_string(), 0x260);
        map.insert("dwWindowHeight".to_string(), 0x614844);
        map.insert("dwWindowWidth".to_string(), 0x614840);

        for (name, value) in &map {
            debug!(
                "found offset: {} at {:#X} (engine2.dll + {:#X})",
                name,
                *value as u64 + view.optional_header().ImageBase,
                value
            );
        }

        map
    }
}

mod input_system {
    use super::*;

    pub fn offsets(view: PeView<'_>) -> BTreeMap<String, Rva> {
        let mut map = BTreeMap::new();

        // Add static offset for dwInputSystem
        map.insert("dwInputSystem".to_string(), 0x3A5B0);

        for (name, value) in &map {
            debug!(
                "found offset: {} at {:#X} (inputsystem.dll + {:#X})",
                name,
                *value as u64 + view.optional_header().ImageBase,
                value
            );
        }

        map
    }
}

mod matchmaking {
    use super::*;

    pub fn offsets(view: PeView<'_>) -> BTreeMap<String, Rva> {
        let mut map = BTreeMap::new();

        // Add static offsets for matchmaking
        map.insert("dwGameTypes".to_string(), 0x1A41C0);
        map.insert("dwGameTypes_mapName".to_string(), 0x120);

        for (name, value) in &map {
            debug!(
                "found offset: {} at {:#X} (matchmaking.dll + {:#X})",
                name,
                *value as u64 + view.optional_header().ImageBase,
                value
            );
        }

        map
    }
}

mod soundsystem {
    use super::*;

    pub fn offsets(view: PeView<'_>) -> BTreeMap<String, Rva> {
        let mut map = BTreeMap::new();

        // Add static offsets for soundsystem
        map.insert("dwSoundSystem".to_string(), 0x39A5E0);
        map.insert("dwSoundSystem_engineViewData".to_string(), 0x7C);

        for (name, value) in &map {
            debug!(
                "found offset: {} at {:#X} (soundsystem.dll + {:#X})",
                name,
                *value as u64 + view.optional_header().ImageBase,
                value
            );
        }

        map
    }
}

pub fn offsets<P: Process + MemoryView>(process: &mut P) -> Result<OffsetMap> {
    let mut map = BTreeMap::new();

    let modules: [(&str, fn(PeView) -> BTreeMap<String, Rva>); 5] = [
        ("client.dll", client::offsets),
        ("engine2.dll", engine2::offsets),
        ("inputsystem.dll", input_system::offsets),
        ("matchmaking.dll", matchmaking::offsets),
        ("soundsystem.dll", soundsystem::offsets),
    ];

    for (module_name, offsets) in &modules {
        let module = process.module_by_name(module_name)?;

        let buf = process
            .read_raw(module.base, module.size as _)
            .data_part()?;

        let view = PeView::from_bytes(&buf)?;

        map.insert(module_name.to_string(), offsets(view));
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::Value;

    use super::*;

    fn setup() -> Result<IntoProcessInstanceArcBox<'static>> {
        let os = memflow_native::create_os(&OsArgs::default(), LibArc::default())?;

        let process = os.into_process_by_name("cs2.exe")?;

        Ok(process)
    }

    fn get_class_field_value(module_name: &str, class_name: &str, field_name: &str) -> Option<u64> {
        let content =
            fs::read_to_string(format!("output/{}.json", module_name.replace(".", "_"))).ok()?;

        let value: Value = serde_json::from_str(&content).ok()?;

        value
            .get(module_name)?
            .get("classes")?
            .get(class_name)?
            .get("fields")?
            .get(field_name)?
            .as_u64()
    }

    fn get_offset_value(module_name: &str, offset_name: &str) -> Option<u64> {
        let content = fs::read_to_string("output/offsets.json").ok()?;
        let value: Value = serde_json::from_str(&content).ok()?;

        let offset = value.get(module_name)?.get(offset_name)?;

        offset.as_u64()
    }

    #[test]
    fn build_number() -> Result<()> {
        let mut process = setup()?;

        let engine_base = process.module_by_name("engine2.dll")?.base;

        let offset = get_offset_value("engine2.dll", "dwBuildNumber").unwrap();

        let build_number: u32 = process.read(engine_base + offset).data_part()?;

        println!("build number: {}", build_number);

        Ok(())
    }

    #[test]
    fn global_vars() -> Result<()> {
        let mut process = setup()?;

        let client_base = process.module_by_name("client.dll")?.base;

        let offset = get_offset_value("client.dll", "dwGlobalVars").unwrap();

        let global_vars: u64 = process.read(client_base + offset).data_part()?;

        let cur_map_name = {
            let addr = process
                .read_addr64((global_vars + 0x180).into())
                .data_part()?;

            process.read_utf8(addr, 128).data_part()?
        };

        println!("current map name: {}", cur_map_name);

        Ok(())
    }

    #[test]
    fn local_player_controller() -> Result<()> {
        let mut process = setup()?;

        let client_base = process.module_by_name("client.dll")?.base;

        let local_player_controller_offset =
            get_offset_value("client.dll", "dwLocalPlayerController").unwrap();

        let player_name_offset =
            get_class_field_value("client.dll", "CBasePlayerController", "m_iszPlayerName")
                .unwrap();

        let local_player_controller: u64 = process
            .read(client_base + local_player_controller_offset)
            .data_part()?;

        let player_name = process
            .read_utf8((local_player_controller + player_name_offset).into(), 4096)
            .data_part()?;

        println!("local player name: {}", player_name);

        Ok(())
    }

    #[test]
    fn local_player_pawn() -> Result<()> {
        #[derive(Debug, Pod)]
        #[repr(C)]
        struct Vector3D {
            x: f32,
            y: f32,
            z: f32,
        }

        let mut process = setup()?;

        let client_base = process.module_by_name("client.dll")?.base;

        let local_player_pawn_offset = get_offset_value("client.dll", "dwLocalPlayerPawn").unwrap();

        let game_scene_node_offset =
            get_class_field_value("client.dll", "C_BaseEntity", "m_pGameSceneNode").unwrap();

        let vec_abs_origin_offset =
            get_class_field_value("client.dll", "CGameSceneNode", "m_vecAbsOrigin").unwrap();

        let local_player_pawn: u64 = process
            .read(client_base + local_player_pawn_offset)
            .data_part()?;

        let game_scene_node: u64 = process
            .read((local_player_pawn + game_scene_node_offset).into())
            .data_part()?;

        let vec_abs_origin: Vector3D = process
            .read((game_scene_node + vec_abs_origin_offset).into())
            .data_part()?;

        println!("local player origin: {:?}", vec_abs_origin);

        Ok(())
    }

    #[test]
    fn window_size() -> Result<()> {
        let mut process = setup()?;

        let engine_base = process.module_by_name("engine2.dll")?.base;

        let window_width_offset = get_offset_value("engine2.dll", "dwWindowWidth").unwrap();
        let window_height_offset = get_offset_value("engine2.dll", "dwWindowHeight").unwrap();

        let window_width: u32 = process
            .read(engine_base + window_width_offset)
            .data_part()?;

        let window_height: u32 = process
            .read(engine_base + window_height_offset)
            .data_part()?;

        println!("window size: {}x{}", window_width, window_height);

        Ok(())
    }
}
