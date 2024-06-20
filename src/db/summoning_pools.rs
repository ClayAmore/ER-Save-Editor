pub mod summoning_pools {
    use once_cell::sync::Lazy;
    use std::{collections::HashMap, sync::Mutex};

    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum SummoningPool {
        SummoningPool10000040,
        SummoningPool10000041,
        SummoningPool10000042,
        SummoningPool10000043,
        SummoningPool10000044,
        SummoningPool10000045,
        SummoningPool11000040,
        SummoningPool11000041,
        SummoningPool11000042,
        SummoningPool11000043,
        SummoningPool11000044,
        SummoningPool11050040,
        SummoningPool11050041,
        SummoningPool12010040,
        SummoningPool12010041,
        SummoningPool12010042,
        SummoningPool12010043,
        SummoningPool12010044,
        SummoningPool12010045,
        SummoningPool12010046,
        SummoningPool12020040,
        SummoningPool12020041,
        SummoningPool12020042,
        SummoningPool12020043,
        SummoningPool12020044,
        SummoningPool12020045,
        SummoningPool12020046,
        SummoningPool12030040,
        SummoningPool12030041,
        SummoningPool12030042,
        SummoningPool12030043,
        SummoningPool12030044,
        SummoningPool12050040,
        SummoningPool12050041,
        SummoningPool12050042,
        SummoningPool12070040,
        SummoningPool12070041,
        SummoningPool13000040,
        SummoningPool13000041,
        SummoningPool13000042,
        SummoningPool13000043,
        SummoningPool13000044,
        SummoningPool13000045,
        SummoningPool13000046,
        SummoningPool13000047,
        SummoningPool14000040,
        SummoningPool14000041,
        SummoningPool14000042,
        SummoningPool14000043,
        SummoningPool15000040,
        SummoningPool15000041,
        SummoningPool15000042,
        SummoningPool15000044,
        SummoningPool15000045,
        SummoningPool15000046,
        SummoningPool15000047,
        SummoningPool15000049,
        SummoningPool16000040,
        SummoningPool16000041,
        SummoningPool16000042,
        SummoningPool16000043,
        SummoningPool16000044,
        SummoningPool19000040,
        SummoningPool30000040,
        SummoningPool30010040,
        SummoningPool30020040,
        SummoningPool30110040,
        SummoningPool30040040,
        SummoningPool30050040,
        SummoningPool30030040,
        SummoningPool30060040,
        SummoningPool30080040,
        SummoningPool30090040,
        SummoningPool30100040,
        SummoningPool30120040,
        SummoningPool30070040,
        SummoningPool30140040,
        SummoningPool30150040,
        SummoningPool30160040,
        SummoningPool30170040,
        SummoningPool30180040,
        SummoningPool30190040,
        SummoningPool30200040,
        SummoningPool31020040,
        SummoningPool31010040,
        SummoningPool31000040,
        SummoningPool31030040,
        SummoningPool31150040,
        SummoningPool31170040,
        SummoningPool31040040,
        SummoningPool31050040,
        SummoningPool31060040,
        SummoningPool31070040,
        SummoningPool31090040,
        SummoningPool31180040,
        SummoningPool31190040,
        SummoningPool31210040,
        SummoningPool31100040,
        SummoningPool31200040,
        SummoningPool31110040,
        SummoningPool31120040,
        SummoningPool31220040,
        SummoningPool32000040,
        SummoningPool32010040,
        SummoningPool32020040,
        SummoningPool32040040,
        SummoningPool32050040,
        SummoningPool32070040,
        SummoningPool32080040,
        SummoningPool32110040,
        SummoningPool34100040,
        SummoningPool34110040,
        SummoningPool34120040,
        SummoningPool34120041,
        SummoningPool34130040,
        SummoningPool35000040,
        SummoningPool35000041,
        SummoningPool35000042,
        SummoningPool39200040,
        SummoningPool39200041,
        SummoningPool1060410040,
        SummoningPool1060420040,
        SummoningPool1060430040,
        SummoningPool1060430041,
        SummoningPool1060430042,
        SummoningPool1060430043,
        SummoningPool1060440040,
        SummoningPool1060330040,
        SummoningPool1060340040,
        SummoningPool1060340041,
        SummoningPool1060340043,
        SummoningPool1060350040,
        SummoningPool1060380040,
        SummoningPool1035530040,
        SummoningPool1036520040,
        SummoningPool1036540040,
        SummoningPool1036540041,
        SummoningPool1037530040,
        SummoningPool1038520040,
        SummoningPool1039540040,
        SummoningPool1040530040,
        SummoningPool1042540040,
        SummoningPool1044530040,
        SummoningPool1045520040,
        SummoningPool1046400040,
        SummoningPool1047400040,
        SummoningPool1048370040,
        SummoningPool1049380040,
        SummoningPool1049380041,
        SummoningPool1050400040,
        SummoningPool1051360040,
        SummoningPool1051370040,
        SummoningPool1051400040,
        SummoningPool1052410040,
        SummoningPool1047510840,
        SummoningPool1053570840,
        SummoningPool1052530840,
        SummoningPool1052570840,
        SummoningPool1051570840,
        SummoningPool1051570841,
        SummoningPool1049560840,
        SummoningPool1049570840,
    }

    pub static SUMMONING_POOLS: Lazy<Mutex<HashMap<SummoningPool, (u32, &str)>>> =
        Lazy::new(|| {
            Mutex::new(HashMap::from([
                (
                    SummoningPool::SummoningPool10000040,
                    (10000040, "Name_10000040"),
                ),
                (
                    SummoningPool::SummoningPool10000041,
                    (10000041, "Name_10000041"),
                ),
                (
                    SummoningPool::SummoningPool10000042,
                    (10000042, "Name_10000042"),
                ),
                (
                    SummoningPool::SummoningPool10000043,
                    (10000043, "Name_10000043"),
                ),
                (
                    SummoningPool::SummoningPool10000044,
                    (10000044, "Name_10000044"),
                ),
                (
                    SummoningPool::SummoningPool10000045,
                    (10000045, "Name_10000045"),
                ),
                (
                    SummoningPool::SummoningPool11000040,
                    (11000040, "Name_11000040"),
                ),
                (
                    SummoningPool::SummoningPool11000041,
                    (11000041, "Name_11000041"),
                ),
                (
                    SummoningPool::SummoningPool11000042,
                    (11000042, "Name_11000042"),
                ),
                (
                    SummoningPool::SummoningPool11000043,
                    (11000043, "Name_11000043"),
                ),
                (
                    SummoningPool::SummoningPool11000044,
                    (11000044, "Name_11000044"),
                ),
                (
                    SummoningPool::SummoningPool11050040,
                    (11050040, "Name_11050040"),
                ),
                (
                    SummoningPool::SummoningPool11050041,
                    (11050041, "Name_11050041"),
                ),
                (
                    SummoningPool::SummoningPool12010040,
                    (12010040, "Name_12010040"),
                ),
                (
                    SummoningPool::SummoningPool12010041,
                    (12010041, "Name_12010041"),
                ),
                (
                    SummoningPool::SummoningPool12010042,
                    (12010042, "Name_12010042"),
                ),
                (
                    SummoningPool::SummoningPool12010043,
                    (12010043, "Name_12010043"),
                ),
                (
                    SummoningPool::SummoningPool12010044,
                    (12010044, "Name_12010044"),
                ),
                (
                    SummoningPool::SummoningPool12010045,
                    (12010045, "Name_12010045"),
                ),
                (
                    SummoningPool::SummoningPool12010046,
                    (12010046, "Name_12010046"),
                ),
                (
                    SummoningPool::SummoningPool12020040,
                    (12020040, "Name_12020040"),
                ),
                (
                    SummoningPool::SummoningPool12020041,
                    (12020041, "Name_12020041"),
                ),
                (
                    SummoningPool::SummoningPool12020042,
                    (12020042, "Name_12020042"),
                ),
                (
                    SummoningPool::SummoningPool12020043,
                    (12020043, "Name_12020043"),
                ),
                (
                    SummoningPool::SummoningPool12020044,
                    (12020044, "Name_12020044"),
                ),
                (
                    SummoningPool::SummoningPool12020045,
                    (12020045, "Name_12020045"),
                ),
                (
                    SummoningPool::SummoningPool12020046,
                    (12020046, "Name_12020046"),
                ),
                (
                    SummoningPool::SummoningPool12030040,
                    (12030040, "Name_12030040"),
                ),
                (
                    SummoningPool::SummoningPool12030041,
                    (12030041, "Name_12030041"),
                ),
                (
                    SummoningPool::SummoningPool12030042,
                    (12030042, "Name_12030042"),
                ),
                (
                    SummoningPool::SummoningPool12030043,
                    (12030043, "Name_12030043"),
                ),
                (
                    SummoningPool::SummoningPool12030044,
                    (12030044, "Name_12030044"),
                ),
                (
                    SummoningPool::SummoningPool12050040,
                    (12050040, "Name_12050040"),
                ),
                (
                    SummoningPool::SummoningPool12050041,
                    (12050041, "Name_12050041"),
                ),
                (
                    SummoningPool::SummoningPool12050042,
                    (12050042, "Name_12050042"),
                ),
                (
                    SummoningPool::SummoningPool12070040,
                    (12070040, "Name_12070040"),
                ),
                (
                    SummoningPool::SummoningPool12070041,
                    (12070041, "Name_12070041"),
                ),
                (
                    SummoningPool::SummoningPool13000040,
                    (13000040, "Name_13000040"),
                ),
                (
                    SummoningPool::SummoningPool13000041,
                    (13000041, "Name_13000041"),
                ),
                (
                    SummoningPool::SummoningPool13000042,
                    (13000042, "Name_13000042"),
                ),
                (
                    SummoningPool::SummoningPool13000043,
                    (13000043, "Name_13000043"),
                ),
                (
                    SummoningPool::SummoningPool13000044,
                    (13000044, "Name_13000044"),
                ),
                (
                    SummoningPool::SummoningPool13000045,
                    (13000045, "Name_13000045"),
                ),
                (
                    SummoningPool::SummoningPool13000046,
                    (13000046, "Name_13000046"),
                ),
                (
                    SummoningPool::SummoningPool13000047,
                    (13000047, "Name_13000047"),
                ),
                (
                    SummoningPool::SummoningPool14000040,
                    (14000040, "Name_14000040"),
                ),
                (
                    SummoningPool::SummoningPool14000041,
                    (14000041, "Name_14000041"),
                ),
                (
                    SummoningPool::SummoningPool14000042,
                    (14000042, "Name_14000042"),
                ),
                (
                    SummoningPool::SummoningPool14000043,
                    (14000043, "Name_14000043"),
                ),
                (
                    SummoningPool::SummoningPool15000040,
                    (15000040, "Name_15000040"),
                ),
                (
                    SummoningPool::SummoningPool15000041,
                    (15000041, "Name_15000041"),
                ),
                (
                    SummoningPool::SummoningPool15000042,
                    (15000042, "Name_15000042"),
                ),
                (
                    SummoningPool::SummoningPool15000044,
                    (15000044, "Name_15000044"),
                ),
                (
                    SummoningPool::SummoningPool15000045,
                    (15000045, "Name_15000045"),
                ),
                (
                    SummoningPool::SummoningPool15000046,
                    (15000046, "Name_15000046"),
                ),
                (
                    SummoningPool::SummoningPool15000047,
                    (15000047, "Name_15000047"),
                ),
                (
                    SummoningPool::SummoningPool15000049,
                    (15000049, "Name_15000049"),
                ),
                (
                    SummoningPool::SummoningPool16000040,
                    (16000040, "Name_16000040"),
                ),
                (
                    SummoningPool::SummoningPool16000041,
                    (16000041, "Name_16000041"),
                ),
                (
                    SummoningPool::SummoningPool16000042,
                    (16000042, "Name_16000042"),
                ),
                (
                    SummoningPool::SummoningPool16000043,
                    (16000043, "Name_16000043"),
                ),
                (
                    SummoningPool::SummoningPool16000044,
                    (16000044, "Name_16000044"),
                ),
                (
                    SummoningPool::SummoningPool19000040,
                    (19000040, "Name_19000040"),
                ),
                (
                    SummoningPool::SummoningPool30000040,
                    (30000040, "Name_30000040"),
                ),
                (
                    SummoningPool::SummoningPool30010040,
                    (30010040, "Name_30010040"),
                ),
                (
                    SummoningPool::SummoningPool30020040,
                    (30020040, "Name_30020040"),
                ),
                (
                    SummoningPool::SummoningPool30110040,
                    (30110040, "Name_30110040"),
                ),
                (
                    SummoningPool::SummoningPool30040040,
                    (30040040, "Name_30040040"),
                ),
                (
                    SummoningPool::SummoningPool30050040,
                    (30050040, "Name_30050040"),
                ),
                (
                    SummoningPool::SummoningPool30030040,
                    (30030040, "Name_30030040"),
                ),
                (
                    SummoningPool::SummoningPool30060040,
                    (30060040, "Name_30060040"),
                ),
                (
                    SummoningPool::SummoningPool30080040,
                    (30080040, "Name_30080040"),
                ),
                (
                    SummoningPool::SummoningPool30090040,
                    (30090040, "Name_30090040"),
                ),
                (
                    SummoningPool::SummoningPool30100040,
                    (30100040, "Name_30100040"),
                ),
                (
                    SummoningPool::SummoningPool30120040,
                    (30120040, "Name_30120040"),
                ),
                (
                    SummoningPool::SummoningPool30070040,
                    (30070040, "Name_30070040"),
                ),
                (
                    SummoningPool::SummoningPool30140040,
                    (30140040, "Name_30140040"),
                ),
                (
                    SummoningPool::SummoningPool30150040,
                    (30150040, "Name_30150040"),
                ),
                (
                    SummoningPool::SummoningPool30160040,
                    (30160040, "Name_30160040"),
                ),
                (
                    SummoningPool::SummoningPool30170040,
                    (30170040, "Name_30170040"),
                ),
                (
                    SummoningPool::SummoningPool30180040,
                    (30180040, "Name_30180040"),
                ),
                (
                    SummoningPool::SummoningPool30190040,
                    (30190040, "Name_30190040"),
                ),
                (
                    SummoningPool::SummoningPool30200040,
                    (30200040, "Name_30200040"),
                ),
                (
                    SummoningPool::SummoningPool31020040,
                    (31020040, "Name_31020040"),
                ),
                (
                    SummoningPool::SummoningPool31010040,
                    (31010040, "Name_31010040"),
                ),
                (
                    SummoningPool::SummoningPool31000040,
                    (31000040, "Name_31000040"),
                ),
                (
                    SummoningPool::SummoningPool31030040,
                    (31030040, "Name_31030040"),
                ),
                (
                    SummoningPool::SummoningPool31150040,
                    (31150040, "Name_31150040"),
                ),
                (
                    SummoningPool::SummoningPool31170040,
                    (31170040, "Name_31170040"),
                ),
                (
                    SummoningPool::SummoningPool31040040,
                    (31040040, "Name_31040040"),
                ),
                (
                    SummoningPool::SummoningPool31050040,
                    (31050040, "Name_31050040"),
                ),
                (
                    SummoningPool::SummoningPool31060040,
                    (31060040, "Name_31060040"),
                ),
                (
                    SummoningPool::SummoningPool31070040,
                    (31070040, "Name_31070040"),
                ),
                (
                    SummoningPool::SummoningPool31090040,
                    (31090040, "Name_31090040"),
                ),
                (
                    SummoningPool::SummoningPool31180040,
                    (31180040, "Name_31180040"),
                ),
                (
                    SummoningPool::SummoningPool31190040,
                    (31190040, "Name_31190040"),
                ),
                (
                    SummoningPool::SummoningPool31210040,
                    (31210040, "Name_31210040"),
                ),
                (
                    SummoningPool::SummoningPool31100040,
                    (31100040, "Name_31100040"),
                ),
                (
                    SummoningPool::SummoningPool31200040,
                    (31200040, "Name_31200040"),
                ),
                (
                    SummoningPool::SummoningPool31110040,
                    (31110040, "Name_31110040"),
                ),
                (
                    SummoningPool::SummoningPool31120040,
                    (31120040, "Name_31120040"),
                ),
                (
                    SummoningPool::SummoningPool31220040,
                    (31220040, "Name_31220040"),
                ),
                (
                    SummoningPool::SummoningPool32000040,
                    (32000040, "Name_32000040"),
                ),
                (
                    SummoningPool::SummoningPool32010040,
                    (32010040, "Name_32010040"),
                ),
                (
                    SummoningPool::SummoningPool32020040,
                    (32020040, "Name_32020040"),
                ),
                (
                    SummoningPool::SummoningPool32040040,
                    (32040040, "Name_32040040"),
                ),
                (
                    SummoningPool::SummoningPool32050040,
                    (32050040, "Name_32050040"),
                ),
                (
                    SummoningPool::SummoningPool32070040,
                    (32070040, "Name_32070040"),
                ),
                (
                    SummoningPool::SummoningPool32080040,
                    (32080040, "Name_32080040"),
                ),
                (
                    SummoningPool::SummoningPool32110040,
                    (32110040, "Name_32110040"),
                ),
                (
                    SummoningPool::SummoningPool34100040,
                    (34100040, "Name_34100040"),
                ),
                (
                    SummoningPool::SummoningPool34110040,
                    (34110040, "Name_34110040"),
                ),
                (
                    SummoningPool::SummoningPool34120040,
                    (34120040, "Name_34120040"),
                ),
                (
                    SummoningPool::SummoningPool34120041,
                    (34120041, "Name_34120041"),
                ),
                (
                    SummoningPool::SummoningPool34130040,
                    (34130040, "Name_34130040"),
                ),
                (
                    SummoningPool::SummoningPool35000040,
                    (35000040, "Name_35000040"),
                ),
                (
                    SummoningPool::SummoningPool35000041,
                    (35000041, "Name_35000041"),
                ),
                (
                    SummoningPool::SummoningPool35000042,
                    (35000042, "Name_35000042"),
                ),
                (
                    SummoningPool::SummoningPool39200040,
                    (39200040, "Name_39200040"),
                ),
                (
                    SummoningPool::SummoningPool39200041,
                    (39200041, "Name_39200041"),
                ),
                (
                    SummoningPool::SummoningPool1060410040,
                    (1060410040, "Name_1060410040"),
                ),
                (
                    SummoningPool::SummoningPool1060420040,
                    (1060420040, "Name_1060420040"),
                ),
                (
                    SummoningPool::SummoningPool1060430040,
                    (1060430040, "Name_1060430040"),
                ),
                (
                    SummoningPool::SummoningPool1060430041,
                    (1060430041, "Name_1060430041"),
                ),
                (
                    SummoningPool::SummoningPool1060430042,
                    (1060430042, "Name_1060430042"),
                ),
                (
                    SummoningPool::SummoningPool1060430043,
                    (1060430043, "Name_1060430043"),
                ),
                (
                    SummoningPool::SummoningPool1060440040,
                    (1060440040, "Name_1060440040"),
                ),
                (
                    SummoningPool::SummoningPool1060330040,
                    (1060330040, "Name_1060330040"),
                ),
                (
                    SummoningPool::SummoningPool1060340040,
                    (1060340040, "Name_1060340040"),
                ),
                (
                    SummoningPool::SummoningPool1060340041,
                    (1060340041, "Name_1060340041"),
                ),
                (
                    SummoningPool::SummoningPool1060340043,
                    (1060340043, "Name_1060340043"),
                ),
                (
                    SummoningPool::SummoningPool1060350040,
                    (1060350040, "Name_1060350040"),
                ),
                (
                    SummoningPool::SummoningPool1060380040,
                    (1060380040, "Name_1060380040"),
                ),
                (
                    SummoningPool::SummoningPool1035530040,
                    (1035530040, "Name_1035530040"),
                ),
                (
                    SummoningPool::SummoningPool1036520040,
                    (1036520040, "Name_1036520040"),
                ),
                (
                    SummoningPool::SummoningPool1036540040,
                    (1036540040, "Name_1036540040"),
                ),
                (
                    SummoningPool::SummoningPool1036540041,
                    (1036540041, "Name_1036540041"),
                ),
                (
                    SummoningPool::SummoningPool1037530040,
                    (1037530040, "Name_1037530040"),
                ),
                (
                    SummoningPool::SummoningPool1038520040,
                    (1038520040, "Name_1038520040"),
                ),
                (
                    SummoningPool::SummoningPool1039540040,
                    (1039540040, "Name_1039540040"),
                ),
                (
                    SummoningPool::SummoningPool1040530040,
                    (1040530040, "Name_1040530040"),
                ),
                (
                    SummoningPool::SummoningPool1042540040,
                    (1042540040, "Name_1042540040"),
                ),
                (
                    SummoningPool::SummoningPool1044530040,
                    (1044530040, "Name_1044530040"),
                ),
                (
                    SummoningPool::SummoningPool1045520040,
                    (1045520040, "Name_1045520040"),
                ),
                (
                    SummoningPool::SummoningPool1046400040,
                    (1046400040, "Name_1046400040"),
                ),
                (
                    SummoningPool::SummoningPool1047400040,
                    (1047400040, "Name_1047400040"),
                ),
                (
                    SummoningPool::SummoningPool1048370040,
                    (1048370040, "Name_1048370040"),
                ),
                (
                    SummoningPool::SummoningPool1049380040,
                    (1049380040, "Name_1049380040"),
                ),
                (
                    SummoningPool::SummoningPool1049380041,
                    (1049380041, "Name_1049380041"),
                ),
                (
                    SummoningPool::SummoningPool1050400040,
                    (1050400040, "Name_1050400040"),
                ),
                (
                    SummoningPool::SummoningPool1051360040,
                    (1051360040, "Name_1051360040"),
                ),
                (
                    SummoningPool::SummoningPool1051370040,
                    (1051370040, "Name_1051370040"),
                ),
                (
                    SummoningPool::SummoningPool1051400040,
                    (1051400040, "Name_1051400040"),
                ),
                (
                    SummoningPool::SummoningPool1052410040,
                    (1052410040, "Name_1052410040"),
                ),
                (
                    SummoningPool::SummoningPool1047510840,
                    (1047510840, "Name_1047510840"),
                ),
                (
                    SummoningPool::SummoningPool1053570840,
                    (1053570840, "Name_1053570840"),
                ),
                (
                    SummoningPool::SummoningPool1052530840,
                    (1052530840, "Name_1052530840"),
                ),
                (
                    SummoningPool::SummoningPool1052570840,
                    (1052570840, "Name_1052570840"),
                ),
                (
                    SummoningPool::SummoningPool1051570840,
                    (1051570840, "Name_1051570840"),
                ),
                (
                    SummoningPool::SummoningPool1051570841,
                    (1051570841, "Name_1051570841"),
                ),
                (
                    SummoningPool::SummoningPool1049560840,
                    (1049560840, "Name_1049560840"),
                ),
                (
                    SummoningPool::SummoningPool1049570840,
                    (1049570840, "Name_1049570840"),
                ),
            ]))
        });
}
