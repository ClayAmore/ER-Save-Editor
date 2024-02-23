pub mod books {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;
    
    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Cookbook {
        // Missionary's Cookbook
        MissionarysCookbook1,
        MissionarysCookbook2,
        MissionarysCookbook3,
        MissionarysCookbook4,
        MissionarysCookbook5,
        MissionarysCookbook6,
        MissionarysCookbook7,

        // Nomadic warrior's Cookbook
        NomadicwarriorsCookbook1,
        NomadicwarriorsCookbook2,
        NomadicwarriorsCookbook3,
        NomadicwarriorsCookbook4,
        NomadicwarriorsCookbook5,
        NomadicwarriorsCookbook6,
        NomadicwarriorsCookbook7,
        NomadicwarriorsCookbook8,
        NomadicwarriorsCookbook9,
        NomadicwarriorsCookbook10,
        NomadicwarriorsCookbook11,
        NomadicwarriorsCookbook12,
        NomadicwarriorsCookbook13,
        NomadicwarriorsCookbook14,
        NomadicwarriorsCookbook15,
        NomadicwarriorsCookbook16,
        NomadicwarriorsCookbook17,
        NomadicwarriorsCookbook18,
        NomadicwarriorsCookbook19,
        NomadicwarriorsCookbook20,
        NomadicwarriorsCookbook21,
        NomadicwarriorsCookbook22,
        NomadicwarriorsCookbook23,
        NomadicwarriorsCookbook24,

        // Armorer's Cookbook
        ArmorersCookbook1,
        ArmorersCookbook2,
        ArmorersCookbook3,
        ArmorersCookbook4,
        ArmorersCookbook5,
        ArmorersCookbook6,
        ArmorersCookbook7,

        // Ancient Dragon Apostle's Cookbook
        AncientDragonApostlesCookbook1,
        AncientDragonApostlesCookbook2,
        AncientDragonApostlesCookbook3,
        AncientDragonApostlesCookbook4,

        // Fevor's Cookbook
        FevorsCookbook1,
        FevorsCookbook2,
        FevorsCookbook3,

        // Perfumer's Cookbook
        PerfumersCookbook1,
        PerfumersCookbook2,
        PerfumersCookbook3,
        PerfumersCookbook4,

        // Glintstone Craftman's Cookbook
        GlintstoneCraftmansCookbook1,
        GlintstoneCraftmansCookbook2,
        GlintstoneCraftmansCookbook3,
        GlintstoneCraftmansCookbook4,
        GlintstoneCraftmansCookbook5,
        GlintstoneCraftmansCookbook6,
        GlintstoneCraftmansCookbook7,
        GlintstoneCraftmansCookbook8,

        // Frenzied's Cookbook
        FrenziedsCookbook1,
        FrenziedsCookbook2,
    }

    pub static COOKBOKS: Lazy<Mutex<HashMap<Cookbook,(u32,&str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            // Missionary's Cookbook
            (Cookbook::MissionarysCookbook1,(67610,"Missionary's Cookbook[1]")),
            (Cookbook::MissionarysCookbook2,(67600,"Missionary's Cookbook[2]")),
            (Cookbook::MissionarysCookbook3,(67650,"Missionary's Cookbook[3]")),
            (Cookbook::MissionarysCookbook4,(67640,"Missionary's Cookbook[4]")),
            (Cookbook::MissionarysCookbook5,(67630,"Missionary's Cookbook[5]")),
            (Cookbook::MissionarysCookbook6,(67130,"Missionary's Cookbook[6]")),
            (Cookbook::MissionarysCookbook7,(68230,"Missionary's Cookbook[7]")),

            // Nomadic warrior's Cookbook
            (Cookbook::NomadicwarriorsCookbook1,(67000,"Nomadic warrior's Cookbook[1]")),
            (Cookbook::NomadicwarriorsCookbook2,(67110,"Nomadic warrior's Cookbook[2]")),
            (Cookbook::NomadicwarriorsCookbook3,(67010,"Nomadic warrior's Cookbook[3]")),
            (Cookbook::NomadicwarriorsCookbook4,(67800,"Nomadic warrior's Cookbook[4]")),
            (Cookbook::NomadicwarriorsCookbook5,(67830,"Nomadic warrior's Cookbook[5]")),
            (Cookbook::NomadicwarriorsCookbook6,(67020,"Nomadic warrior's Cookbook[6]")),
            (Cookbook::NomadicwarriorsCookbook7,(67050,"Nomadic warrior's Cookbook[7]")),
            (Cookbook::NomadicwarriorsCookbook8,(67880,"Nomadic warrior's Cookbook[8]")),
            (Cookbook::NomadicwarriorsCookbook9,(67430,"Nomadic warrior's Cookbook[9]")),
            (Cookbook::NomadicwarriorsCookbook10,(67030,"Nomadic warrior's Cookbook1[0]")),
            (Cookbook::NomadicwarriorsCookbook11,(67220,"Nomadic warrior's Cookbook1[1]")),
            (Cookbook::NomadicwarriorsCookbook12,(67060,"Nomadic warrior's Cookbook1[2]")),
            (Cookbook::NomadicwarriorsCookbook13,(67080,"Nomadic warrior's Cookbook1[3]")),
            (Cookbook::NomadicwarriorsCookbook14,(67870,"Nomadic warrior's Cookbook1[4]")),
            (Cookbook::NomadicwarriorsCookbook15,(67900,"Nomadic warrior's Cookbook1[5]")),
            (Cookbook::NomadicwarriorsCookbook16,(67290,"Nomadic warrior's Cookbook1[6]")),
            (Cookbook::NomadicwarriorsCookbook17,(67100,"Nomadic warrior's Cookbook1[7]")),
            (Cookbook::NomadicwarriorsCookbook18,(67270,"Nomadic warrior's Cookbook1[8]")),
            (Cookbook::NomadicwarriorsCookbook19,(67070,"Nomadic warrior's Cookbook1[9]")),
            (Cookbook::NomadicwarriorsCookbook20,(67230,"Nomadic warrior's Cookbook2[0]")),
            (Cookbook::NomadicwarriorsCookbook21,(67120,"Nomadic warrior's Cookbook2[1]")),
            (Cookbook::NomadicwarriorsCookbook22,(67890,"Nomadic warrior's Cookbook2[2]")),
            (Cookbook::NomadicwarriorsCookbook23,(67090,"Nomadic warrior's Cookbook2[3]")),
            (Cookbook::NomadicwarriorsCookbook24,(67910,"Nomadic warrior's Cookbook2[4]")),

            // Armorer's Cookbook
            (Cookbook::ArmorersCookbook1,(67200,"Armorer's Cookbook[1]")),
            (Cookbook::ArmorersCookbook2,(67210,"Armorer's Cookbook[2]")),
            (Cookbook::ArmorersCookbook3,(67280,"Armorer's Cookbook[3]")),
            (Cookbook::ArmorersCookbook4,(67260,"Armorer's Cookbook[4]")),
            (Cookbook::ArmorersCookbook5,(67310,"Armorer's Cookbook[5]")),
            (Cookbook::ArmorersCookbook6,(67300,"Armorer's Cookbook[6]")),
            (Cookbook::ArmorersCookbook7,(67250,"Armorer's Cookbook[7]")),

            // Ancient Dragon Apostle's Cookbook
            (Cookbook::AncientDragonApostlesCookbook1,(68000,"Ancient Dragon Apostle's Cookbook[1]")),
            (Cookbook::AncientDragonApostlesCookbook2,(68010,"Ancient Dragon Apostle's Cookbook[2]")),
            (Cookbook::AncientDragonApostlesCookbook3,(68030,"Ancient Dragon Apostle's Cookbook[3]")),
            (Cookbook::AncientDragonApostlesCookbook4,(68020,"Ancient Dragon Apostle's Cookbook[4]")),

            // Fevor's Cookbook
            (Cookbook::FevorsCookbook1,(68200,"Fevors Cookbook[1]")),
            (Cookbook::FevorsCookbook2,(68220,"Fevors Cookbook[2]")),
            (Cookbook::FevorsCookbook3,(68210,"Fevors Cookbook[3]")),

            // Perfumer's Cookbook
            (Cookbook::PerfumersCookbook1,(67840,"Perfumer's Cookbook[1]")),
            (Cookbook::PerfumersCookbook2,(67850,"Perfumer's Cookbook[2]")),
            (Cookbook::PerfumersCookbook3,(67860,"Perfumer's Cookbook[3]")),
            (Cookbook::PerfumersCookbook4,(67920,"Perfumer's Cookbook[4]")),

            // Glintstone Craftman's Cookbook
            (Cookbook::GlintstoneCraftmansCookbook1,(67410,"Glintstone Craftman's Cookbook[1]")),
            (Cookbook::GlintstoneCraftmansCookbook2,(67450,"Glintstone Craftman's Cookbook[2]")),
            (Cookbook::GlintstoneCraftmansCookbook3,(67480,"Glintstone Craftman's Cookbook[3]")),
            (Cookbook::GlintstoneCraftmansCookbook4,(67400,"Glintstone Craftman's Cookbook[4]")),
            (Cookbook::GlintstoneCraftmansCookbook5,(67420,"Glintstone Craftman's Cookbook[5]")),
            (Cookbook::GlintstoneCraftmansCookbook6,(67460,"Glintstone Craftman's Cookbook[6]")),
            (Cookbook::GlintstoneCraftmansCookbook7,(67470,"Glintstone Craftman's Cookbook[7]")),
            (Cookbook::GlintstoneCraftmansCookbook8,(67440,"Glintstone Craftman's Cookbook[8]")),

            // Frenzied's Cookbook
            (Cookbook::FrenziedsCookbook1,(68400,"Frenzied's Cookbook[1]")),
            (Cookbook::FrenziedsCookbook2,(68410,"Frenzied's Cookbook[2]")),
        ]))
    });
}