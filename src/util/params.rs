pub mod params {
    use std::{io::Error, mem, str::FromStr};
    
    use binary_reader::{BinaryReader, Endian};
    use crate::util::br_ext::br_ext::BinaryReaderExtensions as br_ext;

    #[allow(non_camel_case_types)]
    #[derive(Clone, PartialEq, Hash, Eq)]
    pub enum Param {
        ActionButtonParam,
        AiSoundParam,
        AssetEnvironmentGeometryParam,
        AssetMaterialSfxParam,
        AssetModelSfxParam,
        AtkParam_Npc,
        AtkParam_Pc,
        AttackElementCorrectParam,
        AutoCreateEnvSoundParam,
        BaseChrSelectMenuParam,
        BehaviorParam,
        BehaviorParam_PC,
        BonfireWarpParam,
        BonfireWarpSubCategoryParam,
        BonfireWarpTabParam,
        BuddyParam,
        BuddyStoneParam,
        BudgetParam,
        Bullet,
        BulletCreateLimitParam,
        CalcCorrectGraph,
        Ceremony,
        CharaInitParam,
        CharMakeMenuListItemParam,
        CharMakeMenuTopParam,
        ChrActivateConditionParam,
        ChrEquipModelParam,
        ChrModelParam,
        ClearCountCorrectParam,
        CoolTimeParam,
        CutsceneGparamTimeParam,
        CutsceneGparamWeatherParam,
        CutsceneMapIdParam,
        CutSceneTextureLoadParam,
        CutsceneTimezoneConvertParam,
        CutsceneWeatherOverrideGparamConvertParam,
        DecalParam,
        DirectionCameraParam,
        EnemyCommonParam,
        EnvObjLotParam,
        EquipMtrlSetParam,
        EquipParamAccessory,
        EquipParamCustomWeapon,
        EquipParamGem,
        EquipParamGoods,
        EquipParamProtector,
        EquipParamWeapon,
        FaceParam,
        FaceRangeParam,
        FeTextEffectParam,
        FinalDamageRateParam,
        FootSfxParam,
        GameAreaParam,
        GameSystemCommonParam,
        GestureParam,
        GparamRefSettings,
        GraphicsCommonParam,
        GraphicsConfig,
        GrassLodRangeParam,
        GrassTypeParam,
        GrassTypeParam_Lv1,
        GrassTypeParam_Lv2,
        HitEffectSeParam,
        HitEffectSfxConceptParam,
        HitEffectSfxParam,
        HitMtrlParam,
        HPEstusFlaskRecoveryParam,
        ItemLotParam_enemy,
        ItemLotParam_map,
        KeyAssignMenuItemParam,
        KeyAssignParam_TypeA,
        KeyAssignParam_TypeB,
        KeyAssignParam_TypeC,
        KnockBackParam,
        KnowledgeLoadScreenItemParam,
        LegacyDistantViewPartsReplaceParam,
        LoadBalancerDrawDistScaleParam,
        LoadBalancerDrawDistScaleParam_ps4,
        LoadBalancerDrawDistScaleParam_ps5,
        LoadBalancerDrawDistScaleParam_xb1,
        LoadBalancerDrawDistScaleParam_xb1x,
        LoadBalancerDrawDistScaleParam_xss,
        LoadBalancerDrawDistScaleParam_xsx,
        LoadBalancerNewDrawDistScaleParam_ps4,
        LoadBalancerNewDrawDistScaleParam_ps5,
        LoadBalancerNewDrawDistScaleParam_win64,
        LoadBalancerNewDrawDistScaleParam_xb1,
        LoadBalancerNewDrawDistScaleParam_xb1x,
        LoadBalancerNewDrawDistScaleParam_xss,
        LoadBalancerNewDrawDistScaleParam_xsx,
        LoadBalancerParam,
        LockCamParam,
        Magic,
        MapDefaultInfoParam,
        MapGdRegionDrawParam,
        MapGdRegionInfoParam,
        MapGridCreateHeightDetailLimitInfo,
        MapGridCreateHeightLimitInfoParam,
        MapMimicryEstablishmentParam,
        MapNameTexParam,
        MapNameTexParam_m61,
        MapPieceTexParam,
        MapPieceTexParam_m61,
        MaterialExParam,
        MenuColorTableParam,
        MenuCommonParam,
        MenuOffscrRendParam,
        MenuPropertyLayoutParam,
        MenuPropertySpecParam,
        MenuValueTableParam,
        MimicryEstablishmentTexParam,
        MimicryEstablishmentTexParam_m61,
        MoveParam,
        MPEstusFlaskRecoveryParam,
        MultiHPEstusFlaskBonusParam,
        MultiMPEstusFlaskBonusParam,
        MultiPlayCorrectionParam,
        MultiSoulBonusRateParam,
        NetworkAreaParam,
        NetworkMsgParam,
        NetworkParam,
        NpcAiActionParam,
        NpcAiBehaviorProbability,
        NpcParam,
        NpcThinkParam,
        ObjActParam,
        PartsDrawParam,
        PhantomParam,
        PlayerCommonParam,
        PlayRegionParam,
        PostureControlParam_Gender,
        PostureControlParam_Pro,
        PostureControlParam_WepLeft,
        PostureControlParam_WepRight,
        RandomAppearParam,
        ReinforceParamProtector,
        ReinforceParamWeapon,
        ResistCorrectParam,
        RideParam,
        RoleParam,
        RollingObjLotParam,
        RuntimeBoneControlParam,
        SeActivationRangeParam,
        SeMaterialConvertParam,
        SfxBlockResShareParam,
        ShopLineupParam,
        ShopLineupParam_Recipe,
        SignPuddleParam,
        SignPuddleSubCategoryParam,
        SignPuddleTabParam,
        SoundAssetSoundObjEnableDistParam,
        SoundAutoEnvSoundGroupParam,
        SoundAutoReverbEvaluationDistParam,
        SoundAutoReverbSelectParam,
        SoundChrPhysicsSeParam,
        SoundCommonIngameParam,
        SoundCutsceneParam,
        SpeedtreeParam,
        SpEffectParam,
        SpEffectSetParam,
        SpEffectVfxParam,
        SwordArtsParam,
        TalkParam,
        ThrowDirectionSfxParam,
        ThrowParam,
        ToughnessParam,
        TutorialParam,
        WaypointParam,
        WeatherAssetCreateParam,
        WeatherAssetReplaceParam,
        WeatherLotParam,
        WeatherLotTexParam,
        WeatherLotTexParam_m61,
        WeatherParam,
        WepAbsorpPosParam,
        WetAspectParam,
        WhiteSignCoolTimeParam,
        WorldMapLegacyConvParam,
        WorldMapPieceParam,
        WorldMapPlaceNameParam,
        WorldMapPointParam,
        WwiseValueToStrParam_BgmBossChrIdConv,
        WwiseValueToStrParam_EnvPlaceType,
        WwiseValueToStrParam_Switch_AttackStrength,
        WwiseValueToStrParam_Switch_AttackType,
        WwiseValueToStrParam_Switch_DamageAmount,
        WwiseValueToStrParam_Switch_DeffensiveMaterial,
        WwiseValueToStrParam_Switch_GrassHitType,
        WwiseValueToStrParam_Switch_HitStop,
        WwiseValueToStrParam_Switch_OffensiveMaterial,
        WwiseValueToStrParam_Switch_PlayerEquipmentBottoms,
        WwiseValueToStrParam_Switch_PlayerEquipmentTops,
        WwiseValueToStrParam_Switch_PlayerShoes,
        WwiseValueToStrParam_Switch_PlayerVoiceType,
    }
    impl FromStr for Param {
        type Err = std::io::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "ActionButtonParam"=>Ok(Self::ActionButtonParam) ,
                "AiSoundParam"=>Ok(Self::AiSoundParam) ,
                "AssetEnvironmentGeometryParam"=>Ok(Self::AssetEnvironmentGeometryParam) ,
                "AssetMaterialSfxParam"=>Ok(Self::AssetMaterialSfxParam) ,
                "AssetModelSfxParam"=>Ok(Self::AssetModelSfxParam) ,
                "AtkParam_Npc"=>Ok(Self::AtkParam_Npc) ,
                "AtkParam_Pc"=>Ok(Self::AtkParam_Pc) ,
                "AttackElementCorrectParam"=>Ok(Self::AttackElementCorrectParam) ,
                "AutoCreateEnvSoundParam"=>Ok(Self::AutoCreateEnvSoundParam) ,
                "BaseChrSelectMenuParam"=>Ok(Self::BaseChrSelectMenuParam) ,
                "BehaviorParam"=>Ok(Self::BehaviorParam) ,
                "BehaviorParam_PC"=>Ok(Self::BehaviorParam_PC) ,
                "BonfireWarpParam"=>Ok(Self::BonfireWarpParam) ,
                "BonfireWarpSubCategoryParam"=>Ok(Self::BonfireWarpSubCategoryParam) ,
                "BonfireWarpTabParam"=>Ok(Self::BonfireWarpTabParam) ,
                "BuddyParam"=>Ok(Self::BuddyParam) ,
                "BuddyStoneParam"=>Ok(Self::BuddyStoneParam) ,
                "BudgetParam"=>Ok(Self::BudgetParam) ,
                "Bullet"=>Ok(Self::Bullet) ,
                "BulletCreateLimitParam"=>Ok(Self::BulletCreateLimitParam) ,
                "CalcCorrectGraph"=>Ok(Self::CalcCorrectGraph) ,
                "Ceremony"=>Ok(Self::Ceremony) ,
                "CharaInitParam"=>Ok(Self::CharaInitParam) ,
                "CharMakeMenuListItemParam"=>Ok(Self::CharMakeMenuListItemParam) ,
                "CharMakeMenuTopParam"=>Ok(Self::CharMakeMenuTopParam) ,
                "ChrActivateConditionParam"=>Ok(Self::ChrActivateConditionParam) ,
                "ChrEquipModelParam"=>Ok(Self::ChrEquipModelParam) ,
                "ChrModelParam"=>Ok(Self::ChrModelParam) ,
                "ClearCountCorrectParam"=>Ok(Self::ClearCountCorrectParam) ,
                "CoolTimeParam"=>Ok(Self::CoolTimeParam) ,
                "CutsceneGparamTimeParam"=>Ok(Self::CutsceneGparamTimeParam) ,
                "CutsceneGparamWeatherParam"=>Ok(Self::CutsceneGparamWeatherParam) ,
                "CutsceneMapIdParam"=>Ok(Self::CutsceneMapIdParam) ,
                "CutSceneTextureLoadParam"=>Ok(Self::CutSceneTextureLoadParam) ,
                "CutsceneTimezoneConvertParam"=>Ok(Self::CutsceneTimezoneConvertParam) ,
                "CutsceneWeatherOverrideGparamConvertParam"=>Ok(Self::CutsceneWeatherOverrideGparamConvertParam) ,
                "DecalParam"=>Ok(Self::DecalParam) ,
                "DirectionCameraParam"=>Ok(Self::DirectionCameraParam) ,
                "EnemyCommonParam"=>Ok(Self::EnemyCommonParam) ,
                "EnvObjLotParam"=>Ok(Self::EnvObjLotParam) ,
                "EquipMtrlSetParam"=>Ok(Self::EquipMtrlSetParam) ,
                "EquipParamAccessory"=>Ok(Self::EquipParamAccessory) ,
                "EquipParamCustomWeapon"=>Ok(Self::EquipParamCustomWeapon) ,
                "EquipParamGem"=>Ok(Self::EquipParamGem) ,
                "EquipParamGoods"=>Ok(Self::EquipParamGoods) ,
                "EquipParamProtector"=>Ok(Self::EquipParamProtector) ,
                "EquipParamWeapon"=>Ok(Self::EquipParamWeapon) ,
                "FaceParam"=>Ok(Self::FaceParam) ,
                "FaceRangeParam"=>Ok(Self::FaceRangeParam) ,
                "FeTextEffectParam"=>Ok(Self::FeTextEffectParam) ,
                "FinalDamageRateParam"=>Ok(Self::FinalDamageRateParam) ,
                "FootSfxParam"=>Ok(Self::FootSfxParam) ,
                "GameAreaParam"=>Ok(Self::GameAreaParam) ,
                "GameSystemCommonParam"=>Ok(Self::GameSystemCommonParam) ,
                "GestureParam"=>Ok(Self::GestureParam) ,
                "GparamRefSettings"=>Ok(Self::GparamRefSettings) ,
                "GraphicsCommonParam"=>Ok(Self::GraphicsCommonParam) ,
                "GraphicsConfig"=>Ok(Self::GraphicsConfig) ,
                "GrassLodRangeParam"=>Ok(Self::GrassLodRangeParam) ,
                "GrassTypeParam"=>Ok(Self::GrassTypeParam) ,
                "GrassTypeParam_Lv1"=>Ok(Self::GrassTypeParam_Lv1) ,
                "GrassTypeParam_Lv2"=>Ok(Self::GrassTypeParam_Lv2) ,
                "HitEffectSeParam"=>Ok(Self::HitEffectSeParam) ,
                "HitEffectSfxConceptParam"=>Ok(Self::HitEffectSfxConceptParam) ,
                "HitEffectSfxParam"=>Ok(Self::HitEffectSfxParam) ,
                "HitMtrlParam"=>Ok(Self::HitMtrlParam) ,
                "HPEstusFlaskRecoveryParam"=>Ok(Self::HPEstusFlaskRecoveryParam) ,
                "ItemLotParam_enemy"=>Ok(Self::ItemLotParam_enemy) ,
                "ItemLotParam_map"=>Ok(Self::ItemLotParam_map) ,
                "KeyAssignMenuItemParam"=>Ok(Self::KeyAssignMenuItemParam) ,
                "KeyAssignParam_TypeA"=>Ok(Self::KeyAssignParam_TypeA) ,
                "KeyAssignParam_TypeB"=>Ok(Self::KeyAssignParam_TypeB) ,
                "KeyAssignParam_TypeC"=>Ok(Self::KeyAssignParam_TypeC) ,
                "KnockBackParam"=>Ok(Self::KnockBackParam) ,
                "KnowledgeLoadScreenItemParam"=>Ok(Self::KnowledgeLoadScreenItemParam) ,
                "LegacyDistantViewPartsReplaceParam"=>Ok(Self::LegacyDistantViewPartsReplaceParam) ,
                "LoadBalancerDrawDistScaleParam"=>Ok(Self::LoadBalancerDrawDistScaleParam) ,
                "LoadBalancerDrawDistScaleParam_ps4"=>Ok(Self::LoadBalancerDrawDistScaleParam_ps4) ,
                "LoadBalancerDrawDistScaleParam_ps5"=>Ok(Self::LoadBalancerDrawDistScaleParam_ps5) ,
                "LoadBalancerDrawDistScaleParam_xb1"=>Ok(Self::LoadBalancerDrawDistScaleParam_xb1) ,
                "LoadBalancerDrawDistScaleParam_xb1x"=>Ok(Self::LoadBalancerDrawDistScaleParam_xb1x) ,
                "LoadBalancerDrawDistScaleParam_xss"=>Ok(Self::LoadBalancerDrawDistScaleParam_xss) ,
                "LoadBalancerDrawDistScaleParam_xsx"=>Ok(Self::LoadBalancerDrawDistScaleParam_xsx) ,
                "LoadBalancerNewDrawDistScaleParam_ps4"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_ps4) ,
                "LoadBalancerNewDrawDistScaleParam_ps5"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_ps5) ,
                "LoadBalancerNewDrawDistScaleParam_win64"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_win64) ,
                "LoadBalancerNewDrawDistScaleParam_xb1"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_xb1) ,
                "LoadBalancerNewDrawDistScaleParam_xb1x"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_xb1x) ,
                "LoadBalancerNewDrawDistScaleParam_xss"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_xss) ,
                "LoadBalancerNewDrawDistScaleParam_xsx"=>Ok(Self::LoadBalancerNewDrawDistScaleParam_xsx) ,
                "LoadBalancerParam"=>Ok(Self::LoadBalancerParam) ,
                "LockCamParam"=>Ok(Self::LockCamParam) ,
                "Magic"=>Ok(Self::Magic) ,
                "MapDefaultInfoParam"=>Ok(Self::MapDefaultInfoParam) ,
                "MapGdRegionDrawParam"=>Ok(Self::MapGdRegionDrawParam) ,
                "MapGdRegionInfoParam"=>Ok(Self::MapGdRegionInfoParam) ,
                "MapGridCreateHeightDetailLimitInfo"=>Ok(Self::MapGridCreateHeightDetailLimitInfo) ,
                "MapGridCreateHeightLimitInfoParam"=>Ok(Self::MapGridCreateHeightLimitInfoParam) ,
                "MapMimicryEstablishmentParam"=>Ok(Self::MapMimicryEstablishmentParam) ,
                "MapNameTexParam"=>Ok(Self::MapNameTexParam) ,
                "MapNameTexParam_m61"=>Ok(Self::MapNameTexParam_m61) ,
                "MapPieceTexParam"=>Ok(Self::MapPieceTexParam) ,
                "MapPieceTexParam_m61"=>Ok(Self::MapPieceTexParam_m61) ,
                "MaterialExParam"=>Ok(Self::MaterialExParam) ,
                "MenuColorTableParam"=>Ok(Self::MenuColorTableParam) ,
                "MenuCommonParam"=>Ok(Self::MenuCommonParam) ,
                "MenuOffscrRendParam"=>Ok(Self::MenuOffscrRendParam) ,
                "MenuPropertyLayoutParam"=>Ok(Self::MenuPropertyLayoutParam) ,
                "MenuPropertySpecParam"=>Ok(Self::MenuPropertySpecParam) ,
                "MenuValueTableParam"=>Ok(Self::MenuValueTableParam) ,
                "MimicryEstablishmentTexParam"=>Ok(Self::MimicryEstablishmentTexParam) ,
                "MimicryEstablishmentTexParam_m61"=>Ok(Self::MimicryEstablishmentTexParam_m61) ,
                "MoveParam"=>Ok(Self::MoveParam) ,
                "MPEstusFlaskRecoveryParam"=>Ok(Self::MPEstusFlaskRecoveryParam) ,
                "MultiHPEstusFlaskBonusParam"=>Ok(Self::MultiHPEstusFlaskBonusParam) ,
                "MultiMPEstusFlaskBonusParam"=>Ok(Self::MultiMPEstusFlaskBonusParam) ,
                "MultiPlayCorrectionParam"=>Ok(Self::MultiPlayCorrectionParam) ,
                "MultiSoulBonusRateParam"=>Ok(Self::MultiSoulBonusRateParam) ,
                "NetworkAreaParam"=>Ok(Self::NetworkAreaParam) ,
                "NetworkMsgParam"=>Ok(Self::NetworkMsgParam) ,
                "NetworkParam"=>Ok(Self::NetworkParam) ,
                "NpcAiActionParam"=>Ok(Self::NpcAiActionParam) ,
                "NpcAiBehaviorProbability"=>Ok(Self::NpcAiBehaviorProbability) ,
                "NpcParam"=>Ok(Self::NpcParam) ,
                "NpcThinkParam"=>Ok(Self::NpcThinkParam) ,
                "ObjActParam"=>Ok(Self::ObjActParam) ,
                "PartsDrawParam"=>Ok(Self::PartsDrawParam) ,
                "PhantomParam"=>Ok(Self::PhantomParam) ,
                "PlayerCommonParam"=>Ok(Self::PlayerCommonParam) ,
                "PlayRegionParam"=>Ok(Self::PlayRegionParam) ,
                "PostureControlParam_Gender"=>Ok(Self::PostureControlParam_Gender) ,
                "PostureControlParam_Pro"=>Ok(Self::PostureControlParam_Pro) ,
                "PostureControlParam_WepLeft"=>Ok(Self::PostureControlParam_WepLeft) ,
                "PostureControlParam_WepRight"=>Ok(Self::PostureControlParam_WepRight) ,
                "RandomAppearParam"=>Ok(Self::RandomAppearParam) ,
                "ReinforceParamProtector"=>Ok(Self::ReinforceParamProtector) ,
                "ReinforceParamWeapon"=>Ok(Self::ReinforceParamWeapon) ,
                "ResistCorrectParam"=>Ok(Self::ResistCorrectParam) ,
                "RideParam"=>Ok(Self::RideParam) ,
                "RoleParam"=>Ok(Self::RoleParam) ,
                "RollingObjLotParam"=>Ok(Self::RollingObjLotParam) ,
                "RuntimeBoneControlParam"=>Ok(Self::RuntimeBoneControlParam) ,
                "SeActivationRangeParam"=>Ok(Self::SeActivationRangeParam) ,
                "SeMaterialConvertParam"=>Ok(Self::SeMaterialConvertParam) ,
                "SfxBlockResShareParam"=>Ok(Self::SfxBlockResShareParam) ,
                "ShopLineupParam"=>Ok(Self::ShopLineupParam) ,
                "ShopLineupParam_Recipe"=>Ok(Self::ShopLineupParam_Recipe) ,
                "SignPuddleParam"=>Ok(Self::SignPuddleParam) ,
                "SignPuddleSubCategoryParam"=>Ok(Self::SignPuddleSubCategoryParam) ,
                "SignPuddleTabParam"=>Ok(Self::SignPuddleTabParam) ,
                "SoundAssetSoundObjEnableDistParam"=>Ok(Self::SoundAssetSoundObjEnableDistParam) ,
                "SoundAutoEnvSoundGroupParam"=>Ok(Self::SoundAutoEnvSoundGroupParam) ,
                "SoundAutoReverbEvaluationDistParam"=>Ok(Self::SoundAutoReverbEvaluationDistParam) ,
                "SoundAutoReverbSelectParam"=>Ok(Self::SoundAutoReverbSelectParam) ,
                "SoundChrPhysicsSeParam"=>Ok(Self::SoundChrPhysicsSeParam) ,
                "SoundCommonIngameParam"=>Ok(Self::SoundCommonIngameParam) ,
                "SoundCutsceneParam"=>Ok(Self::SoundCutsceneParam) ,
                "SpeedtreeParam"=>Ok(Self::SpeedtreeParam) ,
                "SpEffectParam"=>Ok(Self::SpEffectParam) ,
                "SpEffectSetParam"=>Ok(Self::SpEffectSetParam) ,
                "SpEffectVfxParam"=>Ok(Self::SpEffectVfxParam) ,
                "SwordArtsParam"=>Ok(Self::SwordArtsParam) ,
                "TalkParam"=>Ok(Self::TalkParam) ,
                "ThrowDirectionSfxParam"=>Ok(Self::ThrowDirectionSfxParam) ,
                "ThrowParam"=>Ok(Self::ThrowParam) ,
                "ToughnessParam"=>Ok(Self::ToughnessParam) ,
                "TutorialParam"=>Ok(Self::TutorialParam) ,
                "WaypointParam"=>Ok(Self::WaypointParam) ,
                "WeatherAssetCreateParam"=>Ok(Self::WeatherAssetCreateParam) ,
                "WeatherAssetReplaceParam"=>Ok(Self::WeatherAssetReplaceParam) ,
                "WeatherLotParam"=>Ok(Self::WeatherLotParam) ,
                "WeatherLotTexParam"=>Ok(Self::WeatherLotTexParam) ,
                "WeatherLotTexParam_m61"=>Ok(Self::WeatherLotTexParam_m61) ,
                "WeatherParam"=>Ok(Self::WeatherParam) ,
                "WepAbsorpPosParam"=>Ok(Self::WepAbsorpPosParam) ,
                "WetAspectParam"=>Ok(Self::WetAspectParam) ,
                "WhiteSignCoolTimeParam"=>Ok(Self::WhiteSignCoolTimeParam) ,
                "WorldMapLegacyConvParam"=>Ok(Self::WorldMapLegacyConvParam) ,
                "WorldMapPieceParam"=>Ok(Self::WorldMapPieceParam) ,
                "WorldMapPlaceNameParam"=>Ok(Self::WorldMapPlaceNameParam) ,
                "WorldMapPointParam"=>Ok(Self::WorldMapPointParam) ,
                "WwiseValueToStrParam_BgmBossChrIdConv"=>Ok(Self::WwiseValueToStrParam_BgmBossChrIdConv) ,
                "WwiseValueToStrParam_EnvPlaceType"=>Ok(Self::WwiseValueToStrParam_EnvPlaceType) ,
                "WwiseValueToStrParam_Switch_AttackStrength"=>Ok(Self::WwiseValueToStrParam_Switch_AttackStrength) ,
                "WwiseValueToStrParam_Switch_AttackType"=>Ok(Self::WwiseValueToStrParam_Switch_AttackType) ,
                "WwiseValueToStrParam_Switch_DamageAmount"=>Ok(Self::WwiseValueToStrParam_Switch_DamageAmount) ,
                "WwiseValueToStrParam_Switch_DeffensiveMaterial"=>Ok(Self::WwiseValueToStrParam_Switch_DeffensiveMaterial) ,
                "WwiseValueToStrParam_Switch_GrassHitType"=>Ok(Self::WwiseValueToStrParam_Switch_GrassHitType) ,
                "WwiseValueToStrParam_Switch_HitStop"=>Ok(Self::WwiseValueToStrParam_Switch_HitStop) ,
                "WwiseValueToStrParam_Switch_OffensiveMaterial"=>Ok(Self::WwiseValueToStrParam_Switch_OffensiveMaterial) ,
                "WwiseValueToStrParam_Switch_PlayerEquipmentBottoms"=>Ok(Self::WwiseValueToStrParam_Switch_PlayerEquipmentBottoms) ,
                "WwiseValueToStrParam_Switch_PlayerEquipmentTops"=>Ok(Self::WwiseValueToStrParam_Switch_PlayerEquipmentTops) ,
                "WwiseValueToStrParam_Switch_PlayerShoes"=>Ok(Self::WwiseValueToStrParam_Switch_PlayerShoes) ,
                "WwiseValueToStrParam_Switch_PlayerVoiceType"=>Ok(Self::WwiseValueToStrParam_Switch_PlayerVoiceType) ,
                _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData ,format!("Cannot recognize paramtype: {}", s)))
            }
        }
    }

    // region: Row
    #[derive(Default, Clone)]
    pub struct Row<T> where T: Default + Clone{
        pub id: u32,
        pub name: String,
        pub data: T,
        pub data_offset: i64,
    }
    impl<T> Row<T> where T: Default + Clone{
        pub fn read(br: &mut BinaryReader, param: &PARAM<T> , actual_string_offset:&mut  i64) -> Result<Row<T>, Error> {
            let mut row: Row<T> = Row::default();
            let name_offset: i64;
            if (param.format2d & FormatFlags1::LongDataOffset).as_u8() != 0 {
                row.id = br.read_u32()?;
                br.read_i32()?;
                row.data_offset = br.read_i64()?;
                name_offset = br.read_i64()?;
            }
            else {
                row.id = br.read_u32()?;
                row.data_offset = br.read_i32()? as i64;
                name_offset = br.read_i32()? as i64;
            }

            if name_offset != 0 {
                if *actual_string_offset == 0 || name_offset < *actual_string_offset {
                    *actual_string_offset = name_offset;
                }

                if (param.format2e & FormatFlags2::UnicodeRowNames).as_u8() != 0 {
                    
                    row.name = br_ext::get_utf_16(br, name_offset as usize)?;
                }
                else {
                    row.name = br_ext::get_shift_jis(br, name_offset as usize)?;
                }
            }
            
            let prev_pos = br.pos;
            br.jmp(row.data_offset as usize);
            let size = mem::size_of::<T>();
            let bytes = br.read_bytes(size)?;
            let (head, body, _tail) = unsafe { bytes.align_to::<T>() };
            assert!(head.is_empty(), "Data was not aligned");
            row.data = body[0].clone();
            br.jmp(prev_pos);

            Ok(row)
        }
    }
    // endregion

    // region: PARAM
    bitflags::bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct FormatFlags1: u8{
            const None = 0;
            const Flag01 = 0b0000_0001;
            const IntDataOffset = 0b0000_0010;
            const LongDataOffset = 0b0000_0100;
            const Flag08 = 0b0000_1000;
            const Flag10 = 0b0001_0000;
            const Flag20 = 0b0010_0000;
            const Flag40 = 0b0100_0000;
            const OffsetParamType = 0b1000_0000;
        }
    }
    #[allow(unused)]
    impl FormatFlags1 {
        pub fn as_u8(&self) -> u8 {
            self.bits()
        }
    }

    bitflags::bitflags! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct FormatFlags2: u8{
            const None = 0;
            const UnicodeRowNames = 0b0000_0001;
            const Flag02 = 0b0000_0010;
            const Flag04 = 0b0000_0100;
            const Flag08 = 0b0000_1000;
            const Flag10 = 0b0001_0000;
            const Flag20 = 0b0010_0000;
            const Flag40 = 0b0100_0000;
            const Flag80 = 0b1000_0000;
        }
    }
    #[allow(unused)]
    impl FormatFlags2 {
        pub fn as_u8(&self) -> u8 {
            self.bits()
        }
    }

    #[derive(Clone)]
    pub struct PARAM<T> where T: Default + Clone{
        pub big_endian: bool,
        pub format2d: FormatFlags1,
        pub format2e: FormatFlags2,
        pub paramdef_format_version: u8,
        pub unk06: i16,
        pub paramdef_data_version: i16,
        pub param_type: String,
        pub detected_size: i64,
        pub rows: Vec<Row<T>>,
    }
    impl<T> Default for PARAM<T> where T: Default + Clone{
        fn default() -> Self {
            Self {
                big_endian: false,
                format2d: FormatFlags1::None,
                format2e: FormatFlags2::None,
                paramdef_format_version: 0,
                unk06: 0,
                paramdef_data_version: 0,
                param_type: String::new(),
                detected_size: 0,
                rows: Vec::new(),
            }
        }
    }
    impl<T> PARAM<T> where T: Default + Clone {
        pub fn from_bytes(bytes: &[u8])  -> Result<Self, std::io::Error>{
            let mut br = BinaryReader::from_u8(bytes); 
            Self::read(&mut br)
        }

        pub fn read(br: &mut BinaryReader) -> Result<Self, std::io::Error> {
            let mut param = PARAM::default();

            br.jmp(0x2c);
            let big_endian = br.read_u8()?;
            assert!(big_endian == 0 || big_endian == 0xFF);
            param.big_endian = big_endian == 0xFF;
            br.set_endian(if param.big_endian {Endian::Big} else {Endian::Little});
            param.format2d = FormatFlags1::from_bits_truncate(br.read_u8()?);
            param.format2e = FormatFlags2::from_bits_truncate(br.read_u8()?);
            param.paramdef_format_version = br.read_u8()?;
            br.jmp(0);

            let mut actual_string_offset = 0;
            let string_offset = br.read_u32()?;
            if ((param.format2d & FormatFlags1::Flag01).as_u8() != 0) && ((param.format2d & FormatFlags1::IntDataOffset).as_u8() != 0) || ((param.format2d & FormatFlags1::LongDataOffset).as_u8() != 0) {
                assert_eq!(br.read_i16()?, 0);
            }
            else {
                br.read_i16()?;
            }
            param.unk06 = br.read_i16()?;
            param.paramdef_data_version = br.read_i16()?;
            
            let row_count = br.read_i16()?;
            if (param.format2d & FormatFlags1::OffsetParamType).as_u8() != 0 {
                assert_eq!(br.read_i32()?, 0);
                let param_type_offset = br.read_i64()?;
                assert_eq!(br.read_bytes(0x14)?, [0x0; 0x14]);
                param.param_type = br_ext::get_ascii(br, param_type_offset as usize)?;
                actual_string_offset = param_type_offset;
            }
            else {
                param.param_type = br_ext::read_fix_str(br, 0x20)?;
            }
            br.jmp(br.pos + 4);
            
            if (param.format2d & FormatFlags1::Flag01).as_u8() != 0 && (param.format2d & FormatFlags1::IntDataOffset).as_u8() != 0 {
                br.read_i32()?;
                assert_eq!(br.read_i32()?, 0);
                assert_eq!(br.read_i32()?, 0);
                assert_eq!(br.read_i32()?, 0);
            } 
            else if (param.format2d & FormatFlags1::LongDataOffset).as_u8() != 0 {
                br.read_i64()?;
                assert_eq!(br.read_i64()?, 0);
            }

            for _i in 0..row_count {
                param.rows.push(Row::read(br, &param, &mut actual_string_offset)?);
            }

            if param.rows.len() > 1 {
                param.detected_size = param.rows[1].data_offset - param.rows[0].data_offset;
            }
            else if param.rows.len() == 1 {
                param.detected_size = (if actual_string_offset == 0 {string_offset as i64} else {actual_string_offset}) - param.rows[0].data_offset;
            }
            else {
                param.detected_size = -1;
            }
            Ok(param)
        } 
    }
    // endregion
}