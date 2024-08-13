use er_save_lib::SaveApi;

use crate::db::classes::ArcheType;

#[allow(unused)]
#[derive(Clone, Default)]
pub struct StatsViewModel {
    pub arche_type: ArcheType,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub runes: u32,
    pub runes_memory: u32,
}

impl StatsViewModel {
    pub fn from_save(index: usize, save_api: &SaveApi) -> Self {
        let arche_type = ArcheType::from(save_api.archetype(index));
        let vigor = save_api.vigor(index);
        let mind = save_api.mind(index);
        let endurance = save_api.endurance(index);
        let strength = save_api.strength(index);
        let dexterity = save_api.dexterity(index);
        let intelligence = save_api.intelligence(index);
        let faith = save_api.faith(index);
        let arcane = save_api.arcane(index);
        let runes = save_api.runes(index);
        let runes_memory = save_api.runes_memory(index);

        Self {
            arche_type,
            vigor,
            mind,
            endurance,
            strength,
            dexterity,
            intelligence,
            faith,
            arcane,
            runes,
            runes_memory,
        }
    }
}
