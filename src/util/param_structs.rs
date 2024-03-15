#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ACTIONBUTTON_PARAM_ST {
	pub regionType: u8,
	pub category: u8,
	pub padding1: [u8;2],
	pub dummyPoly1: i32,
	pub dummyPoly2: i32,
	pub radius: f32,
	pub angle: i32,
	pub depth: f32,
	pub width: f32,
	pub height: f32,
	pub baseHeightOffset: f32,
	pub angleCheckType: u8,
	pub padding2: [u8;3],
	pub allowAngle: i32,
	pub spotDummyPoly: i32,
	pub textBoxType: u8,
	pub padding3: [u8;2],
	bits_0: u8,
	pub textId: i32,
	pub invalidFlag: i32,
	pub grayoutFlag: i32,
	pub overrideActionButtonIdForRide: i32,
	pub execInvalidTime: f32,
	pub padding6: [u8;28],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ACTIONBUTTON_PARAM_ST {
	pub fn padding5(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isInvalidForRide(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isGrayoutForRide(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn isInvalidForCrouching(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn isGrayoutForCrouching(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn padding4(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
}
impl Default for ACTIONBUTTON_PARAM_ST {
	fn default() -> Self {
		Self {
			 regionType: 0,
			 category: 0,
			 padding1: [0;2],
			 dummyPoly1: -1,
			 dummyPoly2: -1,
			 radius: 0.,
			 angle: 180,
			 depth: 0.,
			 width: 0.,
			 height: 0.,
			 baseHeightOffset: 0.,
			 angleCheckType: 0,
			 padding2: [0;3],
			 allowAngle: 180,
			 spotDummyPoly: -1,
			 textBoxType: 0,
			 padding3: [0;2],
			 bits_0: 0,
			 textId: -1,
			 invalidFlag: 0,
			 grayoutFlag: 0,
			 overrideActionButtonIdForRide: -1,
			 execInvalidTime: 0.,
			 padding6: [0;28],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct AI_ANIM_TBL_PARAM {
	pub atk0_EzStateId: i16,
	pub atk1_EzStateId: i16,
	pub atk2_EzStateId: i16,
	pub atk3_EzStateId: i16,
	pub atk4_EzStateId: i16,
	pub atk5_EzStateId: i16,
	pub atk6_EzStateId: i16,
	pub atk7_EzStateId: i16,
	pub atk8_EzStateId: i16,
	pub atk9_EzStateId: i16,
	pub atk10_EzStateId: i16,
	pub atk11_EzStateId: i16,
	pub atk12_EzStateId: i16,
	pub atk13_EzStateId: i16,
	pub atk14_EzStateId: i16,
	pub atk15_EzStateId: i16,
	pub atk16_EzStateId: i16,
	pub atk17_EzStateId: i16,
	pub atk18_EzStateId: i16,
	pub atk19_EzStateId: i16,
	pub atk20_EzStateId: i16,
	pub atk21_EzStateId: i16,
	pub atk22_EzStateId: i16,
	pub atk23_EzStateId: i16,
	pub atk24_EzStateId: i16,
	pub atk25_EzStateId: i16,
	pub atk26_EzStateId: i16,
	pub atk27_EzStateId: i16,
	pub atk28_EzStateId: i16,
	pub atk29_EzStateId: i16,
	pub atk0_MinDist: i16,
	pub atk1_MinDist: i16,
	pub atk2_MinDist: i16,
	pub atk3_MinDist: i16,
	pub atk4_MinDist: i16,
	pub atk5_MinDist: i16,
	pub atk6_MinDist: i16,
	pub atk7_MinDist: i16,
	pub atk8_MinDist: i16,
	pub atk9_MinDist: i16,
	pub atk10_MinDist: i16,
	pub atk11_MinDist: i16,
	pub atk12_MinDist: i16,
	pub atk13_MinDist: i16,
	pub atk14_MinDist: i16,
	pub atk15_MinDist: i16,
	pub atk16_MinDist: i16,
	pub atk17_MinDist: i16,
	pub atk18_MinDist: i16,
	pub atk19_MinDist: i16,
	pub atk20_MinDist: i16,
	pub atk21_MinDist: i16,
	pub atk22_MinDist: i16,
	pub atk23_MinDist: i16,
	pub atk24_MinDist: i16,
	pub atk25_MinDist: i16,
	pub atk26_MinDist: i16,
	pub atk27_MinDist: i16,
	pub atk28_MinDist: i16,
	pub atk29_MinDist: i16,
	pub atk0_MaxDist: i16,
	pub atk1_MaxDist: i16,
	pub atk2_MaxDist: i16,
	pub atk3_MaxDist: i16,
	pub atk4_MaxDist: i16,
	pub atk5_MaxDist: i16,
	pub atk6_MaxDist: i16,
	pub atk7_MaxDist: i16,
	pub atk8_MaxDist: i16,
	pub atk9_MaxDist: i16,
	pub atk10_MaxDist: i16,
	pub atk11_MaxDist: i16,
	pub atk12_MaxDist: i16,
	pub atk13_MaxDist: i16,
	pub atk14_MaxDist: i16,
	pub atk15_MaxDist: i16,
	pub atk16_MaxDist: i16,
	pub atk17_MaxDist: i16,
	pub atk18_MaxDist: i16,
	pub atk19_MaxDist: i16,
	pub atk20_MaxDist: i16,
	pub atk21_MaxDist: i16,
	pub atk22_MaxDist: i16,
	pub atk23_MaxDist: i16,
	pub atk24_MaxDist: i16,
	pub atk25_MaxDist: i16,
	pub atk26_MaxDist: i16,
	pub atk27_MaxDist: i16,
	pub atk28_MaxDist: i16,
	pub atk29_MaxDist: i16,
	bits_0: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	bits_4: u8,
	bits_5: u8,
	bits_6: u8,
	bits_7: u8,
	bits_8: u8,
	bits_9: u8,
	bits_10: u8,
	bits_11: u8,
	bits_12: u8,
	bits_13: u8,
	bits_14: u8,
	pub pad0: [u8;13],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl AI_ANIM_TBL_PARAM {
	pub fn atk0_AtkDistType(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn atk1_AtkDistType(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn atk2_AtkDistType(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn atk3_AtkDistType(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn atk4_AtkDistType(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn atk5_AtkDistType(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn atk6_AtkDistType(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn atk7_AtkDistType(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn atk8_AtkDistType(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn atk9_AtkDistType(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn atk10_AtkDistType(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn atk11_AtkDistType(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn atk12_AtkDistType(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn atk13_AtkDistType(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn atk14_AtkDistType(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn atk15_AtkDistType(&self) -> bool {	
			self.bits_7 & (1 << 4) != 0
	}
	pub fn atk16_AtkDistType(&self) -> bool {	
			self.bits_8 & (1 << 0) != 0
	}
	pub fn atk17_AtkDistType(&self) -> bool {	
			self.bits_8 & (1 << 4) != 0
	}
	pub fn atk18_AtkDistType(&self) -> bool {	
			self.bits_9 & (1 << 0) != 0
	}
	pub fn atk19_AtkDistType(&self) -> bool {	
			self.bits_9 & (1 << 4) != 0
	}
	pub fn atk20_AtkDistType(&self) -> bool {	
			self.bits_10 & (1 << 0) != 0
	}
	pub fn atk21_AtkDistType(&self) -> bool {	
			self.bits_10 & (1 << 4) != 0
	}
	pub fn atk22_AtkDistType(&self) -> bool {	
			self.bits_11 & (1 << 0) != 0
	}
	pub fn atk23_AtkDistType(&self) -> bool {	
			self.bits_11 & (1 << 4) != 0
	}
	pub fn atk24_AtkDistType(&self) -> bool {	
			self.bits_12 & (1 << 0) != 0
	}
	pub fn atk25_AtkDistType(&self) -> bool {	
			self.bits_12 & (1 << 4) != 0
	}
	pub fn atk26_AtkDistType(&self) -> bool {	
			self.bits_13 & (1 << 0) != 0
	}
	pub fn atk27_AtkDistType(&self) -> bool {	
			self.bits_13 & (1 << 4) != 0
	}
	pub fn atk28_AtkDistType(&self) -> bool {	
			self.bits_14 & (1 << 0) != 0
	}
	pub fn atk29_AtkDistType(&self) -> bool {	
			self.bits_14 & (1 << 4) != 0
	}
}
impl Default for AI_ANIM_TBL_PARAM {
	fn default() -> Self {
		Self {
			 atk0_EzStateId: 0,
			 atk1_EzStateId: 0,
			 atk2_EzStateId: 0,
			 atk3_EzStateId: 0,
			 atk4_EzStateId: 0,
			 atk5_EzStateId: 0,
			 atk6_EzStateId: 0,
			 atk7_EzStateId: 0,
			 atk8_EzStateId: 0,
			 atk9_EzStateId: 0,
			 atk10_EzStateId: 0,
			 atk11_EzStateId: 0,
			 atk12_EzStateId: 0,
			 atk13_EzStateId: 0,
			 atk14_EzStateId: 0,
			 atk15_EzStateId: 0,
			 atk16_EzStateId: 0,
			 atk17_EzStateId: 0,
			 atk18_EzStateId: 0,
			 atk19_EzStateId: 0,
			 atk20_EzStateId: 0,
			 atk21_EzStateId: 0,
			 atk22_EzStateId: 0,
			 atk23_EzStateId: 0,
			 atk24_EzStateId: 0,
			 atk25_EzStateId: 0,
			 atk26_EzStateId: 0,
			 atk27_EzStateId: 0,
			 atk28_EzStateId: 0,
			 atk29_EzStateId: 0,
			 atk0_MinDist: 0,
			 atk1_MinDist: 0,
			 atk2_MinDist: 0,
			 atk3_MinDist: 0,
			 atk4_MinDist: 0,
			 atk5_MinDist: 0,
			 atk6_MinDist: 0,
			 atk7_MinDist: 0,
			 atk8_MinDist: 0,
			 atk9_MinDist: 0,
			 atk10_MinDist: 0,
			 atk11_MinDist: 0,
			 atk12_MinDist: 0,
			 atk13_MinDist: 0,
			 atk14_MinDist: 0,
			 atk15_MinDist: 0,
			 atk16_MinDist: 0,
			 atk17_MinDist: 0,
			 atk18_MinDist: 0,
			 atk19_MinDist: 0,
			 atk20_MinDist: 0,
			 atk21_MinDist: 0,
			 atk22_MinDist: 0,
			 atk23_MinDist: 0,
			 atk24_MinDist: 0,
			 atk25_MinDist: 0,
			 atk26_MinDist: 0,
			 atk27_MinDist: 0,
			 atk28_MinDist: 0,
			 atk29_MinDist: 0,
			 atk0_MaxDist: 0,
			 atk1_MaxDist: 0,
			 atk2_MaxDist: 0,
			 atk3_MaxDist: 0,
			 atk4_MaxDist: 0,
			 atk5_MaxDist: 0,
			 atk6_MaxDist: 0,
			 atk7_MaxDist: 0,
			 atk8_MaxDist: 0,
			 atk9_MaxDist: 0,
			 atk10_MaxDist: 0,
			 atk11_MaxDist: 0,
			 atk12_MaxDist: 0,
			 atk13_MaxDist: 0,
			 atk14_MaxDist: 0,
			 atk15_MaxDist: 0,
			 atk16_MaxDist: 0,
			 atk17_MaxDist: 0,
			 atk18_MaxDist: 0,
			 atk19_MaxDist: 0,
			 atk20_MaxDist: 0,
			 atk21_MaxDist: 0,
			 atk22_MaxDist: 0,
			 atk23_MaxDist: 0,
			 atk24_MaxDist: 0,
			 atk25_MaxDist: 0,
			 atk26_MaxDist: 0,
			 atk27_MaxDist: 0,
			 atk28_MaxDist: 0,
			 atk29_MaxDist: 0,
			 bits_0: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 bits_4: 0,
			 bits_5: 0,
			 bits_6: 0,
			 bits_7: 0,
			 bits_8: 0,
			 bits_9: 0,
			 bits_10: 0,
			 bits_11: 0,
			 bits_12: 0,
			 bits_13: 0,
			 bits_14: 0,
			 pad0: [0;13],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct AI_ATTACK_PARAM_ST {
	pub attackTableId: i32,
	pub attackId: i32,
	pub successDistance: f32,
	pub turnTimeBeforeAttack: f32,
	pub frontAngleRange: i16,
	pub upAngleThreshold: i16,
	pub downAngleThershold: i16,
	pub isFirstAttack: u8,
	pub doesSelectOnOutRange: u8,
	pub minOptimalDistance: f32,
	pub maxOptimalDistance: f32,
	pub baseDirectionForOptimalAngle1: i16,
	pub optimalAttackAngleRange1: i16,
	pub baseDirectionForOptimalAngle2: i16,
	pub optimalAttackAngleRange2: i16,
	pub intervalForExec: f32,
	pub selectionTendency: f32,
	pub shortRangeTendency: f32,
	pub middleRangeTendency: f32,
	pub farRangeTendency: f32,
	pub outRangeTendency: f32,
	pub deriveAttackId1: i32,
	pub deriveAttackId2: i32,
	pub deriveAttackId3: i32,
	pub deriveAttackId4: i32,
	pub deriveAttackId5: i32,
	pub deriveAttackId6: i32,
	pub deriveAttackId7: i32,
	pub deriveAttackId8: i32,
	pub deriveAttackId9: i32,
	pub deriveAttackId10: i32,
	pub deriveAttackId11: i32,
	pub deriveAttackId12: i32,
	pub deriveAttackId13: i32,
	pub deriveAttackId14: i32,
	pub deriveAttackId15: i32,
	pub deriveAttackId16: i32,
	pub goalLifeMin: f32,
	pub goalLifeMax: f32,
	pub doesSelectOnInnerRange: u8,
	pub enableAttackOnBattleStart: u8,
	pub doesSelectOnTargetDown: u8,
	pub pad1: [u8;1],
	pub minArriveDistance: f32,
	pub maxArriveDistance: f32,
	pub comboExecDistance: f32,
	pub comboExecRange: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl AI_ATTACK_PARAM_ST {
}
impl Default for AI_ATTACK_PARAM_ST {
	fn default() -> Self {
		Self {
			 attackTableId: 0,
			 attackId: 0,
			 successDistance: 0.,
			 turnTimeBeforeAttack: 0.,
			 frontAngleRange: 0,
			 upAngleThreshold: 0,
			 downAngleThershold: 0,
			 isFirstAttack: 0,
			 doesSelectOnOutRange: 0,
			 minOptimalDistance: 0.,
			 maxOptimalDistance: 0.,
			 baseDirectionForOptimalAngle1: 0,
			 optimalAttackAngleRange1: 0,
			 baseDirectionForOptimalAngle2: 0,
			 optimalAttackAngleRange2: 0,
			 intervalForExec: 1.,
			 selectionTendency: -1.,
			 shortRangeTendency: -1.,
			 middleRangeTendency: -1.,
			 farRangeTendency: -1.,
			 outRangeTendency: -1.,
			 deriveAttackId1: -1,
			 deriveAttackId2: -1,
			 deriveAttackId3: -1,
			 deriveAttackId4: -1,
			 deriveAttackId5: -1,
			 deriveAttackId6: -1,
			 deriveAttackId7: -1,
			 deriveAttackId8: -1,
			 deriveAttackId9: -1,
			 deriveAttackId10: -1,
			 deriveAttackId11: -1,
			 deriveAttackId12: -1,
			 deriveAttackId13: -1,
			 deriveAttackId14: -1,
			 deriveAttackId15: -1,
			 deriveAttackId16: -1,
			 goalLifeMin: 0.,
			 goalLifeMax: 0.,
			 doesSelectOnInnerRange: 0,
			 enableAttackOnBattleStart: 1,
			 doesSelectOnTargetDown: 1,
			 pad1: [0;1],
			 minArriveDistance: 0.,
			 maxArriveDistance: 0.,
			 comboExecDistance: 4.,
			 comboExecRange: 180.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct AI_ODDS_PARAM {
	pub act0: u8,
	pub act1: u8,
	pub act2: u8,
	pub act3: u8,
	pub act4: u8,
	pub act5: u8,
	pub act6: u8,
	pub act7: u8,
	pub act8: u8,
	pub act9: u8,
	pub act10: u8,
	pub act11: u8,
	pub act12: u8,
	pub act13: u8,
	pub act14: u8,
	pub act15: u8,
	pub act16: u8,
	pub act17: u8,
	pub act18: u8,
	pub act19: u8,
	pub act20: u8,
	pub act21: u8,
	pub act22: u8,
	pub act23: u8,
	pub act24: u8,
	pub act25: u8,
	pub act26: u8,
	pub act27: u8,
	pub act28: u8,
	pub act29: u8,
	pub act30: u8,
	pub act31: u8,
	pub act32: u8,
	pub act33: u8,
	pub act34: u8,
	pub act35: u8,
	pub act36: u8,
	pub act37: u8,
	pub act38: u8,
	pub act39: u8,
	pub act40: u8,
	pub act41: u8,
	pub act42: u8,
	pub act43: u8,
	pub act44: u8,
	pub act45: u8,
	pub act46: u8,
	pub act47: u8,
	pub act48: u8,
	pub act49: u8,
	pub act50: u8,
	pub act51: u8,
	pub act52: u8,
	pub act53: u8,
	pub act54: u8,
	pub act55: u8,
	pub act56: u8,
	pub act57: u8,
	pub act58: u8,
	pub act59: u8,
	pub act60: u8,
	pub act61: u8,
	pub act62: u8,
	pub act63: u8,
	pub act64: u8,
	pub act65: u8,
	pub act66: u8,
	pub act67: u8,
	pub act68: u8,
	pub act69: u8,
	pub act70: u8,
	pub act71: u8,
	pub act72: u8,
	pub act73: u8,
	pub act74: u8,
	pub act75: u8,
	pub act76: u8,
	pub act77: u8,
	pub act78: u8,
	pub act79: u8,
	pub act80: u8,
	pub act81: u8,
	pub act82: u8,
	pub act83: u8,
	pub act84: u8,
	pub act85: u8,
	pub act86: u8,
	pub act87: u8,
	pub act88: u8,
	pub act89: u8,
	pub act90: u8,
	pub act91: u8,
	pub act92: u8,
	pub act93: u8,
	pub act94: u8,
	pub act95: u8,
	pub act96: u8,
	pub act97: u8,
	pub act98: u8,
	pub act99: u8,
	pub pad0: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl AI_ODDS_PARAM {
}
impl Default for AI_ODDS_PARAM {
	fn default() -> Self {
		Self {
			 act0: 0,
			 act1: 0,
			 act2: 0,
			 act3: 0,
			 act4: 0,
			 act5: 0,
			 act6: 0,
			 act7: 0,
			 act8: 0,
			 act9: 0,
			 act10: 0,
			 act11: 0,
			 act12: 0,
			 act13: 0,
			 act14: 0,
			 act15: 0,
			 act16: 0,
			 act17: 0,
			 act18: 0,
			 act19: 0,
			 act20: 0,
			 act21: 0,
			 act22: 0,
			 act23: 0,
			 act24: 0,
			 act25: 0,
			 act26: 0,
			 act27: 0,
			 act28: 0,
			 act29: 0,
			 act30: 0,
			 act31: 0,
			 act32: 0,
			 act33: 0,
			 act34: 0,
			 act35: 0,
			 act36: 0,
			 act37: 0,
			 act38: 0,
			 act39: 0,
			 act40: 0,
			 act41: 0,
			 act42: 0,
			 act43: 0,
			 act44: 0,
			 act45: 0,
			 act46: 0,
			 act47: 0,
			 act48: 0,
			 act49: 0,
			 act50: 0,
			 act51: 0,
			 act52: 0,
			 act53: 0,
			 act54: 0,
			 act55: 0,
			 act56: 0,
			 act57: 0,
			 act58: 0,
			 act59: 0,
			 act60: 0,
			 act61: 0,
			 act62: 0,
			 act63: 0,
			 act64: 0,
			 act65: 0,
			 act66: 0,
			 act67: 0,
			 act68: 0,
			 act69: 0,
			 act70: 0,
			 act71: 0,
			 act72: 0,
			 act73: 0,
			 act74: 0,
			 act75: 0,
			 act76: 0,
			 act77: 0,
			 act78: 0,
			 act79: 0,
			 act80: 0,
			 act81: 0,
			 act82: 0,
			 act83: 0,
			 act84: 0,
			 act85: 0,
			 act86: 0,
			 act87: 0,
			 act88: 0,
			 act89: 0,
			 act90: 0,
			 act91: 0,
			 act92: 0,
			 act93: 0,
			 act94: 0,
			 act95: 0,
			 act96: 0,
			 act97: 0,
			 act98: 0,
			 act99: 0,
			 pad0: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct AI_SOUND_PARAM_ST {
	pub radius: f32,
	pub lifeFrame: f32,
	pub bSpEffectEnable: u8,
	pub r#type: u8,
	bits_0: u8,
	pub forgetTime: f32,
	pub priority: i32,
	pub soundBehaviorId: i32,
	pub aiSoundLevel: u8,
	pub replaningState: u8,
	pub pad1: [u8;6],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl AI_SOUND_PARAM_ST {
	pub fn opposeTarget(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn friendlyTarget(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn selfTarget(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn disableOnTargetPCompany(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
}
impl Default for AI_SOUND_PARAM_ST {
	fn default() -> Self {
		Self {
			 radius: 0.,
			 lifeFrame: 0.,
			 bSpEffectEnable: 0,
			 r#type: 0,
			 bits_0: 1,
			 forgetTime: -1.,
			 priority: 100,
			 soundBehaviorId: -1,
			 aiSoundLevel: 0,
			 replaningState: 0,
			 pad1: [0;6],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct AI_STANDARD_INFO_BANK {
	pub RadarRange: i16,
	pub RadarAngleX: u8,
	pub RadarAngleY: u8,
	pub TerritorySize: i16,
	pub ThreatBeforeAttackRate: u8,
	pub ForceThreatOnFirstLocked: u8,
	pub reserve0: [u8;24],
	pub Attack1_Distance: i16,
	pub Attack1_Margin: i16,
	pub Attack1_Rate: u8,
	pub Attack1_ActionID: u8,
	pub Attack1_DelayMin: u8,
	pub Attack1_DelayMax: u8,
	pub Attack1_ConeAngle: u8,
	pub reserve10: [u8;7],
	pub Attack2_Distance: i16,
	pub Attack2_Margin: i16,
	pub Attack2_Rate: u8,
	pub Attack2_ActionID: u8,
	pub Attack2_DelayMin: u8,
	pub Attack2_DelayMax: u8,
	pub Attack2_ConeAngle: u8,
	pub reserve11: [u8;7],
	pub Attack3_Distance: i16,
	pub Attack3_Margin: i16,
	pub Attack3_Rate: u8,
	pub Attack3_ActionID: u8,
	pub Attack3_DelayMin: u8,
	pub Attack3_DelayMax: u8,
	pub Attack3_ConeAngle: u8,
	pub reserve12: [u8;7],
	pub Attack4_Distance: i16,
	pub Attack4_Margin: i16,
	pub Attack4_Rate: u8,
	pub Attack4_ActionID: u8,
	pub Attack4_DelayMin: u8,
	pub Attack4_DelayMax: u8,
	pub Attack4_ConeAngle: u8,
	pub reserve13: [u8;7],
	pub reserve_last: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl AI_STANDARD_INFO_BANK {
}
impl Default for AI_STANDARD_INFO_BANK {
	fn default() -> Self {
		Self {
			 RadarRange: 20,
			 RadarAngleX: 30,
			 RadarAngleY: 60,
			 TerritorySize: 20,
			 ThreatBeforeAttackRate: 50,
			 ForceThreatOnFirstLocked: 0,
			 reserve0: [0;24],
			 Attack1_Distance: 0,
			 Attack1_Margin: 0,
			 Attack1_Rate: 50,
			 Attack1_ActionID: 0,
			 Attack1_DelayMin: 0,
			 Attack1_DelayMax: 0,
			 Attack1_ConeAngle: 30,
			 reserve10: [0;7],
			 Attack2_Distance: 0,
			 Attack2_Margin: 0,
			 Attack2_Rate: 50,
			 Attack2_ActionID: 0,
			 Attack2_DelayMin: 0,
			 Attack2_DelayMax: 0,
			 Attack2_ConeAngle: 30,
			 reserve11: [0;7],
			 Attack3_Distance: 0,
			 Attack3_Margin: 0,
			 Attack3_Rate: 50,
			 Attack3_ActionID: 0,
			 Attack3_DelayMin: 0,
			 Attack3_DelayMax: 0,
			 Attack3_ConeAngle: 30,
			 reserve12: [0;7],
			 Attack4_Distance: 0,
			 Attack4_Margin: 0,
			 Attack4_Rate: 50,
			 Attack4_ActionID: 0,
			 Attack4_DelayMin: 0,
			 Attack4_DelayMax: 0,
			 Attack4_ConeAngle: 30,
			 reserve13: [0;7],
			 reserve_last: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ASSET_GEOMETORY_PARAM_ST {
	pub soundBankId: i32,
	pub soundBreakSEId: i32,
	pub refDrawParamId: i32,
	pub hitCreateType: i8,
	pub behaviorType: u8,
	pub collisionType: u8,
	pub rainBlockingType: u8,
	pub hp: i16,
	pub defense: i16,
	pub breakStopTime: f32,
	pub breakSfxId: i32,
	pub breakSfxCpId: i32,
	pub breakLandingSfxId: i32,
	pub breakBulletBehaviorId: i32,
	pub breakBulletCpId: i32,
	pub FragmentInvisibleWaitTime: f32,
	pub FragmentInvisibleTime: f32,
	pub BreakAiSoundID: i32,
	pub breakItemLotType: i8,
	pub animBreakIdMax: u8,
	pub breakBulletAttributeDamageType: i8,
	bits_0: u8,
	bits_1: u8,
	pub navimeshFlag: u8,
	pub burnBulletInterval: i16,
	pub clothUpdateDist: f32,
	pub lifeTime_forRuntimeCreate: f32,
	pub contactSeId: i32,
	pub repickAnimIdOffset: i32,
	pub windEffectRate_0: f32,
	pub windEffectRate_1: f32,
	pub windEffectType_0: u8,
	pub windEffectType_1: u8,
	pub overrideMaterialId: i16,
	pub autoCreateOffsetHeight: f32,
	pub burnTime: f32,
	pub burnBraekRate: f32,
	pub burnSfxId: i32,
	pub burnSfxId_1: i32,
	pub burnSfxId_2: i32,
	pub burnSfxId_3: i32,
	pub burnSfxDelayTimeMin: f32,
	pub burnSfxDelayTimeMin_1: f32,
	pub burnSfxDelayTimeMin_2: f32,
	pub burnSfxDelayTimeMin_3: f32,
	pub burnSfxDelayTimeMax: f32,
	pub burnSfxDelayTimeMax_1: f32,
	pub burnSfxDelayTimeMax_2: f32,
	pub burnSfxDelayTimeMax_3: f32,
	pub burnBulletBehaviorId: i32,
	pub burnBulletBehaviorId_1: i32,
	pub burnBulletBehaviorId_2: i32,
	pub burnBulletBehaviorId_3: i32,
	pub burnBulletDelayTime: f32,
	pub paintDecalTargetTextureSize: i16,
	pub navimeshFlag_after: u8,
	pub camNearBehaviorType: i8,
	pub breakItemLotParamId: i32,
	pub pickUpActionButtonParamId: i32,
	pub pickUpItemLotParamId: i32,
	pub autoDrawGroupBackFaceCheck: u8,
	pub autoDrawGroupDepthWrite: u8,
	pub autoDrawGroupShadowTest: u8,
	pub debug_isHeightCheckEnable: u8,
	pub hitCarverCancelAreaFlag: u8,
	pub assetNavimeshNoCombine: u8,
	pub navimeshFlagApply: u8,
	pub navimeshFlagApply_after: u8,
	pub autoDrawGroupPassPixelNum: f32,
	pub pickUpReplacementEventFlag: i32,
	pub pickUpReplacementAnimIdOffset: i32,
	pub pickUpReplacementActionButtonParamId: i32,
	pub pickUpReplacementItemLotParamId: i32,
	pub slidingBulletHitType: u8,
	pub isBushesForDamage: u8,
	pub penetrationBulletType: u8,
	pub unkR3: u8,
	pub unkR4: f32,
	pub soundBreakSECpId: i32,
	pub debug_HeightCheckCapacityMin: f32,
	pub debug_HeightCheckCapacityMax: f32,
	pub repickActionButtonParamId: i32,
	pub repickItemLotParamId: i32,
	pub repickReplacementAnimIdOffset: i32,
	pub repickReplacementActionButtonParamId: i32,
	pub repickReplacementItemLotParamId: i32,
	pub noGenerateCarver: u8,
	pub noHitHugeAfterBreak: u8,
	bits_2: u8,
	pub generateMultiForbiddenRegion: u8,
	pub residentSeId0: i32,
	pub residentSeId1: i32,
	pub residentSeId2: i32,
	pub residentSeId3: i32,
	pub residentSeDmypolyId0: i16,
	pub residentSeDmypolyId1: i16,
	pub residentSeDmypolyId2: i16,
	pub residentSeDmypolyId3: i16,
	pub excludeActivateRatio_Xboxone_Grid: u8,
	pub excludeActivateRatio_Xboxone_Legacy: u8,
	pub excludeActivateRatio_PS4_Grid: u8,
	pub excludeActivateRatio_PS4_Legacy: u8,
	pub Reserve_0: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ASSET_GEOMETORY_PARAM_ST {
	pub fn isBreakByPlayerCollide(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isBreakByEnemyCollide(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isBreak_ByChrRide(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn isDisableBreakForFirstAppear(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn isAnimBreak(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isDamageCover(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn isAttackBacklash(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn Reserve_2(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn isLadder(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isMoveObj(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn isSkydomeFlag(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isAnimPauseOnRemoPlay(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isBurn(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn isEnableRepick(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isBreakOnPickUp(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn isBreakByHugeenemyCollide(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn isEnabledBreakSync(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn isHiddenOnRepick(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn isCreateMultiPlayOnly(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isDisableBulletHitSfx(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn isEnableSignPreBreak(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn isEnableSignPostBreak(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn unkR1(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
}
impl Default for ASSET_GEOMETORY_PARAM_ST {
	fn default() -> Self {
		Self {
			 soundBankId: -1,
			 soundBreakSEId: -1,
			 refDrawParamId: -1,
			 hitCreateType: 0,
			 behaviorType: 1,
			 collisionType: 0,
			 rainBlockingType: 0,
			 hp: -1,
			 defense: 0,
			 breakStopTime: 30.,
			 breakSfxId: -1,
			 breakSfxCpId: -1,
			 breakLandingSfxId: -1,
			 breakBulletBehaviorId: -1,
			 breakBulletCpId: -1,
			 FragmentInvisibleWaitTime: 0.,
			 FragmentInvisibleTime: 0.,
			 BreakAiSoundID: 0,
			 breakItemLotType: 0,
			 animBreakIdMax: 0,
			 breakBulletAttributeDamageType: 0,
			 bits_0: 0,
			 bits_1: 0,
			 navimeshFlag: 0,
			 burnBulletInterval: 30,
			 clothUpdateDist: 30.,
			 lifeTime_forRuntimeCreate: 0.,
			 contactSeId: -1,
			 repickAnimIdOffset: 0,
			 windEffectRate_0: 0.5,
			 windEffectRate_1: 0.5,
			 windEffectType_0: 0,
			 windEffectType_1: 0,
			 overrideMaterialId: -1,
			 autoCreateOffsetHeight: 0.1,
			 burnTime: 0.,
			 burnBraekRate: 0.5,
			 burnSfxId: -1,
			 burnSfxId_1: -1,
			 burnSfxId_2: -1,
			 burnSfxId_3: -1,
			 burnSfxDelayTimeMin: 0.,
			 burnSfxDelayTimeMin_1: 0.,
			 burnSfxDelayTimeMin_2: 0.,
			 burnSfxDelayTimeMin_3: 0.,
			 burnSfxDelayTimeMax: 0.,
			 burnSfxDelayTimeMax_1: 0.,
			 burnSfxDelayTimeMax_2: 0.,
			 burnSfxDelayTimeMax_3: 0.,
			 burnBulletBehaviorId: -1,
			 burnBulletBehaviorId_1: -1,
			 burnBulletBehaviorId_2: -1,
			 burnBulletBehaviorId_3: -1,
			 burnBulletDelayTime: 0.,
			 paintDecalTargetTextureSize: 0,
			 navimeshFlag_after: 0,
			 camNearBehaviorType: 0,
			 breakItemLotParamId: -1,
			 pickUpActionButtonParamId: -1,
			 pickUpItemLotParamId: -1,
			 autoDrawGroupBackFaceCheck: 0,
			 autoDrawGroupDepthWrite: 0,
			 autoDrawGroupShadowTest: 0,
			 debug_isHeightCheckEnable: 0,
			 hitCarverCancelAreaFlag: 0,
			 assetNavimeshNoCombine: 0,
			 navimeshFlagApply: 0,
			 navimeshFlagApply_after: 0,
			 autoDrawGroupPassPixelNum: -1.,
			 pickUpReplacementEventFlag: 0,
			 pickUpReplacementAnimIdOffset: 0,
			 pickUpReplacementActionButtonParamId: -1,
			 pickUpReplacementItemLotParamId: -1,
			 slidingBulletHitType: 0,
			 isBushesForDamage: 0,
			 penetrationBulletType: 0,
			 unkR3: 0,
			 unkR4: 0.,
			 soundBreakSECpId: -1,
			 debug_HeightCheckCapacityMin: -99.,
			 debug_HeightCheckCapacityMax: 99.,
			 repickActionButtonParamId: -1,
			 repickItemLotParamId: -1,
			 repickReplacementAnimIdOffset: 0,
			 repickReplacementActionButtonParamId: -1,
			 repickReplacementItemLotParamId: -1,
			 noGenerateCarver: 0,
			 noHitHugeAfterBreak: 0,
			 bits_2: 1,
			 generateMultiForbiddenRegion: 0,
			 residentSeId0: -1,
			 residentSeId1: -1,
			 residentSeId2: -1,
			 residentSeId3: -1,
			 residentSeDmypolyId0: -1,
			 residentSeDmypolyId1: -1,
			 residentSeDmypolyId2: -1,
			 residentSeDmypolyId3: -1,
			 excludeActivateRatio_Xboxone_Grid: 0,
			 excludeActivateRatio_Xboxone_Legacy: 0,
			 excludeActivateRatio_PS4_Grid: 0,
			 excludeActivateRatio_PS4_Legacy: 0,
			 Reserve_0: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ASSET_MATERIAL_SFX_PARAM_ST {
	pub sfxId_00: i32,
	pub sfxId_01: i32,
	pub sfxId_02: i32,
	pub sfxId_03: i32,
	pub sfxId_04: i32,
	pub sfxId_05: i32,
	pub sfxId_06: i32,
	pub sfxId_07: i32,
	pub sfxId_08: i32,
	pub sfxId_09: i32,
	pub sfxId_10: i32,
	pub sfxId_11: i32,
	pub sfxId_12: i32,
	pub sfxId_13: i32,
	pub sfxId_14: i32,
	pub sfxId_15: i32,
	pub sfxId_16: i32,
	pub sfxId_17: i32,
	pub sfxId_18: i32,
	pub sfxId_19: i32,
	pub sfxId_20: i32,
	pub sfxId_21: i32,
	pub sfxId_22: i32,
	pub sfxId_23: i32,
	pub sfxId_24: i32,
	pub sfxId_25: i32,
	pub sfxId_26: i32,
	pub sfxId_27: i32,
	pub sfxId_28: i32,
	pub sfxId_29: i32,
	pub sfxId_30: i32,
	pub sfxId_31: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ASSET_MATERIAL_SFX_PARAM_ST {
}
impl Default for ASSET_MATERIAL_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 sfxId_00: 0,
			 sfxId_01: 0,
			 sfxId_02: 0,
			 sfxId_03: 0,
			 sfxId_04: 0,
			 sfxId_05: 0,
			 sfxId_06: 0,
			 sfxId_07: 0,
			 sfxId_08: 0,
			 sfxId_09: 0,
			 sfxId_10: 0,
			 sfxId_11: 0,
			 sfxId_12: 0,
			 sfxId_13: 0,
			 sfxId_14: 0,
			 sfxId_15: 0,
			 sfxId_16: 0,
			 sfxId_17: 0,
			 sfxId_18: 0,
			 sfxId_19: 0,
			 sfxId_20: 0,
			 sfxId_21: 0,
			 sfxId_22: 0,
			 sfxId_23: 0,
			 sfxId_24: 0,
			 sfxId_25: 0,
			 sfxId_26: 0,
			 sfxId_27: 0,
			 sfxId_28: 0,
			 sfxId_29: 0,
			 sfxId_30: 0,
			 sfxId_31: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ASSET_MODEL_SFX_PARAM_ST {
	pub sfxId_0: i32,
	pub dmypolyId_0: i32,
	pub reserve_0: [u8;8],
	pub sfxId_1: i32,
	pub dmypolyId_1: i32,
	pub reserve_1: [u8;8],
	pub sfxId_2: i32,
	pub dmypolyId_2: i32,
	pub reserve_2: [u8;8],
	pub sfxId_3: i32,
	pub dmypolyId_3: i32,
	pub reserve_3: [u8;8],
	pub sfxId_4: i32,
	pub dmypolyId_4: i32,
	pub reserve_4: [u8;8],
	pub sfxId_5: i32,
	pub dmypolyId_5: i32,
	pub reserve_5: [u8;8],
	pub sfxId_6: i32,
	pub dmypolyId_6: i32,
	pub reserve_6: [u8;8],
	pub sfxId_7: i32,
	pub dmypolyId_7: i32,
	pub isDisableIV: u8,
	pub reserve_7: [u8;7],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ASSET_MODEL_SFX_PARAM_ST {
}
impl Default for ASSET_MODEL_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 sfxId_0: -1,
			 dmypolyId_0: -1,
			 reserve_0: [0;8],
			 sfxId_1: -1,
			 dmypolyId_1: -1,
			 reserve_1: [0;8],
			 sfxId_2: -1,
			 dmypolyId_2: -1,
			 reserve_2: [0;8],
			 sfxId_3: -1,
			 dmypolyId_3: -1,
			 reserve_3: [0;8],
			 sfxId_4: -1,
			 dmypolyId_4: -1,
			 reserve_4: [0;8],
			 sfxId_5: -1,
			 dmypolyId_5: -1,
			 reserve_5: [0;8],
			 sfxId_6: -1,
			 dmypolyId_6: -1,
			 reserve_6: [0;8],
			 sfxId_7: -1,
			 dmypolyId_7: -1,
			 isDisableIV: 0,
			 reserve_7: [0;7],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ATK_PARAM_ST {
	pub hit0_Radius: f32,
	pub hit1_Radius: f32,
	pub hit2_Radius: f32,
	pub hit3_Radius: f32,
	pub knockbackDist: f32,
	pub hitStopTime: f32,
	pub spEffectId0: i32,
	pub spEffectId1: i32,
	pub spEffectId2: i32,
	pub spEffectId3: i32,
	pub spEffectId4: i32,
	pub hit0_DmyPoly1: i16,
	pub hit1_DmyPoly1: i16,
	pub hit2_DmyPoly1: i16,
	pub hit3_DmyPoly1: i16,
	pub hit0_DmyPoly2: i16,
	pub hit1_DmyPoly2: i16,
	pub hit2_DmyPoly2: i16,
	pub hit3_DmyPoly2: i16,
	pub blowingCorrection: i16,
	pub atkPhysCorrection: i16,
	pub atkMagCorrection: i16,
	pub atkFireCorrection: i16,
	pub atkThunCorrection: i16,
	pub atkStamCorrection: i16,
	pub guardAtkRateCorrection: i16,
	pub guardBreakCorrection: i16,
	pub atkThrowEscapeCorrection: i16,
	pub subCategory1: u8,
	pub subCategory2: u8,
	pub atkPhys: i16,
	pub atkMag: i16,
	pub atkFire: i16,
	pub atkThun: i16,
	pub atkStam: i16,
	pub guardAtkRate: i16,
	pub guardBreakRate: i16,
	pub pad6: [u8;1],
	pub isEnableCalcDamageForBushesObj: u8,
	pub atkThrowEscape: i16,
	pub atkObj: i16,
	pub guardStaminaCutRate: i16,
	pub guardRate: i16,
	pub throwTypeId: i16,
	pub hit0_hitType: u8,
	pub hit1_hitType: u8,
	pub hit2_hitType: u8,
	pub hit3_hitType: u8,
	pub hti0_Priority: u8,
	pub hti1_Priority: u8,
	pub hti2_Priority: u8,
	pub hti3_Priority: u8,
	pub dmgLevel: u8,
	pub mapHitType: u8,
	pub guardCutCancelRate: i8,
	pub atkAttribute: u8,
	pub spAttribute: u8,
	pub atkType: u8,
	pub atkMaterial: u8,
	pub guardRangeType: u8,
	pub defSeMaterial1: i16,
	pub hitSourceType: u8,
	pub throwFlag: u8,
	bits_0: u8,
	pub atkPow_forSfx: i8,
	pub atkDir_forSfx: i8,
	bits_1: u8,
	pub atkBehaviorId: u8,
	pub atkPow_forSe: i8,
	pub atkSuperArmor: f32,
	pub decalId1: i32,
	pub decalId2: i32,
	pub AppearAiSoundId: i32,
	pub HitAiSoundId: i32,
	pub HitRumbleId: i32,
	pub HitRumbleIdByNormal: i32,
	pub HitRumbleIdByMiddle: i32,
	pub HitRumbleIdByRoot: i32,
	pub traceSfxId0: i32,
	pub traceDmyIdHead0: i32,
	pub traceDmyIdTail0: i32,
	pub traceSfxId1: i32,
	pub traceDmyIdHead1: i32,
	pub traceDmyIdTail1: i32,
	pub traceSfxId2: i32,
	pub traceDmyIdHead2: i32,
	pub traceDmyIdTail2: i32,
	pub traceSfxId3: i32,
	pub traceDmyIdHead3: i32,
	pub traceDmyIdTail3: i32,
	pub traceSfxId4: i32,
	pub traceDmyIdHead4: i32,
	pub traceDmyIdTail4: i32,
	pub traceSfxId5: i32,
	pub traceDmyIdHead5: i32,
	pub traceDmyIdTail5: i32,
	pub traceSfxId6: i32,
	pub traceDmyIdHead6: i32,
	pub traceDmyIdTail6: i32,
	pub traceSfxId7: i32,
	pub traceDmyIdHead7: i32,
	pub traceDmyIdTail7: i32,
	pub hit4_Radius: f32,
	pub hit5_Radius: f32,
	pub hit6_Radius: f32,
	pub hit7_Radius: f32,
	pub hit8_Radius: f32,
	pub hit9_Radius: f32,
	pub hit10_Radius: f32,
	pub hit11_Radius: f32,
	pub hit12_Radius: f32,
	pub hit13_Radius: f32,
	pub hit14_Radius: f32,
	pub hit15_Radius: f32,
	pub hit4_DmyPoly1: i16,
	pub hit5_DmyPoly1: i16,
	pub hit6_DmyPoly1: i16,
	pub hit7_DmyPoly1: i16,
	pub hit8_DmyPoly1: i16,
	pub hit9_DmyPoly1: i16,
	pub hit10_DmyPoly1: i16,
	pub hit11_DmyPoly1: i16,
	pub hit12_DmyPoly1: i16,
	pub hit13_DmyPoly1: i16,
	pub hit14_DmyPoly1: i16,
	pub hit15_DmyPoly1: i16,
	pub hit4_DmyPoly2: i16,
	pub hit5_DmyPoly2: i16,
	pub hit6_DmyPoly2: i16,
	pub hit7_DmyPoly2: i16,
	pub hit8_DmyPoly2: i16,
	pub hit9_DmyPoly2: i16,
	pub hit10_DmyPoly2: i16,
	pub hit11_DmyPoly2: i16,
	pub hit12_DmyPoly2: i16,
	pub hit13_DmyPoly2: i16,
	pub hit14_DmyPoly2: i16,
	pub hit15_DmyPoly2: i16,
	pub hit4_hitType: u8,
	pub hit5_hitType: u8,
	pub hit6_hitType: u8,
	pub hit7_hitType: u8,
	pub hit8_hitType: u8,
	pub hit9_hitType: u8,
	pub hit10_hitType: u8,
	pub hit11_hitType: u8,
	pub hit12_hitType: u8,
	pub hit13_hitType: u8,
	pub hit14_hitType: u8,
	pub hit15_hitType: u8,
	pub hti4_Priority: u8,
	pub hti5_Priority: u8,
	pub hti6_Priority: u8,
	pub hti7_Priority: u8,
	pub hti8_Priority: u8,
	pub hti9_Priority: u8,
	pub hti10_Priority: u8,
	pub hti11_Priority: u8,
	pub hti12_Priority: u8,
	pub hti13_Priority: u8,
	pub hti14_Priority: u8,
	pub hti15_Priority: u8,
	pub defSfxMaterial1: i16,
	pub defSeMaterial2: i16,
	pub defSfxMaterial2: i16,
	pub atkDarkCorrection: i16,
	pub atkDark: i16,
	bits_2: u8,
	pub dmgLevel_vsPlayer: i8,
	pub statusAilmentAtkPowerCorrectRate: i16,
	pub spEffectAtkPowerCorrectRate_byPoint: i16,
	pub spEffectAtkPowerCorrectRate_byRate: i16,
	pub spEffectAtkPowerCorrectRate_byDmg: i16,
	pub atkBehaviorId_2: u8,
	pub throwDamageAttribute: u8,
	pub statusAilmentAtkPowerCorrectRate_byPoint: i16,
	pub overwriteAttackElementCorrectId: i32,
	pub decalBaseId1: i16,
	pub decalBaseId2: i16,
	pub wepRegainHpScale: i16,
	pub atkRegainHp: i16,
	pub regainableTimeScale: f32,
	pub regainableHpRateScale: f32,
	pub regainableSlotId: i8,
	pub spAttributeVariationValue: u8,
	pub parryForwardOffset: i16,
	pub atkSuperArmorCorrection: f32,
	pub defSfxMaterialVariationValue: u8,
	pub pad4: [u8;3],
	pub finalDamageRateId: i32,
	pub pad7: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ATK_PARAM_ST {
	pub fn disableGuard(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableStaminaAttack(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn disableHitSpEffect(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn IgnoreNotifyMissSwingForAI(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn repeatHitSfx(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isArrowAtk(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn isGhostAtk(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn isDisableNoDamage(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn opposeTarget(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn friendlyTarget(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn selfTarget(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isCheckDoorPenetration(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isVsRideAtk(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn isAddBaseAtk(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn excludeThreatLvNotify(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn pad5(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn isDisableParry(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn isDisableBothHandsAtkBonus(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isInvalidatedByNoDamageInAir(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn pad2(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
}
impl Default for ATK_PARAM_ST {
	fn default() -> Self {
		Self {
			 hit0_Radius: 0.,
			 hit1_Radius: 0.,
			 hit2_Radius: 0.,
			 hit3_Radius: 0.,
			 knockbackDist: 0.,
			 hitStopTime: 0.,
			 spEffectId0: -1,
			 spEffectId1: -1,
			 spEffectId2: -1,
			 spEffectId3: -1,
			 spEffectId4: -1,
			 hit0_DmyPoly1: 0,
			 hit1_DmyPoly1: 0,
			 hit2_DmyPoly1: 0,
			 hit3_DmyPoly1: 0,
			 hit0_DmyPoly2: 0,
			 hit1_DmyPoly2: 0,
			 hit2_DmyPoly2: 0,
			 hit3_DmyPoly2: 0,
			 blowingCorrection: 0,
			 atkPhysCorrection: 0,
			 atkMagCorrection: 0,
			 atkFireCorrection: 0,
			 atkThunCorrection: 0,
			 atkStamCorrection: 0,
			 guardAtkRateCorrection: 0,
			 guardBreakCorrection: 0,
			 atkThrowEscapeCorrection: 0,
			 subCategory1: 0,
			 subCategory2: 0,
			 atkPhys: 0,
			 atkMag: 0,
			 atkFire: 0,
			 atkThun: 0,
			 atkStam: 0,
			 guardAtkRate: 0,
			 guardBreakRate: 0,
			 pad6: [0;1],
			 isEnableCalcDamageForBushesObj: 0,
			 atkThrowEscape: 0,
			 atkObj: 0,
			 guardStaminaCutRate: 0,
			 guardRate: 0,
			 throwTypeId: 0,
			 hit0_hitType: 0,
			 hit1_hitType: 0,
			 hit2_hitType: 0,
			 hit3_hitType: 0,
			 hti0_Priority: 0,
			 hti1_Priority: 0,
			 hti2_Priority: 0,
			 hti3_Priority: 0,
			 dmgLevel: 0,
			 mapHitType: 0,
			 guardCutCancelRate: 0,
			 atkAttribute: 0,
			 spAttribute: 0,
			 atkType: 0,
			 atkMaterial: 0,
			 guardRangeType: 0,
			 defSeMaterial1: 0,
			 hitSourceType: 0,
			 throwFlag: 0,
			 bits_0: 0,
			 atkPow_forSfx: 0,
			 atkDir_forSfx: 0,
			 bits_1: 1,
			 atkBehaviorId: 0,
			 atkPow_forSe: 0,
			 atkSuperArmor: 0.,
			 decalId1: -1,
			 decalId2: -1,
			 AppearAiSoundId: 0,
			 HitAiSoundId: 0,
			 HitRumbleId: -1,
			 HitRumbleIdByNormal: -1,
			 HitRumbleIdByMiddle: -1,
			 HitRumbleIdByRoot: -1,
			 traceSfxId0: -1,
			 traceDmyIdHead0: -1,
			 traceDmyIdTail0: -1,
			 traceSfxId1: -1,
			 traceDmyIdHead1: -1,
			 traceDmyIdTail1: -1,
			 traceSfxId2: -1,
			 traceDmyIdHead2: -1,
			 traceDmyIdTail2: -1,
			 traceSfxId3: -1,
			 traceDmyIdHead3: -1,
			 traceDmyIdTail3: -1,
			 traceSfxId4: -1,
			 traceDmyIdHead4: -1,
			 traceDmyIdTail4: -1,
			 traceSfxId5: -1,
			 traceDmyIdHead5: -1,
			 traceDmyIdTail5: -1,
			 traceSfxId6: -1,
			 traceDmyIdHead6: -1,
			 traceDmyIdTail6: -1,
			 traceSfxId7: -1,
			 traceDmyIdHead7: -1,
			 traceDmyIdTail7: -1,
			 hit4_Radius: 0.,
			 hit5_Radius: 0.,
			 hit6_Radius: 0.,
			 hit7_Radius: 0.,
			 hit8_Radius: 0.,
			 hit9_Radius: 0.,
			 hit10_Radius: 0.,
			 hit11_Radius: 0.,
			 hit12_Radius: 0.,
			 hit13_Radius: 0.,
			 hit14_Radius: 0.,
			 hit15_Radius: 0.,
			 hit4_DmyPoly1: 0,
			 hit5_DmyPoly1: 0,
			 hit6_DmyPoly1: 0,
			 hit7_DmyPoly1: 0,
			 hit8_DmyPoly1: 0,
			 hit9_DmyPoly1: 0,
			 hit10_DmyPoly1: 0,
			 hit11_DmyPoly1: 0,
			 hit12_DmyPoly1: 0,
			 hit13_DmyPoly1: 0,
			 hit14_DmyPoly1: 0,
			 hit15_DmyPoly1: 0,
			 hit4_DmyPoly2: 0,
			 hit5_DmyPoly2: 0,
			 hit6_DmyPoly2: 0,
			 hit7_DmyPoly2: 0,
			 hit8_DmyPoly2: 0,
			 hit9_DmyPoly2: 0,
			 hit10_DmyPoly2: 0,
			 hit11_DmyPoly2: 0,
			 hit12_DmyPoly2: 0,
			 hit13_DmyPoly2: 0,
			 hit14_DmyPoly2: 0,
			 hit15_DmyPoly2: 0,
			 hit4_hitType: 0,
			 hit5_hitType: 0,
			 hit6_hitType: 0,
			 hit7_hitType: 0,
			 hit8_hitType: 0,
			 hit9_hitType: 0,
			 hit10_hitType: 0,
			 hit11_hitType: 0,
			 hit12_hitType: 0,
			 hit13_hitType: 0,
			 hit14_hitType: 0,
			 hit15_hitType: 0,
			 hti4_Priority: 0,
			 hti5_Priority: 0,
			 hti6_Priority: 0,
			 hti7_Priority: 0,
			 hti8_Priority: 0,
			 hti9_Priority: 0,
			 hti10_Priority: 0,
			 hti11_Priority: 0,
			 hti12_Priority: 0,
			 hti13_Priority: 0,
			 hti14_Priority: 0,
			 hti15_Priority: 0,
			 defSfxMaterial1: 0,
			 defSeMaterial2: 0,
			 defSfxMaterial2: 0,
			 atkDarkCorrection: 0,
			 atkDark: 0,
			 bits_2: 0,
			 dmgLevel_vsPlayer: 0,
			 statusAilmentAtkPowerCorrectRate: 100,
			 spEffectAtkPowerCorrectRate_byPoint: 100,
			 spEffectAtkPowerCorrectRate_byRate: 100,
			 spEffectAtkPowerCorrectRate_byDmg: 100,
			 atkBehaviorId_2: 0,
			 throwDamageAttribute: 0,
			 statusAilmentAtkPowerCorrectRate_byPoint: 100,
			 overwriteAttackElementCorrectId: -1,
			 decalBaseId1: -1,
			 decalBaseId2: -1,
			 wepRegainHpScale: 100,
			 atkRegainHp: 0,
			 regainableTimeScale: 1.,
			 regainableHpRateScale: 1.,
			 regainableSlotId: -1,
			 spAttributeVariationValue: 0,
			 parryForwardOffset: 0,
			 atkSuperArmorCorrection: 0.,
			 defSfxMaterialVariationValue: 0,
			 pad4: [0;3],
			 finalDamageRateId: 0,
			 pad7: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ATTACK_ELEMENT_CORRECT_PARAM_ST {
	bits_0: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	pub overwriteStrengthCorrectRate_byPhysics: i16,
	pub overwriteDexterityCorrectRate_byPhysics: i16,
	pub overwriteMagicCorrectRate_byPhysics: i16,
	pub overwriteFaithCorrectRate_byPhysics: i16,
	pub overwriteLuckCorrectRate_byPhysics: i16,
	pub overwriteStrengthCorrectRate_byMagic: i16,
	pub overwriteDexterityCorrectRate_byMagic: i16,
	pub overwriteMagicCorrectRate_byMagic: i16,
	pub overwriteFaithCorrectRate_byMagic: i16,
	pub overwriteLuckCorrectRate_byMagic: i16,
	pub overwriteStrengthCorrectRate_byFire: i16,
	pub overwriteDexterityCorrectRate_byFire: i16,
	pub overwriteMagicCorrectRate_byFire: i16,
	pub overwriteFaithCorrectRate_byFire: i16,
	pub overwriteLuckCorrectRate_byFire: i16,
	pub overwriteStrengthCorrectRate_byThunder: i16,
	pub overwriteDexterityCorrectRate_byThunder: i16,
	pub overwriteMagicCorrectRate_byThunder: i16,
	pub overwriteFaithCorrectRate_byThunder: i16,
	pub overwriteLuckCorrectRate_byThunder: i16,
	pub overwriteStrengthCorrectRate_byDark: i16,
	pub overwriteDexterityCorrectRate_byDark: i16,
	pub overwriteMagicCorrectRate_byDark: i16,
	pub overwriteFaithCorrectRate_byDark: i16,
	pub overwriteLuckCorrectRate_byDark: i16,
	pub InfluenceStrengthCorrectRate_byPhysics: i16,
	pub InfluenceDexterityCorrectRate_byPhysics: i16,
	pub InfluenceMagicCorrectRate_byPhysics: i16,
	pub InfluenceFaithCorrectRate_byPhysics: i16,
	pub InfluenceLuckCorrectRate_byPhysics: i16,
	pub InfluenceStrengthCorrectRate_byMagic: i16,
	pub InfluenceDexterityCorrectRate_byMagic: i16,
	pub InfluenceMagicCorrectRate_byMagic: i16,
	pub InfluenceFaithCorrectRate_byMagic: i16,
	pub InfluenceLuckCorrectRate_byMagic: i16,
	pub InfluenceStrengthCorrectRate_byFire: i16,
	pub InfluenceDexterityCorrectRate_byFire: i16,
	pub InfluenceMagicCorrectRate_byFire: i16,
	pub InfluenceFaithCorrectRate_byFire: i16,
	pub InfluenceLuckCorrectRate_byFire: i16,
	pub InfluenceStrengthCorrectRate_byThunder: i16,
	pub InfluenceDexterityCorrectRate_byThunder: i16,
	pub InfluenceMagicCorrectRate_byThunder: i16,
	pub InfluenceFaithCorrectRate_byThunder: i16,
	pub InfluenceLuckCorrectRate_byThunder: i16,
	pub InfluenceStrengthCorrectRate_byDark: i16,
	pub InfluenceDexterityCorrectRate_byDark: i16,
	pub InfluenceMagicCorrectRate_byDark: i16,
	pub InfluenceFaithCorrectRate_byDark: i16,
	pub InfluenceLuckCorrectRate_byDark: i16,
	pub pad2: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ATTACK_ELEMENT_CORRECT_PARAM_ST {
	pub fn isStrengthCorrect_byPhysics(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isDexterityCorrect_byPhysics(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isMagicCorrect_byPhysics(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn isFaithCorrect_byPhysics(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn isLuckCorrect_byPhysics(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isStrengthCorrect_byMagic(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn isDexterityCorrect_byMagic(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn isMagicCorrect_byMagic(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn isFaithCorrect_byMagic(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isLuckCorrect_byMagic(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn isStrengthCorrect_byFire(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isDexterityCorrect_byFire(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isMagicCorrect_byFire(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn isFaithCorrect_byFire(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isLuckCorrect_byFire(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn isStrengthCorrect_byThunder(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn isDexterityCorrect_byThunder(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn isMagicCorrect_byThunder(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn isFaithCorrect_byThunder(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isLuckCorrect_byThunder(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn isStrengthCorrect_byDark(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn isDexterityCorrect_byDark(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn isMagicCorrect_byDark(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn isFaithCorrect_byDark(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn isLuckCorrect_byDark(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
}
impl Default for ATTACK_ELEMENT_CORRECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 overwriteStrengthCorrectRate_byPhysics: -1,
			 overwriteDexterityCorrectRate_byPhysics: -1,
			 overwriteMagicCorrectRate_byPhysics: -1,
			 overwriteFaithCorrectRate_byPhysics: -1,
			 overwriteLuckCorrectRate_byPhysics: -1,
			 overwriteStrengthCorrectRate_byMagic: -1,
			 overwriteDexterityCorrectRate_byMagic: -1,
			 overwriteMagicCorrectRate_byMagic: -1,
			 overwriteFaithCorrectRate_byMagic: -1,
			 overwriteLuckCorrectRate_byMagic: -1,
			 overwriteStrengthCorrectRate_byFire: -1,
			 overwriteDexterityCorrectRate_byFire: -1,
			 overwriteMagicCorrectRate_byFire: -1,
			 overwriteFaithCorrectRate_byFire: -1,
			 overwriteLuckCorrectRate_byFire: -1,
			 overwriteStrengthCorrectRate_byThunder: -1,
			 overwriteDexterityCorrectRate_byThunder: -1,
			 overwriteMagicCorrectRate_byThunder: -1,
			 overwriteFaithCorrectRate_byThunder: -1,
			 overwriteLuckCorrectRate_byThunder: -1,
			 overwriteStrengthCorrectRate_byDark: -1,
			 overwriteDexterityCorrectRate_byDark: -1,
			 overwriteMagicCorrectRate_byDark: -1,
			 overwriteFaithCorrectRate_byDark: -1,
			 overwriteLuckCorrectRate_byDark: -1,
			 InfluenceStrengthCorrectRate_byPhysics: 100,
			 InfluenceDexterityCorrectRate_byPhysics: 100,
			 InfluenceMagicCorrectRate_byPhysics: 100,
			 InfluenceFaithCorrectRate_byPhysics: 100,
			 InfluenceLuckCorrectRate_byPhysics: 100,
			 InfluenceStrengthCorrectRate_byMagic: 100,
			 InfluenceDexterityCorrectRate_byMagic: 100,
			 InfluenceMagicCorrectRate_byMagic: 100,
			 InfluenceFaithCorrectRate_byMagic: 100,
			 InfluenceLuckCorrectRate_byMagic: 100,
			 InfluenceStrengthCorrectRate_byFire: 100,
			 InfluenceDexterityCorrectRate_byFire: 100,
			 InfluenceMagicCorrectRate_byFire: 100,
			 InfluenceFaithCorrectRate_byFire: 100,
			 InfluenceLuckCorrectRate_byFire: 100,
			 InfluenceStrengthCorrectRate_byThunder: 100,
			 InfluenceDexterityCorrectRate_byThunder: 100,
			 InfluenceMagicCorrectRate_byThunder: 100,
			 InfluenceFaithCorrectRate_byThunder: 100,
			 InfluenceLuckCorrectRate_byThunder: 100,
			 InfluenceStrengthCorrectRate_byDark: 100,
			 InfluenceDexterityCorrectRate_byDark: 100,
			 InfluenceMagicCorrectRate_byDark: 100,
			 InfluenceFaithCorrectRate_byDark: 100,
			 InfluenceLuckCorrectRate_byDark: 100,
			 pad2: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct AUTO_CREATE_ENV_SOUND_PARAM_ST {
	pub RangeMin: f32,
	pub RangeMax: f32,
	pub LifeTimeMin: f32,
	pub LifeTimeMax: f32,
	pub DeleteDist: f32,
	pub NearDist: f32,
	pub LimiteRotateMin: f32,
	pub LimiteRotateMax: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl AUTO_CREATE_ENV_SOUND_PARAM_ST {
}
impl Default for AUTO_CREATE_ENV_SOUND_PARAM_ST {
	fn default() -> Self {
		Self {
			 RangeMin: 10.,
			 RangeMax: 25.,
			 LifeTimeMin: 30.,
			 LifeTimeMax: 30.,
			 DeleteDist: 30.,
			 NearDist: 15.,
			 LimiteRotateMin: 0.,
			 LimiteRotateMax: 180.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BASECHR_SELECT_MENU_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub chrInitParam: i32,
	pub originChrInitParam: i32,
	pub imageId: i32,
	pub textId: i32,
	pub reserve: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BASECHR_SELECT_MENU_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for BASECHR_SELECT_MENU_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 chrInitParam: 0,
			 originChrInitParam: 0,
			 imageId: 0,
			 textId: 0,
			 reserve: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BEHAVIOR_PARAM_ST {
	pub variationId: i32,
	pub behaviorJudgeId: i32,
	pub ezStateBehaviorType_old: u8,
	pub refType: u8,
	pub pad2: [u8;2],
	pub refId: i32,
	pub consumeSA: f32,
	pub stamina: i32,
	pub consumeDurability: i32,
	pub category: u8,
	pub heroPoint: u8,
	pub pad1: [u8;2],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BEHAVIOR_PARAM_ST {
}
impl Default for BEHAVIOR_PARAM_ST {
	fn default() -> Self {
		Self {
			 variationId: 0,
			 behaviorJudgeId: 0,
			 ezStateBehaviorType_old: 0,
			 refType: 0,
			 pad2: [0;2],
			 refId: -1,
			 consumeSA: 0.,
			 stamina: 0,
			 consumeDurability: 0,
			 category: 0,
			 heroPoint: 0,
			 pad1: [0;2],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BONFIRE_WARP_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub eventflagId: i32,
	pub bonfireEntityId: i32,
	pub pad4: [u8;2],
	pub bonfireSubCategorySortId: i16,
	pub forbiddenIconId: i16,
	pub dispMinZoomStep: u8,
	pub selectMinZoomStep: u8,
	pub bonfireSubCategoryId: i32,
	pub clearedEventFlagId: i32,
	pub iconId: i16,
	bits_1: u8,
	pub pad2: [u8;1],
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad3: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
	pub textId1: i32,
	pub textEnableFlagId1: i32,
	pub textDisableFlagId1: i32,
	pub textId2: i32,
	pub textEnableFlagId2: i32,
	pub textDisableFlagId2: i32,
	pub textId3: i32,
	pub textEnableFlagId3: i32,
	pub textDisableFlagId3: i32,
	pub textId4: i32,
	pub textEnableFlagId4: i32,
	pub textDisableFlagId4: i32,
	pub textId5: i32,
	pub textEnableFlagId5: i32,
	pub textDisableFlagId5: i32,
	pub textId6: i32,
	pub textEnableFlagId6: i32,
	pub textDisableFlagId6: i32,
	pub textId7: i32,
	pub textEnableFlagId7: i32,
	pub textDisableFlagId7: i32,
	pub textId8: i32,
	pub textEnableFlagId8: i32,
	pub textDisableFlagId8: i32,
	pub textType1: u8,
	pub textType2: u8,
	pub textType3: u8,
	pub textType4: u8,
	pub textType5: u8,
	pub textType6: u8,
	pub textType7: u8,
	pub textType8: u8,
	pub noIgnitionSfxDmypolyId_0: i32,
	pub noIgnitionSfxId_0: i32,
	pub noIgnitionSfxDmypolyId_1: i32,
	pub noIgnitionSfxId_1: i32,
	pub unkA8: i32,
	pub unkAC: i32,
	pub unkB0: i32,
	pub unkB4: i32,
	pub unkB8: i32,
	pub unkBC: i32,
	pub unkC0: i32,
	pub unkC4: i32,
	pub unkC8: i32,
	pub unkCC: i32,
	pub unkD0: i32,
	pub unkD4: i32,
	pub unkD8: i32,
	pub unkDC: i32,
	pub unkE0: i32,
	pub unkE4: i32,
	pub unkE8: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BONFIRE_WARP_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn dispMask00(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn dispMask01(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
}
impl Default for BONFIRE_WARP_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 eventflagId: 0,
			 bonfireEntityId: 0,
			 pad4: [0;2],
			 bonfireSubCategorySortId: 0,
			 forbiddenIconId: 0,
			 dispMinZoomStep: 0,
			 selectMinZoomStep: 0,
			 bonfireSubCategoryId: -1,
			 clearedEventFlagId: 0,
			 iconId: 0,
			 bits_1: 0,
			 pad2: [0;1],
			 areaNo: 0,
			 gridXNo: 0,
			 gridZNo: 0,
			 pad3: [0;1],
			 posX: 0.,
			 posY: 0.,
			 posZ: 0.,
			 textId1: -1,
			 textEnableFlagId1: 0,
			 textDisableFlagId1: 0,
			 textId2: -1,
			 textEnableFlagId2: 0,
			 textDisableFlagId2: 0,
			 textId3: -1,
			 textEnableFlagId3: 0,
			 textDisableFlagId3: 0,
			 textId4: -1,
			 textEnableFlagId4: 0,
			 textDisableFlagId4: 0,
			 textId5: -1,
			 textEnableFlagId5: 0,
			 textDisableFlagId5: 0,
			 textId6: -1,
			 textEnableFlagId6: 0,
			 textDisableFlagId6: 0,
			 textId7: -1,
			 textEnableFlagId7: 0,
			 textDisableFlagId7: 0,
			 textId8: -1,
			 textEnableFlagId8: 0,
			 textDisableFlagId8: 0,
			 textType1: 0,
			 textType2: 0,
			 textType3: 0,
			 textType4: 0,
			 textType5: 0,
			 textType6: 0,
			 textType7: 0,
			 textType8: 0,
			 noIgnitionSfxDmypolyId_0: -1,
			 noIgnitionSfxId_0: -1,
			 noIgnitionSfxDmypolyId_1: -1,
			 noIgnitionSfxId_1: -1,
			 unkA8: 0,
			 unkAC: 0,
			 unkB0: 0,
			 unkB4: 0,
			 unkB8: 0,
			 unkBC: 0,
			 unkC0: 0,
			 unkC4: 0,
			 unkC8: 0,
			 unkCC: 0,
			 unkD0: 0,
			 unkD4: 0,
			 unkD8: 0,
			 unkDC: 0,
			 unkE0: 0,
			 unkE4: 0,
			 unkE8: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BONFIRE_WARP_SUB_CATEGORY_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub textId: i32,
	pub tabId: i16,
	pub sortId: i16,
	pub pad: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BONFIRE_WARP_SUB_CATEGORY_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for BONFIRE_WARP_SUB_CATEGORY_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 textId: 0,
			 tabId: 0,
			 sortId: 0,
			 pad: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BONFIRE_WARP_TAB_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub textId: i32,
	pub sortId: i32,
	pub iconId: i16,
	pub pad: [u8;2],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BONFIRE_WARP_TAB_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for BONFIRE_WARP_TAB_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 textId: 0,
			 sortId: 0,
			 iconId: 0,
			 pad: [0;2],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BUDDY_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub triggerSpEffectId: i32,
	pub npcParamId: i32,
	pub npcThinkParamId: i32,
	pub npcParamId_ridden: i32,
	pub npcThinkParamId_ridden: i32,
	pub x_offset: f32,
	pub z_offset: f32,
	pub y_angle: f32,
	pub appearOnAroundSekihi: u8,
	pub disablePCTargetShare: u8,
	pub pcFollowType: u8,
	pub Reserve: [u8;1],
	pub dopingSpEffect_lv0: i32,
	pub dopingSpEffect_lv1: i32,
	pub dopingSpEffect_lv2: i32,
	pub dopingSpEffect_lv3: i32,
	pub dopingSpEffect_lv4: i32,
	pub dopingSpEffect_lv5: i32,
	pub dopingSpEffect_lv6: i32,
	pub dopingSpEffect_lv7: i32,
	pub dopingSpEffect_lv8: i32,
	pub dopingSpEffect_lv9: i32,
	pub dopingSpEffect_lv10: i32,
	pub npcPlayerInitParamId: i32,
	pub generateAnimId: i32,
	pub Reserve2: [u8;4],
	pub Unk1: i32,
	pub Unk2: i32,
	pub Unk3: i32,
	pub Unk4: i32,
	pub Unk5: i32,
	pub Unk6: i32,
	pub Unk7: i32,
	pub Unk8: i32,
	pub Unk9: i32,
	pub Unk10: i32,
	pub Unk11: i32,
	pub Unk12: i32,
	pub Unk13: i32,
	pub Unk14: i32,
	pub Unk15: i32,
	pub Unk16: i32,
	pub Unk17: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BUDDY_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for BUDDY_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 triggerSpEffectId: -1,
			 npcParamId: -1,
			 npcThinkParamId: -1,
			 npcParamId_ridden: -1,
			 npcThinkParamId_ridden: -1,
			 x_offset: 0.,
			 z_offset: 0.,
			 y_angle: 0.,
			 appearOnAroundSekihi: 0,
			 disablePCTargetShare: 0,
			 pcFollowType: 0,
			 Reserve: [0;1],
			 dopingSpEffect_lv0: -1,
			 dopingSpEffect_lv1: -1,
			 dopingSpEffect_lv2: -1,
			 dopingSpEffect_lv3: -1,
			 dopingSpEffect_lv4: -1,
			 dopingSpEffect_lv5: -1,
			 dopingSpEffect_lv6: -1,
			 dopingSpEffect_lv7: -1,
			 dopingSpEffect_lv8: -1,
			 dopingSpEffect_lv9: -1,
			 dopingSpEffect_lv10: -1,
			 npcPlayerInitParamId: -1,
			 generateAnimId: -1,
			 Reserve2: [0;4],
			 Unk1: 0,
			 Unk2: 0,
			 Unk3: 0,
			 Unk4: 0,
			 Unk5: 0,
			 Unk6: 0,
			 Unk7: 0,
			 Unk8: 0,
			 Unk9: 0,
			 Unk10: 0,
			 Unk11: 0,
			 Unk12: 0,
			 Unk13: 0,
			 Unk14: 0,
			 Unk15: 0,
			 Unk16: 0,
			 Unk17: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BUDDY_STONE_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub talkChrEntityId: i32,
	pub eliminateTargetEntityId: i32,
	pub summonedEventFlagId: i32,
	bits_1: u8,
	pub pad2: [u8;3],
	pub buddyId: i32,
	pub dopingSpEffectId: i32,
	pub activateRange: i16,
	pub overwriteReturnRange: i16,
	pub overwriteActivateRegionEntityId: i32,
	pub warnRegionEntityId: i32,
	pub pad3: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BUDDY_STONE_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isSpecial(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
}
impl Default for BUDDY_STONE_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 talkChrEntityId: 0,
			 eliminateTargetEntityId: 0,
			 summonedEventFlagId: 0,
			 bits_1: 0,
			 pad2: [0;3],
			 buddyId: 0,
			 dopingSpEffectId: -1,
			 activateRange: 100,
			 overwriteReturnRange: -1,
			 overwriteActivateRegionEntityId: 0,
			 warnRegionEntityId: 0,
			 pad3: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BUDGET_PARAM_ST {
	pub vram_all: f32,
	pub vram_mapobj_tex: f32,
	pub vram_mapobj_mdl: f32,
	pub vram_map: f32,
	pub vram_chr: f32,
	pub vram_parts: f32,
	pub vram_sfx: f32,
	pub vram_chr_tex: f32,
	pub vram_chr_mdl: f32,
	pub vram_parts_tex: f32,
	pub vram_parts_mdl: f32,
	pub vram_sfx_tex: f32,
	pub vram_sfx_mdl: f32,
	pub vram_gi: f32,
	pub vram_menu_tex: f32,
	pub vram_decal_rt: f32,
	pub vram_decal: f32,
	pub reserve_0: [u8;4],
	pub vram_other_tex: f32,
	pub vram_other_mdl: f32,
	pub havok_anim: f32,
	pub havok_ins: f32,
	pub havok_hit: f32,
	pub vram_other: f32,
	pub vram_detail_all: f32,
	pub vram_chr_and_parts: f32,
	pub havok_navimesh: f32,
	pub reserve_1: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BUDGET_PARAM_ST {
}
impl Default for BUDGET_PARAM_ST {
	fn default() -> Self {
		Self {
			 vram_all: 1.,
			 vram_mapobj_tex: 1.,
			 vram_mapobj_mdl: 1.,
			 vram_map: 1.,
			 vram_chr: 1.,
			 vram_parts: 1.,
			 vram_sfx: 1.,
			 vram_chr_tex: 1.,
			 vram_chr_mdl: 1.,
			 vram_parts_tex: 1.,
			 vram_parts_mdl: 1.,
			 vram_sfx_tex: 1.,
			 vram_sfx_mdl: 1.,
			 vram_gi: 1.,
			 vram_menu_tex: 1.,
			 vram_decal_rt: 1.,
			 vram_decal: 1.,
			 reserve_0: [0;4],
			 vram_other_tex: 1.,
			 vram_other_mdl: 1.,
			 havok_anim: 1.,
			 havok_ins: 1.,
			 havok_hit: 1.,
			 vram_other: 1.,
			 vram_detail_all: 1.,
			 vram_chr_and_parts: 1.,
			 havok_navimesh: 1.,
			 reserve_1: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BULLET_CREATE_LIMIT_PARAM_ST {
	pub limitNum_byGroup: u8,
	bits_0: u8,
	pub pad: [u8;30],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BULLET_CREATE_LIMIT_PARAM_ST {
	pub fn isLimitEachOwner(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn pad2(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for BULLET_CREATE_LIMIT_PARAM_ST {
	fn default() -> Self {
		Self {
			 limitNum_byGroup: 0,
			 bits_0: 0,
			 pad: [0;30],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct BULLET_PARAM_ST {
	pub atkId_Bullet: i32,
	pub sfxId_Bullet: i32,
	pub sfxId_Hit: i32,
	pub sfxId_Flick: i32,
	pub life: f32,
	pub dist: f32,
	pub shootInterval: f32,
	pub gravityInRange: f32,
	pub gravityOutRange: f32,
	pub hormingStopRange: f32,
	pub initVellocity: f32,
	pub accelInRange: f32,
	pub accelOutRange: f32,
	pub maxVellocity: f32,
	pub minVellocity: f32,
	pub accelTime: f32,
	pub homingBeginDist: f32,
	pub hitRadius: f32,
	pub hitRadiusMax: f32,
	pub spreadTime: f32,
	pub expDelay: f32,
	pub hormingOffsetRange: f32,
	pub dmgHitRecordLifeTime: f32,
	pub externalForce: f32,
	pub spEffectIDForShooter: i32,
	pub autoSearchNPCThinkID: i32,
	pub HitBulletID: i32,
	pub spEffectId0: i32,
	pub spEffectId1: i32,
	pub spEffectId2: i32,
	pub spEffectId3: i32,
	pub spEffectId4: i32,
	pub numShoot: i16,
	pub homingAngle: i16,
	pub shootAngle: i16,
	pub shootAngleInterval: i16,
	pub shootAngleXInterval: i16,
	pub damageDamp: i8,
	pub spelDamageDamp: i8,
	pub fireDamageDamp: i8,
	pub thunderDamageDamp: i8,
	pub staminaDamp: i8,
	pub knockbackDamp: i8,
	pub shootAngleXZ: i8,
	pub lockShootLimitAng: u8,
	pub pad2: [u8;1],
	pub prevVelocityDirRate: u8,
	pub atkAttribute: u8,
	pub spAttribute: u8,
	pub Material_AttackType: u8,
	pub Material_AttackMaterial: u8,
	bits_0: u8,
	pub launchConditionType: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	pub darkDamageDamp: i8,
	pub bulletSfxDeleteType_byHit: i8,
	pub bulletSfxDeleteType_byLifeDead: i8,
	pub targetYOffsetRange: f32,
	pub shootAngleYMaxRandom: f32,
	pub shootAngleXMaxRandom: f32,
	pub intervalCreateBulletId: i32,
	pub intervalCreateTimeMin: f32,
	pub intervalCreateTimeMax: f32,
	pub predictionShootObserveTime: f32,
	pub intervalCreateWaitTime: f32,
	pub sfxPostureType: u8,
	pub createLimitGroupId: u8,
	pub pad5: [u8;1],
	bits_4: u8,
	pub randomCreateRadius: f32,
	pub followOffset_BaseHeight: f32,
	pub assetNo_Hit: i32,
	pub lifeRandomRange: f32,
	pub homingAngleX: i16,
	pub ballisticCalcType: u8,
	pub attachEffectType: u8,
	pub seId_Bullet1: i32,
	pub seId_Bullet2: i32,
	pub seId_Hit: i32,
	pub seId_Flick: i32,
	pub howitzerShootAngleXMin: i16,
	pub howitzerShootAngleXMax: i16,
	pub howitzerInitMinVelocity: f32,
	pub howitzerInitMaxVelocity: f32,
	pub sfxId_ForceErase: i32,
	pub bulletSfxDeleteType_byForceErase: i8,
	pub pad3: [u8;1],
	pub followDmypoly_forSfxPose: i16,
	pub followOffset_Radius: f32,
	pub spBulletDistUpRate: f32,
	pub nolockTargetDist: f32,
	pub pad4: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl BULLET_PARAM_ST {
	pub fn isPenetrateChr(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isPenetrateObj(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn pad(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn FollowType(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn EmittePosType(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isAttackSFX(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn isEndlessHit(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn isPenetrateMap(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn isHitBothTeam(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn isUseSharedHitList(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isUseMultiDmyPolyIfPlace(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn isHitOtherBulletForceEraseA(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn isHitOtherBulletForceEraseB(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn isHitForceMagic(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn isIgnoreSfxIfHitWater(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn isIgnoreMoveStateIfHitWater(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn isHitDarkForceMagic(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn dmgCalcSide(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn isEnableAutoHoming(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn isSyncBulletCulcDumypolyPos(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn isOwnerOverrideInitAngle(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn isInheritSfxToChild(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn isInheritSpeedToChild(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn isDisableHitSfx_byChrAndObj(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn isCheckWall_byCenterRay(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn isHitFlare(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isUseBulletWallFilter(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn isNonDependenceMagicForFunnleNum(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn isAiInterruptShootNoDamageBullet(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
}
impl Default for BULLET_PARAM_ST {
	fn default() -> Self {
		Self {
			 atkId_Bullet: -1,
			 sfxId_Bullet: -1,
			 sfxId_Hit: -1,
			 sfxId_Flick: -1,
			 life: -1.,
			 dist: 0.,
			 shootInterval: 0.,
			 gravityInRange: 0.,
			 gravityOutRange: 0.,
			 hormingStopRange: 0.,
			 initVellocity: 0.,
			 accelInRange: 0.,
			 accelOutRange: 0.,
			 maxVellocity: 0.,
			 minVellocity: 0.,
			 accelTime: 0.,
			 homingBeginDist: 0.,
			 hitRadius: -1.,
			 hitRadiusMax: -1.,
			 spreadTime: 0.,
			 expDelay: 0.,
			 hormingOffsetRange: 0.,
			 dmgHitRecordLifeTime: 0.,
			 externalForce: 0.,
			 spEffectIDForShooter: -1,
			 autoSearchNPCThinkID: 0,
			 HitBulletID: -1,
			 spEffectId0: -1,
			 spEffectId1: -1,
			 spEffectId2: -1,
			 spEffectId3: -1,
			 spEffectId4: -1,
			 numShoot: 0,
			 homingAngle: 0,
			 shootAngle: 0,
			 shootAngleInterval: 0,
			 shootAngleXInterval: 0,
			 damageDamp: 0,
			 spelDamageDamp: 0,
			 fireDamageDamp: 0,
			 thunderDamageDamp: 0,
			 staminaDamp: 0,
			 knockbackDamp: 0,
			 shootAngleXZ: 0,
			 lockShootLimitAng: 0,
			 pad2: [0;1],
			 prevVelocityDirRate: 0,
			 atkAttribute: 254,
			 spAttribute: 254,
			 Material_AttackType: 254,
			 Material_AttackMaterial: 254,
			 bits_0: 0,
			 launchConditionType: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 darkDamageDamp: 0,
			 bulletSfxDeleteType_byHit: 0,
			 bulletSfxDeleteType_byLifeDead: 0,
			 targetYOffsetRange: 0.,
			 shootAngleYMaxRandom: 0.,
			 shootAngleXMaxRandom: 0.,
			 intervalCreateBulletId: -1,
			 intervalCreateTimeMin: 0.,
			 intervalCreateTimeMax: 0.,
			 predictionShootObserveTime: 0.,
			 intervalCreateWaitTime: 0.,
			 sfxPostureType: 0,
			 createLimitGroupId: 0,
			 pad5: [0;1],
			 bits_4: 0,
			 randomCreateRadius: 0.,
			 followOffset_BaseHeight: 0.,
			 assetNo_Hit: -1,
			 lifeRandomRange: 0.,
			 homingAngleX: -1,
			 ballisticCalcType: 0,
			 attachEffectType: 0,
			 seId_Bullet1: -1,
			 seId_Bullet2: -1,
			 seId_Hit: -1,
			 seId_Flick: -1,
			 howitzerShootAngleXMin: 0,
			 howitzerShootAngleXMax: 0,
			 howitzerInitMinVelocity: 0.,
			 howitzerInitMaxVelocity: 0.,
			 sfxId_ForceErase: -1,
			 bulletSfxDeleteType_byForceErase: 0,
			 pad3: [0;1],
			 followDmypoly_forSfxPose: -1,
			 followOffset_Radius: 0.,
			 spBulletDistUpRate: 1.,
			 nolockTargetDist: 0.,
			 pad4: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CACL_CORRECT_GRAPH_ST {
	pub stageMaxVal0: f32,
	pub stageMaxVal1: f32,
	pub stageMaxVal2: f32,
	pub stageMaxVal3: f32,
	pub stageMaxVal4: f32,
	pub stageMaxGrowVal0: f32,
	pub stageMaxGrowVal1: f32,
	pub stageMaxGrowVal2: f32,
	pub stageMaxGrowVal3: f32,
	pub stageMaxGrowVal4: f32,
	pub adjPt_maxGrowVal0: f32,
	pub adjPt_maxGrowVal1: f32,
	pub adjPt_maxGrowVal2: f32,
	pub adjPt_maxGrowVal3: f32,
	pub adjPt_maxGrowVal4: f32,
	pub init_inclination_soul: f32,
	pub adjustment_value: f32,
	pub boundry_inclination_soul: f32,
	pub boundry_value: f32,
	pub pad: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CACL_CORRECT_GRAPH_ST {
}
impl Default for CACL_CORRECT_GRAPH_ST {
	fn default() -> Self {
		Self {
			 stageMaxVal0: 0.,
			 stageMaxVal1: 0.,
			 stageMaxVal2: 0.,
			 stageMaxVal3: 0.,
			 stageMaxVal4: 0.,
			 stageMaxGrowVal0: 0.,
			 stageMaxGrowVal1: 0.,
			 stageMaxGrowVal2: 0.,
			 stageMaxGrowVal3: 0.,
			 stageMaxGrowVal4: 0.,
			 adjPt_maxGrowVal0: 0.,
			 adjPt_maxGrowVal1: 0.,
			 adjPt_maxGrowVal2: 0.,
			 adjPt_maxGrowVal3: 0.,
			 adjPt_maxGrowVal4: 0.,
			 init_inclination_soul: 0.,
			 adjustment_value: 0.,
			 boundry_inclination_soul: 0.,
			 boundry_value: 0.,
			 pad: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CAMERA_FADE_PARAM_ST {
	pub NearMinDist: f32,
	pub NearMaxDist: f32,
	pub FarMinDist: f32,
	pub FarMaxDist: f32,
	pub MiddleAlpha: f32,
	pub dummy: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CAMERA_FADE_PARAM_ST {
}
impl Default for CAMERA_FADE_PARAM_ST {
	fn default() -> Self {
		Self {
			 NearMinDist: 0.,
			 NearMaxDist: 0.,
			 FarMinDist: 0.,
			 FarMaxDist: 0.,
			 MiddleAlpha: 0.,
			 dummy: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CEREMONY_PARAM_ST {
	pub eventLayerId: i32,
	pub mapStudioLayerId: i32,
	pub multiPlayAreaOffset: i32,
	pub overrideMapPlaceNameId: i32,
	pub overrideSaveMapNameId: i32,
	pub pad2: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CEREMONY_PARAM_ST {
}
impl Default for CEREMONY_PARAM_ST {
	fn default() -> Self {
		Self {
			 eventLayerId: 0,
			 mapStudioLayerId: 0,
			 multiPlayAreaOffset: 0,
			 overrideMapPlaceNameId: -1,
			 overrideSaveMapNameId: -1,
			 pad2: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CHARACTER_INIT_PARAM {
	pub baseRec_mp: f32,
	pub baseRec_sp: f32,
	pub red_Falldam: f32,
	pub soul: i32,
	pub equip_Wep_Right: i32,
	pub equip_Subwep_Right: i32,
	pub equip_Wep_Left: i32,
	pub equip_Subwep_Left: i32,
	pub equip_Helm: i32,
	pub equip_Armer: i32,
	pub equip_Gaunt: i32,
	pub equip_Leg: i32,
	pub equip_Arrow: i32,
	pub equip_Bolt: i32,
	pub equip_SubArrow: i32,
	pub equip_SubBolt: i32,
	pub equip_Accessory01: i32,
	pub equip_Accessory02: i32,
	pub equip_Accessory03: i32,
	pub equip_Accessory04: i32,
	pub pad8: [u8;4],
	pub elixir_material00: i32,
	pub elixir_material01: i32,
	pub elixir_material02: i32,
	pub equip_Spell_01: i32,
	pub equip_Spell_02: i32,
	pub equip_Spell_03: i32,
	pub equip_Spell_04: i32,
	pub equip_Spell_05: i32,
	pub equip_Spell_06: i32,
	pub equip_Spell_07: i32,
	pub item_01: i32,
	pub item_02: i32,
	pub item_03: i32,
	pub item_04: i32,
	pub item_05: i32,
	pub item_06: i32,
	pub item_07: i32,
	pub item_08: i32,
	pub item_09: i32,
	pub item_10: i32,
	pub npcPlayerFaceGenId: i32,
	pub npcPlayerThinkId: i32,
	pub baseHp: i16,
	pub baseMp: i16,
	pub baseSp: i16,
	pub arrowNum: i16,
	pub boltNum: i16,
	pub subArrowNum: i16,
	pub subBoltNum: i16,
	pub pad4: [u8;6],
	pub soulLv: i16,
	pub baseVit: u8,
	pub baseWil: u8,
	pub baseEnd: u8,
	pub baseStr: u8,
	pub baseDex: u8,
	pub baseMag: u8,
	pub baseFai: u8,
	pub baseLuc: u8,
	pub baseHeroPoint: u8,
	pub baseDurability: u8,
	pub itemNum_01: u8,
	pub itemNum_02: u8,
	pub itemNum_03: u8,
	pub itemNum_04: u8,
	pub itemNum_05: u8,
	pub itemNum_06: u8,
	pub itemNum_07: u8,
	pub itemNum_08: u8,
	pub itemNum_09: u8,
	pub itemNum_10: u8,
	pub pad5: [u8;5],
	pub gestureId0: i8,
	pub gestureId1: i8,
	pub gestureId2: i8,
	pub gestureId3: i8,
	pub gestureId4: i8,
	pub gestureId5: i8,
	pub gestureId6: i8,
	pub npcPlayerType: u8,
	pub npcPlayerDrawType: i8,
	pub npcPlayerSex: u8,
	bits_0: u8,
	pub pad6: [u8;2],
	pub wepParamType_Right1: u8,
	pub wepParamType_Right2: u8,
	pub wepParamType_Right3: u8,
	pub wepParamType_Left1: u8,
	pub wepParamType_Left2: u8,
	pub wepParamType_Left3: u8,
	pub pad2: [u8;26],
	pub equip_Subwep_Right3: i32,
	pub equip_Subwep_Left3: i32,
	pub pad3: [u8;4],
	pub secondaryItem_01: i32,
	pub secondaryItem_02: i32,
	pub secondaryItem_03: i32,
	pub secondaryItem_04: i32,
	pub secondaryItem_05: i32,
	pub secondaryItem_06: i32,
	pub secondaryItemNum_01: u8,
	pub secondaryItemNum_02: u8,
	pub secondaryItemNum_03: u8,
	pub secondaryItemNum_04: u8,
	pub secondaryItemNum_05: u8,
	pub secondaryItemNum_06: u8,
	pub HpEstMax: i8,
	pub MpEstMax: i8,
	pub pad7: [u8;5],
	pub voiceType: u8,
	pub reserve: [u8;6],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CHARACTER_INIT_PARAM {
	pub fn vowType(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isSyncTarget(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn pad(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
}
impl Default for CHARACTER_INIT_PARAM {
	fn default() -> Self {
		Self {
			 baseRec_mp: 0.,
			 baseRec_sp: 0.,
			 red_Falldam: 0.,
			 soul: 0,
			 equip_Wep_Right: -1,
			 equip_Subwep_Right: -1,
			 equip_Wep_Left: -1,
			 equip_Subwep_Left: -1,
			 equip_Helm: -1,
			 equip_Armer: -1,
			 equip_Gaunt: -1,
			 equip_Leg: -1,
			 equip_Arrow: -1,
			 equip_Bolt: -1,
			 equip_SubArrow: -1,
			 equip_SubBolt: -1,
			 equip_Accessory01: -1,
			 equip_Accessory02: -1,
			 equip_Accessory03: -1,
			 equip_Accessory04: -1,
			 pad8: [0;4],
			 elixir_material00: -1,
			 elixir_material01: -1,
			 elixir_material02: -1,
			 equip_Spell_01: -1,
			 equip_Spell_02: -1,
			 equip_Spell_03: -1,
			 equip_Spell_04: -1,
			 equip_Spell_05: -1,
			 equip_Spell_06: -1,
			 equip_Spell_07: -1,
			 item_01: -1,
			 item_02: -1,
			 item_03: -1,
			 item_04: -1,
			 item_05: -1,
			 item_06: -1,
			 item_07: -1,
			 item_08: -1,
			 item_09: -1,
			 item_10: -1,
			 npcPlayerFaceGenId: 0,
			 npcPlayerThinkId: 0,
			 baseHp: 0,
			 baseMp: 0,
			 baseSp: 0,
			 arrowNum: 0,
			 boltNum: 0,
			 subArrowNum: 0,
			 subBoltNum: 0,
			 pad4: [0;6],
			 soulLv: 0,
			 baseVit: 0,
			 baseWil: 0,
			 baseEnd: 0,
			 baseStr: 0,
			 baseDex: 0,
			 baseMag: 0,
			 baseFai: 0,
			 baseLuc: 0,
			 baseHeroPoint: 0,
			 baseDurability: 0,
			 itemNum_01: 0,
			 itemNum_02: 0,
			 itemNum_03: 0,
			 itemNum_04: 0,
			 itemNum_05: 0,
			 itemNum_06: 0,
			 itemNum_07: 0,
			 itemNum_08: 0,
			 itemNum_09: 0,
			 itemNum_10: 0,
			 pad5: [0;5],
			 gestureId0: -1,
			 gestureId1: -1,
			 gestureId2: -1,
			 gestureId3: -1,
			 gestureId4: -1,
			 gestureId5: -1,
			 gestureId6: -1,
			 npcPlayerType: 0,
			 npcPlayerDrawType: 0,
			 npcPlayerSex: 0,
			 bits_0: 0,
			 pad6: [0;2],
			 wepParamType_Right1: 0,
			 wepParamType_Right2: 0,
			 wepParamType_Right3: 0,
			 wepParamType_Left1: 0,
			 wepParamType_Left2: 0,
			 wepParamType_Left3: 0,
			 pad2: [0;26],
			 equip_Subwep_Right3: -1,
			 equip_Subwep_Left3: -1,
			 pad3: [0;4],
			 secondaryItem_01: -1,
			 secondaryItem_02: -1,
			 secondaryItem_03: -1,
			 secondaryItem_04: -1,
			 secondaryItem_05: -1,
			 secondaryItem_06: -1,
			 secondaryItemNum_01: 0,
			 secondaryItemNum_02: 0,
			 secondaryItemNum_03: 0,
			 secondaryItemNum_04: 0,
			 secondaryItemNum_05: 0,
			 secondaryItemNum_06: 0,
			 HpEstMax: -1,
			 MpEstMax: -1,
			 pad7: [0;5],
			 voiceType: 0,
			 reserve: [0;6],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CHARMAKEMENU_LISTITEM_PARAM_ST {
	pub value: i32,
	pub captionId: i32,
	pub iconId: u8,
	pub reserved: [u8;7],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CHARMAKEMENU_LISTITEM_PARAM_ST {
}
impl Default for CHARMAKEMENU_LISTITEM_PARAM_ST {
	fn default() -> Self {
		Self {
			 value: 0,
			 captionId: 0,
			 iconId: 0,
			 reserved: [0;7],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CHARMAKEMENUTOP_PARAM_ST {
	pub commandType: i32,
	pub captionId: i32,
	pub faceParamId: i32,
	pub tableId: i32,
	pub viewCondition: i32,
	pub previewMode: i8,
	pub reserved2: [u8;3],
	pub tableId2: i32,
	pub refFaceParamId: i32,
	pub refTextId: i32,
	pub helpTextId: i32,
	pub unlockEventFlagId: i32,
	pub reserved: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CHARMAKEMENUTOP_PARAM_ST {
}
impl Default for CHARMAKEMENUTOP_PARAM_ST {
	fn default() -> Self {
		Self {
			 commandType: 0,
			 captionId: 0,
			 faceParamId: 0,
			 tableId: 0,
			 viewCondition: 0,
			 previewMode: 0,
			 reserved2: [0;3],
			 tableId2: -1,
			 refFaceParamId: -1,
			 refTextId: -1,
			 helpTextId: -1,
			 unlockEventFlagId: 0,
			 reserved: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CHR_ACTIVATE_CONDITION_PARAM_ST {
	bits_0: u8,
	bits_1: u8,
	pub timeStartHour: u8,
	pub timeStartMin: u8,
	pub timeEndHour: u8,
	pub timeEndMin: u8,
	pub pad2: [u8;2],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CHR_ACTIVATE_CONDITION_PARAM_ST {
	pub fn weatherSunny(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn weatherClearSky(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn weatherWeakCloudy(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn weatherCloudy(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn weatherRain(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn weatherHeavyRain(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn weatherStorm(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn weatherStormForBattle(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn weatherSnow(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn weatherHeavySnow(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn weatherFog(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn weatherHeavyFog(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn weatherHeavyFogRain(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn weatherSandStorm(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
}
impl Default for CHR_ACTIVATE_CONDITION_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 1,
			 bits_1: 1,
			 timeStartHour: 0,
			 timeStartMin: 0,
			 timeEndHour: 0,
			 timeEndMin: 0,
			 pad2: [0;2],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CHR_MODEL_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub modelMemoryType: u8,
	pub texMemoryType: u8,
	pub cameraDitherFadeId: i16,
	pub reportAnimMemSizeMb: f32,
	pub unk: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CHR_MODEL_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for CHR_MODEL_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 modelMemoryType: 0,
			 texMemoryType: 0,
			 cameraDitherFadeId: 0,
			 reportAnimMemSizeMb: 12.,
			 unk: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CLEAR_COUNT_CORRECT_PARAM_ST {
	pub MaxHpRate: f32,
	pub MaxMpRate: f32,
	pub MaxStaminaRate: f32,
	pub PhysicsAttackRate: f32,
	pub SlashAttackRate: f32,
	pub BlowAttackRate: f32,
	pub ThrustAttackRate: f32,
	pub NeturalAttackRate: f32,
	pub MagicAttackRate: f32,
	pub FireAttackRate: f32,
	pub ThunderAttackRate: f32,
	pub DarkAttackRate: f32,
	pub PhysicsDefenseRate: f32,
	pub MagicDefenseRate: f32,
	pub FireDefenseRate: f32,
	pub ThunderDefenseRate: f32,
	pub DarkDefenseRate: f32,
	pub StaminaAttackRate: f32,
	pub SoulRate: f32,
	pub PoisionResistRate: f32,
	pub DiseaseResistRate: f32,
	pub BloodResistRate: f32,
	pub CurseResistRate: f32,
	pub FreezeResistRate: f32,
	pub BloodDamageRate: f32,
	pub SuperArmorDamageRate: f32,
	pub FreezeDamageRate: f32,
	pub SleepResistRate: f32,
	pub MadnessResistRate: f32,
	pub SleepDamageRate: f32,
	pub MadnessDamageRate: f32,
	pub pad1: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CLEAR_COUNT_CORRECT_PARAM_ST {
}
impl Default for CLEAR_COUNT_CORRECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 MaxHpRate: 1.,
			 MaxMpRate: 1.,
			 MaxStaminaRate: 1.,
			 PhysicsAttackRate: 1.,
			 SlashAttackRate: 1.,
			 BlowAttackRate: 1.,
			 ThrustAttackRate: 1.,
			 NeturalAttackRate: 1.,
			 MagicAttackRate: 1.,
			 FireAttackRate: 1.,
			 ThunderAttackRate: 1.,
			 DarkAttackRate: 1.,
			 PhysicsDefenseRate: 1.,
			 MagicDefenseRate: 1.,
			 FireDefenseRate: 1.,
			 ThunderDefenseRate: 1.,
			 DarkDefenseRate: 1.,
			 StaminaAttackRate: 1.,
			 SoulRate: 1.,
			 PoisionResistRate: 1.,
			 DiseaseResistRate: 1.,
			 BloodResistRate: 1.,
			 CurseResistRate: 1.,
			 FreezeResistRate: 1.,
			 BloodDamageRate: 1.,
			 SuperArmorDamageRate: 1.,
			 FreezeDamageRate: 1.,
			 SleepResistRate: 1.,
			 MadnessResistRate: 1.,
			 SleepDamageRate: 1.,
			 MadnessDamageRate: 1.,
			 pad1: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct COMMON_SYSTEM_PARAM_ST {
	pub mapSaveMapNameIdOnGameStart: i32,
	pub reserve0: [u8;60],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl COMMON_SYSTEM_PARAM_ST {
}
impl Default for COMMON_SYSTEM_PARAM_ST {
	fn default() -> Self {
		Self {
			 mapSaveMapNameIdOnGameStart: 0,
			 reserve0: [0;60],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct COOL_TIME_PARAM_ST {
	pub limitationTime_0: f32,
	pub observeTime_0: f32,
	pub limitationTime_1: f32,
	pub observeTime_1: f32,
	pub limitationTime_2: f32,
	pub observeTime_2: f32,
	pub limitationTime_3: f32,
	pub observeTime_3: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl COOL_TIME_PARAM_ST {
}
impl Default for COOL_TIME_PARAM_ST {
	fn default() -> Self {
		Self {
			 limitationTime_0: 0.,
			 observeTime_0: 0.,
			 limitationTime_1: 0.,
			 observeTime_1: 0.,
			 limitationTime_2: 0.,
			 observeTime_2: 0.,
			 limitationTime_3: 0.,
			 observeTime_3: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CUTSCENE_GPARAM_TIME_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub DstTimezone_Morning: u8,
	pub DstTimezone_Noon: u8,
	pub DstTimezone_AfterNoon: u8,
	pub DstTimezone_Evening: u8,
	pub DstTimezone_Night: u8,
	pub DstTimezone_DeepNightA: u8,
	pub DstTimezone_DeepNightB: u8,
	pub reserved: [u8;1],
	pub PostPlayIngameTime: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CUTSCENE_GPARAM_TIME_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParam_Debug(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
}
impl Default for CUTSCENE_GPARAM_TIME_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 DstTimezone_Morning: 0,
			 DstTimezone_Noon: 0,
			 DstTimezone_AfterNoon: 0,
			 DstTimezone_Evening: 0,
			 DstTimezone_Night: 0,
			 DstTimezone_DeepNightA: 0,
			 DstTimezone_DeepNightB: 0,
			 reserved: [0;1],
			 PostPlayIngameTime: -1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CUTSCENE_GPARAM_WEATHER_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub DstWeather_Sunny: i16,
	pub DstWeather_ClearSky: i16,
	pub DstWeather_WeakCloudy: i16,
	pub DstWeather_Cloud: i16,
	pub DstWeather_Rain: i16,
	pub DstWeather_HeavyRain: i16,
	pub DstWeather_Storm: i16,
	pub DstWeather_StormForBattle: i16,
	pub DstWeather_Snow: i16,
	pub DstWeather_HeavySnow: i16,
	pub DstWeather_Fog: i16,
	pub DstWeather_HeavyFog: i16,
	pub DstWeather_SandStorm: i16,
	pub DstWeather_HeavyFogRain: i16,
	pub PostPlayIngameWeather: i16,
	pub IndoorOutdoorType: u8,
	pub TakeOverDstWeather_Sunny: u8,
	pub TakeOverDstWeather_ClearSky: u8,
	pub TakeOverDstWeather_WeakCloudy: u8,
	pub TakeOverDstWeather_Cloud: u8,
	pub TakeOverDstWeather_Rain: u8,
	pub TakeOverDstWeather_HeavyRain: u8,
	pub TakeOverDstWeather_Storm: u8,
	pub TakeOverDstWeather_StormForBattle: u8,
	pub TakeOverDstWeather_Snow: u8,
	pub TakeOverDstWeather_HeavySnow: u8,
	pub TakeOverDstWeather_Fog: u8,
	pub TakeOverDstWeather_HeavyFog: u8,
	pub TakeOverDstWeather_SandStorm: u8,
	pub TakeOverDstWeather_HeavyFogRain: u8,
	pub reserved: [u8;7],
	pub DstWeather_Snowstorm: i16,
	pub DstWeather_LightningStorm: i16,
	pub DstWeather_Reserved3: i16,
	pub DstWeather_Reserved4: i16,
	pub DstWeather_Reserved5: i16,
	pub DstWeather_Reserved6: i16,
	pub DstWeather_Reserved7: i16,
	pub DstWeather_Reserved8: i16,
	pub TakeOverDstWeather_Snowstorm: u8,
	pub TakeOverDstWeather_LightningStorm: u8,
	pub TakeOverDstWeather_Reserved3: u8,
	pub TakeOverDstWeather_Reserved4: u8,
	pub TakeOverDstWeather_Reserved5: u8,
	pub TakeOverDstWeather_Reserved6: u8,
	pub TakeOverDstWeather_Reserved7: u8,
	pub TakeOverDstWeather_Reserved8: u8,
	pub IsEnableApplyMapGdRegionIdForGparam: u8,
	pub reserved2: [u8;1],
	pub OverrideMapGdRegionId: i16,
	pub reserved1: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CUTSCENE_GPARAM_WEATHER_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParam_Debug(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
}
impl Default for CUTSCENE_GPARAM_WEATHER_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 DstWeather_Sunny: 0,
			 DstWeather_ClearSky: 0,
			 DstWeather_WeakCloudy: 0,
			 DstWeather_Cloud: 0,
			 DstWeather_Rain: 0,
			 DstWeather_HeavyRain: 0,
			 DstWeather_Storm: 0,
			 DstWeather_StormForBattle: 0,
			 DstWeather_Snow: 0,
			 DstWeather_HeavySnow: 0,
			 DstWeather_Fog: 0,
			 DstWeather_HeavyFog: 0,
			 DstWeather_SandStorm: 0,
			 DstWeather_HeavyFogRain: 0,
			 PostPlayIngameWeather: -1,
			 IndoorOutdoorType: 0,
			 TakeOverDstWeather_Sunny: 1,
			 TakeOverDstWeather_ClearSky: 1,
			 TakeOverDstWeather_WeakCloudy: 1,
			 TakeOverDstWeather_Cloud: 1,
			 TakeOverDstWeather_Rain: 1,
			 TakeOverDstWeather_HeavyRain: 1,
			 TakeOverDstWeather_Storm: 1,
			 TakeOverDstWeather_StormForBattle: 1,
			 TakeOverDstWeather_Snow: 1,
			 TakeOverDstWeather_HeavySnow: 1,
			 TakeOverDstWeather_Fog: 1,
			 TakeOverDstWeather_HeavyFog: 1,
			 TakeOverDstWeather_SandStorm: 1,
			 TakeOverDstWeather_HeavyFogRain: 1,
			 reserved: [0;7],
			 DstWeather_Snowstorm: 0,
			 DstWeather_LightningStorm: 0,
			 DstWeather_Reserved3: 0,
			 DstWeather_Reserved4: 0,
			 DstWeather_Reserved5: 0,
			 DstWeather_Reserved6: 0,
			 DstWeather_Reserved7: 0,
			 DstWeather_Reserved8: 0,
			 TakeOverDstWeather_Snowstorm: 1,
			 TakeOverDstWeather_LightningStorm: 1,
			 TakeOverDstWeather_Reserved3: 1,
			 TakeOverDstWeather_Reserved4: 1,
			 TakeOverDstWeather_Reserved5: 1,
			 TakeOverDstWeather_Reserved6: 1,
			 TakeOverDstWeather_Reserved7: 1,
			 TakeOverDstWeather_Reserved8: 1,
			 IsEnableApplyMapGdRegionIdForGparam: 0,
			 reserved2: [0;1],
			 OverrideMapGdRegionId: -1,
			 reserved1: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CUTSCENE_MAP_ID_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub PlayMapId: i32,
	pub RequireMapId0: i32,
	pub RequireMapId1: i32,
	pub RequireMapId2: i32,
	pub RefCamPosHitPartsID: i32,
	pub reserved_2: [u8;12],
	pub ClientDisableViewTimeForProgress: i16,
	pub reserved: [u8;2],
	pub HitParts_0: i32,
	pub HitParts_1: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CUTSCENE_MAP_ID_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParam_Debug(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
}
impl Default for CUTSCENE_MAP_ID_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 PlayMapId: 0,
			 RequireMapId0: 0,
			 RequireMapId1: 0,
			 RequireMapId2: 0,
			 RefCamPosHitPartsID: -1,
			 reserved_2: [0;12],
			 ClientDisableViewTimeForProgress: 0,
			 reserved: [0;2],
			 HitParts_0: -1,
			 HitParts_1: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CUTSCENE_TEXTURE_LOAD_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub texName_00: [u8;16],
	pub texName_01: [u8;16],
	pub texName_02: [u8;16],
	pub texName_03: [u8;16],
	pub texName_04: [u8;16],
	pub texName_05: [u8;16],
	pub texName_06: [u8;16],
	pub texName_07: [u8;16],
	pub texName_08: [u8;16],
	pub texName_09: [u8;16],
	pub texName_10: [u8;16],
	pub texName_11: [u8;16],
	pub texName_12: [u8;16],
	pub texName_13: [u8;16],
	pub texName_14: [u8;16],
	pub texName_15: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CUTSCENE_TEXTURE_LOAD_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParam_Debug(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
}
impl Default for CUTSCENE_TEXTURE_LOAD_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 texName_00: [0;16],
			 texName_01: [0;16],
			 texName_02: [0;16],
			 texName_03: [0;16],
			 texName_04: [0;16],
			 texName_05: [0;16],
			 texName_06: [0;16],
			 texName_07: [0;16],
			 texName_08: [0;16],
			 texName_09: [0;16],
			 texName_10: [0;16],
			 texName_11: [0;16],
			 texName_12: [0;16],
			 texName_13: [0;16],
			 texName_14: [0;16],
			 texName_15: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CUTSCENE_TIMEZONE_CONVERT_PARAM_ST {
	pub SrcTimezoneStart: f32,
	pub DstCutscenTime: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CUTSCENE_TIMEZONE_CONVERT_PARAM_ST {
}
impl Default for CUTSCENE_TIMEZONE_CONVERT_PARAM_ST {
	fn default() -> Self {
		Self {
			 SrcTimezoneStart: 0.,
			 DstCutscenTime: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST {
	pub weatherOverrideGparamId: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST {
}
impl Default for CUTSCENE_WEATHER_OVERRIDE_GPARAM_ID_CONVERT_PARAM_ST {
	fn default() -> Self {
		Self {
			 weatherOverrideGparamId: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct DECAL_PARAM_ST {
	pub textureId: i32,
	pub dmypolyId: i32,
	pub pitchAngle: f32,
	pub yawAngle: f32,
	pub nearDistance: f32,
	pub farDistance: f32,
	pub nearSize: f32,
	pub farSize: f32,
	pub maskSpeffectId: i32,
	bits_0: i32,
	pub randomSizeMax: i16,
	pub randomRollMin: f32,
	pub randomRollMax: f32,
	pub randomPitchMin: f32,
	pub randomPitchMax: f32,
	pub randomYawMin: f32,
	pub randomYawMax: f32,
	pub pomHightScale: f32,
	pub pomSampleMin: u8,
	pub pomSampleMax: u8,
	pub blendMode: i8,
	pub appearDirType: i8,
	pub emissiveValueBegin: f32,
	pub emissiveValueEnd: f32,
	pub emissiveTime: f32,
	pub bIntpEnable: u8,
	pub pad_01: [u8;3],
	pub intpIntervalDist: f32,
	pub beginIntpTextureId: i32,
	pub endIntpTextureId: i32,
	pub appearSfxId: i32,
	pub appearSfxOffsetPos: f32,
	pub maskTextureId: i32,
	pub diffuseTextureId: i32,
	pub reflecTextureId: i32,
	pub maskScale: f32,
	pub normalTextureId: i32,
	pub heightTextureId: i32,
	pub emissiveTextureId: i32,
	pub diffuseColorR: u8,
	pub diffuseColorG: u8,
	pub diffuseColorB: u8,
	pub pad_03: [u8;1],
	pub reflecColorR: u8,
	pub reflecColorG: u8,
	pub reflecColorB: u8,
	pub bLifeEnable: u8,
	pub siniScale: f32,
	pub lifeTimeSec: f32,
	pub fadeOutTimeSec: f32,
	pub priority: i16,
	pub bDistThinOutEnable: u8,
	pub bAlignedTexRandomVariationEnable: u8,
	pub distThinOutCheckDist: f32,
	pub distThinOutCheckAngleDeg: f32,
	pub distThinOutMaxNum: u8,
	pub distThinOutCheckNum: u8,
	pub delayAppearFrame: i16,
	bits_1: i32,
	pub fadeInTimeSec: f32,
	pub thinOutOverlapMultiRadius: f32,
	pub thinOutNeighborAddRadius: f32,
	pub thinOutOverlapLimitNum: i32,
	pub thinOutNeighborLimitNum: i32,
	pub thinOutMode: i8,
	pub emissiveColorR: u8,
	pub emissiveColorG: u8,
	pub emissiveColorB: u8,
	pub maxDecalSfxCreatableSlopeAngleDeg: f32,
	pub pad_02: [u8;40],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl DECAL_PARAM_ST {
	pub fn pad_10(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn replaceTextureId_byMaterial(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn dmypolyCategory(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn pad_05(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn useDeferredDecal(&self) -> bool {	
			self.bits_0 & (1 << 11) != 0
	}
	pub fn usePaintDecal(&self) -> bool {	
			self.bits_0 & (1 << 12) != 0
	}
	pub fn bloodTypeEnable(&self) -> bool {	
			self.bits_0 & (1 << 13) != 0
	}
	pub fn bUseNormal(&self) -> bool {	
			self.bits_0 & (1 << 14) != 0
	}
	pub fn pad_08(&self) -> bool {	
			self.bits_0 & (1 << 15) != 0
	}
	pub fn pad_09(&self) -> bool {	
			self.bits_0 & (1 << 16) != 0
	}
	pub fn usePom(&self) -> bool {	
			self.bits_0 & (1 << 17) != 0
	}
	pub fn useEmissive(&self) -> bool {	
			self.bits_0 & (1 << 18) != 0
	}
	pub fn putVertical(&self) -> bool {	
			self.bits_0 & (1 << 19) != 0
	}
	pub fn randVaria_Diffuse(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn randVaria_Mask(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn randVaria_Reflec(&self) -> bool {	
			self.bits_0 & (1 << 8) != 0
	}
	pub fn pad_12(&self) -> bool {	
			self.bits_0 & (1 << 12) != 0
	}
	pub fn randVaria_Normal(&self) -> bool {	
			self.bits_0 & (1 << 16) != 0
	}
	pub fn randVaria_Height(&self) -> bool {	
			self.bits_0 & (1 << 20) != 0
	}
	pub fn randVaria_Emissive(&self) -> bool {	
			self.bits_0 & (1 << 24) != 0
	}
	pub fn pad_11(&self) -> bool {	
			self.bits_0 & (1 << 28) != 0
	}
}
impl Default for DECAL_PARAM_ST {
	fn default() -> Self {
		Self {
			 textureId: -1,
			 dmypolyId: -1,
			 pitchAngle: 0.,
			 yawAngle: 0.,
			 nearDistance: 0.,
			 farDistance: 0.,
			 nearSize: 0.,
			 farSize: 0.,
			 maskSpeffectId: -1,
			 bits_0: 0,
			 randomSizeMax: 100,
			 randomRollMin: 0.,
			 randomRollMax: 0.,
			 randomPitchMin: 0.,
			 randomPitchMax: 0.,
			 randomYawMin: 0.,
			 randomYawMax: 0.,
			 pomHightScale: 1.,
			 pomSampleMin: 8,
			 pomSampleMax: 64,
			 blendMode: 1,
			 appearDirType: 0,
			 emissiveValueBegin: 1.,
			 emissiveValueEnd: 1.,
			 emissiveTime: 0.,
			 bIntpEnable: 0,
			 pad_01: [0;3],
			 intpIntervalDist: 0.1,
			 beginIntpTextureId: -1,
			 endIntpTextureId: -1,
			 appearSfxId: -1,
			 appearSfxOffsetPos: 0.,
			 maskTextureId: -1,
			 diffuseTextureId: -1,
			 reflecTextureId: -1,
			 maskScale: 1.,
			 normalTextureId: -1,
			 heightTextureId: -1,
			 emissiveTextureId: -1,
			 diffuseColorR: 255,
			 diffuseColorG: 255,
			 diffuseColorB: 255,
			 pad_03: [0;1],
			 reflecColorR: 255,
			 reflecColorG: 255,
			 reflecColorB: 255,
			 bLifeEnable: 0,
			 siniScale: 1.,
			 lifeTimeSec: 0.,
			 fadeOutTimeSec: 0.,
			 priority: -1,
			 bDistThinOutEnable: 0,
			 bAlignedTexRandomVariationEnable: 0,
			 distThinOutCheckDist: 0.,
			 distThinOutCheckAngleDeg: 0.,
			 distThinOutMaxNum: 1,
			 distThinOutCheckNum: 1,
			 delayAppearFrame: 0,
			 bits_1: 0,
			 fadeInTimeSec: 0.,
			 thinOutOverlapMultiRadius: 0.,
			 thinOutNeighborAddRadius: 0.,
			 thinOutOverlapLimitNum: 0,
			 thinOutNeighborLimitNum: 0,
			 thinOutMode: 0,
			 emissiveColorR: 255,
			 emissiveColorG: 255,
			 emissiveColorB: 255,
			 maxDecalSfxCreatableSlopeAngleDeg: -1.,
			 pad_02: [0;40],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct DEFAULT_KEY_ASSIGN {
	bits_0: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	pub dummy: [u8;12],
	pub phyisicalKey_0: i32,
	pub traitsType_0: u8,
	pub a2dOperator_0: u8,
	pub applyTarget_0: u8,
	bits_4: u8,
	pub time2_0: f32,
	pub a2dThreshold_0: f32,
	pub phyisicalKey_1: i32,
	pub traitsType_1: u8,
	pub a2dOperator_1: u8,
	pub applyTarget_1: u8,
	bits_5: u8,
	pub time2_1: f32,
	pub a2dThreshold_1: f32,
	pub phyisicalKey_2: i32,
	pub traitsType_2: u8,
	pub a2dOperator_2: u8,
	pub applyTarget_2: u8,
	bits_6: u8,
	pub time2_2: f32,
	pub a2dThreshold_2: f32,
	pub phyisicalKey_3: i32,
	pub traitsType_3: u8,
	pub a2dOperator_3: u8,
	pub applyTarget_3: u8,
	bits_7: u8,
	pub time2_3: f32,
	pub a2dThreshold_3: f32,
	pub phyisicalKey_4: i32,
	pub traitsType_4: u8,
	pub a2dOperator_4: u8,
	pub applyTarget_4: u8,
	bits_8: u8,
	pub time2_4: f32,
	pub a2dThreshold_4: f32,
	pub phyisicalKey_5: i32,
	pub traitsType_5: u8,
	pub a2dOperator_5: u8,
	pub applyTarget_5: u8,
	bits_9: u8,
	pub time2_5: f32,
	pub a2dThreshold_5: f32,
	pub phyisicalKey_6: i32,
	pub traitsType_6: u8,
	pub a2dOperator_6: u8,
	pub applyTarget_6: u8,
	bits_10: u8,
	pub time2_6: f32,
	pub a2dThreshold_6: f32,
	pub phyisicalKey_7: i32,
	pub traitsType_7: u8,
	pub a2dOperator_7: u8,
	pub applyTarget_7: u8,
	bits_11: u8,
	pub time2_7: f32,
	pub a2dThreshold_7: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl DEFAULT_KEY_ASSIGN {
	pub fn priority0(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn priority1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn priority2(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn priority3(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn priority4(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn priority5(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn priority6(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn priority7(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn priority8(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn priority9(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn priority10(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn priority11(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn priority12(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn priority13(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn priority14(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn priority15(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn priority16(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn priority17(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn priority18(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn priority19(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn priority20(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn priority21(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn priority22(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn priority23(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn priority24(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn priority25(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn priority26(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn priority27(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn priority28(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn priority29(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn priority30(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn priority31(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn isAnalog_0(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_0(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_0(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_0(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_1(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_1(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_1(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_1(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_2(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_2(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_2(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_2(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_3(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_3(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_3(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_3(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_4(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_4(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_4(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_4(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_5(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_5(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_5(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_5(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_6(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_6(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_6(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_6(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isAnalog_7(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn enableWin64_7(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enablePS4_7(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn enableXboxOne_7(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
}
impl Default for DEFAULT_KEY_ASSIGN {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 dummy: [0;12],
			 phyisicalKey_0: -1,
			 traitsType_0: 0,
			 a2dOperator_0: 0,
			 applyTarget_0: 0,
			 bits_4: 0,
			 time2_0: 0.,
			 a2dThreshold_0: 0.5,
			 phyisicalKey_1: -1,
			 traitsType_1: 0,
			 a2dOperator_1: 0,
			 applyTarget_1: 0,
			 bits_5: 0,
			 time2_1: 0.,
			 a2dThreshold_1: 0.5,
			 phyisicalKey_2: -1,
			 traitsType_2: 0,
			 a2dOperator_2: 0,
			 applyTarget_2: 0,
			 bits_6: 0,
			 time2_2: 0.,
			 a2dThreshold_2: 0.5,
			 phyisicalKey_3: -1,
			 traitsType_3: 0,
			 a2dOperator_3: 0,
			 applyTarget_3: 0,
			 bits_7: 0,
			 time2_3: 0.,
			 a2dThreshold_3: 0.5,
			 phyisicalKey_4: -1,
			 traitsType_4: 0,
			 a2dOperator_4: 0,
			 applyTarget_4: 0,
			 bits_8: 0,
			 time2_4: 0.,
			 a2dThreshold_4: 0.5,
			 phyisicalKey_5: -1,
			 traitsType_5: 0,
			 a2dOperator_5: 0,
			 applyTarget_5: 0,
			 bits_9: 0,
			 time2_5: 0.,
			 a2dThreshold_5: 0.5,
			 phyisicalKey_6: -1,
			 traitsType_6: 0,
			 a2dOperator_6: 0,
			 applyTarget_6: 0,
			 bits_10: 0,
			 time2_6: 0.,
			 a2dThreshold_6: 0.5,
			 phyisicalKey_7: -1,
			 traitsType_7: 0,
			 a2dOperator_7: 0,
			 applyTarget_7: 0,
			 bits_11: 0,
			 time2_7: 0.,
			 a2dThreshold_7: 0.5,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct DIRECTION_CAMERA_PARAM_ST {
	bits_0: u8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl DIRECTION_CAMERA_PARAM_ST {
	pub fn isUseOption(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn pad2(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for DIRECTION_CAMERA_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ENEMY_COMMON_PARAM_ST {
	pub reserved0: [u8;8],
	pub soundTargetTryApproachTime: i32,
	pub searchTargetTryApproachTime: i32,
	pub memoryTargetTryApproachTime: i32,
	pub reserved5: [u8;40],
	pub activateChrByTime_PhantomId: i32,
	pub findUnfavorableFailedPointDist: f32,
	pub findUnfavorableFailedPointHeight: f32,
	pub reserved18: [u8;184],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ENEMY_COMMON_PARAM_ST {
}
impl Default for ENEMY_COMMON_PARAM_ST {
	fn default() -> Self {
		Self {
			 reserved0: [0;8],
			 soundTargetTryApproachTime: 0,
			 searchTargetTryApproachTime: 0,
			 memoryTargetTryApproachTime: 0,
			 reserved5: [0;40],
			 activateChrByTime_PhantomId: 0,
			 findUnfavorableFailedPointDist: 0.,
			 findUnfavorableFailedPointHeight: 0.,
			 reserved18: [0;184],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ENEMY_STANDARD_INFO_BANK {
	pub EnemyBehaviorID: i32,
	pub HP: i16,
	pub AttackPower: i16,
	pub ChrType: i32,
	pub HitHeight: f32,
	pub HitRadius: f32,
	pub Weight: f32,
	pub DynamicFriction: f32,
	pub StaticFriction: f32,
	pub UpperDefState: i32,
	pub ActionDefState: i32,
	pub RotY_per_Second: f32,
	pub reserve0: [u8;20],
	pub RotY_per_Second_old: u8,
	pub EnableSideStep: u8,
	pub UseRagdollHit: u8,
	pub reserve_last: [u8;5],
	pub stamina: i16,
	pub staminaRecover: i16,
	pub staminaConsumption: i16,
	pub deffenct_Phys: i16,
	pub reserve_last2: [u8;48],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ENEMY_STANDARD_INFO_BANK {
}
impl Default for ENEMY_STANDARD_INFO_BANK {
	fn default() -> Self {
		Self {
			 EnemyBehaviorID: 0,
			 HP: 1,
			 AttackPower: 1,
			 ChrType: 5,
			 HitHeight: 2.,
			 HitRadius: 0.4,
			 Weight: 60.,
			 DynamicFriction: 0.,
			 StaticFriction: 0.,
			 UpperDefState: 0,
			 ActionDefState: 0,
			 RotY_per_Second: 10.,
			 reserve0: [0;20],
			 RotY_per_Second_old: 0,
			 EnableSideStep: 0,
			 UseRagdollHit: 0,
			 reserve_last: [0;5],
			 stamina: 0,
			 staminaRecover: 0,
			 staminaConsumption: 0,
			 deffenct_Phys: 0,
			 reserve_last2: [0;48],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ENV_OBJ_LOT_PARAM_ST {
	pub AssetId_0: i32,
	pub AssetId_1: i32,
	pub AssetId_2: i32,
	pub AssetId_3: i32,
	pub AssetId_4: i32,
	pub AssetId_5: i32,
	pub AssetId_6: i32,
	pub AssetId_7: i32,
	pub CreateWeight_0: u8,
	pub CreateWeight_1: u8,
	pub CreateWeight_2: u8,
	pub CreateWeight_3: u8,
	pub CreateWeight_4: u8,
	pub CreateWeight_5: u8,
	pub CreateWeight_6: u8,
	pub CreateWeight_7: u8,
	pub Reserve_0: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ENV_OBJ_LOT_PARAM_ST {
}
impl Default for ENV_OBJ_LOT_PARAM_ST {
	fn default() -> Self {
		Self {
			 AssetId_0: -1,
			 AssetId_1: -1,
			 AssetId_2: -1,
			 AssetId_3: -1,
			 AssetId_4: -1,
			 AssetId_5: -1,
			 AssetId_6: -1,
			 AssetId_7: -1,
			 CreateWeight_0: 0,
			 CreateWeight_1: 0,
			 CreateWeight_2: 0,
			 CreateWeight_3: 0,
			 CreateWeight_4: 0,
			 CreateWeight_5: 0,
			 CreateWeight_6: 0,
			 CreateWeight_7: 0,
			 Reserve_0: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_MTRL_SET_PARAM_ST {
	pub materialId01: i32,
	pub materialId02: i32,
	pub materialId03: i32,
	pub materialId04: i32,
	pub materialId05: i32,
	pub materialId06: i32,
	pub pad_id: [u8;8],
	pub itemNum01: i8,
	pub itemNum02: i8,
	pub itemNum03: i8,
	pub itemNum04: i8,
	pub itemNum05: i8,
	pub itemNum06: i8,
	pub pad_num: [u8;2],
	pub materialCate01: u8,
	pub materialCate02: u8,
	pub materialCate03: u8,
	pub materialCate04: u8,
	pub materialCate05: u8,
	pub materialCate06: u8,
	pub pad_cate: [u8;2],
	bits_0: u8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_MTRL_SET_PARAM_ST {
	pub fn isDisableDispNum01(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isDisableDispNum02(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isDisableDispNum03(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn isDisableDispNum04(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn isDisableDispNum05(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isDisableDispNum06(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
}
impl Default for EQUIP_MTRL_SET_PARAM_ST {
	fn default() -> Self {
		Self {
			 materialId01: -1,
			 materialId02: -1,
			 materialId03: -1,
			 materialId04: -1,
			 materialId05: -1,
			 materialId06: -1,
			 pad_id: [0;8],
			 itemNum01: -1,
			 itemNum02: -1,
			 itemNum03: -1,
			 itemNum04: -1,
			 itemNum05: -1,
			 itemNum06: -1,
			 pad_num: [0;2],
			 materialCate01: 4,
			 materialCate02: 4,
			 materialCate03: 4,
			 materialCate04: 4,
			 materialCate05: 4,
			 materialCate06: 4,
			 pad_cate: [0;2],
			 bits_0: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_PARAM_ACCESSORY_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub refId: i32,
	pub sfxVariationId: i32,
	pub weight: f32,
	pub behaviorId: i32,
	pub basicPrice: i32,
	pub sellValue: i32,
	pub sortId: i32,
	pub qwcId: i32,
	pub equipModelId: i16,
	pub iconId: i16,
	pub shopLv: i16,
	pub trophySGradeId: i16,
	pub trophySeqId: i16,
	pub equipModelCategory: u8,
	pub equipModelGender: u8,
	pub accessoryCategory: u8,
	pub refCategory: u8,
	pub spEffectCategory: u8,
	pub sortGroupId: u8,
	pub vagrantItemLotId: i32,
	pub vagrantBonusEneDropItemLotId: i32,
	pub vagrantItemEneDropItemLotId: i32,
	bits_1: u8,
	pub rarity: u8,
	pub pad2: [u8;2],
	pub saleValue: i32,
	pub accessoryGroup: i16,
	pub pad3: [u8;1],
	pub compTrophySedId: i8,
	pub residentSpEffectId1: i32,
	pub residentSpEffectId2: i32,
	pub residentSpEffectId3: i32,
	pub residentSpEffectId4: i32,
	pub pad1: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_PARAM_ACCESSORY_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isDeposit(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isEquipOutBrake(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn disableMultiDropShare(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isDiscard(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isDrop(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn showLogCondType(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn showDialogCondType(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
}
impl Default for EQUIP_PARAM_ACCESSORY_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 refId: -1,
			 sfxVariationId: -1,
			 weight: 1.,
			 behaviorId: 0,
			 basicPrice: 0,
			 sellValue: 0,
			 sortId: 0,
			 qwcId: -1,
			 equipModelId: 0,
			 iconId: 0,
			 shopLv: 0,
			 trophySGradeId: -1,
			 trophySeqId: -1,
			 equipModelCategory: 0,
			 equipModelGender: 0,
			 accessoryCategory: 0,
			 refCategory: 0,
			 spEffectCategory: 0,
			 sortGroupId: 255,
			 vagrantItemLotId: 0,
			 vagrantBonusEneDropItemLotId: 0,
			 vagrantItemEneDropItemLotId: 0,
			 bits_1: 0,
			 rarity: 0,
			 pad2: [0;2],
			 saleValue: -1,
			 accessoryGroup: -1,
			 pad3: [0;1],
			 compTrophySedId: -1,
			 residentSpEffectId1: 0,
			 residentSpEffectId2: 0,
			 residentSpEffectId3: 0,
			 residentSpEffectId4: 0,
			 pad1: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_PARAM_CUSTOM_WEAPON_ST {
	pub baseWepId: i32,
	pub gemId: i32,
	pub reinforceLv: u8,
	pub pad: [u8;7],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_PARAM_CUSTOM_WEAPON_ST {
}
impl Default for EQUIP_PARAM_CUSTOM_WEAPON_ST {
	fn default() -> Self {
		Self {
			 baseWepId: 0,
			 gemId: 0,
			 reinforceLv: 0,
			 pad: [0;7],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_PARAM_GEM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub iconId: i16,
	pub rank: i8,
	pub sortGroupId: u8,
	pub spEffectId0: i32,
	pub spEffectId1: i32,
	pub spEffectId2: i32,
	pub itemGetTutorialFlagId: i32,
	pub swordArtsParamId: i32,
	pub mountValue: i32,
	pub sellValue: i32,
	pub saleValue: i32,
	pub sortId: i32,
	pub compTrophySedId: i16,
	pub trophySeqId: i16,
	bits_1: u8,
	bits_2: u8,
	pub rarity: u8,
	bits_3: u8,
	bits_4: u8,
	pub defaultWepAttr: u8,
	pub pad2: [u8;2],
	bits_5: u8,
	bits_6: u8,
	bits_7: u8,
	bits_8: u8,
	bits_9: u8,
	pub reserved2_canMountWep: [u8;3],
	pub spEffectMsgId0: i32,
	pub spEffectMsgId1: i32,
	pub spEffectId_forAtk0: i32,
	pub spEffectId_forAtk1: i32,
	pub spEffectId_forAtk2: i32,
	pub mountWepTextId: i32,
	pub pad6: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_PARAM_GEM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn configurableWepAttr00(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn configurableWepAttr01(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn configurableWepAttr02(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn configurableWepAttr03(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn configurableWepAttr04(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn configurableWepAttr05(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn configurableWepAttr06(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn configurableWepAttr07(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn configurableWepAttr08(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn configurableWepAttr09(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn configurableWepAttr10(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn configurableWepAttr11(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn configurableWepAttr12(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn configurableWepAttr13(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn configurableWepAttr14(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn configurableWepAttr15(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn configurableWepAttr16(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn configurableWepAttr17(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn configurableWepAttr18(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn configurableWepAttr19(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn configurableWepAttr20(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn configurableWepAttr21(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn configurableWepAttr22(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn configurableWepAttr23(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn isDiscard(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn isDrop(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn isDeposit(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn disableMultiDropShare(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn showDialogCondType(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn showLogCondType(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn pad(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn canMountWep_Dagger(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn canMountWep_SwordNormal(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn canMountWep_SwordLarge(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn canMountWep_SwordGigantic(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn canMountWep_SaberNormal(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn canMountWep_SaberLarge(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn canMountWep_katana(&self) -> bool {	
			self.bits_5 & (1 << 6) != 0
	}
	pub fn canMountWep_SwordDoubleEdge(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn canMountWep_SwordPierce(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn canMountWep_RapierHeavy(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
	pub fn canMountWep_AxeNormal(&self) -> bool {	
			self.bits_6 & (1 << 2) != 0
	}
	pub fn canMountWep_AxeLarge(&self) -> bool {	
			self.bits_6 & (1 << 3) != 0
	}
	pub fn canMountWep_HammerNormal(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn canMountWep_HammerLarge(&self) -> bool {	
			self.bits_6 & (1 << 5) != 0
	}
	pub fn canMountWep_Flail(&self) -> bool {	
			self.bits_6 & (1 << 6) != 0
	}
	pub fn canMountWep_SpearNormal(&self) -> bool {	
			self.bits_6 & (1 << 7) != 0
	}
	pub fn canMountWep_SpearLarge(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn canMountWep_SpearHeavy(&self) -> bool {	
			self.bits_7 & (1 << 1) != 0
	}
	pub fn canMountWep_SpearAxe(&self) -> bool {	
			self.bits_7 & (1 << 2) != 0
	}
	pub fn canMountWep_Sickle(&self) -> bool {	
			self.bits_7 & (1 << 3) != 0
	}
	pub fn canMountWep_Knuckle(&self) -> bool {	
			self.bits_7 & (1 << 4) != 0
	}
	pub fn canMountWep_Claw(&self) -> bool {	
			self.bits_7 & (1 << 5) != 0
	}
	pub fn canMountWep_Whip(&self) -> bool {	
			self.bits_7 & (1 << 6) != 0
	}
	pub fn canMountWep_AxhammerLarge(&self) -> bool {	
			self.bits_7 & (1 << 7) != 0
	}
	pub fn canMountWep_BowSmall(&self) -> bool {	
			self.bits_8 & (1 << 0) != 0
	}
	pub fn canMountWep_BowNormal(&self) -> bool {	
			self.bits_8 & (1 << 1) != 0
	}
	pub fn canMountWep_BowLarge(&self) -> bool {	
			self.bits_8 & (1 << 2) != 0
	}
	pub fn canMountWep_ClossBow(&self) -> bool {	
			self.bits_8 & (1 << 3) != 0
	}
	pub fn canMountWep_Ballista(&self) -> bool {	
			self.bits_8 & (1 << 4) != 0
	}
	pub fn canMountWep_Staff(&self) -> bool {	
			self.bits_8 & (1 << 5) != 0
	}
	pub fn canMountWep_Sorcery(&self) -> bool {	
			self.bits_8 & (1 << 6) != 0
	}
	pub fn canMountWep_Talisman(&self) -> bool {	
			self.bits_8 & (1 << 7) != 0
	}
	pub fn canMountWep_ShieldSmall(&self) -> bool {	
			self.bits_9 & (1 << 0) != 0
	}
	pub fn canMountWep_ShieldNormal(&self) -> bool {	
			self.bits_9 & (1 << 1) != 0
	}
	pub fn canMountWep_ShieldLarge(&self) -> bool {	
			self.bits_9 & (1 << 2) != 0
	}
	pub fn canMountWep_Torch(&self) -> bool {	
			self.bits_9 & (1 << 3) != 0
	}
	pub fn reserved_canMountWep(&self) -> bool {	
			self.bits_9 & (1 << 4) != 0
	}
}
impl Default for EQUIP_PARAM_GEM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 iconId: 0,
			 rank: 0,
			 sortGroupId: 255,
			 spEffectId0: -1,
			 spEffectId1: -1,
			 spEffectId2: -1,
			 itemGetTutorialFlagId: 0,
			 swordArtsParamId: -1,
			 mountValue: 0,
			 sellValue: 0,
			 saleValue: -1,
			 sortId: 0,
			 compTrophySedId: -1,
			 trophySeqId: -1,
			 bits_1: 0,
			 bits_2: 0,
			 rarity: 0,
			 bits_3: 0,
			 bits_4: 0,
			 defaultWepAttr: 0,
			 pad2: [0;2],
			 bits_5: 0,
			 bits_6: 0,
			 bits_7: 0,
			 bits_8: 0,
			 bits_9: 0,
			 reserved2_canMountWep: [0;3],
			 spEffectMsgId0: -1,
			 spEffectMsgId1: -1,
			 spEffectId_forAtk0: -1,
			 spEffectId_forAtk1: -1,
			 spEffectId_forAtk2: -1,
			 mountWepTextId: -1,
			 pad6: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_PARAM_GOODS_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub refId_default: i32,
	pub sfxVariationId: i32,
	pub weight: f32,
	pub basicPrice: i32,
	pub sellValue: i32,
	pub behaviorId: i32,
	pub replaceItemId: i32,
	pub sortId: i32,
	pub appearanceReplaceItemId: i32,
	pub yesNoDialogMessageId: i32,
	pub useEnableSpEffectType: i16,
	pub potGroupId: i8,
	pub pad: [u8;1],
	pub iconId: i16,
	pub modelId: i16,
	pub shopLv: i16,
	pub compTrophySedId: i16,
	pub trophySeqId: i16,
	pub maxNum: i16,
	pub consumeHeroPoint: u8,
	pub overDexterity: u8,
	pub goodsType: u8,
	pub refCategory: u8,
	pub spEffectCategory: u8,
	pub pad3: [u8;1],
	pub goodsUseAnim: u8,
	pub opmeMenuType: u8,
	pub useLimitCategory: u8,
	pub replaceCategory: u8,
	pub reserve4: [u8;2],
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	pub syncNumVaryId: u8,
	pub refId_1: i32,
	pub refVirtualWepId: i32,
	pub vagrantItemLotId: i32,
	pub vagrantBonusEneDropItemLotId: i32,
	pub vagrantItemEneDropItemLotId: i32,
	pub castSfxId: i32,
	pub fireSfxId: i32,
	pub effectSfxId: i32,
	bits_4: u8,
	pub suppleType: u8,
	pub autoReplenishType: u8,
	bits_5: u8,
	pub maxRepositoryNum: i16,
	pub sortGroupId: u8,
	bits_6: u8,
	pub saleValue: i32,
	pub rarity: u8,
	pub useLimitSummonBuddy: u8,
	pub useLimitSpEffectType: i16,
	pub aiUseJudgeId: i32,
	pub consumeMP: i16,
	pub consumeHP: i16,
	pub reinforceGoodsId: i32,
	pub reinforceMaterialId: i32,
	pub reinforcePrice: i32,
	pub useLevel_vowType0: i8,
	pub useLevel_vowType1: i8,
	pub useLevel_vowType2: i8,
	pub useLevel_vowType3: i8,
	pub useLevel_vowType4: i8,
	pub useLevel_vowType5: i8,
	pub useLevel_vowType6: i8,
	pub useLevel_vowType7: i8,
	pub useLevel_vowType8: i8,
	pub useLevel_vowType9: i8,
	pub useLevel_vowType10: i8,
	pub useLevel_vowType11: i8,
	pub useLevel_vowType12: i8,
	pub useLevel_vowType13: i8,
	pub useLevel_vowType14: i8,
	pub useLevel_vowType15: i8,
	pub useLevel: i16,
	pub reserve5: [u8;2],
	pub itemGetTutorialFlagId: i32,
	pub reserve3: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_PARAM_GOODS_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn enable_live(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn enable_gray(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn enable_white(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn enable_black(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn enable_multi(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn disable_offline(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isEquip(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn isConsume(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn isAutoEquip(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn isEstablishment(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn isOnlyOne(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isDiscard(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn isDeposit(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn isDisableHand(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn isRemoveItem_forGameClear(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn isSuppleItem(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn isFullSuppleItem(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn isEnhance(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn isFixItem(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn disableMultiDropShare(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn disableUseAtColiseum(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn disableUseAtOutOfColiseum(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn isEnableFastUseItem(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn isApplySpecialEffect(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn enable_ActiveBigRune(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn isBonfireWarpItem(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn enable_Ladder(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn isUseMultiPlayPreparation(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn canMultiUse(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn isShieldEnchant(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn isWarpProhibited(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn isUseMultiPenaltyOnly(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn isDrop(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn showLogCondType(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn isSummonHorse(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn showDialogCondType(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn isSleepCollectionItem(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn enableRiding(&self) -> bool {	
			self.bits_5 & (1 << 6) != 0
	}
	pub fn disableRiding(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn isUseNoAttackRegion(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
}
impl Default for EQUIP_PARAM_GOODS_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 refId_default: -1,
			 sfxVariationId: -1,
			 weight: 1.,
			 basicPrice: 0,
			 sellValue: 0,
			 behaviorId: 0,
			 replaceItemId: -1,
			 sortId: 0,
			 appearanceReplaceItemId: -1,
			 yesNoDialogMessageId: -1,
			 useEnableSpEffectType: 0,
			 potGroupId: -1,
			 pad: [0;1],
			 iconId: 0,
			 modelId: 0,
			 shopLv: 0,
			 compTrophySedId: -1,
			 trophySeqId: -1,
			 maxNum: 0,
			 consumeHeroPoint: 0,
			 overDexterity: 0,
			 goodsType: 0,
			 refCategory: 0,
			 spEffectCategory: 0,
			 pad3: [0;1],
			 goodsUseAnim: 0,
			 opmeMenuType: 0,
			 useLimitCategory: 0,
			 replaceCategory: 0,
			 reserve4: [0;2],
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 syncNumVaryId: 0,
			 refId_1: -1,
			 refVirtualWepId: -1,
			 vagrantItemLotId: 0,
			 vagrantBonusEneDropItemLotId: 0,
			 vagrantItemEneDropItemLotId: 0,
			 castSfxId: -1,
			 fireSfxId: -1,
			 effectSfxId: -1,
			 bits_4: 0,
			 suppleType: 0,
			 autoReplenishType: 0,
			 bits_5: 0,
			 maxRepositoryNum: 0,
			 sortGroupId: 255,
			 bits_6: 1,
			 saleValue: -1,
			 rarity: 0,
			 useLimitSummonBuddy: 0,
			 useLimitSpEffectType: 0,
			 aiUseJudgeId: -1,
			 consumeMP: 0,
			 consumeHP: -1,
			 reinforceGoodsId: -1,
			 reinforceMaterialId: -1,
			 reinforcePrice: 0,
			 useLevel_vowType0: 0,
			 useLevel_vowType1: 0,
			 useLevel_vowType2: 0,
			 useLevel_vowType3: 0,
			 useLevel_vowType4: 0,
			 useLevel_vowType5: 0,
			 useLevel_vowType6: 0,
			 useLevel_vowType7: 0,
			 useLevel_vowType8: 0,
			 useLevel_vowType9: 0,
			 useLevel_vowType10: 0,
			 useLevel_vowType11: 0,
			 useLevel_vowType12: 0,
			 useLevel_vowType13: 0,
			 useLevel_vowType14: 0,
			 useLevel_vowType15: 0,
			 useLevel: 0,
			 reserve5: [0;2],
			 itemGetTutorialFlagId: 0,
			 reserve3: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_PARAM_PROTECTOR_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub sortId: i32,
	pub wanderingEquipId: i32,
	pub resistSleep: i16,
	pub resistMadness: i16,
	pub saDurability: f32,
	pub toughnessCorrectRate: f32,
	pub fixPrice: i32,
	pub basicPrice: i32,
	pub sellValue: i32,
	pub weight: f32,
	pub residentSpEffectId: i32,
	pub residentSpEffectId2: i32,
	pub residentSpEffectId3: i32,
	pub materialSetId: i32,
	pub partsDamageRate: f32,
	pub corectSARecover: f32,
	pub originEquipPro: i32,
	pub originEquipPro1: i32,
	pub originEquipPro2: i32,
	pub originEquipPro3: i32,
	pub originEquipPro4: i32,
	pub originEquipPro5: i32,
	pub originEquipPro6: i32,
	pub originEquipPro7: i32,
	pub originEquipPro8: i32,
	pub originEquipPro9: i32,
	pub originEquipPro10: i32,
	pub originEquipPro11: i32,
	pub originEquipPro12: i32,
	pub originEquipPro13: i32,
	pub originEquipPro14: i32,
	pub originEquipPro15: i32,
	pub faceScaleM_ScaleX: f32,
	pub faceScaleM_ScaleZ: f32,
	pub faceScaleM_MaxX: f32,
	pub faceScaleM_MaxZ: f32,
	pub faceScaleF_ScaleX: f32,
	pub faceScaleF_ScaleZ: f32,
	pub faceScaleF_MaxX: f32,
	pub faceScaleF_MaxZ: f32,
	pub qwcId: i32,
	pub equipModelId: i16,
	pub iconIdM: i16,
	pub iconIdF: i16,
	pub knockBack: i16,
	pub knockbackBounceRate: i16,
	pub durability: i16,
	pub durabilityMax: i16,
	pub pad03: [u8;2],
	pub defFlickPower: i16,
	pub defensePhysics: i16,
	pub defenseMagic: i16,
	pub defenseFire: i16,
	pub defenseThunder: i16,
	pub defenseSlash: i16,
	pub defenseBlow: i16,
	pub defenseThrust: i16,
	pub resistPoison: i16,
	pub resistDisease: i16,
	pub resistBlood: i16,
	pub resistCurse: i16,
	pub reinforceTypeId: i16,
	pub trophySGradeId: i16,
	pub shopLv: i16,
	pub knockbackParamId: u8,
	pub flickDamageCutRate: u8,
	pub equipModelCategory: u8,
	pub equipModelGender: u8,
	pub protectorCategory: u8,
	pub rarity: u8,
	pub sortGroupId: u8,
	pub partsDmgType: u8,
	pub pad04: [u8;2],
	bits_1: u8,
	pub defenseMaterialVariationValue_Weak: u8,
	pub autoFootEffectDecalBaseId2: i16,
	pub autoFootEffectDecalBaseId3: i16,
	pub defenseMaterialVariationValue: u8,
	bits_2: u8,
	pub neutralDamageCutRate: f32,
	pub slashDamageCutRate: f32,
	pub blowDamageCutRate: f32,
	pub thrustDamageCutRate: f32,
	pub magicDamageCutRate: f32,
	pub fireDamageCutRate: f32,
	pub thunderDamageCutRate: f32,
	pub defenseMaterialSfx1: i16,
	pub defenseMaterialSfx_Weak1: i16,
	pub defenseMaterial1: i16,
	pub defenseMaterial_Weak1: i16,
	pub defenseMaterialSfx2: i16,
	pub defenseMaterialSfx_Weak2: i16,
	pub footMaterialSe: i16,
	pub defenseMaterial_Weak2: i16,
	pub autoFootEffectDecalBaseId1: i32,
	pub toughnessDamageCutRate: f32,
	pub toughnessRecoverCorrection: f32,
	pub darkDamageCutRate: f32,
	pub defenseDark: i16,
	bits_3: u8,
	bits_4: u8,
	bits_5: u8,
	bits_6: u8,
	bits_7: u8,
	pub postureControlId: u8,
	pub pad2: [u8;4],
	pub saleValue: i32,
	pub resistFreeze: i16,
	pub invisibleFlag_SexVer00: u8,
	pub invisibleFlag_SexVer01: u8,
	pub invisibleFlag_SexVer02: u8,
	pub invisibleFlag_SexVer03: u8,
	pub invisibleFlag_SexVer04: u8,
	pub invisibleFlag_SexVer05: u8,
	pub invisibleFlag_SexVer06: u8,
	pub invisibleFlag_SexVer07: u8,
	pub invisibleFlag_SexVer08: u8,
	pub invisibleFlag_SexVer09: u8,
	pub invisibleFlag_SexVer10: u8,
	pub invisibleFlag_SexVer11: u8,
	pub invisibleFlag_SexVer12: u8,
	pub invisibleFlag_SexVer13: u8,
	pub invisibleFlag_SexVer14: u8,
	pub invisibleFlag_SexVer15: u8,
	pub invisibleFlag_SexVer16: u8,
	pub invisibleFlag_SexVer17: u8,
	pub invisibleFlag_SexVer18: u8,
	pub invisibleFlag_SexVer19: u8,
	pub invisibleFlag_SexVer20: u8,
	pub invisibleFlag_SexVer21: u8,
	pub invisibleFlag_SexVer22: u8,
	pub invisibleFlag_SexVer23: u8,
	pub invisibleFlag_SexVer24: u8,
	pub invisibleFlag_SexVer25: u8,
	pub invisibleFlag_SexVer26: u8,
	pub invisibleFlag_SexVer27: u8,
	pub invisibleFlag_SexVer28: u8,
	pub invisibleFlag_SexVer29: u8,
	pub invisibleFlag_SexVer30: u8,
	pub invisibleFlag_SexVer31: u8,
	pub invisibleFlag_SexVer32: u8,
	pub invisibleFlag_SexVer33: u8,
	pub invisibleFlag_SexVer34: u8,
	pub invisibleFlag_SexVer35: u8,
	pub invisibleFlag_SexVer36: u8,
	pub invisibleFlag_SexVer37: u8,
	pub invisibleFlag_SexVer38: u8,
	pub invisibleFlag_SexVer39: u8,
	pub invisibleFlag_SexVer40: u8,
	pub invisibleFlag_SexVer41: u8,
	pub invisibleFlag_SexVer42: u8,
	pub invisibleFlag_SexVer43: u8,
	pub invisibleFlag_SexVer44: u8,
	pub invisibleFlag_SexVer45: u8,
	pub invisibleFlag_SexVer46: u8,
	pub invisibleFlag_SexVer47: u8,
	pub invisibleFlag_SexVer48: u8,
	pub invisibleFlag_SexVer49: u8,
	pub invisibleFlag_SexVer50: u8,
	pub invisibleFlag_SexVer51: u8,
	pub invisibleFlag_SexVer52: u8,
	pub invisibleFlag_SexVer53: u8,
	pub invisibleFlag_SexVer54: u8,
	pub invisibleFlag_SexVer55: u8,
	pub invisibleFlag_SexVer56: u8,
	pub invisibleFlag_SexVer57: u8,
	pub invisibleFlag_SexVer58: u8,
	pub invisibleFlag_SexVer59: u8,
	pub invisibleFlag_SexVer60: u8,
	pub invisibleFlag_SexVer61: u8,
	pub invisibleFlag_SexVer62: u8,
	pub invisibleFlag_SexVer63: u8,
	pub invisibleFlag_SexVer64: u8,
	pub invisibleFlag_SexVer65: u8,
	pub invisibleFlag_SexVer66: u8,
	pub invisibleFlag_SexVer67: u8,
	pub invisibleFlag_SexVer68: u8,
	pub invisibleFlag_SexVer69: u8,
	pub invisibleFlag_SexVer70: u8,
	pub invisibleFlag_SexVer71: u8,
	pub invisibleFlag_SexVer72: u8,
	pub invisibleFlag_SexVer73: u8,
	pub invisibleFlag_SexVer74: u8,
	pub invisibleFlag_SexVer75: u8,
	pub invisibleFlag_SexVer76: u8,
	pub invisibleFlag_SexVer77: u8,
	pub invisibleFlag_SexVer78: u8,
	pub invisibleFlag_SexVer79: u8,
	pub invisibleFlag_SexVer80: u8,
	pub invisibleFlag_SexVer81: u8,
	pub invisibleFlag_SexVer82: u8,
	pub invisibleFlag_SexVer83: u8,
	pub invisibleFlag_SexVer84: u8,
	pub invisibleFlag_SexVer85: u8,
	pub invisibleFlag_SexVer86: u8,
	pub invisibleFlag_SexVer87: u8,
	pub invisibleFlag_SexVer88: u8,
	pub invisibleFlag_SexVer89: u8,
	pub invisibleFlag_SexVer90: u8,
	pub invisibleFlag_SexVer91: u8,
	pub invisibleFlag_SexVer92: u8,
	pub invisibleFlag_SexVer93: u8,
	pub invisibleFlag_SexVer94: u8,
	pub invisibleFlag_SexVer95: u8,
	pub pad404: [u8;14],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_PARAM_PROTECTOR_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isDeposit(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn headEquip(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn bodyEquip(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn armEquip(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn legEquip(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn useFaceScale(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isSkipWeakDamageAnim(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn pad06(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn isDiscard(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn isDrop(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn disableMultiDropShare(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn simpleModelForDlc(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn showLogCondType(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn showDialogCondType(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn pad(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn invisibleFlag48(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn invisibleFlag49(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn invisibleFlag50(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn invisibleFlag51(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn invisibleFlag52(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn invisibleFlag53(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn invisibleFlag54(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn invisibleFlag55(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn invisibleFlag56(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn invisibleFlag57(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn invisibleFlag58(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn invisibleFlag59(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn invisibleFlag60(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn invisibleFlag61(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn invisibleFlag62(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn invisibleFlag63(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn invisibleFlag64(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn invisibleFlag65(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn invisibleFlag66(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn invisibleFlag67(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn invisibleFlag68(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn invisibleFlag69(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn invisibleFlag70(&self) -> bool {	
			self.bits_5 & (1 << 6) != 0
	}
	pub fn invisibleFlag71(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn invisibleFlag72(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn invisibleFlag73(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
	pub fn invisibleFlag74(&self) -> bool {	
			self.bits_6 & (1 << 2) != 0
	}
	pub fn invisibleFlag75(&self) -> bool {	
			self.bits_6 & (1 << 3) != 0
	}
	pub fn invisibleFlag76(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn invisibleFlag77(&self) -> bool {	
			self.bits_6 & (1 << 5) != 0
	}
	pub fn invisibleFlag78(&self) -> bool {	
			self.bits_6 & (1 << 6) != 0
	}
	pub fn invisibleFlag79(&self) -> bool {	
			self.bits_6 & (1 << 7) != 0
	}
	pub fn invisibleFlag80(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn padbit(&self) -> bool {	
			self.bits_7 & (1 << 1) != 0
	}
}
impl Default for EQUIP_PARAM_PROTECTOR_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 sortId: 0,
			 wanderingEquipId: 0,
			 resistSleep: 0,
			 resistMadness: 0,
			 saDurability: 0.,
			 toughnessCorrectRate: 0.,
			 fixPrice: 0,
			 basicPrice: 0,
			 sellValue: 0,
			 weight: 1.,
			 residentSpEffectId: 0,
			 residentSpEffectId2: 0,
			 residentSpEffectId3: 0,
			 materialSetId: -1,
			 partsDamageRate: 1.,
			 corectSARecover: 0.,
			 originEquipPro: -1,
			 originEquipPro1: -1,
			 originEquipPro2: -1,
			 originEquipPro3: -1,
			 originEquipPro4: -1,
			 originEquipPro5: -1,
			 originEquipPro6: -1,
			 originEquipPro7: -1,
			 originEquipPro8: -1,
			 originEquipPro9: -1,
			 originEquipPro10: -1,
			 originEquipPro11: -1,
			 originEquipPro12: -1,
			 originEquipPro13: -1,
			 originEquipPro14: -1,
			 originEquipPro15: -1,
			 faceScaleM_ScaleX: 1.,
			 faceScaleM_ScaleZ: 1.,
			 faceScaleM_MaxX: 1.,
			 faceScaleM_MaxZ: 1.,
			 faceScaleF_ScaleX: 1.,
			 faceScaleF_ScaleZ: 1.,
			 faceScaleF_MaxX: 1.,
			 faceScaleF_MaxZ: 1.,
			 qwcId: -1,
			 equipModelId: 0,
			 iconIdM: 0,
			 iconIdF: 0,
			 knockBack: 0,
			 knockbackBounceRate: 0,
			 durability: 100,
			 durabilityMax: 100,
			 pad03: [0;2],
			 defFlickPower: 0,
			 defensePhysics: 100,
			 defenseMagic: 100,
			 defenseFire: 100,
			 defenseThunder: 100,
			 defenseSlash: 0,
			 defenseBlow: 0,
			 defenseThrust: 0,
			 resistPoison: 100,
			 resistDisease: 100,
			 resistBlood: 100,
			 resistCurse: 100,
			 reinforceTypeId: 0,
			 trophySGradeId: -1,
			 shopLv: 0,
			 knockbackParamId: 0,
			 flickDamageCutRate: 0,
			 equipModelCategory: 1,
			 equipModelGender: 0,
			 protectorCategory: 0,
			 rarity: 0,
			 sortGroupId: 255,
			 partsDmgType: 0,
			 pad04: [0;2],
			 bits_1: 0,
			 defenseMaterialVariationValue_Weak: 0,
			 autoFootEffectDecalBaseId2: -1,
			 autoFootEffectDecalBaseId3: -1,
			 defenseMaterialVariationValue: 0,
			 bits_2: 0,
			 neutralDamageCutRate: 1.,
			 slashDamageCutRate: 1.,
			 blowDamageCutRate: 1.,
			 thrustDamageCutRate: 1.,
			 magicDamageCutRate: 1.,
			 fireDamageCutRate: 1.,
			 thunderDamageCutRate: 1.,
			 defenseMaterialSfx1: 50,
			 defenseMaterialSfx_Weak1: 50,
			 defenseMaterial1: 50,
			 defenseMaterial_Weak1: 50,
			 defenseMaterialSfx2: 50,
			 defenseMaterialSfx_Weak2: 50,
			 footMaterialSe: 139,
			 defenseMaterial_Weak2: 50,
			 autoFootEffectDecalBaseId1: -1,
			 toughnessDamageCutRate: 1.,
			 toughnessRecoverCorrection: 0.,
			 darkDamageCutRate: 1.,
			 defenseDark: 100,
			 bits_3: 0,
			 bits_4: 0,
			 bits_5: 0,
			 bits_6: 0,
			 bits_7: 0,
			 postureControlId: 0,
			 pad2: [0;4],
			 saleValue: -1,
			 resistFreeze: 0,
			 invisibleFlag_SexVer00: 0,
			 invisibleFlag_SexVer01: 0,
			 invisibleFlag_SexVer02: 0,
			 invisibleFlag_SexVer03: 0,
			 invisibleFlag_SexVer04: 0,
			 invisibleFlag_SexVer05: 0,
			 invisibleFlag_SexVer06: 0,
			 invisibleFlag_SexVer07: 0,
			 invisibleFlag_SexVer08: 0,
			 invisibleFlag_SexVer09: 0,
			 invisibleFlag_SexVer10: 0,
			 invisibleFlag_SexVer11: 0,
			 invisibleFlag_SexVer12: 0,
			 invisibleFlag_SexVer13: 0,
			 invisibleFlag_SexVer14: 0,
			 invisibleFlag_SexVer15: 0,
			 invisibleFlag_SexVer16: 0,
			 invisibleFlag_SexVer17: 0,
			 invisibleFlag_SexVer18: 0,
			 invisibleFlag_SexVer19: 0,
			 invisibleFlag_SexVer20: 0,
			 invisibleFlag_SexVer21: 0,
			 invisibleFlag_SexVer22: 0,
			 invisibleFlag_SexVer23: 0,
			 invisibleFlag_SexVer24: 0,
			 invisibleFlag_SexVer25: 0,
			 invisibleFlag_SexVer26: 0,
			 invisibleFlag_SexVer27: 0,
			 invisibleFlag_SexVer28: 0,
			 invisibleFlag_SexVer29: 0,
			 invisibleFlag_SexVer30: 0,
			 invisibleFlag_SexVer31: 0,
			 invisibleFlag_SexVer32: 0,
			 invisibleFlag_SexVer33: 0,
			 invisibleFlag_SexVer34: 0,
			 invisibleFlag_SexVer35: 0,
			 invisibleFlag_SexVer36: 0,
			 invisibleFlag_SexVer37: 0,
			 invisibleFlag_SexVer38: 0,
			 invisibleFlag_SexVer39: 0,
			 invisibleFlag_SexVer40: 0,
			 invisibleFlag_SexVer41: 0,
			 invisibleFlag_SexVer42: 0,
			 invisibleFlag_SexVer43: 0,
			 invisibleFlag_SexVer44: 0,
			 invisibleFlag_SexVer45: 0,
			 invisibleFlag_SexVer46: 0,
			 invisibleFlag_SexVer47: 0,
			 invisibleFlag_SexVer48: 0,
			 invisibleFlag_SexVer49: 0,
			 invisibleFlag_SexVer50: 0,
			 invisibleFlag_SexVer51: 0,
			 invisibleFlag_SexVer52: 0,
			 invisibleFlag_SexVer53: 0,
			 invisibleFlag_SexVer54: 0,
			 invisibleFlag_SexVer55: 0,
			 invisibleFlag_SexVer56: 0,
			 invisibleFlag_SexVer57: 0,
			 invisibleFlag_SexVer58: 0,
			 invisibleFlag_SexVer59: 0,
			 invisibleFlag_SexVer60: 0,
			 invisibleFlag_SexVer61: 0,
			 invisibleFlag_SexVer62: 0,
			 invisibleFlag_SexVer63: 0,
			 invisibleFlag_SexVer64: 0,
			 invisibleFlag_SexVer65: 0,
			 invisibleFlag_SexVer66: 0,
			 invisibleFlag_SexVer67: 0,
			 invisibleFlag_SexVer68: 0,
			 invisibleFlag_SexVer69: 0,
			 invisibleFlag_SexVer70: 0,
			 invisibleFlag_SexVer71: 0,
			 invisibleFlag_SexVer72: 0,
			 invisibleFlag_SexVer73: 0,
			 invisibleFlag_SexVer74: 0,
			 invisibleFlag_SexVer75: 0,
			 invisibleFlag_SexVer76: 0,
			 invisibleFlag_SexVer77: 0,
			 invisibleFlag_SexVer78: 0,
			 invisibleFlag_SexVer79: 0,
			 invisibleFlag_SexVer80: 0,
			 invisibleFlag_SexVer81: 0,
			 invisibleFlag_SexVer82: 0,
			 invisibleFlag_SexVer83: 0,
			 invisibleFlag_SexVer84: 0,
			 invisibleFlag_SexVer85: 0,
			 invisibleFlag_SexVer86: 0,
			 invisibleFlag_SexVer87: 0,
			 invisibleFlag_SexVer88: 0,
			 invisibleFlag_SexVer89: 0,
			 invisibleFlag_SexVer90: 0,
			 invisibleFlag_SexVer91: 0,
			 invisibleFlag_SexVer92: 0,
			 invisibleFlag_SexVer93: 0,
			 invisibleFlag_SexVer94: 0,
			 invisibleFlag_SexVer95: 0,
			 pad404: [0;14],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EQUIP_PARAM_WEAPON_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub behaviorVariationId: i32,
	pub sortId: i32,
	pub wanderingEquipId: i32,
	pub weight: f32,
	pub weaponWeightRate: f32,
	pub fixPrice: i32,
	pub reinforcePrice: i32,
	pub sellValue: i32,
	pub correctStrength: f32,
	pub correctAgility: f32,
	pub correctMagic: f32,
	pub correctFaith: f32,
	pub physGuardCutRate: f32,
	pub magGuardCutRate: f32,
	pub fireGuardCutRate: f32,
	pub thunGuardCutRate: f32,
	pub spEffectBehaviorId0: i32,
	pub spEffectBehaviorId1: i32,
	pub spEffectBehaviorId2: i32,
	pub residentSpEffectId: i32,
	pub residentSpEffectId1: i32,
	pub residentSpEffectId2: i32,
	pub materialSetId: i32,
	pub originEquipWep: i32,
	pub originEquipWep1: i32,
	pub originEquipWep2: i32,
	pub originEquipWep3: i32,
	pub originEquipWep4: i32,
	pub originEquipWep5: i32,
	pub originEquipWep6: i32,
	pub originEquipWep7: i32,
	pub originEquipWep8: i32,
	pub originEquipWep9: i32,
	pub originEquipWep10: i32,
	pub originEquipWep11: i32,
	pub originEquipWep12: i32,
	pub originEquipWep13: i32,
	pub originEquipWep14: i32,
	pub originEquipWep15: i32,
	pub weakA_DamageRate: f32,
	pub weakB_DamageRate: f32,
	pub weakC_DamageRate: f32,
	pub weakD_DamageRate: f32,
	pub sleepGuardResist_MaxCorrect: f32,
	pub madnessGuardResist_MaxCorrect: f32,
	pub saWeaponDamage: f32,
	pub equipModelId: i16,
	pub iconId: i16,
	pub durability: i16,
	pub durabilityMax: i16,
	pub attackThrowEscape: i16,
	pub parryDamageLife: i16,
	pub attackBasePhysics: i16,
	pub attackBaseMagic: i16,
	pub attackBaseFire: i16,
	pub attackBaseThunder: i16,
	pub attackBaseStamina: i16,
	pub guardAngle: i16,
	pub saDurability: f32,
	pub staminaGuardDef: i16,
	pub reinforceTypeId: i16,
	pub trophySGradeId: i16,
	pub trophySeqId: i16,
	pub throwAtkRate: i16,
	pub bowDistRate: i16,
	pub equipModelCategory: u8,
	pub equipModelGender: u8,
	pub weaponCategory: u8,
	pub wepmotionCategory: u8,
	pub guardmotionCategory: u8,
	pub atkMaterial: u8,
	pub defSeMaterial1: i16,
	pub correctType_Physics: u8,
	pub spAttribute: u8,
	pub spAtkcategory: i16,
	pub wepmotionOneHandId: u8,
	pub wepmotionBothHandId: u8,
	pub properStrength: u8,
	pub properAgility: u8,
	pub properMagic: u8,
	pub properFaith: u8,
	pub overStrength: u8,
	pub attackBaseParry: u8,
	pub defenseBaseParry: u8,
	pub guardBaseRepel: u8,
	pub attackBaseRepel: u8,
	pub guardCutCancelRate: i8,
	pub guardLevel: i8,
	pub slashGuardCutRate: i8,
	pub blowGuardCutRate: i8,
	pub thrustGuardCutRate: i8,
	pub poisonGuardResist: i8,
	pub diseaseGuardResist: i8,
	pub bloodGuardResist: i8,
	pub curseGuardResist: i8,
	pub atkAttribute: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	bits_4: u8,
	bits_5: u8,
	pub defSfxMaterial1: i16,
	pub wepCollidableType0: u8,
	pub wepCollidableType1: u8,
	pub postureControlId_Right: u8,
	pub postureControlId_Left: u8,
	pub traceSfxId0: i32,
	pub traceDmyIdHead0: i32,
	pub traceDmyIdTail0: i32,
	pub traceSfxId1: i32,
	pub traceDmyIdHead1: i32,
	pub traceDmyIdTail1: i32,
	pub traceSfxId2: i32,
	pub traceDmyIdHead2: i32,
	pub traceDmyIdTail2: i32,
	pub traceSfxId3: i32,
	pub traceDmyIdHead3: i32,
	pub traceDmyIdTail3: i32,
	pub traceSfxId4: i32,
	pub traceDmyIdHead4: i32,
	pub traceDmyIdTail4: i32,
	pub traceSfxId5: i32,
	pub traceDmyIdHead5: i32,
	pub traceDmyIdTail5: i32,
	pub traceSfxId6: i32,
	pub traceDmyIdHead6: i32,
	pub traceDmyIdTail6: i32,
	pub traceSfxId7: i32,
	pub traceDmyIdHead7: i32,
	pub traceDmyIdTail7: i32,
	pub defSfxMaterial2: i16,
	pub defSeMaterial2: i16,
	pub absorpParamId: i32,
	pub toughnessCorrectRate: f32,
	bits_6: u8,
	pub correctType_Magic: u8,
	pub correctType_Fire: u8,
	pub correctType_Thunder: u8,
	pub weakE_DamageRate: f32,
	pub weakF_DamageRate: f32,
	pub darkGuardCutRate: f32,
	pub attackBaseDark: i16,
	pub correctType_Dark: u8,
	pub correctType_Poison: u8,
	pub sortGroupId: u8,
	pub atkAttribute2: u8,
	pub sleepGuardResist: i8,
	pub madnessGuardResist: i8,
	pub correctType_Blood: u8,
	pub properLuck: u8,
	pub freezeGuardResist: i8,
	pub autoReplenishType: u8,
	pub swordArtsParamId: i32,
	pub correctLuck: f32,
	pub arrowBoltEquipId: i32,
	pub DerivationLevelType: u8,
	pub enchantSfxSize: u8,
	pub wepType: i16,
	pub physGuardCutRate_MaxCorrect: f32,
	pub magGuardCutRate_MaxCorrect: f32,
	pub fireGuardCutRate_MaxCorrect: f32,
	pub thunGuardCutRate_MaxCorrect: f32,
	pub darkGuardCutRate_MaxCorrect: f32,
	pub poisonGuardResist_MaxCorrect: f32,
	pub diseaseGuardResist_MaxCorrect: f32,
	pub bloodGuardResist_MaxCorrect: f32,
	pub curseGuardResist_MaxCorrect: f32,
	pub freezeGuardResist_MaxCorrect: f32,
	pub staminaGuardDef_MaxCorrect: f32,
	pub residentSfxId_1: i32,
	pub residentSfxId_2: i32,
	pub residentSfxId_3: i32,
	pub residentSfxId_4: i32,
	pub residentSfx_DmyId_1: i32,
	pub residentSfx_DmyId_2: i32,
	pub residentSfx_DmyId_3: i32,
	pub residentSfx_DmyId_4: i32,
	pub staminaConsumptionRate: f32,
	pub vsPlayerDmgCorrectRate_Physics: f32,
	pub vsPlayerDmgCorrectRate_Magic: f32,
	pub vsPlayerDmgCorrectRate_Fire: f32,
	pub vsPlayerDmgCorrectRate_Thunder: f32,
	pub vsPlayerDmgCorrectRate_Dark: f32,
	pub vsPlayerDmgCorrectRate_Poison: f32,
	pub vsPlayerDmgCorrectRate_Blood: f32,
	pub vsPlayerDmgCorrectRate_Freeze: f32,
	pub attainmentWepStatusStr: i32,
	pub attainmentWepStatusDex: i32,
	pub attainmentWepStatusMag: i32,
	pub attainmentWepStatusFai: i32,
	pub attainmentWepStatusLuc: i32,
	pub attackElementCorrectId: i32,
	pub saleValue: i32,
	pub reinforceShopCategory: u8,
	pub maxArrowQuantity: u8,
	bits_7: u8,
	pub wepSeIdOffset: i8,
	pub baseChangePrice: i32,
	pub levelSyncCorrectId: i16,
	pub correctType_Sleep: u8,
	pub correctType_Madness: u8,
	pub rarity: u8,
	pub gemMountType: u8,
	pub wepRegainHp: i16,
	pub spEffectMsgId0: i32,
	pub spEffectMsgId1: i32,
	pub spEffectMsgId2: i32,
	pub originEquipWep16: i32,
	pub originEquipWep17: i32,
	pub originEquipWep18: i32,
	pub originEquipWep19: i32,
	pub originEquipWep20: i32,
	pub originEquipWep21: i32,
	pub originEquipWep22: i32,
	pub originEquipWep23: i32,
	pub originEquipWep24: i32,
	pub originEquipWep25: i32,
	pub vsPlayerDmgCorrectRate_Sleep: f32,
	pub vsPlayerDmgCorrectRate_Madness: f32,
	pub saGuardCutRate: f32,
	pub defMaterialVariationValue: u8,
	pub spAttributeVariationValue: u8,
	pub stealthAtkRate: i16,
	pub vsPlayerDmgCorrectRate_Disease: f32,
	pub vsPlayerDmgCorrectRate_Curse: f32,
	pub pad: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EQUIP_PARAM_WEAPON_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn rightHandEquipable(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn leftHandEquipable(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn bothHandEquipable(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn arrowSlotEquipable(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn boltSlotEquipable(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn enableGuard(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn enableParry(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn enableMagic(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn enableSorcery(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn enableMiracle(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn enableVowMagic(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isNormalAttackType(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn isBlowAttackType(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn isSlashAttackType(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn isThrustAttackType(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn isEnhance(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn isHeroPointCorrect(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn isCustom(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn disableBaseChangeReset(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn disableRepair(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn isDarkHand(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn simpleModelForDlc(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn lanternWep(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn isVersusGhostWep(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn baseChangeCategory(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn isDragonSlayer(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn isDeposit(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn disableMultiDropShare(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn isDiscard(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn isDrop(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn showLogCondType(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn enableThrow(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn showDialogCondType(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn disableGemAttr(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn isValidTough_ProtSADmg(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn isDualBlade(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
	pub fn isAutoEquip(&self) -> bool {	
			self.bits_6 & (1 << 2) != 0
	}
	pub fn isEnableEmergencyStep(&self) -> bool {	
			self.bits_6 & (1 << 3) != 0
	}
	pub fn invisibleOnRemo(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn unk1(&self) -> bool {	
			self.bits_6 & (1 << 5) != 0
	}
	pub fn residentSfx_1_IsVisibleForHang(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn residentSfx_2_IsVisibleForHang(&self) -> bool {	
			self.bits_7 & (1 << 1) != 0
	}
	pub fn residentSfx_3_IsVisibleForHang(&self) -> bool {	
			self.bits_7 & (1 << 2) != 0
	}
	pub fn residentSfx_4_IsVisibleForHang(&self) -> bool {	
			self.bits_7 & (1 << 3) != 0
	}
	pub fn isSoulParamIdChange_model0(&self) -> bool {	
			self.bits_7 & (1 << 4) != 0
	}
	pub fn isSoulParamIdChange_model1(&self) -> bool {	
			self.bits_7 & (1 << 5) != 0
	}
	pub fn isSoulParamIdChange_model2(&self) -> bool {	
			self.bits_7 & (1 << 6) != 0
	}
	pub fn isSoulParamIdChange_model3(&self) -> bool {	
			self.bits_7 & (1 << 7) != 0
	}
}
impl Default for EQUIP_PARAM_WEAPON_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 behaviorVariationId: 0,
			 sortId: 0,
			 wanderingEquipId: 0,
			 weight: 1.,
			 weaponWeightRate: 0.,
			 fixPrice: 0,
			 reinforcePrice: 0,
			 sellValue: 0,
			 correctStrength: 0.,
			 correctAgility: 0.,
			 correctMagic: 0.,
			 correctFaith: 0.,
			 physGuardCutRate: 0.,
			 magGuardCutRate: 0.,
			 fireGuardCutRate: 0.,
			 thunGuardCutRate: 0.,
			 spEffectBehaviorId0: -1,
			 spEffectBehaviorId1: -1,
			 spEffectBehaviorId2: -1,
			 residentSpEffectId: -1,
			 residentSpEffectId1: -1,
			 residentSpEffectId2: -1,
			 materialSetId: -1,
			 originEquipWep: -1,
			 originEquipWep1: -1,
			 originEquipWep2: -1,
			 originEquipWep3: -1,
			 originEquipWep4: -1,
			 originEquipWep5: -1,
			 originEquipWep6: -1,
			 originEquipWep7: -1,
			 originEquipWep8: -1,
			 originEquipWep9: -1,
			 originEquipWep10: -1,
			 originEquipWep11: -1,
			 originEquipWep12: -1,
			 originEquipWep13: -1,
			 originEquipWep14: -1,
			 originEquipWep15: -1,
			 weakA_DamageRate: 1.,
			 weakB_DamageRate: 1.,
			 weakC_DamageRate: 1.,
			 weakD_DamageRate: 1.,
			 sleepGuardResist_MaxCorrect: 0.,
			 madnessGuardResist_MaxCorrect: 0.,
			 saWeaponDamage: 0.,
			 equipModelId: 0,
			 iconId: 0,
			 durability: 100,
			 durabilityMax: 100,
			 attackThrowEscape: 0,
			 parryDamageLife: -1,
			 attackBasePhysics: 100,
			 attackBaseMagic: 100,
			 attackBaseFire: 100,
			 attackBaseThunder: 100,
			 attackBaseStamina: 100,
			 guardAngle: 0,
			 saDurability: 0.,
			 staminaGuardDef: 0,
			 reinforceTypeId: 0,
			 trophySGradeId: -1,
			 trophySeqId: -1,
			 throwAtkRate: 0,
			 bowDistRate: 0,
			 equipModelCategory: 7,
			 equipModelGender: 0,
			 weaponCategory: 0,
			 wepmotionCategory: 0,
			 guardmotionCategory: 0,
			 atkMaterial: 0,
			 defSeMaterial1: 0,
			 correctType_Physics: 0,
			 spAttribute: 0,
			 spAtkcategory: 0,
			 wepmotionOneHandId: 0,
			 wepmotionBothHandId: 0,
			 properStrength: 0,
			 properAgility: 0,
			 properMagic: 0,
			 properFaith: 0,
			 overStrength: 0,
			 attackBaseParry: 0,
			 defenseBaseParry: 0,
			 guardBaseRepel: 0,
			 attackBaseRepel: 0,
			 guardCutCancelRate: 0,
			 guardLevel: 0,
			 slashGuardCutRate: 0,
			 blowGuardCutRate: 0,
			 thrustGuardCutRate: 0,
			 poisonGuardResist: 0,
			 diseaseGuardResist: 0,
			 bloodGuardResist: 0,
			 curseGuardResist: 0,
			 atkAttribute: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 bits_4: 0,
			 bits_5: 0,
			 defSfxMaterial1: 0,
			 wepCollidableType0: 1,
			 wepCollidableType1: 1,
			 postureControlId_Right: 0,
			 postureControlId_Left: 0,
			 traceSfxId0: -1,
			 traceDmyIdHead0: -1,
			 traceDmyIdTail0: -1,
			 traceSfxId1: -1,
			 traceDmyIdHead1: -1,
			 traceDmyIdTail1: -1,
			 traceSfxId2: -1,
			 traceDmyIdHead2: -1,
			 traceDmyIdTail2: -1,
			 traceSfxId3: -1,
			 traceDmyIdHead3: -1,
			 traceDmyIdTail3: -1,
			 traceSfxId4: -1,
			 traceDmyIdHead4: -1,
			 traceDmyIdTail4: -1,
			 traceSfxId5: -1,
			 traceDmyIdHead5: -1,
			 traceDmyIdTail5: -1,
			 traceSfxId6: -1,
			 traceDmyIdHead6: -1,
			 traceDmyIdTail6: -1,
			 traceSfxId7: -1,
			 traceDmyIdHead7: -1,
			 traceDmyIdTail7: -1,
			 defSfxMaterial2: 0,
			 defSeMaterial2: 0,
			 absorpParamId: -1,
			 toughnessCorrectRate: 0.,
			 bits_6: 0,
			 correctType_Magic: 0,
			 correctType_Fire: 0,
			 correctType_Thunder: 0,
			 weakE_DamageRate: 1.,
			 weakF_DamageRate: 1.,
			 darkGuardCutRate: 0.,
			 attackBaseDark: 0,
			 correctType_Dark: 0,
			 correctType_Poison: 0,
			 sortGroupId: 255,
			 atkAttribute2: 0,
			 sleepGuardResist: 0,
			 madnessGuardResist: 0,
			 correctType_Blood: 0,
			 properLuck: 0,
			 freezeGuardResist: 0,
			 autoReplenishType: 0,
			 swordArtsParamId: 0,
			 correctLuck: 0.,
			 arrowBoltEquipId: 0,
			 DerivationLevelType: 0,
			 enchantSfxSize: 0,
			 wepType: 0,
			 physGuardCutRate_MaxCorrect: 0.,
			 magGuardCutRate_MaxCorrect: 0.,
			 fireGuardCutRate_MaxCorrect: 0.,
			 thunGuardCutRate_MaxCorrect: 0.,
			 darkGuardCutRate_MaxCorrect: 0.,
			 poisonGuardResist_MaxCorrect: 0.,
			 diseaseGuardResist_MaxCorrect: 0.,
			 bloodGuardResist_MaxCorrect: 0.,
			 curseGuardResist_MaxCorrect: 0.,
			 freezeGuardResist_MaxCorrect: 0.,
			 staminaGuardDef_MaxCorrect: 0.,
			 residentSfxId_1: -1,
			 residentSfxId_2: -1,
			 residentSfxId_3: -1,
			 residentSfxId_4: -1,
			 residentSfx_DmyId_1: -1,
			 residentSfx_DmyId_2: -1,
			 residentSfx_DmyId_3: -1,
			 residentSfx_DmyId_4: -1,
			 staminaConsumptionRate: 1.,
			 vsPlayerDmgCorrectRate_Physics: 1.,
			 vsPlayerDmgCorrectRate_Magic: 1.,
			 vsPlayerDmgCorrectRate_Fire: 1.,
			 vsPlayerDmgCorrectRate_Thunder: 1.,
			 vsPlayerDmgCorrectRate_Dark: 1.,
			 vsPlayerDmgCorrectRate_Poison: 1.,
			 vsPlayerDmgCorrectRate_Blood: 1.,
			 vsPlayerDmgCorrectRate_Freeze: 1.,
			 attainmentWepStatusStr: -1,
			 attainmentWepStatusDex: -1,
			 attainmentWepStatusMag: -1,
			 attainmentWepStatusFai: -1,
			 attainmentWepStatusLuc: -1,
			 attackElementCorrectId: 0,
			 saleValue: -1,
			 reinforceShopCategory: 0,
			 maxArrowQuantity: 1,
			 bits_7: 0,
			 wepSeIdOffset: 0,
			 baseChangePrice: 0,
			 levelSyncCorrectId: -1,
			 correctType_Sleep: 0,
			 correctType_Madness: 0,
			 rarity: 0,
			 gemMountType: 0,
			 wepRegainHp: 0,
			 spEffectMsgId0: -1,
			 spEffectMsgId1: -1,
			 spEffectMsgId2: -1,
			 originEquipWep16: -1,
			 originEquipWep17: -1,
			 originEquipWep18: -1,
			 originEquipWep19: -1,
			 originEquipWep20: -1,
			 originEquipWep21: -1,
			 originEquipWep22: -1,
			 originEquipWep23: -1,
			 originEquipWep24: -1,
			 originEquipWep25: -1,
			 vsPlayerDmgCorrectRate_Sleep: 1.,
			 vsPlayerDmgCorrectRate_Madness: 1.,
			 saGuardCutRate: 0.,
			 defMaterialVariationValue: 0,
			 spAttributeVariationValue: 0,
			 stealthAtkRate: 0,
			 vsPlayerDmgCorrectRate_Disease: 1.,
			 vsPlayerDmgCorrectRate_Curse: 1.,
			 pad: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ESTUS_FLASK_RECOVERY_PARAM_ST {
	pub host: u8,
	pub invadeOrb_None: u8,
	pub invadeOrb_Umbasa: u8,
	pub invadeOrb_Berserker: u8,
	pub invadeOrb_Sinners: u8,
	pub invadeSign_None: u8,
	pub invadeSign_Umbasa: u8,
	pub invadeSign_Berserker: u8,
	pub invadeSign_Sinners: u8,
	pub invadeRing_Sinners: u8,
	pub invadeRing_Rosalia: u8,
	pub invadeRing_Forest: u8,
	pub coopSign_None: u8,
	pub coopSign_Umbasa: u8,
	pub coopSign_Berserker: u8,
	pub coopSign_Sinners: u8,
	pub coopRing_RedHunter: u8,
	pub invadeRing_Anor: u8,
	pub paramReplaceRate: i16,
	pub paramReplaceId: i32,
	pub pad: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ESTUS_FLASK_RECOVERY_PARAM_ST {
}
impl Default for ESTUS_FLASK_RECOVERY_PARAM_ST {
	fn default() -> Self {
		Self {
			 host: 0,
			 invadeOrb_None: 0,
			 invadeOrb_Umbasa: 0,
			 invadeOrb_Berserker: 0,
			 invadeOrb_Sinners: 0,
			 invadeSign_None: 0,
			 invadeSign_Umbasa: 0,
			 invadeSign_Berserker: 0,
			 invadeSign_Sinners: 0,
			 invadeRing_Sinners: 0,
			 invadeRing_Rosalia: 0,
			 invadeRing_Forest: 0,
			 coopSign_None: 0,
			 coopSign_Umbasa: 0,
			 coopSign_Berserker: 0,
			 coopSign_Sinners: 0,
			 coopRing_RedHunter: 0,
			 invadeRing_Anor: 0,
			 paramReplaceRate: 0,
			 paramReplaceId: -1,
			 pad: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct EVENT_FLAG_USAGE_PARAM_ST {
	pub usageType: u8,
	pub playlogCategory: u8,
	pub padding1: [u8;2],
	pub flagNum: i32,
	pub padding2: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl EVENT_FLAG_USAGE_PARAM_ST {
}
impl Default for EVENT_FLAG_USAGE_PARAM_ST {
	fn default() -> Self {
		Self {
			 usageType: 0,
			 playlogCategory: 0,
			 padding1: [0;2],
			 flagNum: 1,
			 padding2: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct FACE_PARAM_ST {
	pub face_partsId: u8,
	pub skin_color_R: u8,
	pub skin_color_G: u8,
	pub skin_color_B: u8,
	pub skin_gloss: u8,
	pub skin_pores: u8,
	pub face_beard: u8,
	pub face_aroundEye: u8,
	pub face_aroundEyeColor_R: u8,
	pub face_aroundEyeColor_G: u8,
	pub face_aroundEyeColor_B: u8,
	pub face_cheek: u8,
	pub face_cheekColor_R: u8,
	pub face_cheekColor_G: u8,
	pub face_cheekColor_B: u8,
	pub face_eyeLine: u8,
	pub face_eyeLineColor_R: u8,
	pub face_eyeLineColor_G: u8,
	pub face_eyeLineColor_B: u8,
	pub face_eyeShadowDown: u8,
	pub face_eyeShadowDownColor_R: u8,
	pub face_eyeShadowDownColor_G: u8,
	pub face_eyeShadowDownColor_B: u8,
	pub face_eyeShadowUp: u8,
	pub face_eyeShadowUpColor_R: u8,
	pub face_eyeShadowUpColor_G: u8,
	pub face_eyeShadowUpColor_B: u8,
	pub face_lip: u8,
	pub face_lipColor_R: u8,
	pub face_lipColor_G: u8,
	pub face_lipColor_B: u8,
	pub body_hair: u8,
	pub body_hairColor_R: u8,
	pub body_hairColor_G: u8,
	pub body_hairColor_B: u8,
	pub eye_partsId: u8,
	pub eyeR_irisColor_R: u8,
	pub eyeR_irisColor_G: u8,
	pub eyeR_irisColor_B: u8,
	pub eyeR_irisScale: u8,
	pub eyeR_cataract: u8,
	pub eyeR_cataractColor_R: u8,
	pub eyeR_cataractColor_G: u8,
	pub eyeR_cataractColor_B: u8,
	pub eyeR_scleraColor_R: u8,
	pub eyeR_scleraColor_G: u8,
	pub eyeR_scleraColor_B: u8,
	pub eyeR_irisDistance: u8,
	pub eyeL_irisColor_R: u8,
	pub eyeL_irisColor_G: u8,
	pub eyeL_irisColor_B: u8,
	pub eyeL_irisScale: u8,
	pub eyeL_cataract: u8,
	pub eyeL_cataractColor_R: u8,
	pub eyeL_cataractColor_G: u8,
	pub eyeL_cataractColor_B: u8,
	pub eyeL_scleraColor_R: u8,
	pub eyeL_scleraColor_G: u8,
	pub eyeL_scleraColor_B: u8,
	pub eyeL_irisDistance: u8,
	pub hair_partsId: u8,
	pub hair_color_R: u8,
	pub hair_color_G: u8,
	pub hair_color_B: u8,
	pub hair_shininess: u8,
	pub hair_rootBlack: u8,
	pub hair_whiteDensity: u8,
	pub beard_partsId: u8,
	pub beard_color_R: u8,
	pub beard_color_G: u8,
	pub beard_color_B: u8,
	pub beard_shininess: u8,
	pub beard_rootBlack: u8,
	pub beard_whiteDensity: u8,
	pub eyebrow_partsId: u8,
	pub eyebrow_color_R: u8,
	pub eyebrow_color_G: u8,
	pub eyebrow_color_B: u8,
	pub eyebrow_shininess: u8,
	pub eyebrow_rootBlack: u8,
	pub eyebrow_whiteDensity: u8,
	pub eyelash_partsId: u8,
	pub eyelash_color_R: u8,
	pub eyelash_color_G: u8,
	pub eyelash_color_B: u8,
	pub accessories_partsId: u8,
	pub accessories_color_R: u8,
	pub accessories_color_G: u8,
	pub accessories_color_B: u8,
	pub decal_partsId: u8,
	pub decal_posX: u8,
	pub decal_posY: u8,
	pub decal_angle: u8,
	pub decal_scale: u8,
	pub decal_color_R: u8,
	pub decal_color_G: u8,
	pub decal_color_B: u8,
	pub decal_gloss: u8,
	pub decal_mirror: u8,
	pub chrBodyScaleHead: u8,
	pub chrBodyScaleBreast: u8,
	pub chrBodyScaleAbdomen: u8,
	pub chrBodyScaleRArm: u8,
	pub chrBodyScaleRLeg: u8,
	pub chrBodyScaleLArm: u8,
	pub chrBodyScaleLLeg: u8,
	pub burn_scar: u8,
	bits_0: u8,
	pub pad: [u8;5],
	pub age: u8,
	pub gender: u8,
	pub caricatureGeometry: u8,
	pub caricatureTexture: u8,
	pub faceGeoData00: u8,
	pub faceGeoData01: u8,
	pub faceGeoData02: u8,
	pub faceGeoData03: u8,
	pub faceGeoData04: u8,
	pub faceGeoData05: u8,
	pub faceGeoData06: u8,
	pub faceGeoData07: u8,
	pub faceGeoData08: u8,
	pub faceGeoData09: u8,
	pub faceGeoData10: u8,
	pub faceGeoData11: u8,
	pub faceGeoData12: u8,
	pub faceGeoData13: u8,
	pub faceGeoData14: u8,
	pub faceGeoData15: u8,
	pub faceGeoData16: u8,
	pub faceGeoData17: u8,
	pub faceGeoData18: u8,
	pub faceGeoData19: u8,
	pub faceGeoData20: u8,
	pub faceGeoData21: u8,
	pub faceGeoData22: u8,
	pub faceGeoData23: u8,
	pub faceGeoData24: u8,
	pub faceGeoData25: u8,
	pub faceGeoData26: u8,
	pub faceGeoData27: u8,
	pub faceGeoData28: u8,
	pub faceGeoData29: u8,
	pub faceGeoData30: u8,
	pub faceGeoData31: u8,
	pub faceGeoData32: u8,
	pub faceGeoData33: u8,
	pub faceGeoData34: u8,
	pub faceGeoData35: u8,
	pub faceGeoData36: u8,
	pub faceGeoData37: u8,
	pub faceGeoData38: u8,
	pub faceGeoData39: u8,
	pub faceGeoData40: u8,
	pub faceGeoData41: u8,
	pub faceGeoData42: u8,
	pub faceGeoData43: u8,
	pub faceGeoData44: u8,
	pub faceGeoData45: u8,
	pub faceGeoData46: u8,
	pub faceGeoData47: u8,
	pub faceGeoData48: u8,
	pub faceGeoData49: u8,
	pub faceGeoData50: u8,
	pub faceGeoData51: u8,
	pub faceGeoData52: u8,
	pub faceGeoData53: u8,
	pub faceGeoData54: u8,
	pub faceGeoData55: u8,
	pub faceGeoData56: u8,
	pub faceGeoData57: u8,
	pub faceGeoData58: u8,
	pub faceGeoData59: u8,
	pub faceGeoData60: u8,
	pub faceTexData00: u8,
	pub faceTexData01: u8,
	pub faceTexData02: u8,
	pub faceTexData03: u8,
	pub faceTexData04: u8,
	pub faceTexData05: u8,
	pub faceTexData06: u8,
	pub faceTexData07: u8,
	pub faceTexData08: u8,
	pub faceTexData09: u8,
	pub faceTexData10: u8,
	pub faceTexData11: u8,
	pub faceTexData12: u8,
	pub faceTexData13: u8,
	pub faceTexData14: u8,
	pub faceTexData15: u8,
	pub faceTexData16: u8,
	pub faceTexData17: u8,
	pub faceTexData18: u8,
	pub faceTexData19: u8,
	pub faceTexData20: u8,
	pub faceTexData21: u8,
	pub faceTexData22: u8,
	pub faceTexData23: u8,
	pub faceTexData24: u8,
	pub faceTexData25: u8,
	pub faceTexData26: u8,
	pub faceTexData27: u8,
	pub faceTexData28: u8,
	pub faceTexData29: u8,
	pub faceTexData30: u8,
	pub faceTexData31: u8,
	pub faceTexData32: u8,
	pub faceTexData33: u8,
	pub faceTexData34: u8,
	pub faceTexData35: u8,
	pub faceGeoAsymData00: u8,
	pub faceGeoAsymData01: u8,
	pub faceGeoAsymData02: u8,
	pub faceGeoAsymData03: u8,
	pub faceGeoAsymData04: u8,
	pub faceGeoAsymData05: u8,
	pub faceGeoAsymData06: u8,
	pub faceGeoAsymData07: u8,
	pub faceGeoAsymData08: u8,
	pub faceGeoAsymData09: u8,
	pub faceGeoAsymData10: u8,
	pub faceGeoAsymData11: u8,
	pub faceGeoAsymData12: u8,
	pub faceGeoAsymData13: u8,
	pub faceGeoAsymData14: u8,
	pub faceGeoAsymData15: u8,
	pub faceGeoAsymData16: u8,
	pub faceGeoAsymData17: u8,
	pub faceGeoAsymData18: u8,
	pub faceGeoAsymData19: u8,
	pub faceGeoAsymData20: u8,
	pub faceGeoAsymData21: u8,
	pub faceGeoAsymData22: u8,
	pub faceGeoAsymData23: u8,
	pub faceGeoAsymData24: u8,
	pub faceGeoAsymData25: u8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl FACE_PARAM_ST {
	pub fn override_eye_partsId(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn override_eye_irisColor(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn override_eye_cataract(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn override_eye_cataractColor(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn override_eye_scleraColor(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn override_burn_scar(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn pad2(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
}
impl Default for FACE_PARAM_ST {
	fn default() -> Self {
		Self {
			 face_partsId: 0,
			 skin_color_R: 128,
			 skin_color_G: 128,
			 skin_color_B: 128,
			 skin_gloss: 128,
			 skin_pores: 128,
			 face_beard: 128,
			 face_aroundEye: 128,
			 face_aroundEyeColor_R: 128,
			 face_aroundEyeColor_G: 128,
			 face_aroundEyeColor_B: 128,
			 face_cheek: 128,
			 face_cheekColor_R: 128,
			 face_cheekColor_G: 128,
			 face_cheekColor_B: 128,
			 face_eyeLine: 128,
			 face_eyeLineColor_R: 128,
			 face_eyeLineColor_G: 128,
			 face_eyeLineColor_B: 128,
			 face_eyeShadowDown: 128,
			 face_eyeShadowDownColor_R: 128,
			 face_eyeShadowDownColor_G: 128,
			 face_eyeShadowDownColor_B: 128,
			 face_eyeShadowUp: 128,
			 face_eyeShadowUpColor_R: 128,
			 face_eyeShadowUpColor_G: 128,
			 face_eyeShadowUpColor_B: 128,
			 face_lip: 128,
			 face_lipColor_R: 128,
			 face_lipColor_G: 128,
			 face_lipColor_B: 128,
			 body_hair: 128,
			 body_hairColor_R: 128,
			 body_hairColor_G: 128,
			 body_hairColor_B: 128,
			 eye_partsId: 0,
			 eyeR_irisColor_R: 128,
			 eyeR_irisColor_G: 128,
			 eyeR_irisColor_B: 128,
			 eyeR_irisScale: 128,
			 eyeR_cataract: 128,
			 eyeR_cataractColor_R: 128,
			 eyeR_cataractColor_G: 128,
			 eyeR_cataractColor_B: 128,
			 eyeR_scleraColor_R: 128,
			 eyeR_scleraColor_G: 128,
			 eyeR_scleraColor_B: 128,
			 eyeR_irisDistance: 128,
			 eyeL_irisColor_R: 128,
			 eyeL_irisColor_G: 128,
			 eyeL_irisColor_B: 128,
			 eyeL_irisScale: 128,
			 eyeL_cataract: 128,
			 eyeL_cataractColor_R: 128,
			 eyeL_cataractColor_G: 128,
			 eyeL_cataractColor_B: 128,
			 eyeL_scleraColor_R: 128,
			 eyeL_scleraColor_G: 128,
			 eyeL_scleraColor_B: 128,
			 eyeL_irisDistance: 128,
			 hair_partsId: 0,
			 hair_color_R: 128,
			 hair_color_G: 128,
			 hair_color_B: 128,
			 hair_shininess: 128,
			 hair_rootBlack: 128,
			 hair_whiteDensity: 128,
			 beard_partsId: 0,
			 beard_color_R: 128,
			 beard_color_G: 128,
			 beard_color_B: 128,
			 beard_shininess: 128,
			 beard_rootBlack: 128,
			 beard_whiteDensity: 128,
			 eyebrow_partsId: 0,
			 eyebrow_color_R: 128,
			 eyebrow_color_G: 128,
			 eyebrow_color_B: 128,
			 eyebrow_shininess: 128,
			 eyebrow_rootBlack: 128,
			 eyebrow_whiteDensity: 128,
			 eyelash_partsId: 0,
			 eyelash_color_R: 128,
			 eyelash_color_G: 128,
			 eyelash_color_B: 128,
			 accessories_partsId: 0,
			 accessories_color_R: 128,
			 accessories_color_G: 128,
			 accessories_color_B: 128,
			 decal_partsId: 0,
			 decal_posX: 0,
			 decal_posY: 0,
			 decal_angle: 0,
			 decal_scale: 0,
			 decal_color_R: 128,
			 decal_color_G: 128,
			 decal_color_B: 128,
			 decal_gloss: 128,
			 decal_mirror: 0,
			 chrBodyScaleHead: 128,
			 chrBodyScaleBreast: 128,
			 chrBodyScaleAbdomen: 128,
			 chrBodyScaleRArm: 128,
			 chrBodyScaleRLeg: 128,
			 chrBodyScaleLArm: 128,
			 chrBodyScaleLLeg: 128,
			 burn_scar: 0,
			 bits_0: 0,
			 pad: [0;5],
			 age: 128,
			 gender: 128,
			 caricatureGeometry: 128,
			 caricatureTexture: 128,
			 faceGeoData00: 128,
			 faceGeoData01: 128,
			 faceGeoData02: 128,
			 faceGeoData03: 128,
			 faceGeoData04: 128,
			 faceGeoData05: 128,
			 faceGeoData06: 128,
			 faceGeoData07: 128,
			 faceGeoData08: 128,
			 faceGeoData09: 128,
			 faceGeoData10: 128,
			 faceGeoData11: 128,
			 faceGeoData12: 128,
			 faceGeoData13: 128,
			 faceGeoData14: 128,
			 faceGeoData15: 128,
			 faceGeoData16: 128,
			 faceGeoData17: 128,
			 faceGeoData18: 128,
			 faceGeoData19: 128,
			 faceGeoData20: 128,
			 faceGeoData21: 128,
			 faceGeoData22: 128,
			 faceGeoData23: 128,
			 faceGeoData24: 128,
			 faceGeoData25: 128,
			 faceGeoData26: 128,
			 faceGeoData27: 128,
			 faceGeoData28: 128,
			 faceGeoData29: 128,
			 faceGeoData30: 128,
			 faceGeoData31: 128,
			 faceGeoData32: 128,
			 faceGeoData33: 128,
			 faceGeoData34: 128,
			 faceGeoData35: 128,
			 faceGeoData36: 128,
			 faceGeoData37: 128,
			 faceGeoData38: 128,
			 faceGeoData39: 128,
			 faceGeoData40: 128,
			 faceGeoData41: 128,
			 faceGeoData42: 128,
			 faceGeoData43: 128,
			 faceGeoData44: 128,
			 faceGeoData45: 128,
			 faceGeoData46: 128,
			 faceGeoData47: 128,
			 faceGeoData48: 128,
			 faceGeoData49: 128,
			 faceGeoData50: 128,
			 faceGeoData51: 128,
			 faceGeoData52: 128,
			 faceGeoData53: 128,
			 faceGeoData54: 128,
			 faceGeoData55: 128,
			 faceGeoData56: 128,
			 faceGeoData57: 128,
			 faceGeoData58: 128,
			 faceGeoData59: 128,
			 faceGeoData60: 128,
			 faceTexData00: 128,
			 faceTexData01: 128,
			 faceTexData02: 128,
			 faceTexData03: 128,
			 faceTexData04: 128,
			 faceTexData05: 128,
			 faceTexData06: 128,
			 faceTexData07: 128,
			 faceTexData08: 128,
			 faceTexData09: 128,
			 faceTexData10: 128,
			 faceTexData11: 128,
			 faceTexData12: 128,
			 faceTexData13: 128,
			 faceTexData14: 128,
			 faceTexData15: 128,
			 faceTexData16: 128,
			 faceTexData17: 128,
			 faceTexData18: 128,
			 faceTexData19: 128,
			 faceTexData20: 128,
			 faceTexData21: 128,
			 faceTexData22: 128,
			 faceTexData23: 128,
			 faceTexData24: 128,
			 faceTexData25: 128,
			 faceTexData26: 128,
			 faceTexData27: 128,
			 faceTexData28: 128,
			 faceTexData29: 128,
			 faceTexData30: 128,
			 faceTexData31: 128,
			 faceTexData32: 128,
			 faceTexData33: 128,
			 faceTexData34: 128,
			 faceTexData35: 128,
			 faceGeoAsymData00: 128,
			 faceGeoAsymData01: 128,
			 faceGeoAsymData02: 128,
			 faceGeoAsymData03: 128,
			 faceGeoAsymData04: 128,
			 faceGeoAsymData05: 128,
			 faceGeoAsymData06: 128,
			 faceGeoAsymData07: 128,
			 faceGeoAsymData08: 128,
			 faceGeoAsymData09: 128,
			 faceGeoAsymData10: 128,
			 faceGeoAsymData11: 128,
			 faceGeoAsymData12: 128,
			 faceGeoAsymData13: 128,
			 faceGeoAsymData14: 128,
			 faceGeoAsymData15: 128,
			 faceGeoAsymData16: 128,
			 faceGeoAsymData17: 128,
			 faceGeoAsymData18: 128,
			 faceGeoAsymData19: 128,
			 faceGeoAsymData20: 128,
			 faceGeoAsymData21: 128,
			 faceGeoAsymData22: 128,
			 faceGeoAsymData23: 128,
			 faceGeoAsymData24: 128,
			 faceGeoAsymData25: 128,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct FACE_RANGE_PARAM_ST {
	pub face_partsId: f32,
	pub skin_color_R: f32,
	pub skin_color_G: f32,
	pub skin_color_B: f32,
	pub skin_gloss: f32,
	pub skin_pores: f32,
	pub face_beard: f32,
	pub face_aroundEye: f32,
	pub face_aroundEyeColor_R: f32,
	pub face_aroundEyeColor_G: f32,
	pub face_aroundEyeColor_B: f32,
	pub face_cheek: f32,
	pub face_cheekColor_R: f32,
	pub face_cheekColor_G: f32,
	pub face_cheekColor_B: f32,
	pub face_eyeLine: f32,
	pub face_eyeLineColor_R: f32,
	pub face_eyeLineColor_G: f32,
	pub face_eyeLineColor_B: f32,
	pub face_eyeShadowDown: f32,
	pub face_eyeShadowDownColor_R: f32,
	pub face_eyeShadowDownColor_G: f32,
	pub face_eyeShadowDownColor_B: f32,
	pub face_eyeShadowUp: f32,
	pub face_eyeShadowUpColor_R: f32,
	pub face_eyeShadowUpColor_G: f32,
	pub face_eyeShadowUpColor_B: f32,
	pub face_lip: f32,
	pub face_lipColor_R: f32,
	pub face_lipColor_G: f32,
	pub face_lipColor_B: f32,
	pub body_hair: f32,
	pub body_hairColor_R: f32,
	pub body_hairColor_G: f32,
	pub body_hairColor_B: f32,
	pub eye_partsId: f32,
	pub eyeR_irisColor_R: f32,
	pub eyeR_irisColor_G: f32,
	pub eyeR_irisColor_B: f32,
	pub eyeR_irisScale: f32,
	pub eyeR_cataract: f32,
	pub eyeR_cataractColor_R: f32,
	pub eyeR_cataractColor_G: f32,
	pub eyeR_cataractColor_B: f32,
	pub eyeR_scleraColor_R: f32,
	pub eyeR_scleraColor_G: f32,
	pub eyeR_scleraColor_B: f32,
	pub eyeR_irisDistance: f32,
	pub eyeL_irisColor_R: f32,
	pub eyeL_irisColor_G: f32,
	pub eyeL_irisColor_B: f32,
	pub eyeL_irisScale: f32,
	pub eyeL_cataract: f32,
	pub eyeL_cataractColor_R: f32,
	pub eyeL_cataractColor_G: f32,
	pub eyeL_cataractColor_B: f32,
	pub eyeL_scleraColor_R: f32,
	pub eyeL_scleraColor_G: f32,
	pub eyeL_scleraColor_B: f32,
	pub eyeL_irisDistance: f32,
	pub hair_partsId: f32,
	pub hair_color_R: f32,
	pub hair_color_G: f32,
	pub hair_color_B: f32,
	pub hair_shininess: f32,
	pub hair_rootBlack: f32,
	pub hair_whiteDensity: f32,
	pub beard_partsId: f32,
	pub beard_color_R: f32,
	pub beard_color_G: f32,
	pub beard_color_B: f32,
	pub beard_shininess: f32,
	pub beard_rootBlack: f32,
	pub beard_whiteDensity: f32,
	pub eyebrow_partsId: f32,
	pub eyebrow_color_R: f32,
	pub eyebrow_color_G: f32,
	pub eyebrow_color_B: f32,
	pub eyebrow_shininess: f32,
	pub eyebrow_rootBlack: f32,
	pub eyebrow_whiteDensity: f32,
	pub eyelash_partsId: f32,
	pub eyelash_color_R: f32,
	pub eyelash_color_G: f32,
	pub eyelash_color_B: f32,
	pub accessories_partsId: f32,
	pub accessories_color_R: f32,
	pub accessories_color_G: f32,
	pub accessories_color_B: f32,
	pub decal_partsId: f32,
	pub decal_posX: f32,
	pub decal_posY: f32,
	pub decal_angle: f32,
	pub decal_scale: f32,
	pub decal_color_R: f32,
	pub decal_color_G: f32,
	pub decal_color_B: f32,
	pub decal_gloss: f32,
	pub decal_mirror: f32,
	pub chrBodyScaleHead: f32,
	pub chrBodyScaleBreast: f32,
	pub chrBodyScaleAbdomen: f32,
	pub chrBodyScaleArm: f32,
	pub chrBodyScaleLeg: f32,
	pub age: f32,
	pub gender: f32,
	pub caricatureGeometry: f32,
	pub caricatureTexture: f32,
	pub faceGeoData00: f32,
	pub faceGeoData01: f32,
	pub faceGeoData02: f32,
	pub faceGeoData03: f32,
	pub faceGeoData04: f32,
	pub faceGeoData05: f32,
	pub faceGeoData06: f32,
	pub faceGeoData07: f32,
	pub faceGeoData08: f32,
	pub faceGeoData09: f32,
	pub faceGeoData10: f32,
	pub faceGeoData11: f32,
	pub faceGeoData12: f32,
	pub faceGeoData13: f32,
	pub faceGeoData14: f32,
	pub faceGeoData15: f32,
	pub faceGeoData16: f32,
	pub faceGeoData17: f32,
	pub faceGeoData18: f32,
	pub faceGeoData19: f32,
	pub faceGeoData20: f32,
	pub faceGeoData21: f32,
	pub faceGeoData22: f32,
	pub faceGeoData23: f32,
	pub faceGeoData24: f32,
	pub faceGeoData25: f32,
	pub faceGeoData26: f32,
	pub faceGeoData27: f32,
	pub faceGeoData28: f32,
	pub faceGeoData29: f32,
	pub faceGeoData30: f32,
	pub faceGeoData31: f32,
	pub faceGeoData32: f32,
	pub faceGeoData33: f32,
	pub faceGeoData34: f32,
	pub faceGeoData35: f32,
	pub faceGeoData36: f32,
	pub faceGeoData37: f32,
	pub faceGeoData38: f32,
	pub faceGeoData39: f32,
	pub faceGeoData40: f32,
	pub faceGeoData41: f32,
	pub faceGeoData42: f32,
	pub faceGeoData43: f32,
	pub faceGeoData44: f32,
	pub faceGeoData45: f32,
	pub faceGeoData46: f32,
	pub faceGeoData47: f32,
	pub faceGeoData48: f32,
	pub faceGeoData49: f32,
	pub faceGeoData50: f32,
	pub faceGeoData51: f32,
	pub faceGeoData52: f32,
	pub faceGeoData53: f32,
	pub faceGeoData54: f32,
	pub faceGeoData55: f32,
	pub faceGeoData56: f32,
	pub faceGeoData57: f32,
	pub faceGeoData58: f32,
	pub faceGeoData59: f32,
	pub faceGeoData60: f32,
	pub faceTexData00: f32,
	pub faceTexData01: f32,
	pub faceTexData02: f32,
	pub faceTexData03: f32,
	pub faceTexData04: f32,
	pub faceTexData05: f32,
	pub faceTexData06: f32,
	pub faceTexData07: f32,
	pub faceTexData08: f32,
	pub faceTexData09: f32,
	pub faceTexData10: f32,
	pub faceTexData11: f32,
	pub faceTexData12: f32,
	pub faceTexData13: f32,
	pub faceTexData14: f32,
	pub faceTexData15: f32,
	pub faceTexData16: f32,
	pub faceTexData17: f32,
	pub faceTexData18: f32,
	pub faceTexData19: f32,
	pub faceTexData20: f32,
	pub faceTexData21: f32,
	pub faceTexData22: f32,
	pub faceTexData23: f32,
	pub faceTexData24: f32,
	pub faceTexData25: f32,
	pub faceTexData26: f32,
	pub faceTexData27: f32,
	pub faceTexData28: f32,
	pub faceTexData29: f32,
	pub faceTexData30: f32,
	pub faceTexData31: f32,
	pub faceTexData32: f32,
	pub faceTexData33: f32,
	pub faceTexData34: f32,
	pub faceTexData35: f32,
	pub burn_scar: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl FACE_RANGE_PARAM_ST {
}
impl Default for FACE_RANGE_PARAM_ST {
	fn default() -> Self {
		Self {
			 face_partsId: 0.,
			 skin_color_R: 0.,
			 skin_color_G: 0.,
			 skin_color_B: 0.,
			 skin_gloss: 0.,
			 skin_pores: 0.,
			 face_beard: 0.,
			 face_aroundEye: 0.,
			 face_aroundEyeColor_R: 0.,
			 face_aroundEyeColor_G: 0.,
			 face_aroundEyeColor_B: 0.,
			 face_cheek: 0.,
			 face_cheekColor_R: 0.,
			 face_cheekColor_G: 0.,
			 face_cheekColor_B: 0.,
			 face_eyeLine: 0.,
			 face_eyeLineColor_R: 0.,
			 face_eyeLineColor_G: 0.,
			 face_eyeLineColor_B: 0.,
			 face_eyeShadowDown: 0.,
			 face_eyeShadowDownColor_R: 0.,
			 face_eyeShadowDownColor_G: 0.,
			 face_eyeShadowDownColor_B: 0.,
			 face_eyeShadowUp: 0.,
			 face_eyeShadowUpColor_R: 0.,
			 face_eyeShadowUpColor_G: 0.,
			 face_eyeShadowUpColor_B: 0.,
			 face_lip: 0.,
			 face_lipColor_R: 0.,
			 face_lipColor_G: 0.,
			 face_lipColor_B: 0.,
			 body_hair: 0.,
			 body_hairColor_R: 0.,
			 body_hairColor_G: 0.,
			 body_hairColor_B: 0.,
			 eye_partsId: 0.,
			 eyeR_irisColor_R: 0.,
			 eyeR_irisColor_G: 0.,
			 eyeR_irisColor_B: 0.,
			 eyeR_irisScale: 0.,
			 eyeR_cataract: 0.,
			 eyeR_cataractColor_R: 0.,
			 eyeR_cataractColor_G: 0.,
			 eyeR_cataractColor_B: 0.,
			 eyeR_scleraColor_R: 0.,
			 eyeR_scleraColor_G: 0.,
			 eyeR_scleraColor_B: 0.,
			 eyeR_irisDistance: 0.,
			 eyeL_irisColor_R: 0.,
			 eyeL_irisColor_G: 0.,
			 eyeL_irisColor_B: 0.,
			 eyeL_irisScale: 0.,
			 eyeL_cataract: 0.,
			 eyeL_cataractColor_R: 0.,
			 eyeL_cataractColor_G: 0.,
			 eyeL_cataractColor_B: 0.,
			 eyeL_scleraColor_R: 0.,
			 eyeL_scleraColor_G: 0.,
			 eyeL_scleraColor_B: 0.,
			 eyeL_irisDistance: 0.,
			 hair_partsId: 0.,
			 hair_color_R: 0.,
			 hair_color_G: 0.,
			 hair_color_B: 0.,
			 hair_shininess: 0.,
			 hair_rootBlack: 0.,
			 hair_whiteDensity: 0.,
			 beard_partsId: 0.,
			 beard_color_R: 0.,
			 beard_color_G: 0.,
			 beard_color_B: 0.,
			 beard_shininess: 0.,
			 beard_rootBlack: 0.,
			 beard_whiteDensity: 0.,
			 eyebrow_partsId: 0.,
			 eyebrow_color_R: 0.,
			 eyebrow_color_G: 0.,
			 eyebrow_color_B: 0.,
			 eyebrow_shininess: 0.,
			 eyebrow_rootBlack: 0.,
			 eyebrow_whiteDensity: 0.,
			 eyelash_partsId: 0.,
			 eyelash_color_R: 0.,
			 eyelash_color_G: 0.,
			 eyelash_color_B: 0.,
			 accessories_partsId: 0.,
			 accessories_color_R: 0.,
			 accessories_color_G: 0.,
			 accessories_color_B: 0.,
			 decal_partsId: 0.,
			 decal_posX: 0.,
			 decal_posY: 0.,
			 decal_angle: 0.,
			 decal_scale: 0.,
			 decal_color_R: 0.,
			 decal_color_G: 0.,
			 decal_color_B: 0.,
			 decal_gloss: 0.,
			 decal_mirror: 0.,
			 chrBodyScaleHead: 0.,
			 chrBodyScaleBreast: 0.,
			 chrBodyScaleAbdomen: 0.,
			 chrBodyScaleArm: 0.,
			 chrBodyScaleLeg: 0.,
			 age: 0.,
			 gender: 0.,
			 caricatureGeometry: 0.,
			 caricatureTexture: 0.,
			 faceGeoData00: 0.,
			 faceGeoData01: 0.,
			 faceGeoData02: 0.,
			 faceGeoData03: 0.,
			 faceGeoData04: 0.,
			 faceGeoData05: 0.,
			 faceGeoData06: 0.,
			 faceGeoData07: 0.,
			 faceGeoData08: 0.,
			 faceGeoData09: 0.,
			 faceGeoData10: 0.,
			 faceGeoData11: 0.,
			 faceGeoData12: 0.,
			 faceGeoData13: 0.,
			 faceGeoData14: 0.,
			 faceGeoData15: 0.,
			 faceGeoData16: 0.,
			 faceGeoData17: 0.,
			 faceGeoData18: 0.,
			 faceGeoData19: 0.,
			 faceGeoData20: 0.,
			 faceGeoData21: 0.,
			 faceGeoData22: 0.,
			 faceGeoData23: 0.,
			 faceGeoData24: 0.,
			 faceGeoData25: 0.,
			 faceGeoData26: 0.,
			 faceGeoData27: 0.,
			 faceGeoData28: 0.,
			 faceGeoData29: 0.,
			 faceGeoData30: 0.,
			 faceGeoData31: 0.,
			 faceGeoData32: 0.,
			 faceGeoData33: 0.,
			 faceGeoData34: 0.,
			 faceGeoData35: 0.,
			 faceGeoData36: 0.,
			 faceGeoData37: 0.,
			 faceGeoData38: 0.,
			 faceGeoData39: 0.,
			 faceGeoData40: 0.,
			 faceGeoData41: 0.,
			 faceGeoData42: 0.,
			 faceGeoData43: 0.,
			 faceGeoData44: 0.,
			 faceGeoData45: 0.,
			 faceGeoData46: 0.,
			 faceGeoData47: 0.,
			 faceGeoData48: 0.,
			 faceGeoData49: 0.,
			 faceGeoData50: 0.,
			 faceGeoData51: 0.,
			 faceGeoData52: 0.,
			 faceGeoData53: 0.,
			 faceGeoData54: 0.,
			 faceGeoData55: 0.,
			 faceGeoData56: 0.,
			 faceGeoData57: 0.,
			 faceGeoData58: 0.,
			 faceGeoData59: 0.,
			 faceGeoData60: 0.,
			 faceTexData00: 0.,
			 faceTexData01: 0.,
			 faceTexData02: 0.,
			 faceTexData03: 0.,
			 faceTexData04: 0.,
			 faceTexData05: 0.,
			 faceTexData06: 0.,
			 faceTexData07: 0.,
			 faceTexData08: 0.,
			 faceTexData09: 0.,
			 faceTexData10: 0.,
			 faceTexData11: 0.,
			 faceTexData12: 0.,
			 faceTexData13: 0.,
			 faceTexData14: 0.,
			 faceTexData15: 0.,
			 faceTexData16: 0.,
			 faceTexData17: 0.,
			 faceTexData18: 0.,
			 faceTexData19: 0.,
			 faceTexData20: 0.,
			 faceTexData21: 0.,
			 faceTexData22: 0.,
			 faceTexData23: 0.,
			 faceTexData24: 0.,
			 faceTexData25: 0.,
			 faceTexData26: 0.,
			 faceTexData27: 0.,
			 faceTexData28: 0.,
			 faceTexData29: 0.,
			 faceTexData30: 0.,
			 faceTexData31: 0.,
			 faceTexData32: 0.,
			 faceTexData33: 0.,
			 faceTexData34: 0.,
			 faceTexData35: 0.,
			 burn_scar: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct FE_TEXT_EFFECT_PARAM_ST {
	pub resId: i16,
	pub pad1: [u8;2],
	pub textId: i32,
	pub seId: i32,
	bits_0: u8,
	pub pad2: [u8;19],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl FE_TEXT_EFFECT_PARAM_ST {
	pub fn canMixMapName(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn pad3(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for FE_TEXT_EFFECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 resId: 0,
			 pad1: [0;2],
			 textId: -1,
			 seId: -1,
			 bits_0: 0,
			 pad2: [0;19],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct FINAL_DAMAGE_RATE_PARAM_ST {
	pub physRate: f32,
	pub magRate: f32,
	pub fireRate: f32,
	pub thunRate: f32,
	pub darkRate: f32,
	pub staminaRate: f32,
	pub saRate: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl FINAL_DAMAGE_RATE_PARAM_ST {
}
impl Default for FINAL_DAMAGE_RATE_PARAM_ST {
	fn default() -> Self {
		Self {
			 physRate: 0.,
			 magRate: 0.,
			 fireRate: 0.,
			 thunRate: 0.,
			 darkRate: 0.,
			 staminaRate: 0.,
			 saRate: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct FOOT_SFX_PARAM_ST {
	pub sfxId_00: i32,
	pub sfxId_01: i32,
	pub sfxId_02: i32,
	pub sfxId_03: i32,
	pub sfxId_04: i32,
	pub sfxId_05: i32,
	pub sfxId_06: i32,
	pub sfxId_07: i32,
	pub sfxId_08: i32,
	pub sfxId_09: i32,
	pub sfxId_10: i32,
	pub sfxId_11: i32,
	pub sfxId_12: i32,
	pub sfxId_13: i32,
	pub sfxId_14: i32,
	pub sfxId_15: i32,
	pub sfxId_16: i32,
	pub sfxId_17: i32,
	pub sfxId_18: i32,
	pub sfxId_19: i32,
	pub sfxId_20: i32,
	pub sfxId_21: i32,
	pub sfxId_22: i32,
	pub sfxId_23: i32,
	pub sfxId_24: i32,
	pub sfxId_25: i32,
	pub sfxId_26: i32,
	pub sfxId_27: i32,
	pub sfxId_28: i32,
	pub sfxId_29: i32,
	pub sfxId_30: i32,
	pub sfxId_31: i32,
	pub sfxId_32: i32,
	pub sfxId_33: i32,
	pub sfxId_34: i32,
	pub sfxId_35: i32,
	pub sfxId_36: i32,
	pub sfxId_37: i32,
	pub sfxId_38: i32,
	pub sfxId_39: i32,
	pub sfxId_40: i32,
	pub sfxId_41: i32,
	pub sfxId_42: i32,
	pub sfxId_43: i32,
	pub sfxId_44: i32,
	pub sfxId_45: i32,
	pub sfxId_46: i32,
	pub sfxId_47: i32,
	pub sfxId_48: i32,
	pub sfxId_49: i32,
	pub sfxId_50: i32,
	pub sfxId_51: i32,
	pub sfxId_52: i32,
	pub sfxId_53: i32,
	pub sfxId_54: i32,
	pub sfxId_55: i32,
	pub sfxId_56: i32,
	pub sfxId_57: i32,
	pub sfxId_58: i32,
	pub sfxId_59: i32,
	pub sfxId_60: i32,
	pub sfxId_61: i32,
	pub sfxId_62: i32,
	pub sfxId_63: i32,
	pub sfxId_64: i32,
	pub sfxId_65: i32,
	pub sfxId_66: i32,
	pub sfxId_67: i32,
	pub sfxId_68: i32,
	pub sfxId_69: i32,
	pub sfxId_70: i32,
	pub sfxId_71: i32,
	pub sfxId_72: i32,
	pub sfxId_73: i32,
	pub sfxId_74: i32,
	pub sfxId_75: i32,
	pub sfxId_76: i32,
	pub sfxId_77: i32,
	pub sfxId_78: i32,
	pub sfxId_79: i32,
	pub sfxId_80: i32,
	pub sfxId_81: i32,
	pub sfxId_82: i32,
	pub sfxId_83: i32,
	pub sfxId_84: i32,
	pub sfxId_85: i32,
	pub sfxId_86: i32,
	pub sfxId_87: i32,
	pub sfxId_88: i32,
	pub sfxId_89: i32,
	pub sfxId_90: i32,
	pub sfxId_91: i32,
	pub sfxId_92: i32,
	pub sfxId_93: i32,
	pub sfxId_94: i32,
	pub sfxId_95: i32,
	pub sfxId_96: i32,
	pub sfxId_97: i32,
	pub sfxId_98: i32,
	pub sfxId_99: i32,
	pub sfxId_100: i32,
	pub sfxId_101: i32,
	pub sfxId_102: i32,
	pub sfxId_103: i32,
	pub sfxId_104: i32,
	pub sfxId_105: i32,
	pub sfxId_106: i32,
	pub sfxId_107: i32,
	pub sfxId_108: i32,
	pub sfxId_109: i32,
	pub sfxId_110: i32,
	pub sfxId_111: i32,
	pub sfxId_112: i32,
	pub sfxId_113: i32,
	pub sfxId_114: i32,
	pub sfxId_115: i32,
	pub sfxId_116: i32,
	pub sfxId_117: i32,
	pub sfxId_118: i32,
	pub sfxId_119: i32,
	pub sfxId_120: i32,
	pub sfxId_121: i32,
	pub sfxId_122: i32,
	pub sfxId_123: i32,
	pub sfxId_124: i32,
	pub sfxId_125: i32,
	pub sfxId_126: i32,
	pub sfxId_127: i32,
	pub sfxId_128: i32,
	pub sfxId_129: i32,
	pub sfxId_130: i32,
	pub sfxId_131: i32,
	pub sfxId_132: i32,
	pub sfxId_133: i32,
	pub sfxId_134: i32,
	pub sfxId_135: i32,
	pub sfxId_136: i32,
	pub sfxId_137: i32,
	pub sfxId_138: i32,
	pub sfxId_139: i32,
	pub sfxId_140: i32,
	pub sfxId_141: i32,
	pub sfxId_142: i32,
	pub sfxId_143: i32,
	pub sfxId_144: i32,
	pub sfxId_145: i32,
	pub sfxId_146: i32,
	pub sfxId_147: i32,
	pub sfxId_148: i32,
	pub sfxId_149: i32,
	pub sfxId_150: i32,
	pub sfxId_151: i32,
	pub sfxId_152: i32,
	pub sfxId_153: i32,
	pub sfxId_154: i32,
	pub sfxId_155: i32,
	pub sfxId_156: i32,
	pub sfxId_157: i32,
	pub sfxId_158: i32,
	pub sfxId_159: i32,
	pub sfxId_160: i32,
	pub sfxId_161: i32,
	pub sfxId_162: i32,
	pub sfxId_163: i32,
	pub sfxId_164: i32,
	pub sfxId_165: i32,
	pub sfxId_166: i32,
	pub sfxId_167: i32,
	pub sfxId_168: i32,
	pub sfxId_169: i32,
	pub sfxId_170: i32,
	pub sfxId_171: i32,
	pub sfxId_172: i32,
	pub sfxId_173: i32,
	pub sfxId_174: i32,
	pub sfxId_175: i32,
	pub sfxId_176: i32,
	pub sfxId_177: i32,
	pub sfxId_178: i32,
	pub sfxId_179: i32,
	pub sfxId_180: i32,
	pub sfxId_181: i32,
	pub sfxId_182: i32,
	pub sfxId_183: i32,
	pub sfxId_184: i32,
	pub sfxId_185: i32,
	pub sfxId_186: i32,
	pub sfxId_187: i32,
	pub sfxId_188: i32,
	pub sfxId_189: i32,
	pub sfxId_190: i32,
	pub sfxId_191: i32,
	pub sfxId_192: i32,
	pub sfxId_193: i32,
	pub sfxId_194: i32,
	pub sfxId_195: i32,
	pub sfxId_196: i32,
	pub sfxId_197: i32,
	pub sfxId_198: i32,
	pub sfxId_199: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl FOOT_SFX_PARAM_ST {
}
impl Default for FOOT_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 sfxId_00: 0,
			 sfxId_01: 0,
			 sfxId_02: 0,
			 sfxId_03: 0,
			 sfxId_04: 0,
			 sfxId_05: 0,
			 sfxId_06: 0,
			 sfxId_07: 0,
			 sfxId_08: 0,
			 sfxId_09: 0,
			 sfxId_10: 0,
			 sfxId_11: 0,
			 sfxId_12: 0,
			 sfxId_13: 0,
			 sfxId_14: 0,
			 sfxId_15: 0,
			 sfxId_16: 0,
			 sfxId_17: 0,
			 sfxId_18: 0,
			 sfxId_19: 0,
			 sfxId_20: 0,
			 sfxId_21: 0,
			 sfxId_22: 0,
			 sfxId_23: 0,
			 sfxId_24: 0,
			 sfxId_25: 0,
			 sfxId_26: 0,
			 sfxId_27: 0,
			 sfxId_28: 0,
			 sfxId_29: 0,
			 sfxId_30: 0,
			 sfxId_31: 0,
			 sfxId_32: 0,
			 sfxId_33: 0,
			 sfxId_34: 0,
			 sfxId_35: 0,
			 sfxId_36: 0,
			 sfxId_37: 0,
			 sfxId_38: 0,
			 sfxId_39: 0,
			 sfxId_40: 0,
			 sfxId_41: 0,
			 sfxId_42: 0,
			 sfxId_43: 0,
			 sfxId_44: 0,
			 sfxId_45: 0,
			 sfxId_46: 0,
			 sfxId_47: 0,
			 sfxId_48: 0,
			 sfxId_49: 0,
			 sfxId_50: 0,
			 sfxId_51: 0,
			 sfxId_52: 0,
			 sfxId_53: 0,
			 sfxId_54: 0,
			 sfxId_55: 0,
			 sfxId_56: 0,
			 sfxId_57: 0,
			 sfxId_58: 0,
			 sfxId_59: 0,
			 sfxId_60: 0,
			 sfxId_61: 0,
			 sfxId_62: 0,
			 sfxId_63: 0,
			 sfxId_64: 0,
			 sfxId_65: 0,
			 sfxId_66: 0,
			 sfxId_67: 0,
			 sfxId_68: 0,
			 sfxId_69: 0,
			 sfxId_70: 0,
			 sfxId_71: 0,
			 sfxId_72: 0,
			 sfxId_73: 0,
			 sfxId_74: 0,
			 sfxId_75: 0,
			 sfxId_76: 0,
			 sfxId_77: 0,
			 sfxId_78: 0,
			 sfxId_79: 0,
			 sfxId_80: 0,
			 sfxId_81: 0,
			 sfxId_82: 0,
			 sfxId_83: 0,
			 sfxId_84: 0,
			 sfxId_85: 0,
			 sfxId_86: 0,
			 sfxId_87: 0,
			 sfxId_88: 0,
			 sfxId_89: 0,
			 sfxId_90: 0,
			 sfxId_91: 0,
			 sfxId_92: 0,
			 sfxId_93: 0,
			 sfxId_94: 0,
			 sfxId_95: 0,
			 sfxId_96: 0,
			 sfxId_97: 0,
			 sfxId_98: 0,
			 sfxId_99: 0,
			 sfxId_100: 0,
			 sfxId_101: 0,
			 sfxId_102: 0,
			 sfxId_103: 0,
			 sfxId_104: 0,
			 sfxId_105: 0,
			 sfxId_106: 0,
			 sfxId_107: 0,
			 sfxId_108: 0,
			 sfxId_109: 0,
			 sfxId_110: 0,
			 sfxId_111: 0,
			 sfxId_112: 0,
			 sfxId_113: 0,
			 sfxId_114: 0,
			 sfxId_115: 0,
			 sfxId_116: 0,
			 sfxId_117: 0,
			 sfxId_118: 0,
			 sfxId_119: 0,
			 sfxId_120: 0,
			 sfxId_121: 0,
			 sfxId_122: 0,
			 sfxId_123: 0,
			 sfxId_124: 0,
			 sfxId_125: 0,
			 sfxId_126: 0,
			 sfxId_127: 0,
			 sfxId_128: 0,
			 sfxId_129: 0,
			 sfxId_130: 0,
			 sfxId_131: 0,
			 sfxId_132: 0,
			 sfxId_133: 0,
			 sfxId_134: 0,
			 sfxId_135: 0,
			 sfxId_136: 0,
			 sfxId_137: 0,
			 sfxId_138: 0,
			 sfxId_139: 0,
			 sfxId_140: 0,
			 sfxId_141: 0,
			 sfxId_142: 0,
			 sfxId_143: 0,
			 sfxId_144: 0,
			 sfxId_145: 0,
			 sfxId_146: 0,
			 sfxId_147: 0,
			 sfxId_148: 0,
			 sfxId_149: 0,
			 sfxId_150: 0,
			 sfxId_151: 0,
			 sfxId_152: 0,
			 sfxId_153: 0,
			 sfxId_154: 0,
			 sfxId_155: 0,
			 sfxId_156: 0,
			 sfxId_157: 0,
			 sfxId_158: 0,
			 sfxId_159: 0,
			 sfxId_160: 0,
			 sfxId_161: 0,
			 sfxId_162: 0,
			 sfxId_163: 0,
			 sfxId_164: 0,
			 sfxId_165: 0,
			 sfxId_166: 0,
			 sfxId_167: 0,
			 sfxId_168: 0,
			 sfxId_169: 0,
			 sfxId_170: 0,
			 sfxId_171: 0,
			 sfxId_172: 0,
			 sfxId_173: 0,
			 sfxId_174: 0,
			 sfxId_175: 0,
			 sfxId_176: 0,
			 sfxId_177: 0,
			 sfxId_178: 0,
			 sfxId_179: 0,
			 sfxId_180: 0,
			 sfxId_181: 0,
			 sfxId_182: 0,
			 sfxId_183: 0,
			 sfxId_184: 0,
			 sfxId_185: 0,
			 sfxId_186: 0,
			 sfxId_187: 0,
			 sfxId_188: 0,
			 sfxId_189: 0,
			 sfxId_190: 0,
			 sfxId_191: 0,
			 sfxId_192: 0,
			 sfxId_193: 0,
			 sfxId_194: 0,
			 sfxId_195: 0,
			 sfxId_196: 0,
			 sfxId_197: 0,
			 sfxId_198: 0,
			 sfxId_199: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GAME_AREA_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub bonusSoul_single: i32,
	pub bonusSoul_multi: i32,
	pub humanityPointCountFlagIdTop: i32,
	pub humanityDropPoint1: i16,
	pub humanityDropPoint2: i16,
	pub humanityDropPoint3: i16,
	pub humanityDropPoint4: i16,
	pub humanityDropPoint5: i16,
	pub humanityDropPoint6: i16,
	pub humanityDropPoint7: i16,
	pub humanityDropPoint8: i16,
	pub humanityDropPoint9: i16,
	pub humanityDropPoint10: i16,
	pub soloBreakInPoint_Min: i32,
	pub soloBreakInPoint_Max: i32,
	pub defeatBossFlagId_forSignAimList: i32,
	pub displayAimFlagId: i32,
	pub foundBossFlagId: i32,
	pub foundBossTextId: i32,
	pub notFindBossTextId: i32,
	pub bossChallengeFlagId: i32,
	pub defeatBossFlagId: i32,
	pub bossPosX: f32,
	pub bossPosY: f32,
	pub bossPosZ: f32,
	pub bossMapAreaNo: u8,
	pub bossMapBlockNo: u8,
	pub bossMapMapNo: u8,
	pub reserve: [u8;9],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GAME_AREA_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for GAME_AREA_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 bonusSoul_single: 0,
			 bonusSoul_multi: 0,
			 humanityPointCountFlagIdTop: 0,
			 humanityDropPoint1: 0,
			 humanityDropPoint2: 0,
			 humanityDropPoint3: 0,
			 humanityDropPoint4: 0,
			 humanityDropPoint5: 0,
			 humanityDropPoint6: 0,
			 humanityDropPoint7: 0,
			 humanityDropPoint8: 0,
			 humanityDropPoint9: 0,
			 humanityDropPoint10: 0,
			 soloBreakInPoint_Min: 0,
			 soloBreakInPoint_Max: 10000,
			 defeatBossFlagId_forSignAimList: 0,
			 displayAimFlagId: 0,
			 foundBossFlagId: 0,
			 foundBossTextId: -1,
			 notFindBossTextId: -1,
			 bossChallengeFlagId: 0,
			 defeatBossFlagId: 0,
			 bossPosX: 0.,
			 bossPosY: 0.,
			 bossPosZ: 0.,
			 bossMapAreaNo: 0,
			 bossMapBlockNo: 0,
			 bossMapMapNo: 0,
			 reserve: [0;9],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GAME_INFO_PARAM {
	pub titleMsgId: i32,
	pub contentMsgId: i32,
	pub value: i32,
	pub sortId: i32,
	pub eventId: i32,
	pub Pad: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GAME_INFO_PARAM {
}
impl Default for GAME_INFO_PARAM {
	fn default() -> Self {
		Self {
			 titleMsgId: 0,
			 contentMsgId: 0,
			 value: 0,
			 sortId: 0,
			 eventId: 0,
			 Pad: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GAME_SYSTEM_COMMON_PARAM_ST {
	pub baseToughnessRecoverTime: f32,
	pub chrEventTrun_byLeft90: i32,
	pub chrEventTrun_byRight90: i32,
	pub chrEventTrun_byLeft180: i32,
	pub chrEventTrun_byRight180: i32,
	pub chrEventTrun_90TurnStartAngle: i16,
	pub chrEventTrun_180TurnStartAngle: i16,
	pub stealthAtkDamageRate: f32,
	pub flickDamageCutRateSuccessGurad: f32,
	pub npcTalkAnimBeginDiffAngle: f32,
	pub npcTalkAnimEndDiffAngle: f32,
	pub sleepCollectorItemActionButtonParamId: i32,
	pub allowUseBuddyItem_sfxInterval: f32,
	pub allowUseBuddyItem_sfxDmyPolyId: i32,
	pub allowUseBuddyItem_sfxDmyPolyId_horse: i32,
	pub allowUseBuddyItem_sfxId: i32,
	pub onBuddySummon_inActivateRange_sfxInterval: f32,
	pub onBuddySummon_inActivateRange_sfxDmyPolyId: i32,
	pub onBuddySummon_inActivateRange_sfxDmyPolyId_horse: i32,
	pub onBuddySummon_inActivateRange_sfxId: i32,
	pub onBuddySummon_inActivateRange_spEffectId_pc: i32,
	pub onBuddySummon_inWarnRange_spEffectId_pc: i32,
	pub onBuddySummon_atBuddyUnsummon_spEffectId_pc: i32,
	pub onBuddySummon_inWarnRange_spEffectId_buddy: i32,
	pub morningIngameHour: u8,
	pub morningIngameMinute: u8,
	pub morningIngameSecond: u8,
	pub noonIngameHour: u8,
	pub noonIngameMinute: u8,
	pub noonIngameSecond: u8,
	pub nightIngameHour: u8,
	pub nightIngameMinute: u8,
	pub nightIngameSecond: u8,
	pub aiSightRateStart_Morning_Hour: u8,
	pub aiSightRateStart_Morning_Minute: u8,
	pub aiSightRateStart_Noon_Hour: u8,
	pub aiSightRateStart_Noon_Minute: u8,
	pub aiSightRateStart_Evening_Hour: u8,
	pub aiSightRateStart_Evening_Minute: u8,
	pub aiSightRateStart_Night_Hour: u8,
	pub aiSightRateStart_Night_Minute: u8,
	pub aiSightRateStart_Midnight_Hour: u8,
	pub aiSightRateStart_Midnight_Minute: u8,
	pub saLargeDamageHitSfx_Threshold: u8,
	pub saLargeDamageHitSfx_SfxId: i32,
	pub signCreatableDistFromSafePos: f32,
	pub guestResummonDist: f32,
	pub guestLeavingMessageDistMax: f32,
	pub guestLeavingMessageDistMin: f32,
	pub guestLeaveSessionDist: f32,
	pub retryPointAreaRadius: f32,
	pub sleepCollectorSpEffectId: i32,
	pub recoverBelowMaxHpCompletionNoticeSpEffectId: i32,
	pub estusFlaskRecovery_AbsorptionProductionSfxId_byHp: i32,
	pub estusFlaskRecovery_AbsorptionProductionSfxId_byMp: i32,
	pub respawnSpecialEffectActiveCheckerSpEffectId: i32,
	pub onBuddySummon_inActivateRange_spEffectId_buddy: i32,
	pub estusFlaskRecovery_AddEstusTime: f32,
	pub defeatMultiModeEnemyOfSoulCorrectRate_byHost: f32,
	pub defeatMultiModeEnemyOfSoulCorrectRate_byTeamGhost: f32,
	pub defeatMultiModeBossOfSoulCorrectRate_byHost: f32,
	pub defeatMultiModeBossOfSoulCorrectRate_byTeamGhost: f32,
	pub enemyHpGaugeScreenOffset_byUp: i16,
	pub playRegionCollectDist: i16,
	pub enemyDetectionSpEffect_ShootBulletDummypolyId: i16,
	pub bigRuneGreaterDemonBreakInGoodsNum: i16,
	pub bigRuneGreaterDemonBreakInGoodsId: i32,
	pub rideJumpRegionDefaultSfxId: i32,
	pub saAttackRate_forVsRideAtk: f32,
	pub enemySpEffectIdAfterSleepCollectorItemLot: i32,
	pub afterEndingMapUid: i32,
	pub afterEndingReturnPointEntityId: i32,
	pub enemyDetectionSpEffect_BulletId_byCoopRing_RedHunter: i32,
	pub enemyDetectionSpEffect_BulletId_byInvadeOrb_None: i32,
	pub tutorialFlagOnAccessDistView: i32,
	pub tutorialFlagOnAccessRetryPoint: i32,
	pub tutorialFlagOnGetGroupReward: i32,
	pub tutorialFlagOnEnterRideJumpRegion: i32,
	pub tutorialCheckRideJumpRegionExpandRange: f32,
	pub retryPointActivatedPcAnimId: i32,
	pub retryPointActivatedDialogDelayTime: f32,
	pub retryPointActivatedDialogTextId: i32,
	pub signPuddleOpenPcAnimId: i32,
	pub signPuddleOpenDialogDelayTime: f32,
	pub activityOfDeadSpEffect_BulletId: i32,
	pub activityOfDeadSpEffect_ShootBulletDummypolyId: i32,
	pub activityOfDeadSpEffect_DeadFadeOutTime: f32,
	pub ignorNetStateSyncTime_ForThrow: f32,
	pub netPenaltyPointLanDisconnect: i16,
	pub netPenaltyPointProfileSignout: i16,
	pub netPenaltyPointReboot: i16,
	pub netPnaltyPointSuspend: i16,
	pub netPenaltyForgiveItemLimitTime: f32,
	pub netPenaltyPointThreshold: i16,
	pub uncontrolledMoveThresholdTime: i16,
	pub enemyDetectionSpEffect_BulletId_byNpcEnemy: i32,
	pub activityOfDeadTargetSearchSpEffect_OnHitSpEffect: i32,
	pub activityOfDeadTargetSearchSpEffect_MaxLength: f32,
	pub sightRangeLowerPromiseRate: f32,
	pub saLargeDamageHitSfx_MinDamage: i16,
	pub saLargeDamageHitSfx_ForceDamage: i16,
	pub soloBreakInMaxPoint: i32,
	pub npcTalkTimeOutThreshold: f32,
	pub sendPlayLogIntervalTime: f32,
	pub item370_MaxSfxNum: u8,
	pub chrActivateDist_forLeavePC: u8,
	pub summonDataCoopMatchingLevelUpperAbs: i16,
	pub summonDataCoopMatchingLevelUpperRel: i16,
	pub summonDataCoopMatchingWepLevelMul: i16,
	pub pickUpBerserkerSignSpEffectBulletId: i32,
	pub succeedBerserkerSelfKillingEffectId: i32,
	pub machingLevelWhiteSignUpperRel: u8,
	pub machingLevelWhiteSignUpperAbs: u8,
	pub machingLevelRedSignUpperRel: u8,
	pub machingLevelRedSignUpperAbs: u8,
	pub machingWeaponLevelUpperWhiteSign_0: u8,
	pub machingWeaponLevelUpperWhiteSign_1: u8,
	pub machingWeaponLevelUpperWhiteSign_2: u8,
	pub machingWeaponLevelUpperWhiteSign_3: u8,
	pub machingWeaponLevelUpperWhiteSign_4: u8,
	pub machingWeaponLevelUpperWhiteSign_5: u8,
	pub machingWeaponLevelUpperWhiteSign_6: u8,
	pub machingWeaponLevelUpperWhiteSign_7: u8,
	pub machingWeaponLevelUpperWhiteSign_8: u8,
	pub machingWeaponLevelUpperWhiteSign_9: u8,
	pub machingWeaponLevelUpperWhiteSign_10: u8,
	pub machingWeaponLevelUpperRedSign_0: u8,
	pub machingWeaponLevelUpperRedSign_1: u8,
	pub machingWeaponLevelUpperRedSign_2: u8,
	pub machingWeaponLevelUpperRedSign_3: u8,
	pub machingWeaponLevelUpperRedSign_4: u8,
	pub machingWeaponLevelUpperRedSign_5: u8,
	pub machingWeaponLevelUpperRedSign_6: u8,
	pub machingWeaponLevelUpperRedSign_7: u8,
	pub machingWeaponLevelUpperRedSign_8: u8,
	pub machingWeaponLevelUpperRedSign_9: u8,
	pub machingWeaponLevelUpperRedSign_10: u8,
	pub autoInvadePoint_generateDist: u8,
	pub autoInvadePoint_cancelDist: u8,
	pub sendGlobalEventLogIntervalTime: f32,
	pub addSoloBreakInPoint_White: i16,
	pub addSoloBreakInPoint_Black: i16,
	pub addSoloBreakInPoint_ForceJoin: i16,
	pub addSoloBreakInPoint_VisitorGuardian: i16,
	pub addSoloBreakInPoint_VisitorRedHunter: i16,
	pub invincibleTimer_forNetPC_initSync: u8,
	pub invincibleTimer_forNetPC: u8,
	pub redHunter_HostBossAreaGetSoulRate: f32,
	pub ghostFootprintDecalParamId: i32,
	pub leaveAroundHostWarningTime: f32,
	pub hostModeCostItemId: i32,
	pub aIJump_DecelerateParam: f32,
	pub buddyDisappearDelaySec: f32,
	pub aIJump_AnimYMoveCorrectRate_onJumpOff: f32,
	pub stealthSystemSightRate_NotInStealthRigid_NotSightHide_StealthMode: f32,
	pub stealthSystemSightRate_NotInStealthRigid_SightHide_NotStealthMode: f32,
	pub stealthSystemSightRate_NotInStealthRigid_SightHide_StealthMode: f32,
	pub stealthSystemSightRate_InStealthRigid_NotSightHide_NotStealthMode: f32,
	pub stealthSystemSightRate_InStealthRigid_NotSightHide_StealthMode: f32,
	pub stealthSystemSightRate_InStealthRigid_SightHide_NotStealthMode: f32,
	pub stealthSystemSightRate_InStealthRigid_SightHide_StealthMode: f32,
	pub msbEventGeomTreasureInfo_actionButtonParamId_corpse: i32,
	pub msbEventGeomTreasureInfo_itemGetAnimId_corpse: i32,
	pub msbEventGeomTreasureInfo_actionButtonParamId_box: i32,
	pub msbEventGeomTreasureInfo_itemGetAnimId_box: i32,
	pub msbEventGeomTreasureInfo_actionButtonParamId_shine: i32,
	pub msbEventGeomTreasureInfo_itemGetAnimId_shine: i32,
	pub signPuddleAssetId: i32,
	pub signPuddleAppearDmypolyId0: i32,
	pub signPuddleAppearDmypolyId1: i32,
	pub signPuddleAppearDmypolyId2: i32,
	pub signPuddleAppearDmypolyId3: i32,
	pub fallDamageRate_forRidePC: f32,
	pub fallDamageRate_forRideNPC: f32,
	pub OldMonkOfYellow_CreateSignSpEffectId: i32,
	pub StragglerActivateDist: f32,
	pub SpEffectId_EnableUseItem_StragglerActivate: i32,
	pub SpEffectId_StragglerWakeUp: i32,
	pub SpEffectId_StragglerTarget: i32,
	pub SpEffectId_StragglerOppose: i32,
	pub buddyWarp_TriggerTimeRayBlocked: f32,
	pub buddyWarp_TriggerDistToPlayer: f32,
	pub buddyWarp_ThresholdTimePathStacked: f32,
	pub buddyWarp_ThresholdRangePathStacked: f32,
	pub aiSightRate_morning: f32,
	pub aiSightRate_noonA: f32,
	pub buddyPassThroughTriggerTime: f32,
	pub aiSightRate_evening: f32,
	pub aiSightRate_night: f32,
	pub aiSightRate_midnightA: f32,
	pub reserve4_2: [u8;4],
	pub aiSightRate_sunloss_light: f32,
	pub aiSightRate_sunloss_dark: f32,
	pub aiSightRate_sunloss_veryDark: f32,
	pub stealthSystemSightAngleReduceRate_NotInStealthRigid_NotSightHide_StealthMode: f32,
	pub stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_NotStealthMode: f32,
	pub stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_StealthMode: f32,
	pub stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_NotStealthMode: f32,
	pub stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_StealthMode: f32,
	pub stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_NotStealthMode: f32,
	pub stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_StealthMode: f32,
	pub weatherLotConditionStart_Morning_Hour: u8,
	pub weatherLotConditionStart_Morning_Minute: u8,
	pub weatherLotConditionStart_Day_Hour: u8,
	pub weatherLotConditionStart_Day_Minute: u8,
	pub weatherLotConditionStart_Evening_Hour: u8,
	pub weatherLotConditionStart_Evening_Minute: u8,
	pub weatherLotConditionStart_Night_Hour: u8,
	pub weatherLotConditionStart_Night_Minute: u8,
	pub weatherLotConditionStart_DayBreak_Hour: u8,
	pub weatherLotConditionStart_DayBreak_Minute: u8,
	pub weatherLotCondition_reserved: [u8;2],
	pub pclightScaleChangeStart_Hour: u8,
	pub pclightScaleChangeStart_Minute: u8,
	pub pclightScaleChangeEnd_Hour: u8,
	pub pclightScaleChangeEnd_Minute: u8,
	pub pclightScaleByTimezone: f32,
	pub bigRuneGreaterDemon_SummonBuddySpecialEffectId_Buddy: i32,
	pub bigRuneGreaterDemon_SummonBuddySpecialEffectId_Pc: i32,
	pub homeBonfireParamId: i32,
	pub machingWeaponLevelUpperWhiteSign_11: u8,
	pub machingWeaponLevelUpperWhiteSign_12: u8,
	pub machingWeaponLevelUpperWhiteSign_13: u8,
	pub machingWeaponLevelUpperWhiteSign_14: u8,
	pub machingWeaponLevelUpperWhiteSign_15: u8,
	pub machingWeaponLevelUpperWhiteSign_16: u8,
	pub machingWeaponLevelUpperWhiteSign_17: u8,
	pub machingWeaponLevelUpperWhiteSign_18: u8,
	pub machingWeaponLevelUpperWhiteSign_19: u8,
	pub machingWeaponLevelUpperWhiteSign_20: u8,
	pub machingWeaponLevelUpperWhiteSign_21: u8,
	pub machingWeaponLevelUpperWhiteSign_22: u8,
	pub machingWeaponLevelUpperWhiteSign_23: u8,
	pub machingWeaponLevelUpperWhiteSign_24: u8,
	pub machingWeaponLevelUpperWhiteSign_25: u8,
	pub machingWeaponLevelUpperRedSign_11: u8,
	pub machingWeaponLevelUpperRedSign_12: u8,
	pub machingWeaponLevelUpperRedSign_13: u8,
	pub machingWeaponLevelUpperRedSign_14: u8,
	pub machingWeaponLevelUpperRedSign_15: u8,
	pub machingWeaponLevelUpperRedSign_16: u8,
	pub machingWeaponLevelUpperRedSign_17: u8,
	pub machingWeaponLevelUpperRedSign_18: u8,
	pub machingWeaponLevelUpperRedSign_19: u8,
	pub machingWeaponLevelUpperRedSign_20: u8,
	pub machingWeaponLevelUpperRedSign_21: u8,
	pub machingWeaponLevelUpperRedSign_22: u8,
	pub machingWeaponLevelUpperRedSign_23: u8,
	pub machingWeaponLevelUpperRedSign_24: u8,
	pub machingWeaponLevelUpperRedSign_25: u8,
	pub menuTimezoneStart_Morning_Hour: u8,
	pub menuTimezoneStart_Morning_Minute: u8,
	pub menuTimezoneStart_Day1_Hour: u8,
	pub menuTimezoneStart_Day1_Minute: u8,
	pub menuTimezoneStart_Day2_Hour: u8,
	pub menuTimezoneStart_Day2_Minute: u8,
	pub menuTimezoneStart_Evening_Hour: u8,
	pub menuTimezoneStart_Evening_Minute: u8,
	pub menuTimezoneStart_Night_Hour: u8,
	pub menuTimezoneStart_Night_Minute: u8,
	pub menuTimezoneStart_Midnight_Hour: u8,
	pub menuTimezoneStart_Midnight_Minute: u8,
	pub remotePlayerThreatLvNotify_ThreatLv: i16,
	pub remotePlayerThreatLvNotify_NotifyDist: f32,
	pub remotePlayerThreatLvNotify_EndNotifyDist: f32,
	pub worldMapPointDiscoveryExpandRange: f32,
	pub worldMapPointReentryExpandRange: f32,
	pub remotePlayerThreatLvNotify_NotifyTime: i16,
	pub breakIn_A_rebreakInGoodsNum: i16,
	pub breakIn_A_rebreakInGoodsId: i32,
	pub rideJumpoff_SfxId: i32,
	pub rideJumpoff_SfxHeightOffset: f32,
	pub rideJumpoff_SpEffectId: i32,
	pub rideJumpoff_SpEffectIdPc: i32,
	pub unlockExchangeMenuEventFlagId: i32,
	pub unlockMessageMenuEventFlagId: i32,
	pub breakInOnce_A_rebreakInGoodsNum: i16,
	pub breakIn_B_rebreakInGoodsNum: i16,
	pub breakInOnce_A_rebreakInGoodsId: i32,
	pub breakIn_B_rebreakInGoodsId: i32,
	pub actionButtonInputCancelTime: f32,
	pub blockClearBonusDelayTime: f32,
	pub bonfireCheckEnemyRange: f32,
	pub reserved_124: [u8;48],
	pub reserved_124_1: [u8;32],
	pub unkR00: f32,
	pub unkR04: f32,
	pub unkR08: f32,
	pub unkR12: f32,
	pub unkR16: f32,
	pub unkR20: f32,
	pub unkR24: f32,
	pub unkR28: f32,
	pub unkR32: f32,
	pub unkR36: f32,
	pub unkR40: f32,
	pub unkR44: f32,
	pub unkR48: f32,
	pub unkR52: f32,
	pub reserved_124_2: [u8;40],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GAME_SYSTEM_COMMON_PARAM_ST {
}
impl Default for GAME_SYSTEM_COMMON_PARAM_ST {
	fn default() -> Self {
		Self {
			 baseToughnessRecoverTime: 0.,
			 chrEventTrun_byLeft90: 0,
			 chrEventTrun_byRight90: 0,
			 chrEventTrun_byLeft180: 0,
			 chrEventTrun_byRight180: 0,
			 chrEventTrun_90TurnStartAngle: 0,
			 chrEventTrun_180TurnStartAngle: 0,
			 stealthAtkDamageRate: 0.,
			 flickDamageCutRateSuccessGurad: 0.,
			 npcTalkAnimBeginDiffAngle: 0.,
			 npcTalkAnimEndDiffAngle: 0.,
			 sleepCollectorItemActionButtonParamId: -1,
			 allowUseBuddyItem_sfxInterval: 0.,
			 allowUseBuddyItem_sfxDmyPolyId: -1,
			 allowUseBuddyItem_sfxDmyPolyId_horse: -1,
			 allowUseBuddyItem_sfxId: -1,
			 onBuddySummon_inActivateRange_sfxInterval: 0.,
			 onBuddySummon_inActivateRange_sfxDmyPolyId: -1,
			 onBuddySummon_inActivateRange_sfxDmyPolyId_horse: -1,
			 onBuddySummon_inActivateRange_sfxId: -1,
			 onBuddySummon_inActivateRange_spEffectId_pc: -1,
			 onBuddySummon_inWarnRange_spEffectId_pc: -1,
			 onBuddySummon_atBuddyUnsummon_spEffectId_pc: -1,
			 onBuddySummon_inWarnRange_spEffectId_buddy: -1,
			 morningIngameHour: 0,
			 morningIngameMinute: 0,
			 morningIngameSecond: 0,
			 noonIngameHour: 0,
			 noonIngameMinute: 0,
			 noonIngameSecond: 0,
			 nightIngameHour: 0,
			 nightIngameMinute: 0,
			 nightIngameSecond: 0,
			 aiSightRateStart_Morning_Hour: 0,
			 aiSightRateStart_Morning_Minute: 0,
			 aiSightRateStart_Noon_Hour: 0,
			 aiSightRateStart_Noon_Minute: 0,
			 aiSightRateStart_Evening_Hour: 0,
			 aiSightRateStart_Evening_Minute: 0,
			 aiSightRateStart_Night_Hour: 0,
			 aiSightRateStart_Night_Minute: 0,
			 aiSightRateStart_Midnight_Hour: 0,
			 aiSightRateStart_Midnight_Minute: 0,
			 saLargeDamageHitSfx_Threshold: 0,
			 saLargeDamageHitSfx_SfxId: 0,
			 signCreatableDistFromSafePos: 0.,
			 guestResummonDist: 0.,
			 guestLeavingMessageDistMax: 0.,
			 guestLeavingMessageDistMin: 0.,
			 guestLeaveSessionDist: 0.,
			 retryPointAreaRadius: -1.,
			 sleepCollectorSpEffectId: -1,
			 recoverBelowMaxHpCompletionNoticeSpEffectId: 0,
			 estusFlaskRecovery_AbsorptionProductionSfxId_byHp: 0,
			 estusFlaskRecovery_AbsorptionProductionSfxId_byMp: 0,
			 respawnSpecialEffectActiveCheckerSpEffectId: 0,
			 onBuddySummon_inActivateRange_spEffectId_buddy: -1,
			 estusFlaskRecovery_AddEstusTime: 0.,
			 defeatMultiModeEnemyOfSoulCorrectRate_byHost: 0.,
			 defeatMultiModeEnemyOfSoulCorrectRate_byTeamGhost: 0.,
			 defeatMultiModeBossOfSoulCorrectRate_byHost: 0.,
			 defeatMultiModeBossOfSoulCorrectRate_byTeamGhost: 0.,
			 enemyHpGaugeScreenOffset_byUp: 0,
			 playRegionCollectDist: 0,
			 enemyDetectionSpEffect_ShootBulletDummypolyId: 0,
			 bigRuneGreaterDemonBreakInGoodsNum: 0,
			 bigRuneGreaterDemonBreakInGoodsId: -1,
			 rideJumpRegionDefaultSfxId: 0,
			 saAttackRate_forVsRideAtk: 1.,
			 enemySpEffectIdAfterSleepCollectorItemLot: -1,
			 afterEndingMapUid: 0,
			 afterEndingReturnPointEntityId: 0,
			 enemyDetectionSpEffect_BulletId_byCoopRing_RedHunter: 0,
			 enemyDetectionSpEffect_BulletId_byInvadeOrb_None: 0,
			 tutorialFlagOnAccessDistView: 0,
			 tutorialFlagOnAccessRetryPoint: 0,
			 tutorialFlagOnGetGroupReward: 0,
			 tutorialFlagOnEnterRideJumpRegion: 0,
			 tutorialCheckRideJumpRegionExpandRange: 0.,
			 retryPointActivatedPcAnimId: -1,
			 retryPointActivatedDialogDelayTime: 0.,
			 retryPointActivatedDialogTextId: -1,
			 signPuddleOpenPcAnimId: -1,
			 signPuddleOpenDialogDelayTime: 0.,
			 activityOfDeadSpEffect_BulletId: 0,
			 activityOfDeadSpEffect_ShootBulletDummypolyId: 0,
			 activityOfDeadSpEffect_DeadFadeOutTime: 0.,
			 ignorNetStateSyncTime_ForThrow: 0.,
			 netPenaltyPointLanDisconnect: 0,
			 netPenaltyPointProfileSignout: 0,
			 netPenaltyPointReboot: 0,
			 netPnaltyPointSuspend: 0,
			 netPenaltyForgiveItemLimitTime: 0.,
			 netPenaltyPointThreshold: 0,
			 uncontrolledMoveThresholdTime: 0,
			 enemyDetectionSpEffect_BulletId_byNpcEnemy: 0,
			 activityOfDeadTargetSearchSpEffect_OnHitSpEffect: 0,
			 activityOfDeadTargetSearchSpEffect_MaxLength: 0.,
			 sightRangeLowerPromiseRate: 0.,
			 saLargeDamageHitSfx_MinDamage: -1,
			 saLargeDamageHitSfx_ForceDamage: -1,
			 soloBreakInMaxPoint: 0,
			 npcTalkTimeOutThreshold: 0.,
			 sendPlayLogIntervalTime: 0.,
			 item370_MaxSfxNum: 0,
			 chrActivateDist_forLeavePC: 0,
			 summonDataCoopMatchingLevelUpperAbs: 0,
			 summonDataCoopMatchingLevelUpperRel: 0,
			 summonDataCoopMatchingWepLevelMul: 0,
			 pickUpBerserkerSignSpEffectBulletId: 0,
			 succeedBerserkerSelfKillingEffectId: 0,
			 machingLevelWhiteSignUpperRel: 0,
			 machingLevelWhiteSignUpperAbs: 0,
			 machingLevelRedSignUpperRel: 0,
			 machingLevelRedSignUpperAbs: 0,
			 machingWeaponLevelUpperWhiteSign_0: 0,
			 machingWeaponLevelUpperWhiteSign_1: 0,
			 machingWeaponLevelUpperWhiteSign_2: 0,
			 machingWeaponLevelUpperWhiteSign_3: 0,
			 machingWeaponLevelUpperWhiteSign_4: 0,
			 machingWeaponLevelUpperWhiteSign_5: 0,
			 machingWeaponLevelUpperWhiteSign_6: 0,
			 machingWeaponLevelUpperWhiteSign_7: 0,
			 machingWeaponLevelUpperWhiteSign_8: 0,
			 machingWeaponLevelUpperWhiteSign_9: 0,
			 machingWeaponLevelUpperWhiteSign_10: 0,
			 machingWeaponLevelUpperRedSign_0: 0,
			 machingWeaponLevelUpperRedSign_1: 0,
			 machingWeaponLevelUpperRedSign_2: 0,
			 machingWeaponLevelUpperRedSign_3: 0,
			 machingWeaponLevelUpperRedSign_4: 0,
			 machingWeaponLevelUpperRedSign_5: 0,
			 machingWeaponLevelUpperRedSign_6: 0,
			 machingWeaponLevelUpperRedSign_7: 0,
			 machingWeaponLevelUpperRedSign_8: 0,
			 machingWeaponLevelUpperRedSign_9: 0,
			 machingWeaponLevelUpperRedSign_10: 0,
			 autoInvadePoint_generateDist: 40,
			 autoInvadePoint_cancelDist: 20,
			 sendGlobalEventLogIntervalTime: 0.,
			 addSoloBreakInPoint_White: 0,
			 addSoloBreakInPoint_Black: 0,
			 addSoloBreakInPoint_ForceJoin: 0,
			 addSoloBreakInPoint_VisitorGuardian: 0,
			 addSoloBreakInPoint_VisitorRedHunter: 0,
			 invincibleTimer_forNetPC_initSync: 0,
			 invincibleTimer_forNetPC: 10,
			 redHunter_HostBossAreaGetSoulRate: 0.,
			 ghostFootprintDecalParamId: 0,
			 leaveAroundHostWarningTime: 0.,
			 hostModeCostItemId: 0,
			 aIJump_DecelerateParam: 0.,
			 buddyDisappearDelaySec: 0.,
			 aIJump_AnimYMoveCorrectRate_onJumpOff: 0.,
			 stealthSystemSightRate_NotInStealthRigid_NotSightHide_StealthMode: 1.,
			 stealthSystemSightRate_NotInStealthRigid_SightHide_NotStealthMode: 1.,
			 stealthSystemSightRate_NotInStealthRigid_SightHide_StealthMode: 1.,
			 stealthSystemSightRate_InStealthRigid_NotSightHide_NotStealthMode: 1.,
			 stealthSystemSightRate_InStealthRigid_NotSightHide_StealthMode: 1.,
			 stealthSystemSightRate_InStealthRigid_SightHide_NotStealthMode: 1.,
			 stealthSystemSightRate_InStealthRigid_SightHide_StealthMode: 1.,
			 msbEventGeomTreasureInfo_actionButtonParamId_corpse: 0,
			 msbEventGeomTreasureInfo_itemGetAnimId_corpse: 0,
			 msbEventGeomTreasureInfo_actionButtonParamId_box: 0,
			 msbEventGeomTreasureInfo_itemGetAnimId_box: 0,
			 msbEventGeomTreasureInfo_actionButtonParamId_shine: 0,
			 msbEventGeomTreasureInfo_itemGetAnimId_shine: 0,
			 signPuddleAssetId: 0,
			 signPuddleAppearDmypolyId0: 0,
			 signPuddleAppearDmypolyId1: 0,
			 signPuddleAppearDmypolyId2: 0,
			 signPuddleAppearDmypolyId3: 0,
			 fallDamageRate_forRidePC: 1.,
			 fallDamageRate_forRideNPC: 1.,
			 OldMonkOfYellow_CreateSignSpEffectId: 0,
			 StragglerActivateDist: 0.,
			 SpEffectId_EnableUseItem_StragglerActivate: -1,
			 SpEffectId_StragglerWakeUp: -1,
			 SpEffectId_StragglerTarget: -1,
			 SpEffectId_StragglerOppose: -1,
			 buddyWarp_TriggerTimeRayBlocked: 10.,
			 buddyWarp_TriggerDistToPlayer: 25.,
			 buddyWarp_ThresholdTimePathStacked: 5.,
			 buddyWarp_ThresholdRangePathStacked: 1.,
			 aiSightRate_morning: 1.,
			 aiSightRate_noonA: 1.,
			 buddyPassThroughTriggerTime: 0.5,
			 aiSightRate_evening: 1.,
			 aiSightRate_night: 1.,
			 aiSightRate_midnightA: 1.,
			 reserve4_2: [0;4],
			 aiSightRate_sunloss_light: 1.,
			 aiSightRate_sunloss_dark: 1.,
			 aiSightRate_sunloss_veryDark: 1.,
			 stealthSystemSightAngleReduceRate_NotInStealthRigid_NotSightHide_StealthMode: 0.,
			 stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_NotStealthMode: 0.,
			 stealthSystemSightAngleReduceRate_NotInStealthRigid_SightHide_StealthMode: 0.,
			 stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_NotStealthMode: 0.,
			 stealthSystemSightAngleReduceRate_InStealthRigid_NotSightHide_StealthMode: 0.,
			 stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_NotStealthMode: 0.,
			 stealthSystemSightAngleReduceRate_InStealthRigid_SightHide_StealthMode: 0.,
			 weatherLotConditionStart_Morning_Hour: 7,
			 weatherLotConditionStart_Morning_Minute: 0,
			 weatherLotConditionStart_Day_Hour: 12,
			 weatherLotConditionStart_Day_Minute: 0,
			 weatherLotConditionStart_Evening_Hour: 17,
			 weatherLotConditionStart_Evening_Minute: 0,
			 weatherLotConditionStart_Night_Hour: 19,
			 weatherLotConditionStart_Night_Minute: 0,
			 weatherLotConditionStart_DayBreak_Hour: 5,
			 weatherLotConditionStart_DayBreak_Minute: 0,
			 weatherLotCondition_reserved: [0;2],
			 pclightScaleChangeStart_Hour: 18,
			 pclightScaleChangeStart_Minute: 0,
			 pclightScaleChangeEnd_Hour: 5,
			 pclightScaleChangeEnd_Minute: 0,
			 pclightScaleByTimezone: 1.,
			 bigRuneGreaterDemon_SummonBuddySpecialEffectId_Buddy: -1,
			 bigRuneGreaterDemon_SummonBuddySpecialEffectId_Pc: -1,
			 homeBonfireParamId: 0,
			 machingWeaponLevelUpperWhiteSign_11: 0,
			 machingWeaponLevelUpperWhiteSign_12: 0,
			 machingWeaponLevelUpperWhiteSign_13: 0,
			 machingWeaponLevelUpperWhiteSign_14: 0,
			 machingWeaponLevelUpperWhiteSign_15: 0,
			 machingWeaponLevelUpperWhiteSign_16: 0,
			 machingWeaponLevelUpperWhiteSign_17: 0,
			 machingWeaponLevelUpperWhiteSign_18: 0,
			 machingWeaponLevelUpperWhiteSign_19: 0,
			 machingWeaponLevelUpperWhiteSign_20: 0,
			 machingWeaponLevelUpperWhiteSign_21: 0,
			 machingWeaponLevelUpperWhiteSign_22: 0,
			 machingWeaponLevelUpperWhiteSign_23: 0,
			 machingWeaponLevelUpperWhiteSign_24: 0,
			 machingWeaponLevelUpperWhiteSign_25: 0,
			 machingWeaponLevelUpperRedSign_11: 0,
			 machingWeaponLevelUpperRedSign_12: 0,
			 machingWeaponLevelUpperRedSign_13: 0,
			 machingWeaponLevelUpperRedSign_14: 0,
			 machingWeaponLevelUpperRedSign_15: 0,
			 machingWeaponLevelUpperRedSign_16: 0,
			 machingWeaponLevelUpperRedSign_17: 0,
			 machingWeaponLevelUpperRedSign_18: 0,
			 machingWeaponLevelUpperRedSign_19: 0,
			 machingWeaponLevelUpperRedSign_20: 0,
			 machingWeaponLevelUpperRedSign_21: 0,
			 machingWeaponLevelUpperRedSign_22: 0,
			 machingWeaponLevelUpperRedSign_23: 0,
			 machingWeaponLevelUpperRedSign_24: 0,
			 machingWeaponLevelUpperRedSign_25: 0,
			 menuTimezoneStart_Morning_Hour: 7,
			 menuTimezoneStart_Morning_Minute: 0,
			 menuTimezoneStart_Day1_Hour: 12,
			 menuTimezoneStart_Day1_Minute: 0,
			 menuTimezoneStart_Day2_Hour: 12,
			 menuTimezoneStart_Day2_Minute: 0,
			 menuTimezoneStart_Evening_Hour: 17,
			 menuTimezoneStart_Evening_Minute: 0,
			 menuTimezoneStart_Night_Hour: 19,
			 menuTimezoneStart_Night_Minute: 0,
			 menuTimezoneStart_Midnight_Hour: 5,
			 menuTimezoneStart_Midnight_Minute: 0,
			 remotePlayerThreatLvNotify_ThreatLv: 0,
			 remotePlayerThreatLvNotify_NotifyDist: 0.,
			 remotePlayerThreatLvNotify_EndNotifyDist: 0.,
			 worldMapPointDiscoveryExpandRange: 0.,
			 worldMapPointReentryExpandRange: 0.,
			 remotePlayerThreatLvNotify_NotifyTime: 0,
			 breakIn_A_rebreakInGoodsNum: 0,
			 breakIn_A_rebreakInGoodsId: -1,
			 rideJumpoff_SfxId: -1,
			 rideJumpoff_SfxHeightOffset: 0.,
			 rideJumpoff_SpEffectId: -1,
			 rideJumpoff_SpEffectIdPc: -1,
			 unlockExchangeMenuEventFlagId: 0,
			 unlockMessageMenuEventFlagId: 0,
			 breakInOnce_A_rebreakInGoodsNum: 0,
			 breakIn_B_rebreakInGoodsNum: 0,
			 breakInOnce_A_rebreakInGoodsId: -1,
			 breakIn_B_rebreakInGoodsId: -1,
			 actionButtonInputCancelTime: -1.,
			 blockClearBonusDelayTime: 7.,
			 bonfireCheckEnemyRange: -1.,
			 reserved_124: [0;48],
			 reserved_124_1: [0;32],
			 unkR00: 0.,
			 unkR04: 0.,
			 unkR08: 0.,
			 unkR12: 0.,
			 unkR16: 0.,
			 unkR20: 0.,
			 unkR24: 0.,
			 unkR28: 0.,
			 unkR32: 0.,
			 unkR36: 0.,
			 unkR40: 0.,
			 unkR44: 0.,
			 unkR48: 0.,
			 unkR52: 0.,
			 reserved_124_2: [0;40],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_AA_QUALITY_DETAIL {
	pub enabled: u8,
	pub forceFXAA2: u8,
	pub dmy: [u8;2],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_AA_QUALITY_DETAIL {
}
impl Default for CS_AA_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 0,
			 forceFXAA2: 0,
			 dmy: [0;2],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_DECAL_QUALITY_DETAIL {
	pub enabled: u8,
	pub dmy: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_DECAL_QUALITY_DETAIL {
}
impl Default for CS_DECAL_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 1,
			 dmy: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_DOF_QUALITY_DETAIL {
	pub enabled: u8,
	pub dmy: [u8;3],
	pub forceHiResoBlur: i32,
	pub maxBlurLevel: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_DOF_QUALITY_DETAIL {
}
impl Default for CS_DOF_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 1,
			 dmy: [0;3],
			 forceHiResoBlur: -1,
			 maxBlurLevel: 1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_EFFECT_QUALITY_DETAIL {
	pub softParticleEnabled: u8,
	pub glowEnabled: u8,
	pub distortionEnable: u8,
	pub cs_upScaleEnabledType: u8,
	pub fNumOnceEmitsScale: f32,
	pub fEmitSpanScale: f32,
	pub fLodDistance1Scale: f32,
	pub fLodDistance2Scale: f32,
	pub fLodDistance3Scale: f32,
	pub fLodDistance4Scale: f32,
	pub fScaleRenderDistanceScale: f32,
	pub dmy: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_EFFECT_QUALITY_DETAIL {
}
impl Default for CS_EFFECT_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 softParticleEnabled: 1,
			 glowEnabled: 1,
			 distortionEnable: 1,
			 cs_upScaleEnabledType: 0,
			 fNumOnceEmitsScale: 0.9,
			 fEmitSpanScale: 1.1,
			 fLodDistance1Scale: 0.9,
			 fLodDistance2Scale: 0.9,
			 fLodDistance3Scale: 0.9,
			 fLodDistance4Scale: 0.9,
			 fScaleRenderDistanceScale: 1.2,
			 dmy: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_LIGHTING_QUALITY_DETAIL {
	pub localLightDistFactor: f32,
	pub localLightShadowEnabled: u8,
	pub forwardPassLightingEnabled: u8,
	pub localLightShadowSpecLevelMax: u8,
	pub dmy: [u8;1],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_LIGHTING_QUALITY_DETAIL {
}
impl Default for CS_LIGHTING_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 localLightDistFactor: 0.75,
			 localLightShadowEnabled: 1,
			 forwardPassLightingEnabled: 1,
			 localLightShadowSpecLevelMax: 1,
			 dmy: [0;1],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_MOTION_BLUR_QUALITY_DETAIL {
	pub enabled: u8,
	pub ombEnabled: u8,
	pub forceScaleVelocityBuffer: u8,
	pub cheapFilterMode: u8,
	pub sampleCountBias: i32,
	pub recurrenceCountBias: i32,
	pub blurMaxLengthScale: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_MOTION_BLUR_QUALITY_DETAIL {
}
impl Default for CS_MOTION_BLUR_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 1,
			 ombEnabled: 1,
			 forceScaleVelocityBuffer: 1,
			 cheapFilterMode: 0,
			 sampleCountBias: -2,
			 recurrenceCountBias: 0,
			 blurMaxLengthScale: 0.75,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_RAYTRACING_QUALITY_DETAIL {
	pub enableRaytraceAO: u8,
	pub enableRaytraceShadows: u8,
	pub Unk0x02: u8,
	pub Unk0x03: u8,
	pub UnkFloat0x04: f32,
	pub Unk0x08: i32,
	pub UnkFloat0x0C: f32,
	pub Unk0x10: i32,
	pub UnkFloat0x14: f32,
	pub UnkFloat0x18: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_RAYTRACING_QUALITY_DETAIL {
}
impl Default for CS_RAYTRACING_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enableRaytraceAO: 0,
			 enableRaytraceShadows: 0,
			 Unk0x02: 0,
			 Unk0x03: 0,
			 UnkFloat0x04: 0.,
			 Unk0x08: 0,
			 UnkFloat0x0C: 0.,
			 Unk0x10: 0,
			 UnkFloat0x14: 0.,
			 UnkFloat0x18: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_REFLECTION_QUALITY_DETAIL {
	pub enabled: u8,
	pub localLightEnabled: u8,
	pub localLightForceEnabled: u8,
	pub dmy: [u8;1],
	pub resolutionDivider: i32,
	pub ssrEnabled: u8,
	pub ssrGaussianBlurEnabled: u8,
	pub dmy2: [u8;2],
	pub ssrDepthRejectThresholdScale: f32,
	pub ssrRayTraceStepScale: f32,
	pub ssrFadeToViewerBias: f32,
	pub ssrFresnelRejectBias: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_REFLECTION_QUALITY_DETAIL {
}
impl Default for CS_REFLECTION_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 1,
			 localLightEnabled: 1,
			 localLightForceEnabled: 0,
			 dmy: [0;1],
			 resolutionDivider: 2,
			 ssrEnabled: 1,
			 ssrGaussianBlurEnabled: 1,
			 dmy2: [0;2],
			 ssrDepthRejectThresholdScale: 1.,
			 ssrRayTraceStepScale: 1.,
			 ssrFadeToViewerBias: 0.,
			 ssrFresnelRejectBias: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_SHADER_QUALITY_DETAIL {
	pub sssEnabled: u8,
	pub tessellationEnabled: u8,
	pub highPrecisionNormalEnabled: u8,
	pub dmy: [u8;1],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_SHADER_QUALITY_DETAIL {
}
impl Default for CS_SHADER_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 sssEnabled: 1,
			 tessellationEnabled: 0,
			 highPrecisionNormalEnabled: 0,
			 dmy: [0;1],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_SHADOW_QUALITY_DETAIL {
	pub enabled: u8,
	pub maxFilterLevel: u8,
	pub dmy: [u8;2],
	pub textureSizeScaler: i32,
	pub textureSizeDivider: i32,
	pub textureMinSize: i32,
	pub textureMaxSize: i32,
	pub blurCountBias: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_SHADOW_QUALITY_DETAIL {
}
impl Default for CS_SHADOW_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 1,
			 maxFilterLevel: 1,
			 dmy: [0;2],
			 textureSizeScaler: 1,
			 textureSizeDivider: 2,
			 textureMinSize: 128,
			 textureMaxSize: 1024,
			 blurCountBias: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_SSAO_QUALITY_DETAIL {
	pub enabled: u8,
	pub cs_reprojEnabledType: u8,
	pub cs_upScaleEnabledType: u8,
	pub cs_useNormalEnabledType: u8,
	pub dmy: [u8;1],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_SSAO_QUALITY_DETAIL {
}
impl Default for CS_SSAO_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 enabled: 1,
			 cs_reprojEnabledType: 1,
			 cs_upScaleEnabledType: 0,
			 cs_useNormalEnabledType: 1,
			 dmy: [0;1],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_TEXTURE_FILTER_QUALITY_DETAIL {
	pub filter: u8,
	pub dmy: [u8;3],
	pub maxAnisoLevel: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_TEXTURE_FILTER_QUALITY_DETAIL {
}
impl Default for CS_TEXTURE_FILTER_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 filter: 3,
			 dmy: [0;3],
			 maxAnisoLevel: 4,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL {
	pub fogEnabled: u8,
	pub fogShadowEnabled: u8,
	pub dmy: [u8;2],
	pub fogShadowSampleCountBias: i32,
	pub fogLocalLightDistScale: f32,
	pub fogVolueSizeScaler: i32,
	pub fogVolueSizeDivider: i32,
	pub fogVolumeDepthScaler: i32,
	pub fogVolumeDepthDivider: i32,
	pub fogVolumeEnabled: u8,
	pub fogVolumeUpScaleType: u8,
	pub fogVolumeEdgeCorrectionLevel: u8,
	pub fogVolumeRayMarcingSampleCountOffset: i8,
	pub fogVolumeShadowEnabled: u8,
	pub fogVolumeForceShadowing: u8,
	pub fogVolumeResolution: u8,
	pub pad2: [u8;1],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL {
}
impl Default for CS_VOLUMETRIC_EFFECT_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 fogEnabled: 1,
			 fogShadowEnabled: 1,
			 dmy: [0;2],
			 fogShadowSampleCountBias: 0,
			 fogLocalLightDistScale: 0.,
			 fogVolueSizeScaler: 1,
			 fogVolueSizeDivider: 1,
			 fogVolumeDepthScaler: 1,
			 fogVolumeDepthDivider: 1,
			 fogVolumeEnabled: 1,
			 fogVolumeUpScaleType: 1,
			 fogVolumeEdgeCorrectionLevel: 2,
			 fogVolumeRayMarcingSampleCountOffset: 0,
			 fogVolumeShadowEnabled: 1,
			 fogVolumeForceShadowing: 0,
			 fogVolumeResolution: 0,
			 pad2: [0;1],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_WATER_QUALITY_DETAIL {
	pub interactionEnabled: u8,
	pub dmy: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_WATER_QUALITY_DETAIL {
}
impl Default for CS_WATER_QUALITY_DETAIL {
	fn default() -> Self {
		Self {
			 interactionEnabled: 1,
			 dmy: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GESTURE_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub itemId: i32,
	pub msgAnimId: i32,
	bits_1: u8,
	pub pad1: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GESTURE_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn cannotUseRiding(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn pad2(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
}
impl Default for GESTURE_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 itemId: 0,
			 msgAnimId: 0,
			 bits_1: 0,
			 pad1: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GPARAM_GRID_REGION_INFO_PARAM_ST {
	pub GparamGridRegionId: i32,
	pub Reserve: [u8;28],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GPARAM_GRID_REGION_INFO_PARAM_ST {
}
impl Default for GPARAM_GRID_REGION_INFO_PARAM_ST {
	fn default() -> Self {
		Self {
			 GparamGridRegionId: 0,
			 Reserve: [0;28],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GPARAM_REF_SETTINGS_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub RefTargetMapId: i32,
	pub Reserve: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GPARAM_REF_SETTINGS_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for GPARAM_REF_SETTINGS_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 RefTargetMapId: -1,
			 Reserve: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GRAPHICS_COMMON_PARAM_ST {
	pub hitBulletDecalOffset_HitIns: f32,
	pub reserved02: [u8;8],
	pub charaWetDecalFadeRange: f32,
	pub reserved04: [u8;240],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GRAPHICS_COMMON_PARAM_ST {
}
impl Default for GRAPHICS_COMMON_PARAM_ST {
	fn default() -> Self {
		Self {
			 hitBulletDecalOffset_HitIns: 0.05,
			 reserved02: [0;8],
			 charaWetDecalFadeRange: 0.6,
			 reserved04: [0;240],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_GRAPHICS_CONFIG_PARAM_ST {
	pub m_textureFilterQuality: u8,
	pub m_aaQuality: u8,
	pub m_ssaoQuality: u8,
	pub m_dofQuality: u8,
	pub m_motionBlurQuality: u8,
	pub m_shadowQuality: u8,
	pub m_lightingQuality: u8,
	pub m_effectQuality: u8,
	pub m_decalQuality: u8,
	pub m_reflectionQuality: u8,
	pub m_waterQuality: u8,
	pub m_shaderQuality: u8,
	pub m_volumetricEffectQuality: u8,
	pub m_dummy: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_GRAPHICS_CONFIG_PARAM_ST {
}
impl Default for CS_GRAPHICS_CONFIG_PARAM_ST {
	fn default() -> Self {
		Self {
			 m_textureFilterQuality: 2,
			 m_aaQuality: 3,
			 m_ssaoQuality: 3,
			 m_dofQuality: 3,
			 m_motionBlurQuality: 3,
			 m_shadowQuality: 3,
			 m_lightingQuality: 3,
			 m_effectQuality: 3,
			 m_decalQuality: 3,
			 m_reflectionQuality: 3,
			 m_waterQuality: 3,
			 m_shaderQuality: 3,
			 m_volumetricEffectQuality: 3,
			 m_dummy: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GRASS_LOD_RANGE_PARAM_ST {
	pub LOD0_range: f32,
	pub LOD0_play: f32,
	pub LOD1_range: f32,
	pub LOD1_play: f32,
	pub LOD2_range: f32,
	pub LOD2_play: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GRASS_LOD_RANGE_PARAM_ST {
}
impl Default for GRASS_LOD_RANGE_PARAM_ST {
	fn default() -> Self {
		Self {
			 LOD0_range: 0.,
			 LOD0_play: 0.,
			 LOD1_range: 0.,
			 LOD1_play: 0.,
			 LOD2_range: 0.,
			 LOD2_play: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GRASS_MAP_SETTINGS_PARAM_ST {
	pub grassType0: i32,
	pub grassType1: i32,
	pub grassType2: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GRASS_MAP_SETTINGS_PARAM_ST {
}
impl Default for GRASS_MAP_SETTINGS_PARAM_ST {
	fn default() -> Self {
		Self {
			 grassType0: 0,
			 grassType1: 0,
			 grassType2: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct GRASS_TYPE_PARAM_ST {
	pub lodRange: i16,
	pub lod0ClusterType: u8,
	pub lod1ClusterType: u8,
	pub lod2ClusterType: u8,
	pub pad0: [u8;2],
	pub distributionType: u8,
	pub baseDensity: f32,
	pub model0Name: [u8;16],
	pub flatTextureName: [u8;32],
	pub billboardTextureName: [u8;32],
	pub normalInfluence: u8,
	pub inclinationMax: u8,
	pub inclinationJitter: u8,
	pub scaleBaseMin: u8,
	pub scaleBaseMax: u8,
	pub scaleHeightMin: u8,
	pub scaleHeightMax: u8,
	pub colorShade1_r: u8,
	pub colorShade1_g: u8,
	pub colorShade1_b: u8,
	pub colorShade2_r: u8,
	pub colorShade2_g: u8,
	pub colorShade2_b: u8,
	pub flatSplitType: u8,
	pub flatBladeCount: u8,
	pub flatSlant: i8,
	pub flatRadius: f32,
	pub castShadow: u8,
	pub windAmplitude: u8,
	pub pad1: [u8;1],
	pub windCycle: u8,
	pub orientationAngle: f32,
	pub orientationRange: f32,
	pub spacing: f32,
	pub dithering: u8,
	pub pad: [u8;3],
	pub simpleModelName: [u8;16],
	pub model1Name: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl GRASS_TYPE_PARAM_ST {
}
impl Default for GRASS_TYPE_PARAM_ST {
	fn default() -> Self {
		Self {
			 lodRange: 0,
			 lod0ClusterType: 0,
			 lod1ClusterType: 0,
			 lod2ClusterType: 0,
			 pad0: [0;2],
			 distributionType: 0,
			 baseDensity: 1.,
			 model0Name: [0;16],
			 flatTextureName: [0;32],
			 billboardTextureName: [0;32],
			 normalInfluence: 0,
			 inclinationMax: 90,
			 inclinationJitter: 0,
			 scaleBaseMin: 100,
			 scaleBaseMax: 100,
			 scaleHeightMin: 100,
			 scaleHeightMax: 100,
			 colorShade1_r: 255,
			 colorShade1_g: 255,
			 colorShade1_b: 255,
			 colorShade2_r: 255,
			 colorShade2_g: 255,
			 colorShade2_b: 255,
			 flatSplitType: 0,
			 flatBladeCount: 2,
			 flatSlant: 0,
			 flatRadius: 0.,
			 castShadow: 1,
			 windAmplitude: 80,
			 pad1: [0;1],
			 windCycle: 40,
			 orientationAngle: -1.,
			 orientationRange: -1.,
			 spacing: 0.,
			 dithering: 0,
			 pad: [0;3],
			 simpleModelName: [0;16],
			 model1Name: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct HIT_EFFECT_SE_PARAM_ST {
	pub Iron_Slash_S: i32,
	pub Iron_Slash_L: i32,
	pub Iron_Slash_LL: i32,
	pub Iron_Thrust_S: i32,
	pub Iron_Thrust_L: i32,
	pub Iron_Thrust_LL: i32,
	pub Iron_Blow_S: i32,
	pub Iron_Blow_L: i32,
	pub Iron_Blow_LL: i32,
	pub Fire_Slash_S: i32,
	pub Fire_Slash_L: i32,
	pub Fire_Slash_LL: i32,
	pub Fire_Thrust_S: i32,
	pub Fire_Thrust_L: i32,
	pub Fire_Thrust_LL: i32,
	pub Fire_Blow_S: i32,
	pub Fire_Blow_L: i32,
	pub Fire_Blow_LL: i32,
	pub Wood_Slash_S: i32,
	pub Wood_Slash_L: i32,
	pub Wood_Slash_LL: i32,
	pub Wood_Thrust_S: i32,
	pub Wood_Thrust_L: i32,
	pub Wood_Thrust_LL: i32,
	pub Wood_Blow_S: i32,
	pub Wood_Blow_L: i32,
	pub Wood_Blow_LL: i32,
	pub Body_Slash_S: i32,
	pub Body_Slash_L: i32,
	pub Body_Slash_LL: i32,
	pub Body_Thrust_S: i32,
	pub Body_Thrust_L: i32,
	pub Body_Thrust_LL: i32,
	pub Body_Blow_S: i32,
	pub Body_Blow_L: i32,
	pub Body_Blow_LL: i32,
	pub Eclipse_Slash_S: i32,
	pub Eclipse_Slash_L: i32,
	pub Eclipse_Slash_LL: i32,
	pub Eclipse_Thrust_S: i32,
	pub Eclipse_Thrust_L: i32,
	pub Eclipse_Thrust_LL: i32,
	pub Eclipse_Blow_S: i32,
	pub Eclipse_Blow_L: i32,
	pub Eclipse_Blow_LL: i32,
	pub Energy_Slash_S: i32,
	pub Energy_Slash_L: i32,
	pub Energy_Slash_LL: i32,
	pub Energy_Thrust_S: i32,
	pub Energy_Thrust_L: i32,
	pub Energy_Thrust_LL: i32,
	pub Energy_Blow_S: i32,
	pub Energy_Blow_L: i32,
	pub Energy_Blow_LL: i32,
	pub None_Slash_S: i32,
	pub None_Slash_L: i32,
	pub None_Slash_LL: i32,
	pub None_Thrust_S: i32,
	pub None_Thrust_L: i32,
	pub None_Thrust_LL: i32,
	pub None_Blow_S: i32,
	pub None_Blow_L: i32,
	pub None_Blow_LL: i32,
	pub Dmy1_Slash_S: i32,
	pub Dmy1_Slash_L: i32,
	pub Dmy1_Slash_LL: i32,
	pub Dmy1_Thrust_S: i32,
	pub Dmy1_Thrust_L: i32,
	pub Dmy1_Thrust_LL: i32,
	pub Dmy1_Blow_S: i32,
	pub Dmy1_Blow_L: i32,
	pub Dmy1_Blow_LL: i32,
	pub Dmy2_Slash_S: i32,
	pub Dmy2_Slash_L: i32,
	pub Dmy2_Slash_LL: i32,
	pub Dmy2_Thrust_S: i32,
	pub Dmy2_Thrust_L: i32,
	pub Dmy2_Thrust_LL: i32,
	pub Dmy2_Blow_S: i32,
	pub Dmy2_Blow_L: i32,
	pub Dmy2_Blow_LL: i32,
	pub Dmy3_Slash_S: i32,
	pub Dmy3_Slash_L: i32,
	pub Dmy3_Slash_LL: i32,
	pub Dmy3_Thrust_S: i32,
	pub Dmy3_Thrust_L: i32,
	pub Dmy3_Thrust_LL: i32,
	pub Dmy3_Blow_S: i32,
	pub Dmy3_Blow_L: i32,
	pub Dmy3_Blow_LL: i32,
	pub Maggot_Slash_S: i32,
	pub Maggot_Slash_L: i32,
	pub Maggot_Slash_LL: i32,
	pub Maggot_Thrust_S: i32,
	pub Maggot_Thrust_L: i32,
	pub Maggot_Thrust_LL: i32,
	pub Maggot_Blow_S: i32,
	pub Maggot_Blow_L: i32,
	pub Maggot_Blow_LL: i32,
	pub Wax_Slash_S: i32,
	pub Wax_Slash_L: i32,
	pub Wax_Slash_LL: i32,
	pub Wax_Thrust_S: i32,
	pub Wax_Thrust_L: i32,
	pub Wax_Thrust_LL: i32,
	pub Wax_Blow_S: i32,
	pub Wax_Blow_L: i32,
	pub Wax_Blow_LL: i32,
	pub FireFlame_Slash_S: i32,
	pub FireFlame_Slash_L: i32,
	pub FireFlame_Slash_LL: i32,
	pub FireFlame_Thrust_S: i32,
	pub FireFlame_Thrust_L: i32,
	pub FireFlame_Thrust_LL: i32,
	pub FireFlame_Blow_S: i32,
	pub FireFlame_Blow_L: i32,
	pub FireFlame_Blow_LL: i32,
	pub EclipseGas_Slash_S: i32,
	pub EclipseGas_Slash_L: i32,
	pub EclipseGas_Slash_LL: i32,
	pub EclipseGas_Thrust_S: i32,
	pub EclipseGas_Thrust_L: i32,
	pub EclipseGas_Thrust_LL: i32,
	pub EclipseGas_Blow_S: i32,
	pub EclipseGas_Blow_L: i32,
	pub EclipseGas_Blow_LL: i32,
	pub EnergyStrong_Slash_S: i32,
	pub EnergyStrong_Slash_L: i32,
	pub EnergyStrong_Slash_LL: i32,
	pub EnergyStrong_Thrust_S: i32,
	pub EnergyStrong_Thrust_L: i32,
	pub EnergyStrong_Thrust_LL: i32,
	pub EnergyStrong_Blow_S: i32,
	pub EnergyStrong_Blow_L: i32,
	pub EnergyStrong_Blow_LL: i32,
	pub reserve: [u8;100],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl HIT_EFFECT_SE_PARAM_ST {
}
impl Default for HIT_EFFECT_SE_PARAM_ST {
	fn default() -> Self {
		Self {
			 Iron_Slash_S: 0,
			 Iron_Slash_L: 0,
			 Iron_Slash_LL: 0,
			 Iron_Thrust_S: 0,
			 Iron_Thrust_L: 0,
			 Iron_Thrust_LL: 0,
			 Iron_Blow_S: 0,
			 Iron_Blow_L: 0,
			 Iron_Blow_LL: 0,
			 Fire_Slash_S: 0,
			 Fire_Slash_L: 0,
			 Fire_Slash_LL: 0,
			 Fire_Thrust_S: 0,
			 Fire_Thrust_L: 0,
			 Fire_Thrust_LL: 0,
			 Fire_Blow_S: 0,
			 Fire_Blow_L: 0,
			 Fire_Blow_LL: 0,
			 Wood_Slash_S: 0,
			 Wood_Slash_L: 0,
			 Wood_Slash_LL: 0,
			 Wood_Thrust_S: 0,
			 Wood_Thrust_L: 0,
			 Wood_Thrust_LL: 0,
			 Wood_Blow_S: 0,
			 Wood_Blow_L: 0,
			 Wood_Blow_LL: 0,
			 Body_Slash_S: 0,
			 Body_Slash_L: 0,
			 Body_Slash_LL: 0,
			 Body_Thrust_S: 0,
			 Body_Thrust_L: 0,
			 Body_Thrust_LL: 0,
			 Body_Blow_S: 0,
			 Body_Blow_L: 0,
			 Body_Blow_LL: 0,
			 Eclipse_Slash_S: 0,
			 Eclipse_Slash_L: 0,
			 Eclipse_Slash_LL: 0,
			 Eclipse_Thrust_S: 0,
			 Eclipse_Thrust_L: 0,
			 Eclipse_Thrust_LL: 0,
			 Eclipse_Blow_S: 0,
			 Eclipse_Blow_L: 0,
			 Eclipse_Blow_LL: 0,
			 Energy_Slash_S: 0,
			 Energy_Slash_L: 0,
			 Energy_Slash_LL: 0,
			 Energy_Thrust_S: 0,
			 Energy_Thrust_L: 0,
			 Energy_Thrust_LL: 0,
			 Energy_Blow_S: 0,
			 Energy_Blow_L: 0,
			 Energy_Blow_LL: 0,
			 None_Slash_S: 0,
			 None_Slash_L: 0,
			 None_Slash_LL: 0,
			 None_Thrust_S: 0,
			 None_Thrust_L: 0,
			 None_Thrust_LL: 0,
			 None_Blow_S: 0,
			 None_Blow_L: 0,
			 None_Blow_LL: 0,
			 Dmy1_Slash_S: 0,
			 Dmy1_Slash_L: 0,
			 Dmy1_Slash_LL: 0,
			 Dmy1_Thrust_S: 0,
			 Dmy1_Thrust_L: 0,
			 Dmy1_Thrust_LL: 0,
			 Dmy1_Blow_S: 0,
			 Dmy1_Blow_L: 0,
			 Dmy1_Blow_LL: 0,
			 Dmy2_Slash_S: 0,
			 Dmy2_Slash_L: 0,
			 Dmy2_Slash_LL: 0,
			 Dmy2_Thrust_S: 0,
			 Dmy2_Thrust_L: 0,
			 Dmy2_Thrust_LL: 0,
			 Dmy2_Blow_S: 0,
			 Dmy2_Blow_L: 0,
			 Dmy2_Blow_LL: 0,
			 Dmy3_Slash_S: 0,
			 Dmy3_Slash_L: 0,
			 Dmy3_Slash_LL: 0,
			 Dmy3_Thrust_S: 0,
			 Dmy3_Thrust_L: 0,
			 Dmy3_Thrust_LL: 0,
			 Dmy3_Blow_S: 0,
			 Dmy3_Blow_L: 0,
			 Dmy3_Blow_LL: 0,
			 Maggot_Slash_S: 0,
			 Maggot_Slash_L: 0,
			 Maggot_Slash_LL: 0,
			 Maggot_Thrust_S: 0,
			 Maggot_Thrust_L: 0,
			 Maggot_Thrust_LL: 0,
			 Maggot_Blow_S: 0,
			 Maggot_Blow_L: 0,
			 Maggot_Blow_LL: 0,
			 Wax_Slash_S: 0,
			 Wax_Slash_L: 0,
			 Wax_Slash_LL: 0,
			 Wax_Thrust_S: 0,
			 Wax_Thrust_L: 0,
			 Wax_Thrust_LL: 0,
			 Wax_Blow_S: 0,
			 Wax_Blow_L: 0,
			 Wax_Blow_LL: 0,
			 FireFlame_Slash_S: 0,
			 FireFlame_Slash_L: 0,
			 FireFlame_Slash_LL: 0,
			 FireFlame_Thrust_S: 0,
			 FireFlame_Thrust_L: 0,
			 FireFlame_Thrust_LL: 0,
			 FireFlame_Blow_S: 0,
			 FireFlame_Blow_L: 0,
			 FireFlame_Blow_LL: 0,
			 EclipseGas_Slash_S: 0,
			 EclipseGas_Slash_L: 0,
			 EclipseGas_Slash_LL: 0,
			 EclipseGas_Thrust_S: 0,
			 EclipseGas_Thrust_L: 0,
			 EclipseGas_Thrust_LL: 0,
			 EclipseGas_Blow_S: 0,
			 EclipseGas_Blow_L: 0,
			 EclipseGas_Blow_LL: 0,
			 EnergyStrong_Slash_S: 0,
			 EnergyStrong_Slash_L: 0,
			 EnergyStrong_Slash_LL: 0,
			 EnergyStrong_Thrust_S: 0,
			 EnergyStrong_Thrust_L: 0,
			 EnergyStrong_Thrust_LL: 0,
			 EnergyStrong_Blow_S: 0,
			 EnergyStrong_Blow_L: 0,
			 EnergyStrong_Blow_LL: 0,
			 reserve: [0;100],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct HIT_EFFECT_SFX_CONCEPT_PARAM_ST {
	pub atkIron_1: i16,
	pub atkIron_2: i16,
	pub atkLeather_1: i16,
	pub atkLeather_2: i16,
	pub atkWood_1: i16,
	pub atkWood_2: i16,
	pub atkBody_1: i16,
	pub atkBody_2: i16,
	pub atkStone_1: i16,
	pub atkStone_2: i16,
	pub pad: [u8;4],
	pub atkNone_1: i16,
	pub atkNone_2: i16,
	pub reserve: [u8;52],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl HIT_EFFECT_SFX_CONCEPT_PARAM_ST {
}
impl Default for HIT_EFFECT_SFX_CONCEPT_PARAM_ST {
	fn default() -> Self {
		Self {
			 atkIron_1: 0,
			 atkIron_2: 0,
			 atkLeather_1: 0,
			 atkLeather_2: 0,
			 atkWood_1: 0,
			 atkWood_2: 0,
			 atkBody_1: 0,
			 atkBody_2: 0,
			 atkStone_1: 0,
			 atkStone_2: 0,
			 pad: [0;4],
			 atkNone_1: 0,
			 atkNone_2: 0,
			 reserve: [0;52],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct HIT_EFFECT_SFX_PARAM_ST {
	pub Slash_Normal: i32,
	pub Slash_S: i32,
	pub Slash_L: i32,
	pub Slash_Specific1: i32,
	pub Slash_Specific2: i32,
	pub Blow_Normal: i32,
	pub Blow_S: i32,
	pub Blow_L: i32,
	pub Blow_Specific1: i32,
	pub Blow_Specific2: i32,
	pub Thrust_Normal: i32,
	pub Thrust_S: i32,
	pub Thrust_L: i32,
	pub Thrust_Specific1: i32,
	pub Thrust_Specific2: i32,
	pub Neutral_Normal: i32,
	pub Neutral_S: i32,
	pub Neutral_L: i32,
	pub Neutral_Specific1: i32,
	pub Neutral_Specific2: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl HIT_EFFECT_SFX_PARAM_ST {
}
impl Default for HIT_EFFECT_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 Slash_Normal: 0,
			 Slash_S: 0,
			 Slash_L: 0,
			 Slash_Specific1: 0,
			 Slash_Specific2: 0,
			 Blow_Normal: 0,
			 Blow_S: 0,
			 Blow_L: 0,
			 Blow_Specific1: 0,
			 Blow_Specific2: 0,
			 Thrust_Normal: 0,
			 Thrust_S: 0,
			 Thrust_L: 0,
			 Thrust_Specific1: 0,
			 Thrust_Specific2: 0,
			 Neutral_Normal: 0,
			 Neutral_S: 0,
			 Neutral_L: 0,
			 Neutral_Specific1: 0,
			 Neutral_Specific2: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct HIT_MTRL_PARAM_ST {
	pub aiVolumeRate: f32,
	pub spEffectIdOnHit0: i32,
	pub spEffectIdOnHit1: i32,
	bits_0: u8,
	pub hardnessType: u8,
	pub pad2: [u8;6],
	pub spEffectIdOnHit0_ClearCount_2: i32,
	pub spEffectIdOnHit0_ClearCount_3: i32,
	pub spEffectIdOnHit0_ClearCount_4: i32,
	pub spEffectIdOnHit0_ClearCount_5: i32,
	pub spEffectIdOnHit0_ClearCount_6: i32,
	pub spEffectIdOnHit0_ClearCount_7: i32,
	pub spEffectIdOnHit0_ClearCount_8: i32,
	pub spEffectIdOnHit1_ClearCount_2: i32,
	pub spEffectIdOnHit1_ClearCount_3: i32,
	pub spEffectIdOnHit1_ClearCount_4: i32,
	pub spEffectIdOnHit1_ClearCount_5: i32,
	pub spEffectIdOnHit1_ClearCount_6: i32,
	pub spEffectIdOnHit1_ClearCount_7: i32,
	pub spEffectIdOnHit1_ClearCount_8: i32,
	pub replaceMateiralId_Rain: i16,
	pub pad4: [u8;2],
	pub spEffectId_forWet00: i32,
	pub spEffectId_forWet01: i32,
	pub spEffectId_forWet02: i32,
	pub spEffectId_forWet03: i32,
	pub spEffectId_forWet04: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl HIT_MTRL_PARAM_ST {
	pub fn footEffectHeightType(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn footEffectDirType(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn floorHeightType(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn disableFallDamage(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn isHardnessForSoundReverb(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
}
impl Default for HIT_MTRL_PARAM_ST {
	fn default() -> Self {
		Self {
			 aiVolumeRate: 1.,
			 spEffectIdOnHit0: -1,
			 spEffectIdOnHit1: -1,
			 bits_0: 0,
			 hardnessType: 0,
			 pad2: [0;6],
			 spEffectIdOnHit0_ClearCount_2: -1,
			 spEffectIdOnHit0_ClearCount_3: -1,
			 spEffectIdOnHit0_ClearCount_4: -1,
			 spEffectIdOnHit0_ClearCount_5: -1,
			 spEffectIdOnHit0_ClearCount_6: -1,
			 spEffectIdOnHit0_ClearCount_7: -1,
			 spEffectIdOnHit0_ClearCount_8: -1,
			 spEffectIdOnHit1_ClearCount_2: -1,
			 spEffectIdOnHit1_ClearCount_3: -1,
			 spEffectIdOnHit1_ClearCount_4: -1,
			 spEffectIdOnHit1_ClearCount_5: -1,
			 spEffectIdOnHit1_ClearCount_6: -1,
			 spEffectIdOnHit1_ClearCount_7: -1,
			 spEffectIdOnHit1_ClearCount_8: -1,
			 replaceMateiralId_Rain: -1,
			 pad4: [0;2],
			 spEffectId_forWet00: -1,
			 spEffectId_forWet01: -1,
			 spEffectId_forWet02: -1,
			 spEffectId_forWet03: -1,
			 spEffectId_forWet04: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ITEMLOT_PARAM_ST {
	pub lotItemId01: i32,
	pub lotItemId02: i32,
	pub lotItemId03: i32,
	pub lotItemId04: i32,
	pub lotItemId05: i32,
	pub lotItemId06: i32,
	pub lotItemId07: i32,
	pub lotItemId08: i32,
	pub lotItemCategory01: i32,
	pub lotItemCategory02: i32,
	pub lotItemCategory03: i32,
	pub lotItemCategory04: i32,
	pub lotItemCategory05: i32,
	pub lotItemCategory06: i32,
	pub lotItemCategory07: i32,
	pub lotItemCategory08: i32,
	pub lotItemBasePoint01: i16,
	pub lotItemBasePoint02: i16,
	pub lotItemBasePoint03: i16,
	pub lotItemBasePoint04: i16,
	pub lotItemBasePoint05: i16,
	pub lotItemBasePoint06: i16,
	pub lotItemBasePoint07: i16,
	pub lotItemBasePoint08: i16,
	pub cumulateLotPoint01: i16,
	pub cumulateLotPoint02: i16,
	pub cumulateLotPoint03: i16,
	pub cumulateLotPoint04: i16,
	pub cumulateLotPoint05: i16,
	pub cumulateLotPoint06: i16,
	pub cumulateLotPoint07: i16,
	pub cumulateLotPoint08: i16,
	pub getItemFlagId01: i32,
	pub getItemFlagId02: i32,
	pub getItemFlagId03: i32,
	pub getItemFlagId04: i32,
	pub getItemFlagId05: i32,
	pub getItemFlagId06: i32,
	pub getItemFlagId07: i32,
	pub getItemFlagId08: i32,
	pub getItemFlagId: i32,
	pub cumulateNumFlagId: i32,
	pub cumulateNumMax: u8,
	pub lotItem_Rarity: i8,
	pub lotItemNum01: u8,
	pub lotItemNum02: u8,
	pub lotItemNum03: u8,
	pub lotItemNum04: u8,
	pub lotItemNum05: u8,
	pub lotItemNum06: u8,
	pub lotItemNum07: u8,
	pub lotItemNum08: u8,
	bits_0: i16,
	pub GameClearOffset: i8,
	bits_1: u8,
	pub PAD2: i16,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ITEMLOT_PARAM_ST {
	pub fn enableLuck01(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn enableLuck02(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn enableLuck03(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn enableLuck04(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn enableLuck05(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn enableLuck06(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn enableLuck07(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn enableLuck08(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn cumulateReset01(&self) -> bool {	
			self.bits_0 & (1 << 8) != 0
	}
	pub fn cumulateReset02(&self) -> bool {	
			self.bits_0 & (1 << 9) != 0
	}
	pub fn cumulateReset03(&self) -> bool {	
			self.bits_0 & (1 << 10) != 0
	}
	pub fn cumulateReset04(&self) -> bool {	
			self.bits_0 & (1 << 11) != 0
	}
	pub fn cumulateReset05(&self) -> bool {	
			self.bits_0 & (1 << 12) != 0
	}
	pub fn cumulateReset06(&self) -> bool {	
			self.bits_0 & (1 << 13) != 0
	}
	pub fn cumulateReset07(&self) -> bool {	
			self.bits_0 & (1 << 14) != 0
	}
	pub fn cumulateReset08(&self) -> bool {	
			self.bits_0 & (1 << 15) != 0
	}
	pub fn canExecByFriendlyGhost(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn canExecByHostileGhost(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn PAD1(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
}
impl Default for ITEMLOT_PARAM_ST {
	fn default() -> Self {
		Self {
			 lotItemId01: 0,
			 lotItemId02: 0,
			 lotItemId03: 0,
			 lotItemId04: 0,
			 lotItemId05: 0,
			 lotItemId06: 0,
			 lotItemId07: 0,
			 lotItemId08: 0,
			 lotItemCategory01: 0,
			 lotItemCategory02: 0,
			 lotItemCategory03: 0,
			 lotItemCategory04: 0,
			 lotItemCategory05: 0,
			 lotItemCategory06: 0,
			 lotItemCategory07: 0,
			 lotItemCategory08: 0,
			 lotItemBasePoint01: 0,
			 lotItemBasePoint02: 0,
			 lotItemBasePoint03: 0,
			 lotItemBasePoint04: 0,
			 lotItemBasePoint05: 0,
			 lotItemBasePoint06: 0,
			 lotItemBasePoint07: 0,
			 lotItemBasePoint08: 0,
			 cumulateLotPoint01: 0,
			 cumulateLotPoint02: 0,
			 cumulateLotPoint03: 0,
			 cumulateLotPoint04: 0,
			 cumulateLotPoint05: 0,
			 cumulateLotPoint06: 0,
			 cumulateLotPoint07: 0,
			 cumulateLotPoint08: 0,
			 getItemFlagId01: 0,
			 getItemFlagId02: 0,
			 getItemFlagId03: 0,
			 getItemFlagId04: 0,
			 getItemFlagId05: 0,
			 getItemFlagId06: 0,
			 getItemFlagId07: 0,
			 getItemFlagId08: 0,
			 getItemFlagId: 0,
			 cumulateNumFlagId: 0,
			 cumulateNumMax: 0,
			 lotItem_Rarity: -1,
			 lotItemNum01: 0,
			 lotItemNum02: 0,
			 lotItemNum03: 0,
			 lotItemNum04: 0,
			 lotItemNum05: 0,
			 lotItemNum06: 0,
			 lotItemNum07: 0,
			 lotItemNum08: 0,
			 bits_0: 0,
			 GameClearOffset: -1,
			 bits_1: 0,
			 PAD2: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct CS_KEY_ASSIGN_MENUITEM_PARAM {
	pub textID: i32,
	pub key: i32,
	pub enableUnassign: u8,
	pub enablePadConfig: u8,
	pub enableMouseConfig: u8,
	pub group: u8,
	pub mappingTextID: i32,
	pub viewPad: u8,
	pub viewKeyboardMouse: u8,
	pub padding: [u8;6],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl CS_KEY_ASSIGN_MENUITEM_PARAM {
}
impl Default for CS_KEY_ASSIGN_MENUITEM_PARAM {
	fn default() -> Self {
		Self {
			 textID: 0,
			 key: -1,
			 enableUnassign: 1,
			 enablePadConfig: 1,
			 enableMouseConfig: 1,
			 group: 0,
			 mappingTextID: 0,
			 viewPad: 1,
			 viewKeyboardMouse: 1,
			 padding: [0;6],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct KEY_ASSIGN_PARAM_ST {
	pub padKeyId: i32,
	pub keyboardModifyKey: i32,
	pub keyboardKeyId: i32,
	pub mouseModifyKey: i32,
	pub mouseKeyId: i32,
	pub reserved: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl KEY_ASSIGN_PARAM_ST {
}
impl Default for KEY_ASSIGN_PARAM_ST {
	fn default() -> Self {
		Self {
			 padKeyId: -1,
			 keyboardModifyKey: 0,
			 keyboardKeyId: -1,
			 mouseModifyKey: 0,
			 mouseKeyId: 0,
			 reserved: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct KNOCKBACK_PARAM_ST {
	pub damage_Min_ContTime: f32,
	pub damage_S_ContTime: f32,
	pub damage_M_ContTime: f32,
	pub damage_L_ContTime: f32,
	pub damage_BlowS_ContTime: f32,
	pub damage_BlowM_ContTime: f32,
	pub damage_Strike_ContTime: f32,
	pub damage_Uppercut_ContTime: f32,
	pub damage_Push_ContTime: f32,
	pub damage_Breath_ContTime: f32,
	pub damage_HeadShot_ContTime: f32,
	pub guard_S_ContTime: f32,
	pub guard_L_ContTime: f32,
	pub guard_LL_ContTime: f32,
	pub guardBrake_ContTime: f32,
	pub damage_Min_DecTime: f32,
	pub damage_S_DecTime: f32,
	pub damage_M_DecTime: f32,
	pub damage_L_DecTime: f32,
	pub damage_BlowS_DecTime: f32,
	pub damage_BlowM_DecTime: f32,
	pub damage_Strike_DecTime: f32,
	pub damage_Uppercut_DecTime: f32,
	pub damage_Push_DecTime: f32,
	pub damage_Breath_DecTime: f32,
	pub damage_HeadShot_DecTime: f32,
	pub guard_S_DecTime: f32,
	pub guard_L_DecTime: f32,
	pub guard_LL_DecTime: f32,
	pub guardBrake_DecTime: f32,
	pub pad: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl KNOCKBACK_PARAM_ST {
}
impl Default for KNOCKBACK_PARAM_ST {
	fn default() -> Self {
		Self {
			 damage_Min_ContTime: 0.,
			 damage_S_ContTime: 0.,
			 damage_M_ContTime: 0.,
			 damage_L_ContTime: 0.,
			 damage_BlowS_ContTime: 0.,
			 damage_BlowM_ContTime: 0.,
			 damage_Strike_ContTime: 0.,
			 damage_Uppercut_ContTime: 0.,
			 damage_Push_ContTime: 0.,
			 damage_Breath_ContTime: 0.,
			 damage_HeadShot_ContTime: 0.,
			 guard_S_ContTime: 0.,
			 guard_L_ContTime: 0.,
			 guard_LL_ContTime: 0.,
			 guardBrake_ContTime: 0.,
			 damage_Min_DecTime: 0.,
			 damage_S_DecTime: 0.,
			 damage_M_DecTime: 0.,
			 damage_L_DecTime: 0.,
			 damage_BlowS_DecTime: 0.,
			 damage_BlowM_DecTime: 0.,
			 damage_Strike_DecTime: 0.,
			 damage_Uppercut_DecTime: 0.,
			 damage_Push_DecTime: 0.,
			 damage_Breath_DecTime: 0.,
			 damage_HeadShot_DecTime: 0.,
			 guard_S_DecTime: 0.,
			 guard_L_DecTime: 0.,
			 guard_LL_DecTime: 0.,
			 guardBrake_DecTime: 0.,
			 pad: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub unlockFlagId: i32,
	pub invalidFlagId: i32,
	pub msgId: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for KNOWLEDGE_LOADSCREEN_ITEM_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 unlockFlagId: 0,
			 invalidFlagId: 0,
			 msgId: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM {
	pub TargetMapId: i32,
	pub TargetEventId: i32,
	pub SrcAssetId: i32,
	pub SrcAssetPartsNo: i32,
	pub DstAssetId: i32,
	pub DstAssetPartsNo: i32,
	pub SrcAssetIdRangeMin: i32,
	pub SrcAssetIdRangeMax: i32,
	pub DstAssetIdRangeMin: i32,
	pub DstAssetIdRangeMax: i32,
	pub LimitedMapRegionId0: i8,
	pub LimitedMapRegionId1: i8,
	pub LimitedMapRegionId2: i8,
	pub LimitedMapRegionId3: i8,
	pub reserve: [u8;4],
	pub LimitedMapRegionAssetId: i32,
	pub LimitedMapRegioAssetPartsNo: i32,
	pub LimitedMapRegioAssetIdRangeMin: i32,
	pub LimitedMapRegioAssetIdRangeMax: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM {
}
impl Default for LEGACY_DISTANT_VIEW_PARTS_REPLACE_PARAM {
	fn default() -> Self {
		Self {
			 TargetMapId: -1,
			 TargetEventId: 0,
			 SrcAssetId: -1,
			 SrcAssetPartsNo: -1,
			 DstAssetId: -1,
			 DstAssetPartsNo: -1,
			 SrcAssetIdRangeMin: -1,
			 SrcAssetIdRangeMax: -1,
			 DstAssetIdRangeMin: -1,
			 DstAssetIdRangeMax: -1,
			 LimitedMapRegionId0: -1,
			 LimitedMapRegionId1: -1,
			 LimitedMapRegionId2: -1,
			 LimitedMapRegionId3: -1,
			 reserve: [0;4],
			 LimitedMapRegionAssetId: -1,
			 LimitedMapRegioAssetPartsNo: -1,
			 LimitedMapRegioAssetIdRangeMin: -1,
			 LimitedMapRegioAssetIdRangeMax: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST {
	pub Lv00: f32,
	pub Lv01: f32,
	pub Lv02: f32,
	pub Lv03: f32,
	pub Lv04: f32,
	pub Lv05: f32,
	pub Lv06: f32,
	pub Lv07: f32,
	pub Lv08: f32,
	pub Lv09: f32,
	pub Lv10: f32,
	pub Lv11: f32,
	pub Lv12: f32,
	pub Lv13: f32,
	pub Lv14: f32,
	pub Lv15: f32,
	pub Lv16: f32,
	pub Lv17: f32,
	pub Lv18: f32,
	pub Lv19: f32,
	pub Lv20: f32,
	pub reserve: [u8;44],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST {
}
impl Default for LOAD_BALANCER_DRAW_DIST_SCALE_PARAM_ST {
	fn default() -> Self {
		Self {
			 Lv00: 1.,
			 Lv01: 1.,
			 Lv02: 1.,
			 Lv03: 1.,
			 Lv04: 1.,
			 Lv05: 1.,
			 Lv06: 1.,
			 Lv07: 1.,
			 Lv08: 1.,
			 Lv09: 1.,
			 Lv10: 1.,
			 Lv11: 1.,
			 Lv12: 1.,
			 Lv13: 1.,
			 Lv14: 1.,
			 Lv15: 1.,
			 Lv16: 1.,
			 Lv17: 1.,
			 Lv18: 1.,
			 Lv19: 1.,
			 Lv20: 1.,
			 reserve: [0;44],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {
	pub DrawDist_LvBegin: u8,
	pub DrawDist_LvEnd: u8,
	pub reserve0: [u8;2],
	pub DrawDist_ScaleBegin: f32,
	pub DrawDist_ScaleEnd: f32,
	pub ShadwDrawDist_LvBegin: u8,
	pub ShadwDrawDist_LvEnd: u8,
	pub reserve1: [u8;2],
	pub ShadwDrawDist_ScaleBegin: f32,
	pub ShadwDrawDist_ScaleEnd: f32,
	pub reserve2: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {
}
impl Default for LOAD_BALANCER_NEW_DRAW_DIST_SCALE_PARAM_ST {
	fn default() -> Self {
		Self {
			 DrawDist_LvBegin: 21,
			 DrawDist_LvEnd: 21,
			 reserve0: [0;2],
			 DrawDist_ScaleBegin: 1.,
			 DrawDist_ScaleEnd: 1.,
			 ShadwDrawDist_LvBegin: 21,
			 ShadwDrawDist_LvEnd: 21,
			 reserve1: [0;2],
			 ShadwDrawDist_ScaleBegin: 1.,
			 ShadwDrawDist_ScaleEnd: 1.,
			 reserve2: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct LOAD_BALANCER_PARAM_ST {
	pub lowerFpsThreshold: f32,
	pub upperFpsThreshold: f32,
	pub lowerFpsContinousCount: i32,
	pub upperFpsContinousCount: i32,
	pub downAfterChangeSleep: i32,
	pub upAfterChangeSleep: i32,
	pub postProcessLightShaft: u8,
	pub postProcessBloom: u8,
	pub postProcessGlow: u8,
	pub postProcessAA: u8,
	pub postProcessSSAO: u8,
	pub postProcessDOF: u8,
	pub postProcessMotionBlur: u8,
	pub postProcessMotionBlurIteration: u8,
	pub reserve0: [u8;1],
	pub shadowBlur: u8,
	pub sfxParticleHalf: u8,
	pub sfxReflection: u8,
	pub sfxWaterInteraction: u8,
	pub sfxGlow: u8,
	pub sfxDistortion: u8,
	pub sftSoftSprite: u8,
	pub sfxLightShaft: u8,
	pub sfxScaleRenderDistanceScale: u8,
	pub dynamicResolution: u8,
	pub shadowCascade0ResolutionHalf: u8,
	pub shadowCascade1ResolutionHalf: u8,
	pub chrWetDisablePlayer: u8,
	pub chrWetDisableRemotePlayer: u8,
	pub chrWetDisableEnemy: u8,
	pub dynamicResolutionPercentageMin: u8,
	pub dynamicResolutionPercentageMax: u8,
	pub reserve1: [u8;30],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl LOAD_BALANCER_PARAM_ST {
}
impl Default for LOAD_BALANCER_PARAM_ST {
	fn default() -> Self {
		Self {
			 lowerFpsThreshold: 23.,
			 upperFpsThreshold: 27.,
			 lowerFpsContinousCount: 5,
			 upperFpsContinousCount: 20,
			 downAfterChangeSleep: 30,
			 upAfterChangeSleep: 10,
			 postProcessLightShaft: 20,
			 postProcessBloom: 20,
			 postProcessGlow: 20,
			 postProcessAA: 20,
			 postProcessSSAO: 20,
			 postProcessDOF: 20,
			 postProcessMotionBlur: 20,
			 postProcessMotionBlurIteration: 20,
			 reserve0: [0;1],
			 shadowBlur: 20,
			 sfxParticleHalf: 20,
			 sfxReflection: 20,
			 sfxWaterInteraction: 20,
			 sfxGlow: 20,
			 sfxDistortion: 20,
			 sftSoftSprite: 20,
			 sfxLightShaft: 20,
			 sfxScaleRenderDistanceScale: 20,
			 dynamicResolution: 1,
			 shadowCascade0ResolutionHalf: 0,
			 shadowCascade1ResolutionHalf: 13,
			 chrWetDisablePlayer: 21,
			 chrWetDisableRemotePlayer: 21,
			 chrWetDisableEnemy: 21,
			 dynamicResolutionPercentageMin: 100,
			 dynamicResolutionPercentageMax: 100,
			 reserve1: [0;30],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct LOCK_CAM_PARAM_ST {
	pub camDistTarget: f32,
	pub rotRangeMinX: f32,
	pub lockRotXShiftRatio: f32,
	pub chrOrgOffset_Y: f32,
	pub chrLockRangeMaxRadius: f32,
	pub camFovY: f32,
	pub chrLockRangeMaxRadius_forD: f32,
	pub chrLockRangeMaxRadius_forPD: f32,
	pub closeMaxHeight: f32,
	pub closeMinHeight: f32,
	pub closeAngRange: f32,
	pub closeMaxRadius: f32,
	pub closeMaxRadius_forD: f32,
	pub closeMaxRadius_forPD: f32,
	pub bulletMaxRadius: f32,
	pub bulletMaxRadius_forD: f32,
	pub bulletMaxRadius_forPD: f32,
	pub bulletAngRange: f32,
	pub lockTgtKeepTime: f32,
	pub chrTransChaseRateForNormal: f32,
	pub pad: [u8;48],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl LOCK_CAM_PARAM_ST {
}
impl Default for LOCK_CAM_PARAM_ST {
	fn default() -> Self {
		Self {
			 camDistTarget: 4.,
			 rotRangeMinX: -40.,
			 lockRotXShiftRatio: 0.6,
			 chrOrgOffset_Y: 1.42,
			 chrLockRangeMaxRadius: 15.,
			 camFovY: 43.,
			 chrLockRangeMaxRadius_forD: -1.,
			 chrLockRangeMaxRadius_forPD: -1.,
			 closeMaxHeight: 0.,
			 closeMinHeight: 0.,
			 closeAngRange: 0.,
			 closeMaxRadius: 0.,
			 closeMaxRadius_forD: 0.,
			 closeMaxRadius_forPD: 0.,
			 bulletMaxRadius: 0.,
			 bulletMaxRadius_forD: 0.,
			 bulletMaxRadius_forPD: 0.,
			 bulletAngRange: 0.,
			 lockTgtKeepTime: 2.,
			 chrTransChaseRateForNormal: -1.,
			 pad: [0;48],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAGIC_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub yesNoDialogMessageId: i32,
	pub limitCancelSpEffectId: i32,
	pub sortId: i16,
	pub requirementLuck: u8,
	pub aiNotifyType: u8,
	pub mp: i16,
	pub stamina: i16,
	pub iconId: i16,
	pub behaviorId: i16,
	pub mtrlItemId: i16,
	pub replaceMagicId: i16,
	pub maxQuantity: i16,
	pub refCategory1: u8,
	pub overDexterity: u8,
	pub refCategory2: u8,
	pub slotLength: u8,
	pub requirementIntellect: u8,
	pub requirementFaith: u8,
	pub analogDexterityMin: u8,
	pub analogDexterityMax: u8,
	pub ezStateBehaviorType: u8,
	pub refCategory3: u8,
	pub spEffectCategory: u8,
	pub refType: u8,
	pub opmeMenuType: u8,
	pub refCategory4: u8,
	pub hasSpEffectType: i16,
	pub replaceCategory: u8,
	pub useLimitCategory: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	bits_4: u8,
	pub castSfxId: i32,
	pub fireSfxId: i32,
	pub effectSfxId: i32,
	pub toughnessCorrectRate: f32,
	pub ReplacementStatusType: u8,
	pub ReplacementStatus1: i8,
	pub ReplacementStatus2: i8,
	pub ReplacementStatus3: i8,
	pub ReplacementStatus4: i8,
	pub refCategory5: u8,
	pub consumeSA: i16,
	pub ReplacementMagic1: i32,
	pub ReplacementMagic2: i32,
	pub ReplacementMagic3: i32,
	pub ReplacementMagic4: i32,
	pub mp_charge: i16,
	pub stamina_charge: i16,
	pub createLimitGroupId: u8,
	pub refCategory6: u8,
	pub subCategory1: u8,
	pub subCategory2: u8,
	pub refCategory7: u8,
	pub refCategory8: u8,
	pub refCategory9: u8,
	pub refCategory10: u8,
	pub refId1: i32,
	pub refId2: i32,
	pub refId3: i32,
	pub aiUseJudgeId: i32,
	pub refId4: i32,
	pub refId5: i32,
	pub refId6: i32,
	pub refId7: i32,
	pub refId8: i32,
	pub refId9: i32,
	pub refId10: i32,
	pub consumeType1: u8,
	pub consumeType2: u8,
	pub consumeType3: u8,
	pub consumeType4: u8,
	pub consumeType5: u8,
	pub consumeType6: u8,
	pub consumeType7: u8,
	pub consumeType8: u8,
	pub consumeType9: u8,
	pub consumeType10: u8,
	pub consumeLoopMP_forMenu: i16,
	pub pad: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAGIC_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn vowType0(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn vowType1(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn vowType2(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn vowType3(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn vowType4(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn vowType5(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn vowType6(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn vowType7(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn enable_multi(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn enable_multi_only(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn isEnchant(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn isShieldEnchant(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn enable_live(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn enable_gray(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn enable_white(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn enable_black(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn disableOffline(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn castResonanceMagic(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn isValidTough_ProtSADmg(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn isWarpMagic(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn enableRiding(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn disableRiding(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn isUseNoAttackRegion(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn pad_1(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn vowType8(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn vowType9(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn vowType10(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn vowType11(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn vowType12(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn vowType13(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn vowType14(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn vowType15(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
}
impl Default for MAGIC_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 yesNoDialogMessageId: 0,
			 limitCancelSpEffectId: -1,
			 sortId: 0,
			 requirementLuck: 0,
			 aiNotifyType: 0,
			 mp: 0,
			 stamina: 0,
			 iconId: 0,
			 behaviorId: 0,
			 mtrlItemId: -1,
			 replaceMagicId: -1,
			 maxQuantity: 0,
			 refCategory1: 0,
			 overDexterity: 0,
			 refCategory2: 0,
			 slotLength: 0,
			 requirementIntellect: 0,
			 requirementFaith: 0,
			 analogDexterityMin: 0,
			 analogDexterityMax: 0,
			 ezStateBehaviorType: 0,
			 refCategory3: 0,
			 spEffectCategory: 0,
			 refType: 0,
			 opmeMenuType: 0,
			 refCategory4: 0,
			 hasSpEffectType: 0,
			 replaceCategory: 0,
			 useLimitCategory: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 bits_4: 0,
			 castSfxId: -1,
			 fireSfxId: -1,
			 effectSfxId: -1,
			 toughnessCorrectRate: 0.,
			 ReplacementStatusType: 0,
			 ReplacementStatus1: -1,
			 ReplacementStatus2: -1,
			 ReplacementStatus3: -1,
			 ReplacementStatus4: -1,
			 refCategory5: 0,
			 consumeSA: 0,
			 ReplacementMagic1: -1,
			 ReplacementMagic2: -1,
			 ReplacementMagic3: -1,
			 ReplacementMagic4: -1,
			 mp_charge: 0,
			 stamina_charge: 0,
			 createLimitGroupId: 0,
			 refCategory6: 0,
			 subCategory1: 0,
			 subCategory2: 0,
			 refCategory7: 0,
			 refCategory8: 0,
			 refCategory9: 0,
			 refCategory10: 0,
			 refId1: -1,
			 refId2: -1,
			 refId3: -1,
			 aiUseJudgeId: -1,
			 refId4: -1,
			 refId5: -1,
			 refId6: -1,
			 refId7: -1,
			 refId8: -1,
			 refId9: -1,
			 refId10: -1,
			 consumeType1: 0,
			 consumeType2: 0,
			 consumeType3: 0,
			 consumeType4: 0,
			 consumeType5: 0,
			 consumeType6: 0,
			 consumeType7: 0,
			 consumeType8: 0,
			 consumeType9: 0,
			 consumeType10: 0,
			 consumeLoopMP_forMenu: -1,
			 pad: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_DEFAULT_INFO_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub EnableFastTravelEventFlagId: i32,
	pub WeatherLotTimeOffsetIngameSeconds: i32,
	pub WeatherCreateAssetLimitId: i8,
	pub MapAiSightType: u8,
	pub SoundIndoorType: u8,
	pub ReverbDefaultType: i8,
	pub BgmPlaceInfo: i16,
	pub EnvPlaceInfo: i16,
	pub MapAdditionalSoundBankId: i32,
	pub MapHeightForSound: i16,
	pub IsEnableBlendTimezoneEnvmap: u8,
	pub OverrideGIResolution_XSS: i8,
	pub MapLoHiChangeBorderDist_XZ: f32,
	pub MapLoHiChangeBorderDist_Y: f32,
	pub MapLoHiChangePlayDist: f32,
	pub MapAutoDrawGroupBackFacePixelNum: i32,
	pub PlayerLigntScale: f32,
	pub IsEnableTimezonnePlayerLigntScale: u8,
	pub isDisableAutoCliffWind: u8,
	pub OpenChrActivateThreshold: i16,
	pub MapMimicryEstablishmentParamId: i32,
	pub OverrideGIResolution_XSX: i8,
	pub Reserve: [u8;7],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_DEFAULT_INFO_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MAP_DEFAULT_INFO_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 EnableFastTravelEventFlagId: 0,
			 WeatherLotTimeOffsetIngameSeconds: 0,
			 WeatherCreateAssetLimitId: -1,
			 MapAiSightType: 0,
			 SoundIndoorType: 0,
			 ReverbDefaultType: -1,
			 BgmPlaceInfo: 0,
			 EnvPlaceInfo: 0,
			 MapAdditionalSoundBankId: -1,
			 MapHeightForSound: 0,
			 IsEnableBlendTimezoneEnvmap: 1,
			 OverrideGIResolution_XSS: -1,
			 MapLoHiChangeBorderDist_XZ: 40.,
			 MapLoHiChangeBorderDist_Y: 40.,
			 MapLoHiChangePlayDist: 5.,
			 MapAutoDrawGroupBackFacePixelNum: 32400,
			 PlayerLigntScale: 1.,
			 IsEnableTimezonnePlayerLigntScale: 1,
			 isDisableAutoCliffWind: 0,
			 OpenChrActivateThreshold: -1,
			 MapMimicryEstablishmentParamId: -1,
			 OverrideGIResolution_XSX: -1,
			 Reserve: [0;7],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_GD_REGION_DRAW_PARAM {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub overrideIVLocalLightScale: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_GD_REGION_DRAW_PARAM {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MAP_GD_REGION_DRAW_PARAM {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 overrideIVLocalLightScale: -1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_GD_REGION_ID_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub mapRegionId: i32,
	pub Reserve: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_GD_REGION_ID_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MAP_GD_REGION_ID_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 mapRegionId: 0,
			 Reserve: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {
	pub GridEnableCreateHeightMin: f32,
	pub GridEnableCreateHeightMax: f32,
	pub Reserve: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {
}
impl Default for MAP_GRID_CREATE_HEIGHT_LIMIT_INFO_PARAM_ST {
	fn default() -> Self {
		Self {
			 GridEnableCreateHeightMin: -99999.,
			 GridEnableCreateHeightMax: 99999.,
			 Reserve: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {
	pub mimicryEstablishment0: f32,
	pub mimicryEstablishment1: f32,
	pub mimicryEstablishment2: f32,
	pub mimicryBeginSfxId0: i32,
	pub mimicrySfxId0: i32,
	pub mimicryEndSfxId0: i32,
	pub mimicryBeginSfxId1: i32,
	pub mimicrySfxId1: i32,
	pub mimicryEndSfxId1: i32,
	pub mimicryBeginSfxId2: i32,
	pub mimicrySfxId2: i32,
	pub mimicryEndSfxId2: i32,
	pub pad1: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {
}
impl Default for MAP_MIMICRY_ESTABLISHMENT_PARAM_ST {
	fn default() -> Self {
		Self {
			 mimicryEstablishment0: -1.,
			 mimicryEstablishment1: -1.,
			 mimicryEstablishment2: -1.,
			 mimicryBeginSfxId0: -1,
			 mimicrySfxId0: -1,
			 mimicryEndSfxId0: -1,
			 mimicryBeginSfxId1: -1,
			 mimicrySfxId1: -1,
			 mimicryEndSfxId1: -1,
			 mimicryBeginSfxId2: -1,
			 mimicrySfxId2: -1,
			 mimicryEndSfxId2: -1,
			 pad1: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_NAME_TEX_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub srcR: u8,
	pub srcG: u8,
	pub srcB: u8,
	pub pad1: [u8;1],
	pub mapNameId: i32,
	pub pad2: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_NAME_TEX_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MAP_NAME_TEX_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 srcR: 0,
			 srcG: 0,
			 srcB: 0,
			 pad1: [0;1],
			 mapNameId: 0,
			 pad2: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MAP_PIECE_TEX_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub srcR: u8,
	pub srcG: u8,
	pub srcB: u8,
	pub pad1: [u8;1],
	pub saveMapNameId: i32,
	pub multiPlayAreaId: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MAP_PIECE_TEX_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MAP_PIECE_TEX_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 srcR: 0,
			 srcG: 0,
			 srcB: 0,
			 pad1: [0;1],
			 saveMapNameId: -1,
			 multiPlayAreaId: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MATERIAL_EX_PARAM_ST {
	pub paramName: [u8;32],
	pub materialId: i32,
	pub materialParamValue0: f32,
	pub materialParamValue1: f32,
	pub materialParamValue2: f32,
	pub materialParamValue3: f32,
	pub materialParamValue4: f32,
	pub pad: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MATERIAL_EX_PARAM_ST {
}
impl Default for MATERIAL_EX_PARAM_ST {
	fn default() -> Self {
		Self {
			 paramName: [0;32],
			 materialId: -1,
			 materialParamValue0: 0.,
			 materialParamValue1: 0.,
			 materialParamValue2: 0.,
			 materialParamValue3: 0.,
			 materialParamValue4: 1.,
			 pad: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MENU_COMMON_PARAM_ST {
	pub soloPlayDeath_ToFadeOutTime: f32,
	pub partyGhostDeath_ToFadeOutTime: f32,
	pub playerMaxHpLimit: i32,
	pub playerMaxMpLimit: i32,
	pub playerMaxSpLimit: i32,
	pub actionPanelChangeThreshold_Vel: f32,
	pub actionPanelChangeThreshold_PassTime: f32,
	pub kgIconVspace: i32,
	pub worldMapCursorSelectRadius: f32,
	pub reserved8: [u8;4],
	pub decalPosOffsetX: i32,
	pub decalPosOffsetY: i32,
	pub targetStateSearchDurationTime: f32,
	pub targetStateBattleDurationTime: f32,
	pub worldMapCursorSpeed: f32,
	pub worldMapCursorFirstDistance: f32,
	pub worldMapCursorFirstDelay: f32,
	pub worldMapCursorWaitTime: f32,
	pub worldMapCursorSnapRadius: f32,
	pub worldMapCursorSnapTime: f32,
	pub itemGetLogAliveTime: f32,
	pub playerMaxSaLimit: i32,
	pub worldMap_IsChangeableLayerEventFlagId: i32,
	pub worldMap_TravelMargin: f32,
	pub systemAnnounceScrollBufferTime: f32,
	pub systemAnnounceScrollSpeed: i32,
	pub systemAnnounceNoScrollWaitTime: f32,
	pub systemAnnounceScrollCount: u8,
	pub reserved17: [u8;3],
	pub compassMemoDispDistance: f32,
	pub compassBonfireDispDistance: f32,
	pub markerGoalThreshold: f32,
	pub svSliderStep: f32,
	pub preOpeningMovie_WaitSec: f32,
	pub kgIconScale: f32,
	pub kgIconScale_forTable: f32,
	pub kgIconVspace_forTable: i32,
	pub kgIconScale_forConfig: f32,
	pub kgIconVspace_forConfig: i32,
	pub worldMap_SearchRadius: f32,
	pub tutorialDisplayTime: f32,
	pub compassFriendHostInnerDistance: f32,
	pub compassEnemyHostInnerDistance: f32,
	pub compassFriendGuestInnerDistance: f32,
	pub cutsceneKeyGuideAliveTime: f32,
	pub autoHideHpThresholdRatio: f32,
	pub autoHideHpThresholdValue: i32,
	pub autoHideMpThresholdRatio: f32,
	pub autoHideMpThresholdValue: i32,
	pub autoHideSpThresholdRatio: f32,
	pub autoHideSpThresholdValue: i32,
	pub worldMapZoomAnimationTime: f32,
	pub worldMapIconScaleMin: f32,
	pub worldMap_TravelMargin_Point: f32,
	pub enemyTagSafeLeft: i16,
	pub enemyTagSafeRight: i16,
	pub enemyTagSafeTop: i16,
	pub enemyTagSafeBottom: i16,
	pub pcHorseHpRecoverDispThreshold: i32,
	pub reserved33: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MENU_COMMON_PARAM_ST {
}
impl Default for MENU_COMMON_PARAM_ST {
	fn default() -> Self {
		Self {
			 soloPlayDeath_ToFadeOutTime: 0.,
			 partyGhostDeath_ToFadeOutTime: 0.,
			 playerMaxHpLimit: 0,
			 playerMaxMpLimit: 0,
			 playerMaxSpLimit: 0,
			 actionPanelChangeThreshold_Vel: 0.,
			 actionPanelChangeThreshold_PassTime: 0.,
			 kgIconVspace: 0,
			 worldMapCursorSelectRadius: 0.1,
			 reserved8: [0;4],
			 decalPosOffsetX: 0,
			 decalPosOffsetY: 0,
			 targetStateSearchDurationTime: 0.,
			 targetStateBattleDurationTime: 0.,
			 worldMapCursorSpeed: 1.,
			 worldMapCursorFirstDistance: 1.,
			 worldMapCursorFirstDelay: 0.01,
			 worldMapCursorWaitTime: 0.,
			 worldMapCursorSnapRadius: 0.1,
			 worldMapCursorSnapTime: 0.01,
			 itemGetLogAliveTime: 0.01,
			 playerMaxSaLimit: 0,
			 worldMap_IsChangeableLayerEventFlagId: 0,
			 worldMap_TravelMargin: 0.,
			 systemAnnounceScrollBufferTime: 0.,
			 systemAnnounceScrollSpeed: 100,
			 systemAnnounceNoScrollWaitTime: 0.,
			 systemAnnounceScrollCount: 1,
			 reserved17: [0;3],
			 compassMemoDispDistance: 50.,
			 compassBonfireDispDistance: 50.,
			 markerGoalThreshold: 0.,
			 svSliderStep: 10.,
			 preOpeningMovie_WaitSec: 0.,
			 kgIconScale: 100.,
			 kgIconScale_forTable: 100.,
			 kgIconVspace_forTable: 0,
			 kgIconScale_forConfig: 100.,
			 kgIconVspace_forConfig: 0,
			 worldMap_SearchRadius: 256.,
			 tutorialDisplayTime: 3.,
			 compassFriendHostInnerDistance: 0.,
			 compassEnemyHostInnerDistance: 0.,
			 compassFriendGuestInnerDistance: 0.,
			 cutsceneKeyGuideAliveTime: 5.,
			 autoHideHpThresholdRatio: -1.,
			 autoHideHpThresholdValue: -1,
			 autoHideMpThresholdRatio: -1.,
			 autoHideMpThresholdValue: -1,
			 autoHideSpThresholdRatio: -1.,
			 autoHideSpThresholdValue: -1,
			 worldMapZoomAnimationTime: 0.5,
			 worldMapIconScaleMin: 1.,
			 worldMap_TravelMargin_Point: 0.,
			 enemyTagSafeLeft: 0,
			 enemyTagSafeRight: 1920,
			 enemyTagSafeTop: 0,
			 enemyTagSafeBottom: 1080,
			 pcHorseHpRecoverDispThreshold: 0,
			 reserved33: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MENU_OFFSCR_REND_PARAM_ST {
	pub camAtPosX: f32,
	pub camAtPosY: f32,
	pub camAtPosZ: f32,
	pub camDist: f32,
	pub camRotX: f32,
	pub camRotY: f32,
	pub camFov: f32,
	pub camDistMin: f32,
	pub camDistMax: f32,
	pub camRotXMin: f32,
	pub camRotXMax: f32,
	pub GparamID: i32,
	pub envTexId: i32,
	pub Grapm_ID_forPS4: i32,
	pub Grapm_ID_forXB1: i32,
	pub pad: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MENU_OFFSCR_REND_PARAM_ST {
}
impl Default for MENU_OFFSCR_REND_PARAM_ST {
	fn default() -> Self {
		Self {
			 camAtPosX: 0.,
			 camAtPosY: 0.,
			 camAtPosZ: 0.,
			 camDist: 10.,
			 camRotX: 0.,
			 camRotY: 0.,
			 camFov: 49.,
			 camDistMin: 0.,
			 camDistMax: 100.,
			 camRotXMin: -89.,
			 camRotXMax: 89.,
			 GparamID: 10,
			 envTexId: 10,
			 Grapm_ID_forPS4: 10,
			 Grapm_ID_forXB1: 10,
			 pad: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MENU_PARAM_COLOR_TABLE_ST {
	pub lerpMode: u8,
	pub pad1: [u8;3],
	pub h: i16,
	pub pad2: [u8;2],
	pub s1: f32,
	pub v1: f32,
	pub s2: f32,
	pub v2: f32,
	pub s3: f32,
	pub v3: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MENU_PARAM_COLOR_TABLE_ST {
}
impl Default for MENU_PARAM_COLOR_TABLE_ST {
	fn default() -> Self {
		Self {
			 lerpMode: 0,
			 pad1: [0;3],
			 h: 0,
			 pad2: [0;2],
			 s1: 1.,
			 v1: 1.,
			 s2: 1.,
			 v2: 1.,
			 s3: 1.,
			 v3: 1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MENUPROPERTY_LAYOUT {
	pub LayoutPath: [u8;16],
	pub PropertyID: i32,
	pub CaptionTextID: i32,
	pub HelpTextID: i32,
	pub reserved: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MENUPROPERTY_LAYOUT {
}
impl Default for MENUPROPERTY_LAYOUT {
	fn default() -> Self {
		Self {
			 LayoutPath: [0;16],
			 PropertyID: 0,
			 CaptionTextID: 0,
			 HelpTextID: 0,
			 reserved: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MENUPROPERTY_SPEC {
	pub CaptionTextID: i32,
	pub IconID: i32,
	pub RequiredPropertyID: i32,
	pub CompareType: i8,
	pub pad2: [u8;1],
	pub FormatType: i16,
	pub pad: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MENUPROPERTY_SPEC {
}
impl Default for MENUPROPERTY_SPEC {
	fn default() -> Self {
		Self {
			 CaptionTextID: 0,
			 IconID: 0,
			 RequiredPropertyID: 0,
			 CompareType: 0,
			 pad2: [0;1],
			 FormatType: 0,
			 pad: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MENU_VALUE_TABLE_SPEC {
	pub value: i32,
	pub textId: i32,
	pub compareType: i8,
	pub padding: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MENU_VALUE_TABLE_SPEC {
}
impl Default for MENU_VALUE_TABLE_SPEC {
	fn default() -> Self {
		Self {
			 value: 0,
			 textId: 0,
			 compareType: 0,
			 padding: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MIMICRY_ESTABLISHMENT_TEX_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub srcR: u8,
	pub srcG: u8,
	pub srcB: u8,
	pub pad1: [u8;1],
	pub mimicryEstablishmentParamId: i32,
	pub pad2: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MIMICRY_ESTABLISHMENT_TEX_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MIMICRY_ESTABLISHMENT_TEX_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 srcR: 0,
			 srcG: 0,
			 srcB: 0,
			 pad1: [0;1],
			 mimicryEstablishmentParamId: -1,
			 pad2: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MISSILE_PARAM_ST {
	pub FFXID: i32,
	pub LifeTime: i16,
	pub HitSphereRadius: i16,
	pub HitDamage: i16,
	pub reserve0: [u8;6],
	pub InitVelocity: f32,
	pub distance: f32,
	pub gravityInRange: f32,
	pub gravityOutRange: f32,
	pub mp: i32,
	pub accelInRange: f32,
	pub accelOutRange: f32,
	pub reserve1: [u8;20],
	pub HitMissileID: i16,
	pub DiedNaturaly: u8,
	pub ExplosionDie: u8,
	pub behaviorId: i32,
	pub reserve_last: [u8;56],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MISSILE_PARAM_ST {
}
impl Default for MISSILE_PARAM_ST {
	fn default() -> Self {
		Self {
			 FFXID: 0,
			 LifeTime: 0,
			 HitSphereRadius: 0,
			 HitDamage: 0,
			 reserve0: [0;6],
			 InitVelocity: 0.,
			 distance: 0.,
			 gravityInRange: 0.,
			 gravityOutRange: 0.,
			 mp: 0,
			 accelInRange: 0.,
			 accelOutRange: 0.,
			 reserve1: [0;20],
			 HitMissileID: 0,
			 DiedNaturaly: 0,
			 ExplosionDie: 0,
			 behaviorId: 0,
			 reserve_last: [0;56],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MODEL_SFX_PARAM_ST {
	pub sfxId_0: i32,
	pub dmypolyId_0: i32,
	pub reserve_0: [u8;8],
	pub sfxId_1: i32,
	pub dmypolyId_1: i32,
	pub reserve_1: [u8;8],
	pub sfxId_2: i32,
	pub dmypolyId_2: i32,
	pub reserve_2: [u8;8],
	pub sfxId_3: i32,
	pub dmypolyId_3: i32,
	pub reserve_3: [u8;8],
	pub sfxId_4: i32,
	pub dmypolyId_4: i32,
	pub reserve_4: [u8;8],
	pub sfxId_5: i32,
	pub dmypolyId_5: i32,
	pub reserve_5: [u8;8],
	pub sfxId_6: i32,
	pub dmypolyId_6: i32,
	pub reserve_6: [u8;8],
	pub sfxId_7: i32,
	pub dmypolyId_7: i32,
	pub reserve_7: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MODEL_SFX_PARAM_ST {
}
impl Default for MODEL_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 sfxId_0: -1,
			 dmypolyId_0: -1,
			 reserve_0: [0;8],
			 sfxId_1: -1,
			 dmypolyId_1: -1,
			 reserve_1: [0;8],
			 sfxId_2: -1,
			 dmypolyId_2: -1,
			 reserve_2: [0;8],
			 sfxId_3: -1,
			 dmypolyId_3: -1,
			 reserve_3: [0;8],
			 sfxId_4: -1,
			 dmypolyId_4: -1,
			 reserve_4: [0;8],
			 sfxId_5: -1,
			 dmypolyId_5: -1,
			 reserve_5: [0;8],
			 sfxId_6: -1,
			 dmypolyId_6: -1,
			 reserve_6: [0;8],
			 sfxId_7: -1,
			 dmypolyId_7: -1,
			 reserve_7: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MOVE_PARAM_ST {
	pub stayId: i32,
	pub walkF: i32,
	pub walkB: i32,
	pub walkL: i32,
	pub walkR: i32,
	pub dashF: i32,
	pub dashB: i32,
	pub dashL: i32,
	pub dashR: i32,
	pub superDash: i32,
	pub escapeF: i32,
	pub escapeB: i32,
	pub escapeL: i32,
	pub escapeR: i32,
	pub turnL: i32,
	pub trunR: i32,
	pub largeTurnL: i32,
	pub largeTurnR: i32,
	pub stepMove: i32,
	pub flyStay: i32,
	pub flyWalkF: i32,
	pub flyWalkFL: i32,
	pub flyWalkFR: i32,
	pub flyWalkFL2: i32,
	pub flyWalkFR2: i32,
	pub flyDashF: i32,
	pub flyDashFL: i32,
	pub flyDashFR: i32,
	pub flyDashFL2: i32,
	pub flyDashFR2: i32,
	pub dashEscapeF: i32,
	pub dashEscapeB: i32,
	pub dashEscapeL: i32,
	pub dashEscapeR: i32,
	pub analogMoveParamId: i32,
	pub turnNoAnimAngle: u8,
	pub turn45Angle: u8,
	pub turn90Angle: u8,
	pub turnWaitNoAnimAngle: u8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MOVE_PARAM_ST {
}
impl Default for MOVE_PARAM_ST {
	fn default() -> Self {
		Self {
			 stayId: -1,
			 walkF: -1,
			 walkB: -1,
			 walkL: -1,
			 walkR: -1,
			 dashF: -1,
			 dashB: -1,
			 dashL: -1,
			 dashR: -1,
			 superDash: -1,
			 escapeF: -1,
			 escapeB: -1,
			 escapeL: -1,
			 escapeR: -1,
			 turnL: -1,
			 trunR: -1,
			 largeTurnL: -1,
			 largeTurnR: -1,
			 stepMove: -1,
			 flyStay: -1,
			 flyWalkF: -1,
			 flyWalkFL: -1,
			 flyWalkFR: -1,
			 flyWalkFL2: -1,
			 flyWalkFR2: -1,
			 flyDashF: -1,
			 flyDashFL: -1,
			 flyDashFR: -1,
			 flyDashFL2: -1,
			 flyDashFR2: -1,
			 dashEscapeF: -1,
			 dashEscapeB: -1,
			 dashEscapeL: -1,
			 dashEscapeR: -1,
			 analogMoveParamId: -1,
			 turnNoAnimAngle: 0,
			 turn45Angle: 0,
			 turn90Angle: 0,
			 turnWaitNoAnimAngle: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MULTI_ESTUS_FLASK_BONUS_PARAM_ST {
	pub host: u8,
	pub WhiteGhost_None: u8,
	pub WhiteGhost_Umbasa: u8,
	pub WhiteGhost_Berserker: u8,
	pub BlackGhost_None_Sign: u8,
	pub BlackGhost_Umbasa_Sign: u8,
	pub BlackGhost_Berserker_Sign: u8,
	pub BlackGhost_None_Invade: u8,
	pub BlackGhost_Umbasa_Invade: u8,
	pub BlackGhost_Berserker_Invade: u8,
	pub RedHunter1: u8,
	pub RedHunter2: u8,
	pub GuardianOfForest: u8,
	pub GuardianOfAnor: u8,
	pub BattleRoyal: u8,
	pub YellowMonk: u8,
	pub pad1: [u8;48],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MULTI_ESTUS_FLASK_BONUS_PARAM_ST {
}
impl Default for MULTI_ESTUS_FLASK_BONUS_PARAM_ST {
	fn default() -> Self {
		Self {
			 host: 0,
			 WhiteGhost_None: 0,
			 WhiteGhost_Umbasa: 0,
			 WhiteGhost_Berserker: 0,
			 BlackGhost_None_Sign: 0,
			 BlackGhost_Umbasa_Sign: 0,
			 BlackGhost_Berserker_Sign: 0,
			 BlackGhost_None_Invade: 0,
			 BlackGhost_Umbasa_Invade: 0,
			 BlackGhost_Berserker_Invade: 0,
			 RedHunter1: 0,
			 RedHunter2: 0,
			 GuardianOfForest: 0,
			 GuardianOfAnor: 0,
			 BattleRoyal: 0,
			 YellowMonk: 0,
			 pad1: [0;48],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MULTI_PLAY_CORRECTION_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub client1SpEffectId: i32,
	pub client2SpEffectId: i32,
	pub client3SpEffectId: i32,
	pub bOverrideSpEffect: u8,
	pub pad3: [u8;15],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MULTI_PLAY_CORRECTION_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for MULTI_PLAY_CORRECTION_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 client1SpEffectId: -1,
			 client2SpEffectId: -1,
			 client3SpEffectId: -1,
			 bOverrideSpEffect: 0,
			 pad3: [0;15],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct MULTI_SOUL_BONUS_RATE_PARAM_ST {
	pub host: f32,
	pub WhiteGhost_None: f32,
	pub WhiteGhost_Umbasa: f32,
	pub WhiteGhost_Berserker: f32,
	pub BlackGhost_None_Sign: f32,
	pub BlackGhost_Umbasa_Sign: f32,
	pub BlackGhost_Berserker_Sign: f32,
	pub BlackGhost_None_Invade: f32,
	pub BlackGhost_Umbasa_Invade: f32,
	pub BlackGhost_Berserker_Invade: f32,
	pub RedHunter1: f32,
	pub RedHunter2: f32,
	pub GuardianOfForest: f32,
	pub GuardianOfAnor: f32,
	pub BattleRoyal: f32,
	pub YellowMonk: f32,
	pub pad1: [u8;64],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl MULTI_SOUL_BONUS_RATE_PARAM_ST {
}
impl Default for MULTI_SOUL_BONUS_RATE_PARAM_ST {
	fn default() -> Self {
		Self {
			 host: 0.,
			 WhiteGhost_None: 0.,
			 WhiteGhost_Umbasa: 0.,
			 WhiteGhost_Berserker: 0.,
			 BlackGhost_None_Sign: 0.,
			 BlackGhost_Umbasa_Sign: 0.,
			 BlackGhost_Berserker_Sign: 0.,
			 BlackGhost_None_Invade: 0.,
			 BlackGhost_Umbasa_Invade: 0.,
			 BlackGhost_Berserker_Invade: 0.,
			 RedHunter1: 0.,
			 RedHunter2: 0.,
			 GuardianOfForest: 0.,
			 GuardianOfAnor: 0.,
			 BattleRoyal: 0.,
			 YellowMonk: 0.,
			 pad1: [0;64],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NETWORK_AREA_PARAM_ST {
	pub cellSizeX: f32,
	pub cellSizeY: f32,
	pub cellSizeZ: f32,
	pub cellOffsetX: f32,
	pub cellOffsetY: f32,
	pub cellOffsetZ: f32,
	bits_0: u8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NETWORK_AREA_PARAM_ST {
	pub fn enableBloodstain(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn enableBloodMessage(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn enableGhost(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn enableMultiPlay(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn enableRingSearch(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn enableBreakInSearch(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
}
impl Default for NETWORK_AREA_PARAM_ST {
	fn default() -> Self {
		Self {
			 cellSizeX: 30.,
			 cellSizeY: 8.,
			 cellSizeZ: 30.,
			 cellOffsetX: 0.,
			 cellOffsetY: 0.,
			 cellOffsetZ: 0.,
			 bits_0: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NETWORK_MSG_PARAM_ST {
	pub priority: i16,
	pub forcePlay: u8,
	pub pad1: [u8;1],
	pub normalWhite: i32,
	pub umbasaWhite: i32,
	pub berserkerWhite: i32,
	pub sinnerHeroWhite: i32,
	pub normalBlack: i32,
	pub umbasaBlack: i32,
	pub berserkerBlack: i32,
	pub forceJoinBlack: i32,
	pub forceJoinUmbasaBlack: i32,
	pub forceJoinBerserkerBlack: i32,
	pub sinnerHunterVisitor: i32,
	pub redHunterVisitor: i32,
	pub guardianOfBossVisitor: i32,
	pub guardianOfForestMapVisitor: i32,
	pub guardianOfAnolisVisitor: i32,
	pub rosaliaBlack: i32,
	pub forceJoinRosaliaBlack: i32,
	pub redHunterVisitor2: i32,
	pub npc1: i32,
	pub npc2: i32,
	pub npc3: i32,
	pub npc4: i32,
	pub battleRoyal: i32,
	pub npc5: i32,
	pub npc6: i32,
	pub npc7: i32,
	pub npc8: i32,
	pub npc9: i32,
	pub npc10: i32,
	pub npc11: i32,
	pub npc12: i32,
	pub npc13: i32,
	pub npc14: i32,
	pub npc15: i32,
	pub npc16: i32,
	pub forceJoinBlack_B: i32,
	pub normalWhite_Npc: i32,
	pub forceJoinBlack_Npc: i32,
	pub forceJoinBlack_B_Npc: i32,
	pub forceJoinBlack_C_Npc: i32,
	pub pad2: [u8;28],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NETWORK_MSG_PARAM_ST {
}
impl Default for NETWORK_MSG_PARAM_ST {
	fn default() -> Self {
		Self {
			 priority: 0,
			 forcePlay: 0,
			 pad1: [0;1],
			 normalWhite: -1,
			 umbasaWhite: -1,
			 berserkerWhite: -1,
			 sinnerHeroWhite: -1,
			 normalBlack: -1,
			 umbasaBlack: -1,
			 berserkerBlack: -1,
			 forceJoinBlack: -1,
			 forceJoinUmbasaBlack: -1,
			 forceJoinBerserkerBlack: -1,
			 sinnerHunterVisitor: -1,
			 redHunterVisitor: -1,
			 guardianOfBossVisitor: -1,
			 guardianOfForestMapVisitor: -1,
			 guardianOfAnolisVisitor: -1,
			 rosaliaBlack: -1,
			 forceJoinRosaliaBlack: -1,
			 redHunterVisitor2: -1,
			 npc1: -1,
			 npc2: -1,
			 npc3: -1,
			 npc4: -1,
			 battleRoyal: -1,
			 npc5: -1,
			 npc6: -1,
			 npc7: -1,
			 npc8: -1,
			 npc9: -1,
			 npc10: -1,
			 npc11: -1,
			 npc12: -1,
			 npc13: -1,
			 npc14: -1,
			 npc15: -1,
			 npc16: -1,
			 forceJoinBlack_B: -1,
			 normalWhite_Npc: -1,
			 forceJoinBlack_Npc: -1,
			 forceJoinBlack_B_Npc: -1,
			 forceJoinBlack_C_Npc: -1,
			 pad2: [0;28],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NETWORK_PARAM_ST {
	pub signVerticalOffset: f32,
	pub maxSignPosCorrectionRange: f32,
	pub summonTimeoutTime: f32,
	pub pad_0: [u8;4],
	pub signPuddleActiveMessageIntervalSec: f32,
	pub keyGuideHeight_0: f32,
	pub reloadSignIntervalTime1: f32,
	pub reloadSignIntervalTime2: f32,
	pub reloadSignTotalCount_0: i32,
	pub reloadSignCellCount_0: i32,
	pub updateSignIntervalTime: f32,
	pub basicExclusiveRange_0: f32,
	pub basicExclusiveHeight_0: f32,
	pub previewChrWaitingTime: f32,
	pub signVisibleRange_0: f32,
	pub cellGroupHorizontalRange_0: i32,
	pub cellGroupTopRange_0: i32,
	pub cellGroupBottomRange_0: i32,
	pub minWhitePhantomLimitTimeScale: f32,
	pub minSmallPhantomLimitTimeScale: f32,
	pub whiteKeywordLimitTimeScale: f32,
	pub smallKeywordLimitTimeScale: f32,
	pub blackKeywordLimitTimeScale: f32,
	pub dragonKeywordLimitTimeScale: f32,
	pub singGetMax: i32,
	pub signDownloadSpan: f32,
	pub signUpdateSpan: f32,
	pub signPad: [u8;4],
	pub maxBreakInTargetListCount: i32,
	pub breakInRequestIntervalTimeSec: f32,
	pub breakInRequestTimeOutSec: f32,
	pub pad_1: [u8;4],
	pub keyGuideRange: f32,
	pub keyGuideHeight_1: f32,
	pub reloadSignTotalCount_1: i32,
	pub reloadNewSignCellCount: i32,
	pub reloadRandomSignCellCount: i32,
	pub maxSignTotalCount_0: i32,
	pub maxSignCellCount_0: i32,
	pub basicExclusiveRange_1: f32,
	pub basicExclusiveHeight_1: f32,
	pub signVisibleRange_1: f32,
	pub maxWriteSignCount: i32,
	pub maxReadSignCount: i32,
	pub reloadSignIntervalTime_0: f32,
	pub cellGroupHorizontalRange_1: i32,
	pub cellGroupTopRange_1: i32,
	pub cellGroupBottomRange_1: i32,
	pub lifeTime_0: i32,
	pub downloadSpan_0: f32,
	pub downloadEvaluationSpan: f32,
	pub pad_2: [u8;4],
	pub deadingGhostStartPosThreshold: f32,
	pub keyGuideHeight_2: f32,
	pub keyGuideRangePlayer: f32,
	pub keyGuideHeightPlayer: f32,
	pub reloadSignTotalCount_2: i32,
	pub reloadSignCellCount_1: i32,
	pub maxSignTotalCount_1: i32,
	pub maxSignCellCount_1: i32,
	pub reloadSignIntervalTime_1: f32,
	pub signVisibleRange_2: f32,
	pub basicExclusiveRange_2: f32,
	pub basicExclusiveHeight_2: f32,
	pub cellGroupHorizontalRange_2: i32,
	pub cellGroupTopRange_2: i32,
	pub cellGroupBottomRange_2: i32,
	pub lifeTime_1: i32,
	pub recordDeadingGhostTotalTime: f32,
	pub recordDeadingGhostMinTime: f32,
	pub downloadSpan_1: f32,
	pub statueCreatableDistance: f32,
	pub reloadGhostTotalCount: i32,
	pub reloadGhostCellCount: i32,
	pub maxGhostTotalCount: i32,
	pub distanceOfBeginRecordVersus: f32,
	pub distanceOfEndRecordVersus: f32,
	pub updateWanderGhostIntervalTime: f32,
	pub updateVersusGhostIntervalTime: f32,
	pub recordWanderingGhostTime: f32,
	pub recordWanderingGhostMinTime: f32,
	pub updateBonfireGhostIntervalTime: f32,
	pub replayGhostRangeOnView: f32,
	pub replayGhostRangeOutView: f32,
	pub replayBonfireGhostTime: f32,
	pub minBonfireGhostValidRange: f32,
	pub maxBonfireGhostValidRange: f32,
	pub minReplayIntervalTime: f32,
	pub maxReplayIntervalTime: f32,
	pub reloadGhostIntervalTime: f32,
	pub cellGroupHorizontalRange_3: i32,
	pub cellGroupTopRange_3: i32,
	pub replayBonfirePhantomParamIdForCodename: i32,
	pub replayBonfireModeRange: f32,
	pub replayBonfirePhantomParamId: i32,
	pub ghostpad: [u8;4],
	pub reloadVisitListCoolTime: f32,
	pub maxCoopBlueSummonCount: i32,
	pub maxBellGuardSummonCount: i32,
	pub maxVisitListCount: i32,
	pub reloadSearch_CoopBlue_Min: f32,
	pub reloadSearch_CoopBlue_Max: f32,
	pub reloadSearch_BellGuard_Min: f32,
	pub reloadSearch_BellGuard_Max: f32,
	pub reloadSearch_RatKing_Min: f32,
	pub reloadSearch_RatKing_Max: f32,
	pub visitpad00: [u8;8],
	pub srttMaxLimit: f32,
	pub srttMeanLimit: f32,
	pub srttMeanDeviationLimit: f32,
	pub darkPhantomLimitBoostTime: f32,
	pub darkPhantomLimitBoostScale: f32,
	pub multiplayDisableLifeTime: f32,
	pub abyssMultiplayLimit: u8,
	pub phantomWarpMinimumTime: u8,
	pub phantomReturnDelayTime: u8,
	pub terminateTimeoutTime: u8,
	pub penaltyPointLanDisconnect: i16,
	pub penaltyPointSignout: i16,
	pub penaltyPointReboot: i16,
	pub penaltyPointBeginPenalize: i16,
	pub penaltyForgiveItemLimitTime: f32,
	pub allAreaSearchRate_CoopBlue: u8,
	pub allAreaSearchRate_VsBlue: u8,
	pub allAreaSearchRate_BellGuard: u8,
	pub bloodMessageEvalHealRate: u8,
	pub smallGoldSuccessHostRewardId: i32,
	pub doorInvalidPlayAreaExtents: f32,
	pub signDisplayMax: u8,
	pub bloodStainDisplayMax: u8,
	pub bloodMessageDisplayMax: u8,
	pub pad00: [u8;9],
	pub pad10: [u8;32],
	pub summonMessageInterval: f32,
	pub hostRegisterUpdateTime: f32,
	pub hostTimeOutTime: f32,
	pub guestUpdateTime: f32,
	pub guestPlayerNoTimeOutTime: f32,
	pub hostPlayerNoTimeOutTime: f32,
	pub requestSearchQuickMatchLimit: i32,
	pub AvatarMatchSearchMax: i32,
	pub BattleRoyalMatchSearchMin: i32,
	pub BattleRoyalMatchSearchMax: i32,
	pub pad11: [u8;8],
	pub VisitorListMax: i32,
	pub VisitorTimeOutTime: f32,
	pub DownloadSpan_2: f32,
	pub VisitorGuestRequestMessageIntervalSec: f32,
	pub wanderGhostIntervalLifeTime: f32,
	pub pad13: [u8;12],
	pub YellowMonkTimeOutTime: f32,
	pub YellowMonkDownloadSpan: f32,
	pub YellowMonkOverallFlowTimeOutTime: f32,
	pub pad14_0: [u8;4],
	pub pad14_1: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NETWORK_PARAM_ST {
}
impl Default for NETWORK_PARAM_ST {
	fn default() -> Self {
		Self {
			 signVerticalOffset: 0.,
			 maxSignPosCorrectionRange: 0.,
			 summonTimeoutTime: 0.,
			 pad_0: [0;4],
			 signPuddleActiveMessageIntervalSec: 1.,
			 keyGuideHeight_0: 1.,
			 reloadSignIntervalTime1: 1.,
			 reloadSignIntervalTime2: 1.,
			 reloadSignTotalCount_0: 1,
			 reloadSignCellCount_0: 1,
			 updateSignIntervalTime: 1.,
			 basicExclusiveRange_0: 1.,
			 basicExclusiveHeight_0: 1.,
			 previewChrWaitingTime: 1.,
			 signVisibleRange_0: 1.,
			 cellGroupHorizontalRange_0: 1,
			 cellGroupTopRange_0: 1,
			 cellGroupBottomRange_0: 1,
			 minWhitePhantomLimitTimeScale: 1.,
			 minSmallPhantomLimitTimeScale: 1.,
			 whiteKeywordLimitTimeScale: 1.,
			 smallKeywordLimitTimeScale: 1.,
			 blackKeywordLimitTimeScale: 1.,
			 dragonKeywordLimitTimeScale: 1.,
			 singGetMax: 1,
			 signDownloadSpan: 1.,
			 signUpdateSpan: 1.,
			 signPad: [0;4],
			 maxBreakInTargetListCount: 1,
			 breakInRequestIntervalTimeSec: 4.,
			 breakInRequestTimeOutSec: 20.,
			 pad_1: [0;4],
			 keyGuideRange: 1.,
			 keyGuideHeight_1: 1.,
			 reloadSignTotalCount_1: 1,
			 reloadNewSignCellCount: 1,
			 reloadRandomSignCellCount: 1,
			 maxSignTotalCount_0: 1,
			 maxSignCellCount_0: 1,
			 basicExclusiveRange_1: 1.,
			 basicExclusiveHeight_1: 1.,
			 signVisibleRange_1: 1.,
			 maxWriteSignCount: 1,
			 maxReadSignCount: 1,
			 reloadSignIntervalTime_0: 1.,
			 cellGroupHorizontalRange_1: 1,
			 cellGroupTopRange_1: 1,
			 cellGroupBottomRange_1: 1,
			 lifeTime_0: 1,
			 downloadSpan_0: 0.,
			 downloadEvaluationSpan: 0.,
			 pad_2: [0;4],
			 deadingGhostStartPosThreshold: 1.,
			 keyGuideHeight_2: 1.,
			 keyGuideRangePlayer: 1.,
			 keyGuideHeightPlayer: 1.,
			 reloadSignTotalCount_2: 1,
			 reloadSignCellCount_1: 1,
			 maxSignTotalCount_1: 1,
			 maxSignCellCount_1: 1,
			 reloadSignIntervalTime_1: 1.,
			 signVisibleRange_2: 1.,
			 basicExclusiveRange_2: 1.,
			 basicExclusiveHeight_2: 1.,
			 cellGroupHorizontalRange_2: 1,
			 cellGroupTopRange_2: 1,
			 cellGroupBottomRange_2: 1,
			 lifeTime_1: 1,
			 recordDeadingGhostTotalTime: 0.,
			 recordDeadingGhostMinTime: 5.,
			 downloadSpan_1: 0.,
			 statueCreatableDistance: 80.,
			 reloadGhostTotalCount: 1,
			 reloadGhostCellCount: 1,
			 maxGhostTotalCount: 1,
			 distanceOfBeginRecordVersus: 1.,
			 distanceOfEndRecordVersus: 1.,
			 updateWanderGhostIntervalTime: 1.,
			 updateVersusGhostIntervalTime: 1.,
			 recordWanderingGhostTime: 1.,
			 recordWanderingGhostMinTime: 5.,
			 updateBonfireGhostIntervalTime: 1.,
			 replayGhostRangeOnView: 1.,
			 replayGhostRangeOutView: 1.,
			 replayBonfireGhostTime: 1.,
			 minBonfireGhostValidRange: 1.,
			 maxBonfireGhostValidRange: 1.,
			 minReplayIntervalTime: 1.,
			 maxReplayIntervalTime: 1.,
			 reloadGhostIntervalTime: 1.,
			 cellGroupHorizontalRange_3: 1,
			 cellGroupTopRange_3: 1,
			 replayBonfirePhantomParamIdForCodename: 0,
			 replayBonfireModeRange: 1.,
			 replayBonfirePhantomParamId: 0,
			 ghostpad: [0;4],
			 reloadVisitListCoolTime: 1.,
			 maxCoopBlueSummonCount: 1,
			 maxBellGuardSummonCount: 1,
			 maxVisitListCount: 1,
			 reloadSearch_CoopBlue_Min: 0.,
			 reloadSearch_CoopBlue_Max: 0.,
			 reloadSearch_BellGuard_Min: 0.,
			 reloadSearch_BellGuard_Max: 0.,
			 reloadSearch_RatKing_Min: 0.,
			 reloadSearch_RatKing_Max: 0.,
			 visitpad00: [0;8],
			 srttMaxLimit: 1000.,
			 srttMeanLimit: 1000.,
			 srttMeanDeviationLimit: 1000.,
			 darkPhantomLimitBoostTime: 1000.,
			 darkPhantomLimitBoostScale: 1000.,
			 multiplayDisableLifeTime: 1.,
			 abyssMultiplayLimit: 10,
			 phantomWarpMinimumTime: 5,
			 phantomReturnDelayTime: 5,
			 terminateTimeoutTime: 30,
			 penaltyPointLanDisconnect: 0,
			 penaltyPointSignout: 0,
			 penaltyPointReboot: 0,
			 penaltyPointBeginPenalize: 0,
			 penaltyForgiveItemLimitTime: 0.,
			 allAreaSearchRate_CoopBlue: 0,
			 allAreaSearchRate_VsBlue: 0,
			 allAreaSearchRate_BellGuard: 0,
			 bloodMessageEvalHealRate: 100,
			 smallGoldSuccessHostRewardId: 0,
			 doorInvalidPlayAreaExtents: 1.,
			 signDisplayMax: 10,
			 bloodStainDisplayMax: 7,
			 bloodMessageDisplayMax: 3,
			 pad00: [0;9],
			 pad10: [0;32],
			 summonMessageInterval: 1.,
			 hostRegisterUpdateTime: 1.,
			 hostTimeOutTime: 1.,
			 guestUpdateTime: 1.,
			 guestPlayerNoTimeOutTime: 1.,
			 hostPlayerNoTimeOutTime: 1.,
			 requestSearchQuickMatchLimit: 1,
			 AvatarMatchSearchMax: 1,
			 BattleRoyalMatchSearchMin: 1,
			 BattleRoyalMatchSearchMax: 1,
			 pad11: [0;8],
			 VisitorListMax: 1,
			 VisitorTimeOutTime: 1.,
			 DownloadSpan_2: 1.,
			 VisitorGuestRequestMessageIntervalSec: 1.,
			 wanderGhostIntervalLifeTime: 40.,
			 pad13: [0;12],
			 YellowMonkTimeOutTime: 1.,
			 YellowMonkDownloadSpan: 1.,
			 YellowMonkOverallFlowTimeOutTime: 1.,
			 pad14_0: [0;4],
			 pad14_1: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NPC_AI_ACTION_PARAM_ST {
	pub moveDir: u8,
	pub key1: u8,
	pub key2: u8,
	pub key3: u8,
	pub bMoveDirHold: u8,
	pub bKeyHold1: u8,
	pub bKeyHold2: u8,
	pub bKeyHold3: u8,
	pub gestureId: i32,
	pub bLifeEndSuccess: u8,
	pub pad1: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NPC_AI_ACTION_PARAM_ST {
}
impl Default for NPC_AI_ACTION_PARAM_ST {
	fn default() -> Self {
		Self {
			 moveDir: 0,
			 key1: 0,
			 key2: 0,
			 key3: 0,
			 bMoveDirHold: 0,
			 bKeyHold1: 0,
			 bKeyHold2: 0,
			 bKeyHold3: 0,
			 gestureId: 0,
			 bLifeEndSuccess: 0,
			 pad1: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST {
	pub param000: i16,
	pub param001: i16,
	pub param002: i16,
	pub param003: i16,
	pub param004: i16,
	pub param005: i16,
	pub param006: i16,
	pub param007: i16,
	pub param008: i16,
	pub param009: i16,
	pub param010: i16,
	pub param011: i16,
	pub param012: i16,
	pub param013: i16,
	pub param014: i16,
	pub param015: i16,
	pub param016: i16,
	pub param017: i16,
	pub param018: i16,
	pub param019: i16,
	pub param020: i16,
	pub param021: i16,
	pub param022: i16,
	pub param023: i16,
	pub param024: i16,
	pub param025: i16,
	pub param026: i16,
	pub param027: i16,
	pub param028: i16,
	pub param029: i16,
	pub param030: i16,
	pub param031: i16,
	pub param032: i16,
	pub param033: i16,
	pub param034: i16,
	pub param035: i16,
	pub param036: i16,
	pub param037: i16,
	pub param038: i16,
	pub param039: i16,
	pub param040: i16,
	pub param041: i16,
	pub param042: i16,
	pub param043: i16,
	pub param044: i16,
	pub param045: i16,
	pub param046: i16,
	pub param047: i16,
	pub param048: i16,
	pub param049: i16,
	pub param050: i16,
	pub param051: i16,
	pub param052: i16,
	pub param053: i16,
	pub param054: i16,
	pub param055: i16,
	pub param056: i16,
	pub param057: i16,
	pub param058: i16,
	pub param059: i16,
	pub param060: i16,
	pub param061: i16,
	pub param062: i16,
	pub param063: i16,
	pub param064: i16,
	pub param065: i16,
	pub param066: i16,
	pub param067: i16,
	pub param068: i16,
	pub param069: i16,
	pub param070: i16,
	pub param071: i16,
	pub param072: i16,
	pub param073: i16,
	pub param074: i16,
	pub param075: i16,
	pub param076: i16,
	pub param077: i16,
	pub param078: i16,
	pub param079: i16,
	pub param080: i16,
	pub param081: i16,
	pub param082: i16,
	pub param083: i16,
	pub param084: i16,
	pub param085: i16,
	pub param086: i16,
	pub param087: i16,
	pub param088: i16,
	pub param089: i16,
	pub param090: i16,
	pub param091: i16,
	pub param092: i16,
	pub param093: i16,
	pub param094: i16,
	pub param095: i16,
	pub param096: i16,
	pub param097: i16,
	pub param098: i16,
	pub param099: i16,
	pub param100: i16,
	pub param101: i16,
	pub param102: i16,
	pub param103: i16,
	pub param104: i16,
	pub param105: i16,
	pub param106: i16,
	pub param107: i16,
	pub param108: i16,
	pub param109: i16,
	pub param110: i16,
	pub param111: i16,
	pub param112: i16,
	pub param113: i16,
	pub param114: i16,
	pub param115: i16,
	pub param116: i16,
	pub param117: i16,
	pub param118: i16,
	pub param119: i16,
	pub param120: i16,
	pub param121: i16,
	pub param122: i16,
	pub param123: i16,
	pub param124: i16,
	pub param125: i16,
	pub param126: i16,
	pub param127: i16,
	pub param128: i16,
	pub param129: i16,
	pub param130: i16,
	pub param131: i16,
	pub param132: i16,
	pub param133: i16,
	pub param134: i16,
	pub param135: i16,
	pub param136: i16,
	pub param137: i16,
	pub param138: i16,
	pub param139: i16,
	pub param140: i16,
	pub param141: i16,
	pub param142: i16,
	pub param143: i16,
	pub param144: i16,
	pub param145: i16,
	pub param146: i16,
	pub param147: i16,
	pub param148: i16,
	pub param149: i16,
	pub param150: i16,
	pub param151: i16,
	pub param152: i16,
	pub param153: i16,
	pub param154: i16,
	pub param155: i16,
	pub param156: i16,
	pub param157: i16,
	pub param158: i16,
	pub param159: i16,
	pub param160: i16,
	pub param161: i16,
	pub param162: i16,
	pub param163: i16,
	pub param164: i16,
	pub param165: i16,
	pub param166: i16,
	pub param167: i16,
	pub param168: i16,
	pub param169: i16,
	pub param170: i16,
	pub param171: i16,
	pub param172: i16,
	pub param173: i16,
	pub param174: i16,
	pub param175: i16,
	pub param176: i16,
	pub param177: i16,
	pub param178: i16,
	pub param179: i16,
	pub param180: i16,
	pub param181: i16,
	pub param182: i16,
	pub param183: i16,
	pub param184: i16,
	pub param185: i16,
	pub param186: i16,
	pub param187: i16,
	pub param188: i16,
	pub param189: i16,
	pub param190: i16,
	pub param191: i16,
	pub param192: i16,
	pub param193: i16,
	pub param194: i16,
	pub param195: i16,
	pub param196: i16,
	pub param197: i16,
	pub param198: i16,
	pub param199: i16,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST {
}
impl Default for NPC_AI_BEHAVIOR_PROBABILITY_PARAM_ST {
	fn default() -> Self {
		Self {
			 param000: 0,
			 param001: 0,
			 param002: 0,
			 param003: 0,
			 param004: 0,
			 param005: 0,
			 param006: 0,
			 param007: 0,
			 param008: 0,
			 param009: 0,
			 param010: 0,
			 param011: 0,
			 param012: 0,
			 param013: 0,
			 param014: 0,
			 param015: 0,
			 param016: 0,
			 param017: 0,
			 param018: 0,
			 param019: 0,
			 param020: 0,
			 param021: 0,
			 param022: 0,
			 param023: 0,
			 param024: 0,
			 param025: 0,
			 param026: 0,
			 param027: 0,
			 param028: 0,
			 param029: 0,
			 param030: 0,
			 param031: 0,
			 param032: 0,
			 param033: 0,
			 param034: 0,
			 param035: 0,
			 param036: 0,
			 param037: 0,
			 param038: 0,
			 param039: 0,
			 param040: 0,
			 param041: 0,
			 param042: 0,
			 param043: 0,
			 param044: 0,
			 param045: 0,
			 param046: 0,
			 param047: 0,
			 param048: 0,
			 param049: 0,
			 param050: 0,
			 param051: 0,
			 param052: 0,
			 param053: 0,
			 param054: 0,
			 param055: 0,
			 param056: 0,
			 param057: 0,
			 param058: 0,
			 param059: 0,
			 param060: 0,
			 param061: 0,
			 param062: 0,
			 param063: 0,
			 param064: 0,
			 param065: 0,
			 param066: 0,
			 param067: 0,
			 param068: 0,
			 param069: 0,
			 param070: 0,
			 param071: 0,
			 param072: 0,
			 param073: 0,
			 param074: 0,
			 param075: 0,
			 param076: 0,
			 param077: 0,
			 param078: 0,
			 param079: 0,
			 param080: 0,
			 param081: 0,
			 param082: 0,
			 param083: 0,
			 param084: 0,
			 param085: 0,
			 param086: 0,
			 param087: 0,
			 param088: 0,
			 param089: 0,
			 param090: 0,
			 param091: 0,
			 param092: 0,
			 param093: 0,
			 param094: 0,
			 param095: 0,
			 param096: 0,
			 param097: 0,
			 param098: 0,
			 param099: 0,
			 param100: 0,
			 param101: 0,
			 param102: 0,
			 param103: 0,
			 param104: 0,
			 param105: 0,
			 param106: 0,
			 param107: 0,
			 param108: 0,
			 param109: 0,
			 param110: 0,
			 param111: 0,
			 param112: 0,
			 param113: 0,
			 param114: 0,
			 param115: 0,
			 param116: 0,
			 param117: 0,
			 param118: 0,
			 param119: 0,
			 param120: 0,
			 param121: 0,
			 param122: 0,
			 param123: 0,
			 param124: 0,
			 param125: 0,
			 param126: 0,
			 param127: 0,
			 param128: 0,
			 param129: 0,
			 param130: 0,
			 param131: 0,
			 param132: 0,
			 param133: 0,
			 param134: 0,
			 param135: 0,
			 param136: 0,
			 param137: 0,
			 param138: 0,
			 param139: 0,
			 param140: 0,
			 param141: 0,
			 param142: 0,
			 param143: 0,
			 param144: 0,
			 param145: 0,
			 param146: 0,
			 param147: 0,
			 param148: 0,
			 param149: 0,
			 param150: 0,
			 param151: 0,
			 param152: 0,
			 param153: 0,
			 param154: 0,
			 param155: 0,
			 param156: 0,
			 param157: 0,
			 param158: 0,
			 param159: 0,
			 param160: 0,
			 param161: 0,
			 param162: 0,
			 param163: 0,
			 param164: 0,
			 param165: 0,
			 param166: 0,
			 param167: 0,
			 param168: 0,
			 param169: 0,
			 param170: 0,
			 param171: 0,
			 param172: 0,
			 param173: 0,
			 param174: 0,
			 param175: 0,
			 param176: 0,
			 param177: 0,
			 param178: 0,
			 param179: 0,
			 param180: 0,
			 param181: 0,
			 param182: 0,
			 param183: 0,
			 param184: 0,
			 param185: 0,
			 param186: 0,
			 param187: 0,
			 param188: 0,
			 param189: 0,
			 param190: 0,
			 param191: 0,
			 param192: 0,
			 param193: 0,
			 param194: 0,
			 param195: 0,
			 param196: 0,
			 param197: 0,
			 param198: 0,
			 param199: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NPC_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub behaviorVariationId: i32,
	pub resistCorrectId_poison: i32,
	pub nameId: i32,
	pub turnVellocity: f32,
	pub hitHeight: f32,
	pub hitRadius: f32,
	pub weight: i32,
	pub hitYOffset: f32,
	pub hp: i32,
	pub mp: i32,
	pub getSoul: i32,
	pub itemLotId_enemy: i32,
	pub itemLotId_map: i32,
	pub maxAnkleRollAngle: f32,
	pub chrHitGroupAndNavimesh: u8,
	pub faceIconId: u8,
	pub deactivateDist: i16,
	pub chrActivateConditionParamId: i32,
	pub footIkErrorHeightLimit: f32,
	pub humanityLotId: i32,
	pub spEffectID0: i32,
	pub spEffectID1: i32,
	pub spEffectID2: i32,
	pub spEffectID3: i32,
	pub spEffectID4: i32,
	pub spEffectID5: i32,
	pub spEffectID6: i32,
	pub spEffectID7: i32,
	pub GameClearSpEffectID: i32,
	pub physGuardCutRate: f32,
	pub magGuardCutRate: f32,
	pub fireGuardCutRate: f32,
	pub thunGuardCutRate: f32,
	pub animIdOffset: i32,
	pub lockGazePoint0: i16,
	pub lockGazePoint1: i16,
	pub lockGazePoint2: i16,
	pub lockGazePoint3: i16,
	pub lockGazePoint4: i16,
	pub lockGazePoint5: i16,
	pub networkWarpDist: f32,
	pub dbgBehaviorR1: i32,
	pub dbgBehaviorL1: i32,
	pub dbgBehaviorR2: i32,
	pub dbgBehaviorL2: i32,
	pub dbgBehaviorRL: i32,
	pub dbgBehaviorRR: i32,
	pub dbgBehaviorRD: i32,
	pub dbgBehaviorRU: i32,
	pub dbgBehaviorLL: i32,
	pub dbgBehaviorLR: i32,
	pub dbgBehaviorLD: i32,
	pub dbgBehaviorLU: i32,
	pub animIdOffset2: i32,
	pub partsDamageRate1: f32,
	pub partsDamageRate2: f32,
	pub partsDamageRate3: f32,
	pub partsDamageRate4: f32,
	pub partsDamageRate5: f32,
	pub partsDamageRate6: f32,
	pub partsDamageRate7: f32,
	pub partsDamageRate8: f32,
	pub weakPartsDamageRate: f32,
	pub superArmorRecoverCorrection: f32,
	pub superArmorBrakeKnockbackDist: f32,
	pub stamina: i16,
	pub staminaRecoverBaseVel: i16,
	pub def_phys: i16,
	pub def_slash: i16,
	pub def_blow: i16,
	pub def_thrust: i16,
	pub def_mag: i16,
	pub def_fire: i16,
	pub def_thunder: i16,
	pub defFlickPower: i16,
	pub resist_poison: i16,
	pub resist_desease: i16,
	pub resist_blood: i16,
	pub resist_curse: i16,
	pub ghostModelId: i16,
	pub normalChangeResouceId: i16,
	pub guardAngle: i16,
	pub slashGuardCutRate: i16,
	pub blowGuardCutRate: i16,
	pub thrustGuardCutRate: i16,
	pub lockGazePoint6: i16,
	pub normalChangeTexChrId: i16,
	pub dropType: i16,
	pub knockbackRate: u8,
	pub knockbackParamId: u8,
	pub fallDamageDump: u8,
	pub staminaGuardDef: u8,
	pub resist_sleep: i16,
	pub resist_madness: i16,
	pub sleepGuardResist: i8,
	pub madnessGuardResist: i8,
	pub lockGazePoint7: i16,
	pub mpRecoverBaseVel: u8,
	pub flickDamageCutRate: u8,
	pub defaultLodParamId: i8,
	pub drawType: i8,
	pub npcType: u8,
	pub teamType: u8,
	pub moveType: u8,
	pub lockDist: u8,
	pub materialSe_Weak1: i16,
	pub materialSfx_Weak1: i16,
	pub partsDamageType: u8,
	pub vowType: u8,
	pub guardLevel: i8,
	pub burnSfxType: u8,
	pub poisonGuardResist: i8,
	pub diseaseGuardResist: i8,
	pub bloodGuardResist: i8,
	pub curseGuardResist: i8,
	pub parryAttack: u8,
	pub parryDefence: u8,
	pub sfxSize: u8,
	pub pushOutCamRegionRadius: u8,
	pub hitStopType: u8,
	pub ladderEndChkOffsetTop: u8,
	pub ladderEndChkOffsetLow: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	bits_4: u8,
	bits_5: u8,
	bits_6: u8,
	bits_7: u8,
	pub itemSearchRadius: f32,
	pub chrHitHeight: f32,
	pub chrHitRadius: f32,
	pub specialTurnType: u8,
	bits_8: u8,
	pub def_dark: i16,
	pub threatLv: i32,
	pub specialTurnDistanceThreshold: f32,
	pub autoFootEffectSfxId: i32,
	pub materialSe1: i16,
	pub materialSfx1: i16,
	pub materialSe_Weak2: i16,
	pub materialSfx_Weak2: i16,
	pub materialSe2: i16,
	pub materialSfx2: i16,
	pub spEffectID8: i32,
	pub spEffectID9: i32,
	pub spEffectID10: i32,
	pub spEffectID11: i32,
	pub spEffectID12: i32,
	pub spEffectID13: i32,
	pub spEffectID14: i32,
	pub spEffectID15: i32,
	pub autoFootEffectDecalBaseId1: i32,
	pub toughness: i32,
	pub toughnessRecoverCorrection: f32,
	pub neutralDamageCutRate: f32,
	pub slashDamageCutRate: f32,
	pub blowDamageCutRate: f32,
	pub thrustDamageCutRate: f32,
	pub magicDamageCutRate: f32,
	pub fireDamageCutRate: f32,
	pub thunderDamageCutRate: f32,
	pub darkDamageCutRate: f32,
	pub darkGuardCutRate: f32,
	pub clothUpdateOffset: i8,
	pub npcPlayerWeightType: u8,
	pub normalChangeModelId: i16,
	pub normalChangeAnimChrId: i16,
	pub paintRenderTargetSize: i16,
	pub resistCorrectId_disease: i32,
	pub phantomShaderId: i32,
	pub multiPlayCorrectionParamId: i32,
	pub maxAnklePitchAngle: f32,
	pub resist_freeze: i16,
	pub freezeGuardResist: i8,
	pub pad1: [u8;1],
	pub lockCameraParamId: i32,
	pub spEffectID16: i32,
	pub spEffectID17: i32,
	pub spEffectID18: i32,
	pub spEffectID19: i32,
	pub spEffectID20: i32,
	pub spEffectID21: i32,
	pub spEffectID22: i32,
	pub spEffectID23: i32,
	pub spEffectID24: i32,
	pub spEffectID25: i32,
	pub spEffectID26: i32,
	pub spEffectID27: i32,
	pub spEffectID28: i32,
	pub spEffectID29: i32,
	pub spEffectID30: i32,
	pub spEffectID31: i32,
	pub disableLockOnAng: f32,
	pub clothOffLodLevel: i8,
	bits_9: u8,
	pub estusFlaskRecoveryParamId: i16,
	pub roleNameId: i32,
	pub estusFlaskLotPoint: i16,
	pub hpEstusFlaskLotPoint: i16,
	pub mpEstusFlaskLotPoint: i16,
	pub estusFlaskRecovery_failedLotPointAdd: i16,
	pub hpEstusFlaskRecovery_failedLotPointAdd: i16,
	pub mpEstusFlaskRecovery_failedLotPointAdd: i16,
	pub WanderGhostPhantomId: i32,
	pub hearingHeadSize: f32,
	pub SoundBankId: i16,
	pub forwardUndulationLimit: u8,
	pub sideUndulationLimit: u8,
	pub deactiveMoveSpeed: f32,
	pub deactiveMoveDist: f32,
	pub enableSoundObjDist: f32,
	pub undulationCorrectGain: f32,
	pub autoFootEffectDecalBaseId2: i16,
	pub autoFootEffectDecalBaseId3: i16,
	pub RetargetReferenceChrId: i16,
	pub SfxResBankId: i16,
	pub updateActivatePriolity: f32,
	pub chrNavimeshFlag_Alive: u8,
	pub chrNavimeshFlag_Dead: u8,
	pub pad7: [u8;1],
	pub wheelRotType: u8,
	pub wheelRotRadius: f32,
	pub retargetMoveRate: f32,
	pub ladderWarpOffset: f32,
	pub loadAssetId: i32,
	pub overlapCameraDmypolyId: i32,
	pub residentMaterialExParamId00: i32,
	pub residentMaterialExParamId01: i32,
	pub residentMaterialExParamId02: i32,
	pub residentMaterialExParamId03: i32,
	pub residentMaterialExParamId04: i32,
	pub sleepCollectorItemLotId_enemy: i32,
	pub sleepCollectorItemLotId_map: i32,
	pub footIkErrorOnGain: f32,
	pub footIkErrorOffGain: f32,
	pub SoundAddBankId: i16,
	pub materialVariationValue: u8,
	pub materialVariationValue_Weak: u8,
	pub superArmorDurability: f32,
	pub saRecoveryRate: f32,
	pub saGuardCutRate: f32,
	pub resistCorrectId_blood: i32,
	pub resistCorrectId_curse: i32,
	pub resistCorrectId_freeze: i32,
	pub resistCorrectId_sleep: i32,
	pub resistCorrectId_madness: i32,
	pub chrDeadTutorialFlagId: i32,
	pub stepDispInterpolateTime: f32,
	pub stepDispInterpolateTriggerValue: f32,
	pub lockScoreOffset: f32,
	pub pad12: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NPC_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn useRagdollCamHit(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn disableClothRigidHit(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn useUndulationAddAnimFB(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isWeakA(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isGhost(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn isNoDamageMotion(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isUnduration(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn isChangeWanderGhost(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn modelDispMask0(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn modelDispMask1(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn modelDispMask2(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn modelDispMask3(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn modelDispMask4(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn modelDispMask5(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn modelDispMask6(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn modelDispMask7(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn modelDispMask8(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn modelDispMask9(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn modelDispMask10(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn modelDispMask11(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn modelDispMask12(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn modelDispMask13(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn modelDispMask14(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn modelDispMask15(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn isEnableNeckTurn(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn disableRespawn(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn isMoveAnimWait(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn isCrowd(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn isWeakB(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn isWeakC(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn isWeakD(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn doesAlwaysUseSpecialTurn(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn isRideAtkTarget(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn isEnableStepDispInterpolate(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn isStealthTarget(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn disableInitializeDead(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn isHitRumble(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn isSmoothTurn(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn isWeakE(&self) -> bool {	
			self.bits_5 & (1 << 6) != 0
	}
	pub fn isWeakF(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn modelDispMask16(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn modelDispMask17(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
	pub fn modelDispMask18(&self) -> bool {	
			self.bits_6 & (1 << 2) != 0
	}
	pub fn modelDispMask19(&self) -> bool {	
			self.bits_6 & (1 << 3) != 0
	}
	pub fn modelDispMask20(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn modelDispMask21(&self) -> bool {	
			self.bits_6 & (1 << 5) != 0
	}
	pub fn modelDispMask22(&self) -> bool {	
			self.bits_6 & (1 << 6) != 0
	}
	pub fn modelDispMask23(&self) -> bool {	
			self.bits_6 & (1 << 7) != 0
	}
	pub fn modelDispMask24(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn modelDispMask25(&self) -> bool {	
			self.bits_7 & (1 << 1) != 0
	}
	pub fn modelDispMask26(&self) -> bool {	
			self.bits_7 & (1 << 2) != 0
	}
	pub fn modelDispMask27(&self) -> bool {	
			self.bits_7 & (1 << 3) != 0
	}
	pub fn modelDispMask28(&self) -> bool {	
			self.bits_7 & (1 << 4) != 0
	}
	pub fn modelDispMask29(&self) -> bool {	
			self.bits_7 & (1 << 5) != 0
	}
	pub fn modelDispMask30(&self) -> bool {	
			self.bits_7 & (1 << 6) != 0
	}
	pub fn modelDispMask31(&self) -> bool {	
			self.bits_7 & (1 << 7) != 0
	}
	pub fn isSoulGetByBoss(&self) -> bool {	
			self.bits_8 & (1 << 0) != 0
	}
	pub fn isBulletOwner_byObject(&self) -> bool {	
			self.bits_8 & (1 << 1) != 0
	}
	pub fn isUseLowHitFootIk(&self) -> bool {	
			self.bits_8 & (1 << 2) != 0
	}
	pub fn isCalculatePvPDamage(&self) -> bool {	
			self.bits_8 & (1 << 3) != 0
	}
	pub fn isHostSyncChr(&self) -> bool {	
			self.bits_8 & (1 << 4) != 0
	}
	pub fn isSkipWeakDamageAnim(&self) -> bool {	
			self.bits_8 & (1 << 5) != 0
	}
	pub fn isKeepHitOnRide(&self) -> bool {	
			self.bits_8 & (1 << 6) != 0
	}
	pub fn isSpCollide(&self) -> bool {	
			self.bits_8 & (1 << 7) != 0
	}
	pub fn isUseFootIKNormalByUnduration(&self) -> bool {	
			self.bits_9 & (1 << 0) != 0
	}
	pub fn attachHitInitializeDead(&self) -> bool {	
			self.bits_9 & (1 << 1) != 0
	}
	pub fn excludeGroupRewardCheck(&self) -> bool {	
			self.bits_9 & (1 << 2) != 0
	}
	pub fn enableAILockDmyPoly_212(&self) -> bool {	
			self.bits_9 & (1 << 3) != 0
	}
	pub fn enableAILockDmyPoly_213(&self) -> bool {	
			self.bits_9 & (1 << 4) != 0
	}
	pub fn enableAILockDmyPoly_214(&self) -> bool {	
			self.bits_9 & (1 << 5) != 0
	}
	pub fn disableActivateOpen_xb1(&self) -> bool {	
			self.bits_9 & (1 << 6) != 0
	}
	pub fn disableActivateLegacy_xb1(&self) -> bool {	
			self.bits_9 & (1 << 7) != 0
	}
}
impl Default for NPC_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 behaviorVariationId: 0,
			 resistCorrectId_poison: -1,
			 nameId: -1,
			 turnVellocity: 0.,
			 hitHeight: 0.,
			 hitRadius: 0.,
			 weight: 0,
			 hitYOffset: 0.,
			 hp: 0,
			 mp: 0,
			 getSoul: 0,
			 itemLotId_enemy: -1,
			 itemLotId_map: -1,
			 maxAnkleRollAngle: -1.,
			 chrHitGroupAndNavimesh: 0,
			 faceIconId: 0,
			 deactivateDist: -1,
			 chrActivateConditionParamId: 0,
			 footIkErrorHeightLimit: 0.,
			 humanityLotId: -1,
			 spEffectID0: -1,
			 spEffectID1: -1,
			 spEffectID2: -1,
			 spEffectID3: -1,
			 spEffectID4: -1,
			 spEffectID5: -1,
			 spEffectID6: -1,
			 spEffectID7: -1,
			 GameClearSpEffectID: -1,
			 physGuardCutRate: 0.,
			 magGuardCutRate: 0.,
			 fireGuardCutRate: 0.,
			 thunGuardCutRate: 0.,
			 animIdOffset: 0,
			 lockGazePoint0: -1,
			 lockGazePoint1: -1,
			 lockGazePoint2: -1,
			 lockGazePoint3: -1,
			 lockGazePoint4: -1,
			 lockGazePoint5: -1,
			 networkWarpDist: 0.,
			 dbgBehaviorR1: -1,
			 dbgBehaviorL1: -1,
			 dbgBehaviorR2: -1,
			 dbgBehaviorL2: -1,
			 dbgBehaviorRL: -1,
			 dbgBehaviorRR: -1,
			 dbgBehaviorRD: -1,
			 dbgBehaviorRU: -1,
			 dbgBehaviorLL: -1,
			 dbgBehaviorLR: -1,
			 dbgBehaviorLD: -1,
			 dbgBehaviorLU: -1,
			 animIdOffset2: 0,
			 partsDamageRate1: 1.,
			 partsDamageRate2: 1.,
			 partsDamageRate3: 1.,
			 partsDamageRate4: 1.,
			 partsDamageRate5: 1.,
			 partsDamageRate6: 1.,
			 partsDamageRate7: 1.,
			 partsDamageRate8: 1.,
			 weakPartsDamageRate: 1.,
			 superArmorRecoverCorrection: 0.,
			 superArmorBrakeKnockbackDist: 0.,
			 stamina: 0,
			 staminaRecoverBaseVel: 0,
			 def_phys: 0,
			 def_slash: 0,
			 def_blow: 0,
			 def_thrust: 0,
			 def_mag: 0,
			 def_fire: 0,
			 def_thunder: 0,
			 defFlickPower: 0,
			 resist_poison: 0,
			 resist_desease: 0,
			 resist_blood: 0,
			 resist_curse: 0,
			 ghostModelId: -1,
			 normalChangeResouceId: -1,
			 guardAngle: 0,
			 slashGuardCutRate: 0,
			 blowGuardCutRate: 0,
			 thrustGuardCutRate: 0,
			 lockGazePoint6: -1,
			 normalChangeTexChrId: -1,
			 dropType: 0,
			 knockbackRate: 0,
			 knockbackParamId: 0,
			 fallDamageDump: 0,
			 staminaGuardDef: 0,
			 resist_sleep: 0,
			 resist_madness: 0,
			 sleepGuardResist: 0,
			 madnessGuardResist: 0,
			 lockGazePoint7: -1,
			 mpRecoverBaseVel: 0,
			 flickDamageCutRate: 0,
			 defaultLodParamId: -1,
			 drawType: 0,
			 npcType: 0,
			 teamType: 0,
			 moveType: 0,
			 lockDist: 0,
			 materialSe_Weak1: 0,
			 materialSfx_Weak1: 0,
			 partsDamageType: 0,
			 vowType: 0,
			 guardLevel: 0,
			 burnSfxType: 0,
			 poisonGuardResist: 0,
			 diseaseGuardResist: 0,
			 bloodGuardResist: 0,
			 curseGuardResist: 0,
			 parryAttack: 0,
			 parryDefence: 0,
			 sfxSize: 0,
			 pushOutCamRegionRadius: 12,
			 hitStopType: 0,
			 ladderEndChkOffsetTop: 15,
			 ladderEndChkOffsetLow: 8,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 bits_4: 0,
			 bits_5: 0,
			 bits_6: 0,
			 bits_7: 0,
			 itemSearchRadius: 0.,
			 chrHitHeight: 0.,
			 chrHitRadius: 0.,
			 specialTurnType: 0,
			 bits_8: 0,
			 def_dark: 0,
			 threatLv: 1,
			 specialTurnDistanceThreshold: 4.,
			 autoFootEffectSfxId: -1,
			 materialSe1: 0,
			 materialSfx1: 0,
			 materialSe_Weak2: 0,
			 materialSfx_Weak2: 0,
			 materialSe2: 0,
			 materialSfx2: 0,
			 spEffectID8: -1,
			 spEffectID9: -1,
			 spEffectID10: -1,
			 spEffectID11: -1,
			 spEffectID12: -1,
			 spEffectID13: -1,
			 spEffectID14: -1,
			 spEffectID15: -1,
			 autoFootEffectDecalBaseId1: -1,
			 toughness: 0,
			 toughnessRecoverCorrection: 0.,
			 neutralDamageCutRate: 1.,
			 slashDamageCutRate: 1.,
			 blowDamageCutRate: 1.,
			 thrustDamageCutRate: 1.,
			 magicDamageCutRate: 1.,
			 fireDamageCutRate: 1.,
			 thunderDamageCutRate: 1.,
			 darkDamageCutRate: 1.,
			 darkGuardCutRate: 0.,
			 clothUpdateOffset: 0,
			 npcPlayerWeightType: 0,
			 normalChangeModelId: -1,
			 normalChangeAnimChrId: -1,
			 paintRenderTargetSize: 256,
			 resistCorrectId_disease: -1,
			 phantomShaderId: -1,
			 multiPlayCorrectionParamId: -1,
			 maxAnklePitchAngle: -1.,
			 resist_freeze: 0,
			 freezeGuardResist: 0,
			 pad1: [0;1],
			 lockCameraParamId: -1,
			 spEffectID16: -1,
			 spEffectID17: -1,
			 spEffectID18: -1,
			 spEffectID19: -1,
			 spEffectID20: -1,
			 spEffectID21: -1,
			 spEffectID22: -1,
			 spEffectID23: -1,
			 spEffectID24: -1,
			 spEffectID25: -1,
			 spEffectID26: -1,
			 spEffectID27: -1,
			 spEffectID28: -1,
			 spEffectID29: -1,
			 spEffectID30: -1,
			 spEffectID31: -1,
			 disableLockOnAng: 0.,
			 clothOffLodLevel: -1,
			 bits_9: 0,
			 estusFlaskRecoveryParamId: -1,
			 roleNameId: -1,
			 estusFlaskLotPoint: 0,
			 hpEstusFlaskLotPoint: 0,
			 mpEstusFlaskLotPoint: 0,
			 estusFlaskRecovery_failedLotPointAdd: 0,
			 hpEstusFlaskRecovery_failedLotPointAdd: 0,
			 mpEstusFlaskRecovery_failedLotPointAdd: 0,
			 WanderGhostPhantomId: -1,
			 hearingHeadSize: -1.,
			 SoundBankId: -1,
			 forwardUndulationLimit: 0,
			 sideUndulationLimit: 0,
			 deactiveMoveSpeed: 0.,
			 deactiveMoveDist: 0.,
			 enableSoundObjDist: 48.,
			 undulationCorrectGain: 0.1,
			 autoFootEffectDecalBaseId2: -1,
			 autoFootEffectDecalBaseId3: -1,
			 RetargetReferenceChrId: -1,
			 SfxResBankId: -1,
			 updateActivatePriolity: 1.,
			 chrNavimeshFlag_Alive: 0,
			 chrNavimeshFlag_Dead: 0,
			 pad7: [0;1],
			 wheelRotType: 0,
			 wheelRotRadius: 0.,
			 retargetMoveRate: 1.,
			 ladderWarpOffset: 0.,
			 loadAssetId: -1,
			 overlapCameraDmypolyId: -1,
			 residentMaterialExParamId00: -1,
			 residentMaterialExParamId01: -1,
			 residentMaterialExParamId02: -1,
			 residentMaterialExParamId03: -1,
			 residentMaterialExParamId04: -1,
			 sleepCollectorItemLotId_enemy: -1,
			 sleepCollectorItemLotId_map: -1,
			 footIkErrorOnGain: 0.1,
			 footIkErrorOffGain: 0.4,
			 SoundAddBankId: -1,
			 materialVariationValue: 0,
			 materialVariationValue_Weak: 0,
			 superArmorDurability: 0.,
			 saRecoveryRate: 1.,
			 saGuardCutRate: 0.,
			 resistCorrectId_blood: -1,
			 resistCorrectId_curse: -1,
			 resistCorrectId_freeze: -1,
			 resistCorrectId_sleep: -1,
			 resistCorrectId_madness: -1,
			 chrDeadTutorialFlagId: 0,
			 stepDispInterpolateTime: 0.5,
			 stepDispInterpolateTriggerValue: 0.6,
			 lockScoreOffset: 0.,
			 pad12: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct NPC_THINK_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub logicId: i32,
	pub battleGoalID: i32,
	pub searchEye_dist: i16,
	pub searchEye_angY: u8,
	bits_1: u8,
	pub spEffectId_RangedAttack: i32,
	pub searchTargetLv1ForgetTime: f32,
	pub searchTargetLv2ForgetTime: f32,
	pub BackHomeLife_OnHitEneWal: f32,
	pub SightTargetForgetTime: f32,
	pub idAttackCannotMove: i32,
	pub ear_dist: f32,
	pub callHelp_ActionAnimId: i32,
	pub callHelp_CallActionId: i32,
	pub eye_dist: i16,
	pub isGuard_Act: u8,
	pub pad6: [u8;1],
	pub ear_soundcut_dist: i16,
	pub nose_dist: i16,
	pub maxBackhomeDist: i16,
	pub backhomeDist: i16,
	pub backhomeBattleDist: i16,
	pub nonBattleActLife: i16,
	pub BackHome_LookTargetTime: i16,
	pub BackHome_LookTargetDist: i16,
	pub SoundTargetForgetTime: f32,
	pub BattleStartDist: i16,
	pub callHelp_MyPeerId: i16,
	pub callHelp_CallPeerId: i16,
	pub targetSys_DmgEffectRate: i16,
	pub TeamAttackEffectivity: u8,
	pub eye_angX: u8,
	pub eye_angY: u8,
	pub disableDark: u8,
	pub caravanRole: u8,
	pub callHelp_CallValidMinDistTarget: u8,
	pub callHelp_CallValidRange: u8,
	pub callHelp_ForgetTimeByArrival: u8,
	pub callHelp_MinWaitTime: u8,
	pub callHelp_MaxWaitTime: u8,
	pub goalAction_ToCaution: u8,
	pub ear_listenLevel: u8,
	pub callHelp_ReplyBehaviorType: u8,
	pub disablePathMove: u8,
	pub skipArrivalVisibleCheck: u8,
	pub thinkAttr_doAdmirer: u8,
	bits_2: u8,
	pub enableNaviFlg_reserve1: [u8;3],
	pub searchThreshold_Lv0toLv1: i32,
	pub searchThreshold_Lv1toLv2: i32,
	pub platoonReplyTime: f32,
	pub platoonReplyAddRandomTime: f32,
	pub searchEye_angX: u8,
	pub isUpdateBattleSight: u8,
	pub battleEye_updateDist: i16,
	pub battleEye_updateAngX: u8,
	pub battleEye_updateAngY: u8,
	pub pad4: [u8;16],
	pub eye_BackOffsetDist: i16,
	pub eye_BeginDist: i16,
	pub actTypeOnFailedPath: u8,
	pub goalAction_ToCautionImportant: u8,
	pub shiftAnimeId_RangedAttack: i32,
	pub actTypeOnNonBtlFailedPath: u8,
	pub isBuddyAI: u8,
	pub goalAction_ToSearchLv1: u8,
	pub goalAction_ToSearchLv2: u8,
	pub enableJumpMove: u8,
	pub disableLocalSteering: u8,
	pub goalAction_ToDisappear: u8,
	pub changeStateAction_ToNormal: u8,
	pub MemoryTargetForgetTime: f32,
	pub rangedAttackId: i32,
	pub useFall_onNormalCaution: u8,
	pub useFall_onSearchBattle: u8,
	pub enableJumpMove_onBattle: u8,
	pub backToHomeStuckAct: u8,
	pub pad3: [u8;4],
	pub soundBehaviorId01: i32,
	pub soundBehaviorId02: i32,
	pub soundBehaviorId03: i32,
	pub soundBehaviorId04: i32,
	pub soundBehaviorId05: i32,
	pub soundBehaviorId06: i32,
	pub soundBehaviorId07: i32,
	pub soundBehaviorId08: i32,
	pub weaponOffSpecialEffectId: i32,
	pub weaponOnSpecialEffectId: i32,
	pub weaponOffAnimId: i32,
	pub weaponOnAnimId: i32,
	pub surpriseAnimId: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl NPC_THINK_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isNoAvoidHugeEnemy(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn enableWeaponOnOff(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn targetAILockDmyPoly(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn pad8(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn enableNaviFlg_Edge(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn enableNaviFlg_LargeSpace(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn enableNaviFlg_Ladder(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn enableNaviFlg_Hole(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn enableNaviFlg_Door(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn enableNaviFlg_InSideWall(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn enableNaviFlg_Lava(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn enableNaviFlg_Edge_Ordinary(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
}
impl Default for NPC_THINK_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 logicId: -1,
			 battleGoalID: -1,
			 searchEye_dist: 0,
			 searchEye_angY: 0,
			 bits_1: 0,
			 spEffectId_RangedAttack: -1,
			 searchTargetLv1ForgetTime: 5.,
			 searchTargetLv2ForgetTime: 8.,
			 BackHomeLife_OnHitEneWal: 5.,
			 SightTargetForgetTime: 600.,
			 idAttackCannotMove: 0,
			 ear_dist: 0.,
			 callHelp_ActionAnimId: -1,
			 callHelp_CallActionId: -1,
			 eye_dist: 0,
			 isGuard_Act: 0,
			 pad6: [0;1],
			 ear_soundcut_dist: 0,
			 nose_dist: 0,
			 maxBackhomeDist: 0,
			 backhomeDist: 0,
			 backhomeBattleDist: 0,
			 nonBattleActLife: 0,
			 BackHome_LookTargetTime: 3,
			 BackHome_LookTargetDist: 10,
			 SoundTargetForgetTime: 3.,
			 BattleStartDist: 0,
			 callHelp_MyPeerId: 0,
			 callHelp_CallPeerId: 0,
			 targetSys_DmgEffectRate: 100,
			 TeamAttackEffectivity: 25,
			 eye_angX: 0,
			 eye_angY: 0,
			 disableDark: 0,
			 caravanRole: 0,
			 callHelp_CallValidMinDistTarget: 5,
			 callHelp_CallValidRange: 15,
			 callHelp_ForgetTimeByArrival: 0,
			 callHelp_MinWaitTime: 0,
			 callHelp_MaxWaitTime: 0,
			 goalAction_ToCaution: 0,
			 ear_listenLevel: 128,
			 callHelp_ReplyBehaviorType: 0,
			 disablePathMove: 0,
			 skipArrivalVisibleCheck: 0,
			 thinkAttr_doAdmirer: 0,
			 bits_2: 1,
			 enableNaviFlg_reserve1: [0;3],
			 searchThreshold_Lv0toLv1: 10,
			 searchThreshold_Lv1toLv2: 70,
			 platoonReplyTime: 0.,
			 platoonReplyAddRandomTime: 0.,
			 searchEye_angX: 0,
			 isUpdateBattleSight: 0,
			 battleEye_updateDist: 0,
			 battleEye_updateAngX: 0,
			 battleEye_updateAngY: 0,
			 pad4: [0;16],
			 eye_BackOffsetDist: 0,
			 eye_BeginDist: 0,
			 actTypeOnFailedPath: 0,
			 goalAction_ToCautionImportant: 0,
			 shiftAnimeId_RangedAttack: -1,
			 actTypeOnNonBtlFailedPath: 0,
			 isBuddyAI: 0,
			 goalAction_ToSearchLv1: 0,
			 goalAction_ToSearchLv2: 0,
			 enableJumpMove: 0,
			 disableLocalSteering: 0,
			 goalAction_ToDisappear: 0,
			 changeStateAction_ToNormal: 0,
			 MemoryTargetForgetTime: 150.,
			 rangedAttackId: -1,
			 useFall_onNormalCaution: 2,
			 useFall_onSearchBattle: 2,
			 enableJumpMove_onBattle: 0,
			 backToHomeStuckAct: 0,
			 pad3: [0;4],
			 soundBehaviorId01: -1,
			 soundBehaviorId02: -1,
			 soundBehaviorId03: -1,
			 soundBehaviorId04: -1,
			 soundBehaviorId05: -1,
			 soundBehaviorId06: -1,
			 soundBehaviorId07: -1,
			 soundBehaviorId08: -1,
			 weaponOffSpecialEffectId: -1,
			 weaponOnSpecialEffectId: -1,
			 weaponOffAnimId: -1,
			 weaponOnAnimId: -1,
			 surpriseAnimId: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct OBJ_ACT_PARAM_ST {
	pub actionEnableMsgId: i32,
	pub actionFailedMsgId: i32,
	pub spQualifiedPassEventFlag: i32,
	pub playerAnimId: i32,
	pub chrAnimId: i32,
	pub validDist: i16,
	pub spQualifiedId: i16,
	pub spQualifiedId2: i16,
	pub objDummyId: u8,
	pub isEventKickSync: u8,
	pub objAnimId: i32,
	pub validPlayerAngle: u8,
	pub spQualifiedType: u8,
	pub spQualifiedType2: u8,
	pub validObjAngle: u8,
	pub chrSorbType: u8,
	pub eventKickTiming: u8,
	pub pad1: [u8;2],
	pub actionButtonParamId: i32,
	pub enableTreasureDelaySec: f32,
	pub preActionSfxDmypolyId: i32,
	pub preActionSfxId: i32,
	pub pad2: [u8;40],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl OBJ_ACT_PARAM_ST {
}
impl Default for OBJ_ACT_PARAM_ST {
	fn default() -> Self {
		Self {
			 actionEnableMsgId: -1,
			 actionFailedMsgId: -1,
			 spQualifiedPassEventFlag: 0,
			 playerAnimId: 0,
			 chrAnimId: 0,
			 validDist: 150,
			 spQualifiedId: 0,
			 spQualifiedId2: 0,
			 objDummyId: 0,
			 isEventKickSync: 0,
			 objAnimId: 0,
			 validPlayerAngle: 30,
			 spQualifiedType: 0,
			 spQualifiedType2: 0,
			 validObjAngle: 30,
			 chrSorbType: 0,
			 eventKickTiming: 0,
			 pad1: [0;2],
			 actionButtonParamId: -1,
			 enableTreasureDelaySec: 0.,
			 preActionSfxDmypolyId: -1,
			 preActionSfxId: -1,
			 pad2: [0;40],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct OBJECT_MATERIAL_SFX_PARAM_ST {
	pub sfxId_00: i32,
	pub sfxId_01: i32,
	pub sfxId_02: i32,
	pub sfxId_03: i32,
	pub sfxId_04: i32,
	pub sfxId_05: i32,
	pub sfxId_06: i32,
	pub sfxId_07: i32,
	pub sfxId_08: i32,
	pub sfxId_09: i32,
	pub sfxId_10: i32,
	pub sfxId_11: i32,
	pub sfxId_12: i32,
	pub sfxId_13: i32,
	pub sfxId_14: i32,
	pub sfxId_15: i32,
	pub sfxId_16: i32,
	pub sfxId_17: i32,
	pub sfxId_18: i32,
	pub sfxId_19: i32,
	pub sfxId_20: i32,
	pub sfxId_21: i32,
	pub sfxId_22: i32,
	pub sfxId_23: i32,
	pub sfxId_24: i32,
	pub sfxId_25: i32,
	pub sfxId_26: i32,
	pub sfxId_27: i32,
	pub sfxId_28: i32,
	pub sfxId_29: i32,
	pub sfxId_30: i32,
	pub sfxId_31: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl OBJECT_MATERIAL_SFX_PARAM_ST {
}
impl Default for OBJECT_MATERIAL_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 sfxId_00: 0,
			 sfxId_01: 0,
			 sfxId_02: 0,
			 sfxId_03: 0,
			 sfxId_04: 0,
			 sfxId_05: 0,
			 sfxId_06: 0,
			 sfxId_07: 0,
			 sfxId_08: 0,
			 sfxId_09: 0,
			 sfxId_10: 0,
			 sfxId_11: 0,
			 sfxId_12: 0,
			 sfxId_13: 0,
			 sfxId_14: 0,
			 sfxId_15: 0,
			 sfxId_16: 0,
			 sfxId_17: 0,
			 sfxId_18: 0,
			 sfxId_19: 0,
			 sfxId_20: 0,
			 sfxId_21: 0,
			 sfxId_22: 0,
			 sfxId_23: 0,
			 sfxId_24: 0,
			 sfxId_25: 0,
			 sfxId_26: 0,
			 sfxId_27: 0,
			 sfxId_28: 0,
			 sfxId_29: 0,
			 sfxId_30: 0,
			 sfxId_31: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct OBJECT_PARAM_ST {
	pub hp: i16,
	pub defense: i16,
	pub extRefTexId: i16,
	pub materialId: i16,
	pub animBreakIdMax: u8,
	bits_0: u8,
	bits_1: u8,
	pub defaultLodParamId: i8,
	pub breakSfxId: i32,
	pub breakSfxCpId: i32,
	pub breakBulletBehaviorId: i32,
	pub breakBulletCpId: i32,
	pub breakFallHeight: u8,
	pub windEffectType_0: u8,
	pub windEffectType_1: u8,
	pub camAvoidType: u8,
	pub windEffectRate_0: f32,
	pub windEffectRate_1: f32,
	pub breakStopTime: f32,
	pub burnTime: f32,
	pub burnBraekRate: f32,
	pub burnSfxId: i32,
	pub burnSfxId_1: i32,
	pub burnSfxId_2: i32,
	pub burnSfxId_3: i32,
	pub burnBulletBehaviorId: i32,
	pub burnBulletBehaviorId_1: i32,
	pub burnBulletBehaviorId_2: i32,
	pub burnBulletBehaviorId_3: i32,
	pub burnBulletInterval: i16,
	pub navimeshFlag: u8,
	pub collisionType: u8,
	pub burnBulletDelayTime: f32,
	pub burnSfxDelayTimeMin: f32,
	pub burnSfxDelayTimeMin_1: f32,
	pub burnSfxDelayTimeMin_2: f32,
	pub burnSfxDelayTimeMin_3: f32,
	pub burnSfxDelayTimeMax: f32,
	pub burnSfxDelayTimeMax_1: f32,
	pub burnSfxDelayTimeMax_2: f32,
	pub burnSfxDelayTimeMax_3: f32,
	pub BreakAiSoundID: i32,
	pub FragmentInvisibleWaitTime: f32,
	pub FragmentInvisibleTime: f32,
	pub pad_3: [u8;16],
	pub RigidPenetrationScale_Soft: f32,
	pub RigidPenetrationScale_Normal: f32,
	pub RigidPenetrationScale_Hard: f32,
	pub LandTouchSfxId: i32,
	bits_2: u8,
	pub paintDecalTargetTextureSize: i16,
	pub lifeTime_forDC: f32,
	pub clothUpdateDist: f32,
	pub contactSeId: i32,
	pub breakLandingSfxId: i32,
	pub waypointDummyPolyId_0: i32,
	pub waypointParamId_0: i32,
	pub soundBankId: i32,
	pub refDrawParamId: i32,
	pub autoCreateDynamicOffsetHeight: f32,
	pub reserved0: i32,
	pub soundBreakSEId: i32,
	pub pad_5: [u8;40],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl OBJECT_PARAM_ST {
	pub fn isCamHit(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isBreakByPlayerCollide(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isAnimBreak(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn isPenetrationBulletHit(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn isChrHit(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isAttackBacklash(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn isDisableBreakForFirstAppear(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn isLadder(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn isAnimPauseOnRemoPlay(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isDamageNoHit(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn isMoveObj(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isRopeBridge(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isAddRigidImpulse_ByDamage(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn isBreak_ByChrRide(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isBurn(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn isBreakByEnemyCollide(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn isDamageCover(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
}
impl Default for OBJECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 hp: -1,
			 defense: 0,
			 extRefTexId: -1,
			 materialId: -1,
			 animBreakIdMax: 0,
			 bits_0: 0,
			 bits_1: 0,
			 defaultLodParamId: -1,
			 breakSfxId: -1,
			 breakSfxCpId: -1,
			 breakBulletBehaviorId: -1,
			 breakBulletCpId: -1,
			 breakFallHeight: 0,
			 windEffectType_0: 0,
			 windEffectType_1: 0,
			 camAvoidType: 1,
			 windEffectRate_0: 0.5,
			 windEffectRate_1: 0.5,
			 breakStopTime: 0.,
			 burnTime: 0.,
			 burnBraekRate: 0.5,
			 burnSfxId: -1,
			 burnSfxId_1: -1,
			 burnSfxId_2: -1,
			 burnSfxId_3: -1,
			 burnBulletBehaviorId: -1,
			 burnBulletBehaviorId_1: -1,
			 burnBulletBehaviorId_2: -1,
			 burnBulletBehaviorId_3: -1,
			 burnBulletInterval: 30,
			 navimeshFlag: 0,
			 collisionType: 0,
			 burnBulletDelayTime: 0.,
			 burnSfxDelayTimeMin: 0.,
			 burnSfxDelayTimeMin_1: 0.,
			 burnSfxDelayTimeMin_2: 0.,
			 burnSfxDelayTimeMin_3: 0.,
			 burnSfxDelayTimeMax: 0.,
			 burnSfxDelayTimeMax_1: 0.,
			 burnSfxDelayTimeMax_2: 0.,
			 burnSfxDelayTimeMax_3: 0.,
			 BreakAiSoundID: 0,
			 FragmentInvisibleWaitTime: 0.,
			 FragmentInvisibleTime: 0.,
			 pad_3: [0;16],
			 RigidPenetrationScale_Soft: 0.,
			 RigidPenetrationScale_Normal: 0.,
			 RigidPenetrationScale_Hard: 0.,
			 LandTouchSfxId: -1,
			 bits_2: 0,
			 paintDecalTargetTextureSize: 256,
			 lifeTime_forDC: 0.,
			 clothUpdateDist: 0.,
			 contactSeId: -1,
			 breakLandingSfxId: -1,
			 waypointDummyPolyId_0: -1,
			 waypointParamId_0: -1,
			 soundBankId: -1,
			 refDrawParamId: -1,
			 autoCreateDynamicOffsetHeight: 0.1,
			 reserved0: -1,
			 soundBreakSEId: -1,
			 pad_5: [0;40],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct PARTS_DRAW_PARAM_ST {
	pub lv01_BorderDist: f32,
	pub lv01_PlayDist: f32,
	pub lv12_BorderDist: f32,
	pub lv12_PlayDist: f32,
	pub lv23_BorderDist: f32,
	pub lv23_PlayDist: f32,
	pub lv34_BorderDist: f32,
	pub lv34_PlayDist: f32,
	pub lv45_BorderDist: f32,
	pub lv45_PlayDist: f32,
	pub tex_lv01_BorderDist: f32,
	pub tex_lv01_PlayDist: f32,
	bits_0: i32,
	pub drawFadeRange: f32,
	pub shadowDrawDist: f32,
	pub shadowFadeRange: f32,
	pub motionBlur_BorderDist: f32,
	pub isPointLightShadowSrc: i8,
	pub isDirLightShadowSrc: i8,
	pub isShadowDst: i8,
	pub isShadowOnly: i8,
	pub drawByReflectCam: i8,
	pub drawOnlyReflectCam: i8,
	pub IncludeLodMapLv: i8,
	pub isNoFarClipDraw: u8,
	pub lodType: u8,
	pub shadowDrawLodOffset: i8,
	pub isTraceCameraXZ: u8,
	pub isSkydomeDrawPhase: u8,
	pub DistantViewModel_BorderDist: f32,
	pub DistantViewModel_PlayDist: f32,
	pub LimtedActivate_BorderDist_forGrid: f32,
	pub LimtedActivate_PlayDist_forGrid: f32,
	pub zSortOffsetForNoFarClipDraw: f32,
	pub shadowDrawAlphaTestDist: f32,
	pub fowardDrawEnvmapBlendType: u8,
	pub LBDrawDistScaleParamID: u8,
	pub resereve: [u8;34],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl PARTS_DRAW_PARAM_ST {
	pub fn enableCrossFade(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
}
impl Default for PARTS_DRAW_PARAM_ST {
	fn default() -> Self {
		Self {
			 lv01_BorderDist: 5.,
			 lv01_PlayDist: 1.,
			 lv12_BorderDist: 20.,
			 lv12_PlayDist: 2.,
			 lv23_BorderDist: 30.,
			 lv23_PlayDist: 0.,
			 lv34_BorderDist: 9999.,
			 lv34_PlayDist: 0.,
			 lv45_BorderDist: 9999.,
			 lv45_PlayDist: 0.,
			 tex_lv01_BorderDist: 30.,
			 tex_lv01_PlayDist: 1.,
			 bits_0: 0,
			 drawFadeRange: 0.,
			 shadowDrawDist: 9999.,
			 shadowFadeRange: 0.,
			 motionBlur_BorderDist: 20.,
			 isPointLightShadowSrc: 0,
			 isDirLightShadowSrc: 0,
			 isShadowDst: 0,
			 isShadowOnly: 0,
			 drawByReflectCam: 0,
			 drawOnlyReflectCam: 0,
			 IncludeLodMapLv: -1,
			 isNoFarClipDraw: 0,
			 lodType: 0,
			 shadowDrawLodOffset: 0,
			 isTraceCameraXZ: 0,
			 isSkydomeDrawPhase: 0,
			 DistantViewModel_BorderDist: 30.,
			 DistantViewModel_PlayDist: 5.,
			 LimtedActivate_BorderDist_forGrid: -1.,
			 LimtedActivate_PlayDist_forGrid: 10.,
			 zSortOffsetForNoFarClipDraw: 0.,
			 shadowDrawAlphaTestDist: 9999.,
			 fowardDrawEnvmapBlendType: 0,
			 LBDrawDistScaleParamID: 0,
			 resereve: [0;34],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct PERFORMANCE_CHECK_PARAM {
	pub workTag: u8,
	pub categoryTag: u8,
	pub compareType: u8,
	pub dummy1: [u8;1],
	pub compareValue: f32,
	pub dummy2: [u8;8],
	pub userTag: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl PERFORMANCE_CHECK_PARAM {
}
impl Default for PERFORMANCE_CHECK_PARAM {
	fn default() -> Self {
		Self {
			 workTag: 0,
			 categoryTag: 0,
			 compareType: 0,
			 dummy1: [0;1],
			 compareValue: 0.,
			 dummy2: [0;8],
			 userTag: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct PHANTOM_PARAM_ST {
	pub edgeColorA: f32,
	pub frontColorA: f32,
	pub diffMulColorA: f32,
	pub specMulColorA: f32,
	pub lightColorA: f32,
	pub edgeColorR: u8,
	pub edgeColorG: u8,
	pub edgeColorB: u8,
	pub frontColorR: u8,
	pub frontColorG: u8,
	pub frontColorB: u8,
	pub diffMulColorR: u8,
	pub diffMulColorG: u8,
	pub diffMulColorB: u8,
	pub specMulColorR: u8,
	pub specMulColorG: u8,
	pub specMulColorB: u8,
	pub lightColorR: u8,
	pub lightColorG: u8,
	pub lightColorB: u8,
	pub reserve: [u8;1],
	pub alpha: f32,
	pub blendRate: f32,
	pub blendType: u8,
	pub isEdgeSubtract: u8,
	pub isFrontSubtract: u8,
	pub isNo2Pass: u8,
	pub edgePower: f32,
	pub glowScale: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl PHANTOM_PARAM_ST {
}
impl Default for PHANTOM_PARAM_ST {
	fn default() -> Self {
		Self {
			 edgeColorA: 1.,
			 frontColorA: 0.,
			 diffMulColorA: 1.,
			 specMulColorA: 1.,
			 lightColorA: 0.,
			 edgeColorR: 255,
			 edgeColorG: 255,
			 edgeColorB: 255,
			 frontColorR: 0,
			 frontColorG: 0,
			 frontColorB: 0,
			 diffMulColorR: 255,
			 diffMulColorG: 255,
			 diffMulColorB: 255,
			 specMulColorR: 255,
			 specMulColorG: 255,
			 specMulColorB: 255,
			 lightColorR: 0,
			 lightColorG: 0,
			 lightColorB: 0,
			 reserve: [0;1],
			 alpha: 1.,
			 blendRate: 1.,
			 blendType: 0,
			 isEdgeSubtract: 0,
			 isFrontSubtract: 0,
			 isNo2Pass: 0,
			 edgePower: 1.,
			 glowScale: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct PLAYER_COMMON_PARAM_ST {
	pub playerFootEffect_bySFX: i32,
	pub snipeModeDrawAlpha_FadeTime: f32,
	pub toughnessRecoverCorrection: f32,
	pub baseMagicSlotSize: u8,
	pub baseAccSlotNum: u8,
	pub reserved02: [u8;2],
	pub animeID_DropItemPick: i32,
	pub resistRecoverPoint_Sleep_Player: f32,
	pub flareOverrideHomingAngle: i32,
	pub flareOverrideHomingStopRange: f32,
	pub animeID_SleepCollectorItemPick: i32,
	pub unlockEventFlagBaseId_forWepAttr: i32,
	pub systemEnchant_BigRune: i32,
	pub lowStatus_AtkPowDown: f32,
	pub lowStatus_ConsumeStaminaRate: f32,
	pub lowStatus_AtkGuardBreak: i16,
	pub guardStatusCorrect_MaxStatusVal: i16,
	pub unlockEventFlagStepNum_forWepAttr: i16,
	pub retributionMagic_damageCountNum: i16,
	pub retributionMagic_damageCountRemainTime: i16,
	pub retributionMagic_burstDmypolyId: i16,
	pub retributionMagic_burstMagicParamId: i32,
	pub chrAimCam_rideOffsetHeight: f32,
	pub reserved23: [u8;4],
	pub arrowCaseWepBindDmypolyId: i32,
	pub boltPouchWepBindDmypolyId: i32,
	pub estusFlaskAllocateRate: f32,
	pub reserved38: [u8;2],
	pub kickAcceptanceDeg: u8,
	pub npcPlayerAnalogWeightRate_Light: u8,
	pub npcPlayerAnalogWeightRate_Normal: u8,
	pub npcPlayerAnalogWeightRate_Heavy: u8,
	pub npcPlayerAnalogWeightRate_WeightOver: u8,
	pub npcPlayerAnalogWeightRate_SuperLight: u8,
	pub reserved45: [u8;4],
	pub clearCountCorrectBaseSpEffectId: i32,
	pub arrowBoltModelIdOffset: i32,
	pub arrowBoltRemainingNumModelMaskThreshold1: i8,
	pub arrowBoltRemainingNumModelMaskThreshold2: i8,
	pub reserved27: [u8;2],
	pub resistRecoverPoint_Poision_Player: f32,
	pub resistRecoverPoint_Desease_Player: f32,
	pub resistRecoverPoint_Blood_Player: f32,
	pub resistRecoverPoint_Curse_Player: f32,
	pub resistRecoverPoint_Freeze_Player: f32,
	pub resistRecoverPoint_Poision_Enemy: f32,
	pub resistRecoverPoint_Desease_Enemy: f32,
	pub resistRecoverPoint_Blood_Enemy: f32,
	pub resistRecoverPoint_Curse_Enemy: f32,
	pub resistRecoverPoint_Freeze_Enemy: f32,
	pub requestTimeLeftBothHand: f32,
	pub resistRecoverPoint_Madness_Player: f32,
	pub animeID_MaterialItemPick: i32,
	pub hpEstusFlaskAllocateRateForYellowMonk: f32,
	pub hpEstusFlaskAllocateOffsetForYellowMonk: i32,
	pub mpEstusFlaskAllocateRateForYellowMonk: f32,
	pub mpEstusFlaskAllocateOffsetForYellowMonk: i32,
	pub resistRecoverPoint_Sleep_Enemy: f32,
	pub resistRecoverPoint_Madness_Enemy: f32,
	pub resistCurseItemId: i32,
	pub resistCurseItemMaxNum: u8,
	pub reserved_123: [u8;3],
	pub resistCurseItemSpEffectBaseId: i32,
	pub resistCurseItemLotParamId_map: i32,
	pub reserved41: [u8;52],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl PLAYER_COMMON_PARAM_ST {
}
impl Default for PLAYER_COMMON_PARAM_ST {
	fn default() -> Self {
		Self {
			 playerFootEffect_bySFX: 0,
			 snipeModeDrawAlpha_FadeTime: 0.,
			 toughnessRecoverCorrection: 0.,
			 baseMagicSlotSize: 0,
			 baseAccSlotNum: 0,
			 reserved02: [0;2],
			 animeID_DropItemPick: 0,
			 resistRecoverPoint_Sleep_Player: 0.,
			 flareOverrideHomingAngle: -1,
			 flareOverrideHomingStopRange: -1.,
			 animeID_SleepCollectorItemPick: 0,
			 unlockEventFlagBaseId_forWepAttr: 0,
			 systemEnchant_BigRune: 0,
			 lowStatus_AtkPowDown: 0.,
			 lowStatus_ConsumeStaminaRate: 0.,
			 lowStatus_AtkGuardBreak: 0,
			 guardStatusCorrect_MaxStatusVal: 0,
			 unlockEventFlagStepNum_forWepAttr: 1,
			 retributionMagic_damageCountNum: 0,
			 retributionMagic_damageCountRemainTime: 0,
			 retributionMagic_burstDmypolyId: 0,
			 retributionMagic_burstMagicParamId: -1,
			 chrAimCam_rideOffsetHeight: 0.,
			 reserved23: [0;4],
			 arrowCaseWepBindDmypolyId: 0,
			 boltPouchWepBindDmypolyId: 0,
			 estusFlaskAllocateRate: 0.,
			 reserved38: [0;2],
			 kickAcceptanceDeg: 0,
			 npcPlayerAnalogWeightRate_Light: 0,
			 npcPlayerAnalogWeightRate_Normal: 0,
			 npcPlayerAnalogWeightRate_Heavy: 0,
			 npcPlayerAnalogWeightRate_WeightOver: 0,
			 npcPlayerAnalogWeightRate_SuperLight: 0,
			 reserved45: [0;4],
			 clearCountCorrectBaseSpEffectId: 0,
			 arrowBoltModelIdOffset: 0,
			 arrowBoltRemainingNumModelMaskThreshold1: 0,
			 arrowBoltRemainingNumModelMaskThreshold2: 0,
			 reserved27: [0;2],
			 resistRecoverPoint_Poision_Player: 0.,
			 resistRecoverPoint_Desease_Player: 0.,
			 resistRecoverPoint_Blood_Player: 0.,
			 resistRecoverPoint_Curse_Player: 0.,
			 resistRecoverPoint_Freeze_Player: 0.,
			 resistRecoverPoint_Poision_Enemy: 0.,
			 resistRecoverPoint_Desease_Enemy: 0.,
			 resistRecoverPoint_Blood_Enemy: 0.,
			 resistRecoverPoint_Curse_Enemy: 0.,
			 resistRecoverPoint_Freeze_Enemy: 0.,
			 requestTimeLeftBothHand: 0.,
			 resistRecoverPoint_Madness_Player: 0.,
			 animeID_MaterialItemPick: 0,
			 hpEstusFlaskAllocateRateForYellowMonk: 0.,
			 hpEstusFlaskAllocateOffsetForYellowMonk: 0,
			 mpEstusFlaskAllocateRateForYellowMonk: 0.,
			 mpEstusFlaskAllocateOffsetForYellowMonk: 0,
			 resistRecoverPoint_Sleep_Enemy: 0.,
			 resistRecoverPoint_Madness_Enemy: 0.,
			 resistCurseItemId: -1,
			 resistCurseItemMaxNum: 0,
			 reserved_123: [0;3],
			 resistCurseItemSpEffectBaseId: -1,
			 resistCurseItemLotParamId_map: -1,
			 reserved41: [0;52],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct PLAY_REGION_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub matchAreaId: i32,
	pub multiPlayStartLimitEventFlagId: i32,
	pub otherDisableDistance: f32,
	pub pcPositionSaveLimitEventFlagId: i32,
	pub bossAreaId: i32,
	pub cultNpcWhiteGhostEntityId_byFree: i16,
	pub bMapGuradianRegion: u8,
	bits_1: u8,
	pub warpItemUsePermitBonfireId_1: i32,
	pub warpItemUsePermitBonfireId_2: i32,
	pub warpItemUsePermitBonfireId_3: i32,
	pub warpItemUsePermitBonfireId_4: i32,
	pub warpItemUsePermitBonfireId_5: i32,
	pub warpItemProhibitionEventFlagId_1: i32,
	pub warpItemProhibitionEventFlagId_2: i32,
	pub warpItemProhibitionEventFlagId_3: i32,
	pub warpItemProhibitionEventFlagId_4: i32,
	pub warpItemProhibitionEventFlagId_5: i32,
	bits_2: u8,
	bits_3: u8,
	pub pad2: [u8;2],
	pub multiPlayHASHostLimitEventFlagId: i32,
	pub otherMaxDistance: f32,
	pub signPuddleOpenEventFlagId: i32,
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad4: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
	pub breakInLimitEventFlagId_1: i32,
	pub whiteSignLimitEventFlagId_1: i32,
	pub matchAreaSignCreateLimitEventFlagId: i32,
	pub signAimId_1: i32,
	pub signAimId_2: i32,
	pub signAimId_3: i32,
	pub signAimId_4: i32,
	pub signAimId_5: i32,
	pub signAimId_6: i32,
	pub signAimId_7: i32,
	pub signAimId_8: i32,
	pub redSignLimitEventFlagId_1: i32,
	pub breakInLimitEventFlagId_2: i32,
	pub breakInLimitEventFlagId_3: i32,
	pub whiteSignLimitEventFlagId_2: i32,
	pub whiteSignLimitEventFlagId_3: i32,
	pub redSignLimitEventFlagId_2: i32,
	pub redSignLimitEventFlagId_3: i32,
	pub bossId_1: i32,
	pub bossId_2: i32,
	pub bossId_3: i32,
	pub bossId_4: i32,
	pub bossId_5: i32,
	pub bossId_6: i32,
	pub bossId_7: i32,
	pub bossId_8: i32,
	pub bossId_9: i32,
	pub bossId_10: i32,
	pub bossId_11: i32,
	pub bossId_12: i32,
	pub bossId_13: i32,
	pub bossId_14: i32,
	pub bossId_15: i32,
	pub bossId_16: i32,
	pub mapMenuUnlockEventId: i32,
	pub pad5: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl PLAY_REGION_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn bYellowCostumeRegion(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn multiPlayStartLimitEventFlagId_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn breakInLimitEventFlagId_1_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn whiteSignLimitEventFlagId_1_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn redSignLimitEventFlagId_1_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn breakInLimitEventFlagId_2_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn breakInLimitEventFlagId_3_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn whiteSignLimitEventFlagId_2_targetFlagState(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn enableBloodstain(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn enableBloodMessage(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn enableGhost(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn dispMask00(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn dispMask01(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn whiteSignLimitEventFlagId_3_targetFlagState(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn redSignLimitEventFlagId_2_targetFlagState(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn redSignLimitEventFlagId_3_targetFlagState(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn isAutoIntrudePoint(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn pad1(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
}
impl Default for PLAY_REGION_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 matchAreaId: 0,
			 multiPlayStartLimitEventFlagId: 0,
			 otherDisableDistance: 0.,
			 pcPositionSaveLimitEventFlagId: 0,
			 bossAreaId: 0,
			 cultNpcWhiteGhostEntityId_byFree: -1,
			 bMapGuradianRegion: 0,
			 bits_1: 0,
			 warpItemUsePermitBonfireId_1: 0,
			 warpItemUsePermitBonfireId_2: 0,
			 warpItemUsePermitBonfireId_3: 0,
			 warpItemUsePermitBonfireId_4: 0,
			 warpItemUsePermitBonfireId_5: 0,
			 warpItemProhibitionEventFlagId_1: 0,
			 warpItemProhibitionEventFlagId_2: 0,
			 warpItemProhibitionEventFlagId_3: 0,
			 warpItemProhibitionEventFlagId_4: 0,
			 warpItemProhibitionEventFlagId_5: 0,
			 bits_2: 1,
			 bits_3: 0,
			 pad2: [0;2],
			 multiPlayHASHostLimitEventFlagId: 0,
			 otherMaxDistance: 1000.,
			 signPuddleOpenEventFlagId: 0,
			 areaNo: 0,
			 gridXNo: 0,
			 gridZNo: 0,
			 pad4: [0;1],
			 posX: 0.,
			 posY: 0.,
			 posZ: 0.,
			 breakInLimitEventFlagId_1: 0,
			 whiteSignLimitEventFlagId_1: 0,
			 matchAreaSignCreateLimitEventFlagId: 0,
			 signAimId_1: 0,
			 signAimId_2: 0,
			 signAimId_3: 0,
			 signAimId_4: 0,
			 signAimId_5: 0,
			 signAimId_6: 0,
			 signAimId_7: 0,
			 signAimId_8: 0,
			 redSignLimitEventFlagId_1: 0,
			 breakInLimitEventFlagId_2: 0,
			 breakInLimitEventFlagId_3: 0,
			 whiteSignLimitEventFlagId_2: 0,
			 whiteSignLimitEventFlagId_3: 0,
			 redSignLimitEventFlagId_2: 0,
			 redSignLimitEventFlagId_3: 0,
			 bossId_1: 0,
			 bossId_2: 0,
			 bossId_3: 0,
			 bossId_4: 0,
			 bossId_5: 0,
			 bossId_6: 0,
			 bossId_7: 0,
			 bossId_8: 0,
			 bossId_9: 0,
			 bossId_10: 0,
			 bossId_11: 0,
			 bossId_12: 0,
			 bossId_13: 0,
			 bossId_14: 0,
			 bossId_15: 0,
			 bossId_16: 0,
			 mapMenuUnlockEventId: 0,
			 pad5: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct POSTURE_CONTROL_PARAM_GENDER_ST {
	pub a000_rightElbowIO: i16,
	pub a000_leftElbowIO: i16,
	pub a000_bothLegsIO: i16,
	pub a002_rightElbowIO: i16,
	pub a002_leftElbowIO: i16,
	pub a002_bothLegsIO: i16,
	pub a003_rightElbowIO: i16,
	pub a003_leftElbowIO: i16,
	pub a003_bothLegsIO: i16,
	pub a010_rightElbowIO: i16,
	pub a010_leftElbowIO: i16,
	pub a010_bothLegsIO: i16,
	pub a012_rightElbowIO: i16,
	pub a012_leftElbowIO: i16,
	pub a012_bothLegsIO: i16,
	pub a013_rightElbowIO: i16,
	pub a013_leftElbowIO: i16,
	pub a013_bothLegsIO: i16,
	pub a014_rightElbowIO: i16,
	pub a014_leftElbowIO: i16,
	pub a014_bothLegsIO: i16,
	pub a015_rightElbowIO: i16,
	pub a015_leftElbowIO: i16,
	pub a015_bothLegsIO: i16,
	pub a016_rightElbowIO: i16,
	pub a016_leftElbowIO: i16,
	pub a016_bothLegsIO: i16,
	pub pad: [u8;10],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl POSTURE_CONTROL_PARAM_GENDER_ST {
}
impl Default for POSTURE_CONTROL_PARAM_GENDER_ST {
	fn default() -> Self {
		Self {
			 a000_rightElbowIO: 0,
			 a000_leftElbowIO: 0,
			 a000_bothLegsIO: 0,
			 a002_rightElbowIO: 0,
			 a002_leftElbowIO: 0,
			 a002_bothLegsIO: 0,
			 a003_rightElbowIO: 0,
			 a003_leftElbowIO: 0,
			 a003_bothLegsIO: 0,
			 a010_rightElbowIO: 0,
			 a010_leftElbowIO: 0,
			 a010_bothLegsIO: 0,
			 a012_rightElbowIO: 0,
			 a012_leftElbowIO: 0,
			 a012_bothLegsIO: 0,
			 a013_rightElbowIO: 0,
			 a013_leftElbowIO: 0,
			 a013_bothLegsIO: 0,
			 a014_rightElbowIO: 0,
			 a014_leftElbowIO: 0,
			 a014_bothLegsIO: 0,
			 a015_rightElbowIO: 0,
			 a015_leftElbowIO: 0,
			 a015_bothLegsIO: 0,
			 a016_rightElbowIO: 0,
			 a016_leftElbowIO: 0,
			 a016_bothLegsIO: 0,
			 pad: [0;10],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct POSTURE_CONTROL_PARAM_PRO_ST {
	pub a000_rightArmIO: i16,
	pub a000_rightArmFB: i16,
	pub a000_leftArmIO: i16,
	pub a000_leftArmFB: i16,
	pub a002_rightArmIO: i16,
	pub a002_rightArmFB: i16,
	pub a002_leftArmIO: i16,
	pub a002_leftArmFB: i16,
	pub a003_rightArmIO: i16,
	pub a003_rightArmFB: i16,
	pub a003_leftArmIO: i16,
	pub a003_leftArmFB: i16,
	pub a010_rightArmIO: i16,
	pub a010_rightArmFB: i16,
	pub a010_leftArmIO: i16,
	pub a010_leftArmFB: i16,
	pub a012_rightArmIO: i16,
	pub a012_rightArmFB: i16,
	pub a012_leftArmIO: i16,
	pub a012_leftArmFB: i16,
	pub a013_rightArmIO: i16,
	pub a013_rightArmFB: i16,
	pub a013_leftArmIO: i16,
	pub a013_leftArmFB: i16,
	pub a014_rightArmIO: i16,
	pub a014_rightArmFB: i16,
	pub a014_leftArmIO: i16,
	pub a014_leftArmFB: i16,
	pub a015_rightArmIO: i16,
	pub a015_rightArmFB: i16,
	pub a015_leftArmIO: i16,
	pub a015_leftArmFB: i16,
	pub a016_rightArmIO: i16,
	pub a016_rightArmFB: i16,
	pub a016_leftArmIO: i16,
	pub a016_leftArmFB: i16,
	pub pad: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl POSTURE_CONTROL_PARAM_PRO_ST {
}
impl Default for POSTURE_CONTROL_PARAM_PRO_ST {
	fn default() -> Self {
		Self {
			 a000_rightArmIO: 0,
			 a000_rightArmFB: 0,
			 a000_leftArmIO: 0,
			 a000_leftArmFB: 0,
			 a002_rightArmIO: 0,
			 a002_rightArmFB: 0,
			 a002_leftArmIO: 0,
			 a002_leftArmFB: 0,
			 a003_rightArmIO: 0,
			 a003_rightArmFB: 0,
			 a003_leftArmIO: 0,
			 a003_leftArmFB: 0,
			 a010_rightArmIO: 0,
			 a010_rightArmFB: 0,
			 a010_leftArmIO: 0,
			 a010_leftArmFB: 0,
			 a012_rightArmIO: 0,
			 a012_rightArmFB: 0,
			 a012_leftArmIO: 0,
			 a012_leftArmFB: 0,
			 a013_rightArmIO: 0,
			 a013_rightArmFB: 0,
			 a013_leftArmIO: 0,
			 a013_leftArmFB: 0,
			 a014_rightArmIO: 0,
			 a014_rightArmFB: 0,
			 a014_leftArmIO: 0,
			 a014_leftArmFB: 0,
			 a015_rightArmIO: 0,
			 a015_rightArmFB: 0,
			 a015_leftArmIO: 0,
			 a015_leftArmFB: 0,
			 a016_rightArmIO: 0,
			 a016_rightArmFB: 0,
			 a016_leftArmIO: 0,
			 a016_leftArmFB: 0,
			 pad: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct POSTURE_CONTROL_PARAM_WEP_LEFT_ST {
	pub a000_leftArmFB: i16,
	pub a000_leftWristFB: i16,
	pub a000_leftWristIO: i16,
	pub a002_leftArmFB: i16,
	pub a002_leftWristFB: i16,
	pub a002_leftWristIO: i16,
	pub a003_leftArmFB: i16,
	pub a003_leftWristFB: i16,
	pub a003_leftWristIO: i16,
	pub pad: [u8;14],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl POSTURE_CONTROL_PARAM_WEP_LEFT_ST {
}
impl Default for POSTURE_CONTROL_PARAM_WEP_LEFT_ST {
	fn default() -> Self {
		Self {
			 a000_leftArmFB: 0,
			 a000_leftWristFB: 0,
			 a000_leftWristIO: 0,
			 a002_leftArmFB: 0,
			 a002_leftWristFB: 0,
			 a002_leftWristIO: 0,
			 a003_leftArmFB: 0,
			 a003_leftWristFB: 0,
			 a003_leftWristIO: 0,
			 pad: [0;14],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct POSTURE_CONTROL_PARAM_WEP_RIGHT_ST {
	pub a000_rightArmFB: i16,
	pub a000_rightWristFB: i16,
	pub a000_rightWristIO: i16,
	pub a000_leftArmFB: i16,
	pub a000_leftWristFB: i16,
	pub a000_leftWristIO: i16,
	pub a002_rightArmFB: i16,
	pub a002_rightWristFB: i16,
	pub a002_rightWristIO: i16,
	pub a002_leftArmFB: i16,
	pub a002_leftWristFB: i16,
	pub a002_leftWristIO: i16,
	pub a003_rightArmFB: i16,
	pub a003_rightWristFB: i16,
	pub a003_rightWristIO: i16,
	pub a003_leftArmFB: i16,
	pub a003_leftWristFB: i16,
	pub a003_leftWristIO: i16,
	pub a010_rightArmFB: i16,
	pub a010_rightWristFB: i16,
	pub a010_rightWristIO: i16,
	pub a010_leftArmFB: i16,
	pub a010_leftWristFB: i16,
	pub a010_leftWristIO: i16,
	pub a012_rightArmFB: i16,
	pub a012_rightWristFB: i16,
	pub a012_rightWristIO: i16,
	pub a012_leftArmFB: i16,
	pub a012_leftWristFB: i16,
	pub a012_leftWristIO: i16,
	pub a013_rightArmFB: i16,
	pub a013_rightWristFB: i16,
	pub a013_rightWristIO: i16,
	pub a013_leftArmFB: i16,
	pub a013_leftWristFB: i16,
	pub a013_leftWristIO: i16,
	pub a014_rightArmFB: i16,
	pub a014_rightWristFB: i16,
	pub a014_rightWristIO: i16,
	pub a014_leftArmFB: i16,
	pub a014_leftWristFB: i16,
	pub a014_leftWristIO: i16,
	pub a015_rightArmFB: i16,
	pub a015_rightWristFB: i16,
	pub a015_rightWristIO: i16,
	pub a015_leftArmFB: i16,
	pub a015_leftWristFB: i16,
	pub a015_leftWristIO: i16,
	pub a016_rightArmFB: i16,
	pub a016_rightWristFB: i16,
	pub a016_rightWristIO: i16,
	pub a016_leftArmFB: i16,
	pub a016_leftWristFB: i16,
	pub a016_leftWristIO: i16,
	pub pad: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl POSTURE_CONTROL_PARAM_WEP_RIGHT_ST {
}
impl Default for POSTURE_CONTROL_PARAM_WEP_RIGHT_ST {
	fn default() -> Self {
		Self {
			 a000_rightArmFB: 0,
			 a000_rightWristFB: 0,
			 a000_rightWristIO: 0,
			 a000_leftArmFB: 0,
			 a000_leftWristFB: 0,
			 a000_leftWristIO: 0,
			 a002_rightArmFB: 0,
			 a002_rightWristFB: 0,
			 a002_rightWristIO: 0,
			 a002_leftArmFB: 0,
			 a002_leftWristFB: 0,
			 a002_leftWristIO: 0,
			 a003_rightArmFB: 0,
			 a003_rightWristFB: 0,
			 a003_rightWristIO: 0,
			 a003_leftArmFB: 0,
			 a003_leftWristFB: 0,
			 a003_leftWristIO: 0,
			 a010_rightArmFB: 0,
			 a010_rightWristFB: 0,
			 a010_rightWristIO: 0,
			 a010_leftArmFB: 0,
			 a010_leftWristFB: 0,
			 a010_leftWristIO: 0,
			 a012_rightArmFB: 0,
			 a012_rightWristFB: 0,
			 a012_rightWristIO: 0,
			 a012_leftArmFB: 0,
			 a012_leftWristFB: 0,
			 a012_leftWristIO: 0,
			 a013_rightArmFB: 0,
			 a013_rightWristFB: 0,
			 a013_rightWristIO: 0,
			 a013_leftArmFB: 0,
			 a013_leftWristFB: 0,
			 a013_leftWristIO: 0,
			 a014_rightArmFB: 0,
			 a014_rightWristFB: 0,
			 a014_rightWristIO: 0,
			 a014_leftArmFB: 0,
			 a014_leftWristFB: 0,
			 a014_leftWristIO: 0,
			 a015_rightArmFB: 0,
			 a015_rightWristFB: 0,
			 a015_rightWristIO: 0,
			 a015_leftArmFB: 0,
			 a015_leftWristFB: 0,
			 a015_leftWristIO: 0,
			 a016_rightArmFB: 0,
			 a016_rightWristFB: 0,
			 a016_rightWristIO: 0,
			 a016_leftArmFB: 0,
			 a016_leftWristFB: 0,
			 a016_leftWristIO: 0,
			 pad: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct RANDOM_APPEAR_EDIT_PARAM_ST {
	pub appearNum: i32,
	pub paramId1: i32,
	pub rate1: i32,
	pub paramId2: i32,
	pub rate2: i32,
	pub paramId3: i32,
	pub rate3: i32,
	pub paramId4: i32,
	pub rate4: i32,
	pub paramId5: i32,
	pub rate5: i32,
	pub paramId6: i32,
	pub rate6: i32,
	pub paramId7: i32,
	pub rate7: i32,
	pub paramId8: i32,
	pub rate8: i32,
	pub paramId9: i32,
	pub rate9: i32,
	pub paramId10: i32,
	pub rate10: i32,
	pub paramId11: i32,
	pub rate11: i32,
	pub paramId12: i32,
	pub rate12: i32,
	pub paramId13: i32,
	pub rate13: i32,
	pub paramId14: i32,
	pub rate14: i32,
	pub paramId15: i32,
	pub rate15: i32,
	pub paramId16: i32,
	pub rate16: i32,
	pub paramId17: i32,
	pub rate17: i32,
	pub paramId18: i32,
	pub rate18: i32,
	pub paramId19: i32,
	pub rate19: i32,
	pub paramId20: i32,
	pub rate20: i32,
	pub paramId21: i32,
	pub rate21: i32,
	pub paramId22: i32,
	pub rate22: i32,
	pub paramId23: i32,
	pub rate23: i32,
	pub paramId24: i32,
	pub rate24: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl RANDOM_APPEAR_EDIT_PARAM_ST {
}
impl Default for RANDOM_APPEAR_EDIT_PARAM_ST {
	fn default() -> Self {
		Self {
			 appearNum: 0,
			 paramId1: -1,
			 rate1: 0,
			 paramId2: -1,
			 rate2: 0,
			 paramId3: -1,
			 rate3: 0,
			 paramId4: -1,
			 rate4: 0,
			 paramId5: -1,
			 rate5: 0,
			 paramId6: -1,
			 rate6: 0,
			 paramId7: -1,
			 rate7: 0,
			 paramId8: -1,
			 rate8: 0,
			 paramId9: -1,
			 rate9: 0,
			 paramId10: -1,
			 rate10: 0,
			 paramId11: -1,
			 rate11: 0,
			 paramId12: -1,
			 rate12: 0,
			 paramId13: -1,
			 rate13: 0,
			 paramId14: -1,
			 rate14: 0,
			 paramId15: -1,
			 rate15: 0,
			 paramId16: -1,
			 rate16: 0,
			 paramId17: -1,
			 rate17: 0,
			 paramId18: -1,
			 rate18: 0,
			 paramId19: -1,
			 rate19: 0,
			 paramId20: -1,
			 rate20: 0,
			 paramId21: -1,
			 rate21: 0,
			 paramId22: -1,
			 rate22: 0,
			 paramId23: -1,
			 rate23: 0,
			 paramId24: -1,
			 rate24: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct RANDOM_APPEAR_PARAM_ST {
	bits_0: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	bits_4: u8,
	bits_5: u8,
	bits_6: u8,
	bits_7: u8,
	bits_8: u8,
	bits_9: u8,
	bits_10: u8,
	bits_11: u8,
	bits_12: u8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl RANDOM_APPEAR_PARAM_ST {
	pub fn slot0(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn slot1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn slot2(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn slot3(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn slot4(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn slot5(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn slot6(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn slot7(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn slot8(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn slot9(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn slot10(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn slot11(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn slot12(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn slot13(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn slot14(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn slot15(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn slot16(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn slot17(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn slot18(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn slot19(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn slot20(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn slot21(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn slot22(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn slot23(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn slot24(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn slot25(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn slot26(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn slot27(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn slot28(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn slot29(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn slot30(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn slot31(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn slot32(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn slot33(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn slot34(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn slot35(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn slot36(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn slot37(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn slot38(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn slot39(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn slot40(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn slot41(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn slot42(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn slot43(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn slot44(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn slot45(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn slot46(&self) -> bool {	
			self.bits_5 & (1 << 6) != 0
	}
	pub fn slot47(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn slot48(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn slot49(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
	pub fn slot50(&self) -> bool {	
			self.bits_6 & (1 << 2) != 0
	}
	pub fn slot51(&self) -> bool {	
			self.bits_6 & (1 << 3) != 0
	}
	pub fn slot52(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn slot53(&self) -> bool {	
			self.bits_6 & (1 << 5) != 0
	}
	pub fn slot54(&self) -> bool {	
			self.bits_6 & (1 << 6) != 0
	}
	pub fn slot55(&self) -> bool {	
			self.bits_6 & (1 << 7) != 0
	}
	pub fn slot56(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn slot57(&self) -> bool {	
			self.bits_7 & (1 << 1) != 0
	}
	pub fn slot58(&self) -> bool {	
			self.bits_7 & (1 << 2) != 0
	}
	pub fn slot59(&self) -> bool {	
			self.bits_7 & (1 << 3) != 0
	}
	pub fn slot60(&self) -> bool {	
			self.bits_7 & (1 << 4) != 0
	}
	pub fn slot61(&self) -> bool {	
			self.bits_7 & (1 << 5) != 0
	}
	pub fn slot62(&self) -> bool {	
			self.bits_7 & (1 << 6) != 0
	}
	pub fn slot63(&self) -> bool {	
			self.bits_7 & (1 << 7) != 0
	}
	pub fn slot64(&self) -> bool {	
			self.bits_8 & (1 << 0) != 0
	}
	pub fn slot65(&self) -> bool {	
			self.bits_8 & (1 << 1) != 0
	}
	pub fn slot66(&self) -> bool {	
			self.bits_8 & (1 << 2) != 0
	}
	pub fn slot67(&self) -> bool {	
			self.bits_8 & (1 << 3) != 0
	}
	pub fn slot68(&self) -> bool {	
			self.bits_8 & (1 << 4) != 0
	}
	pub fn slot69(&self) -> bool {	
			self.bits_8 & (1 << 5) != 0
	}
	pub fn slot70(&self) -> bool {	
			self.bits_8 & (1 << 6) != 0
	}
	pub fn slot71(&self) -> bool {	
			self.bits_8 & (1 << 7) != 0
	}
	pub fn slot72(&self) -> bool {	
			self.bits_9 & (1 << 0) != 0
	}
	pub fn slot73(&self) -> bool {	
			self.bits_9 & (1 << 1) != 0
	}
	pub fn slot74(&self) -> bool {	
			self.bits_9 & (1 << 2) != 0
	}
	pub fn slot75(&self) -> bool {	
			self.bits_9 & (1 << 3) != 0
	}
	pub fn slot76(&self) -> bool {	
			self.bits_9 & (1 << 4) != 0
	}
	pub fn slot77(&self) -> bool {	
			self.bits_9 & (1 << 5) != 0
	}
	pub fn slot78(&self) -> bool {	
			self.bits_9 & (1 << 6) != 0
	}
	pub fn slot79(&self) -> bool {	
			self.bits_9 & (1 << 7) != 0
	}
	pub fn slot80(&self) -> bool {	
			self.bits_10 & (1 << 0) != 0
	}
	pub fn slot81(&self) -> bool {	
			self.bits_10 & (1 << 1) != 0
	}
	pub fn slot82(&self) -> bool {	
			self.bits_10 & (1 << 2) != 0
	}
	pub fn slot83(&self) -> bool {	
			self.bits_10 & (1 << 3) != 0
	}
	pub fn slot84(&self) -> bool {	
			self.bits_10 & (1 << 4) != 0
	}
	pub fn slot85(&self) -> bool {	
			self.bits_10 & (1 << 5) != 0
	}
	pub fn slot86(&self) -> bool {	
			self.bits_10 & (1 << 6) != 0
	}
	pub fn slot87(&self) -> bool {	
			self.bits_10 & (1 << 7) != 0
	}
	pub fn slot88(&self) -> bool {	
			self.bits_11 & (1 << 0) != 0
	}
	pub fn slot89(&self) -> bool {	
			self.bits_11 & (1 << 1) != 0
	}
	pub fn slot90(&self) -> bool {	
			self.bits_11 & (1 << 2) != 0
	}
	pub fn slot91(&self) -> bool {	
			self.bits_11 & (1 << 3) != 0
	}
	pub fn slot92(&self) -> bool {	
			self.bits_11 & (1 << 4) != 0
	}
	pub fn slot93(&self) -> bool {	
			self.bits_11 & (1 << 5) != 0
	}
	pub fn slot94(&self) -> bool {	
			self.bits_11 & (1 << 6) != 0
	}
	pub fn slot95(&self) -> bool {	
			self.bits_11 & (1 << 7) != 0
	}
	pub fn slot96(&self) -> bool {	
			self.bits_12 & (1 << 0) != 0
	}
	pub fn slot97(&self) -> bool {	
			self.bits_12 & (1 << 1) != 0
	}
	pub fn slot98(&self) -> bool {	
			self.bits_12 & (1 << 2) != 0
	}
	pub fn slot99(&self) -> bool {	
			self.bits_12 & (1 << 3) != 0
	}
	pub fn pad(&self) -> bool {	
			self.bits_12 & (1 << 4) != 0
	}
}
impl Default for RANDOM_APPEAR_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 bits_4: 0,
			 bits_5: 0,
			 bits_6: 0,
			 bits_7: 0,
			 bits_8: 0,
			 bits_9: 0,
			 bits_10: 0,
			 bits_11: 0,
			 bits_12: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct REINFORCE_PARAM_PROTECTOR_ST {
	pub physicsDefRate: f32,
	pub magicDefRate: f32,
	pub fireDefRate: f32,
	pub thunderDefRate: f32,
	pub slashDefRate: f32,
	pub blowDefRate: f32,
	pub thrustDefRate: f32,
	pub resistPoisonRate: f32,
	pub resistDiseaseRate: f32,
	pub resistBloodRate: f32,
	pub resistCurseRate: f32,
	pub residentSpEffectId1: u8,
	pub residentSpEffectId2: u8,
	pub residentSpEffectId3: u8,
	pub materialSetId: u8,
	pub darkDefRate: f32,
	pub resistFreezeRate: f32,
	pub resistSleepRate: f32,
	pub resistMadnessRate: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl REINFORCE_PARAM_PROTECTOR_ST {
}
impl Default for REINFORCE_PARAM_PROTECTOR_ST {
	fn default() -> Self {
		Self {
			 physicsDefRate: 1.,
			 magicDefRate: 1.,
			 fireDefRate: 1.,
			 thunderDefRate: 1.,
			 slashDefRate: 1.,
			 blowDefRate: 1.,
			 thrustDefRate: 1.,
			 resistPoisonRate: 1.,
			 resistDiseaseRate: 1.,
			 resistBloodRate: 1.,
			 resistCurseRate: 1.,
			 residentSpEffectId1: 0,
			 residentSpEffectId2: 0,
			 residentSpEffectId3: 0,
			 materialSetId: 0,
			 darkDefRate: 1.,
			 resistFreezeRate: 1.,
			 resistSleepRate: 1.,
			 resistMadnessRate: 1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct REINFORCE_PARAM_WEAPON_ST {
	pub physicsAtkRate: f32,
	pub magicAtkRate: f32,
	pub fireAtkRate: f32,
	pub thunderAtkRate: f32,
	pub staminaAtkRate: f32,
	pub saWeaponAtkRate: f32,
	pub saDurabilityRate: f32,
	pub correctStrengthRate: f32,
	pub correctAgilityRate: f32,
	pub correctMagicRate: f32,
	pub correctFaithRate: f32,
	pub physicsGuardCutRate: f32,
	pub magicGuardCutRate: f32,
	pub fireGuardCutRate: f32,
	pub thunderGuardCutRate: f32,
	pub poisonGuardResistRate: f32,
	pub diseaseGuardResistRate: f32,
	pub bloodGuardResistRate: f32,
	pub curseGuardResistRate: f32,
	pub staminaGuardDefRate: f32,
	pub spEffectId1: u8,
	pub spEffectId2: u8,
	pub spEffectId3: u8,
	pub residentSpEffectId1: u8,
	pub residentSpEffectId2: u8,
	pub residentSpEffectId3: u8,
	pub materialSetId: u8,
	pub maxReinforceLevel: u8,
	pub darkAtkRate: f32,
	pub darkGuardCutRate: f32,
	pub correctLuckRate: f32,
	pub freezeGuardDefRate: f32,
	pub reinforcePriceRate: f32,
	pub baseChangePriceRate: f32,
	pub enableGemRank: i8,
	pub pad2: [u8;3],
	pub sleepGuardDefRate: f32,
	pub madnessGuardDefRate: f32,
	pub baseAtkRate: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl REINFORCE_PARAM_WEAPON_ST {
}
impl Default for REINFORCE_PARAM_WEAPON_ST {
	fn default() -> Self {
		Self {
			 physicsAtkRate: 1.,
			 magicAtkRate: 1.,
			 fireAtkRate: 1.,
			 thunderAtkRate: 1.,
			 staminaAtkRate: 1.,
			 saWeaponAtkRate: 1.,
			 saDurabilityRate: 1.,
			 correctStrengthRate: 1.,
			 correctAgilityRate: 1.,
			 correctMagicRate: 1.,
			 correctFaithRate: 1.,
			 physicsGuardCutRate: 1.,
			 magicGuardCutRate: 1.,
			 fireGuardCutRate: 1.,
			 thunderGuardCutRate: 1.,
			 poisonGuardResistRate: 1.,
			 diseaseGuardResistRate: 1.,
			 bloodGuardResistRate: 1.,
			 curseGuardResistRate: 1.,
			 staminaGuardDefRate: 1.,
			 spEffectId1: 0,
			 spEffectId2: 0,
			 spEffectId3: 0,
			 residentSpEffectId1: 0,
			 residentSpEffectId2: 0,
			 residentSpEffectId3: 0,
			 materialSetId: 0,
			 maxReinforceLevel: 0,
			 darkAtkRate: 1.,
			 darkGuardCutRate: 1.,
			 correctLuckRate: 1.,
			 freezeGuardDefRate: 1.,
			 reinforcePriceRate: 1.,
			 baseChangePriceRate: 1.,
			 enableGemRank: 0,
			 pad2: [0;3],
			 sleepGuardDefRate: 1.,
			 madnessGuardDefRate: 1.,
			 baseAtkRate: 1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct RESIST_CORRECT_PARAM_ST {
	pub addPoint1: f32,
	pub addPoint2: f32,
	pub addPoint3: f32,
	pub addPoint4: f32,
	pub addPoint5: f32,
	pub addRate1: f32,
	pub addRate2: f32,
	pub addRate3: f32,
	pub addRate4: f32,
	pub addRate5: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl RESIST_CORRECT_PARAM_ST {
}
impl Default for RESIST_CORRECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 addPoint1: 0.,
			 addPoint2: 0.,
			 addPoint3: 0.,
			 addPoint4: 0.,
			 addPoint5: 0.,
			 addRate1: 1.,
			 addRate2: 1.,
			 addRate3: 1.,
			 addRate4: 1.,
			 addRate5: 1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct REVERB_AUX_SEND_BUS_PARAM_ST {
	pub ReverbAuxSendBusName: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl REVERB_AUX_SEND_BUS_PARAM_ST {
}
impl Default for REVERB_AUX_SEND_BUS_PARAM_ST {
	fn default() -> Self {
		Self {
			 ReverbAuxSendBusName: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct RIDE_PARAM_ST {
	pub atkChrId: i32,
	pub defChrId: i32,
	pub rideCamParamId: i32,
	pub atkChrAnimId: i32,
	pub defChrAnimId: i32,
	pub defAdjustDmyId: i32,
	pub defCheckDmyId: i32,
	pub diffAngMyToDef: f32,
	pub dist: f32,
	pub upperYRange: f32,
	pub lowerYRange: f32,
	pub diffAngMin: f32,
	pub diffAngMax: f32,
	pub pad: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl RIDE_PARAM_ST {
}
impl Default for RIDE_PARAM_ST {
	fn default() -> Self {
		Self {
			 atkChrId: 0,
			 defChrId: 0,
			 rideCamParamId: -1,
			 atkChrAnimId: 0,
			 defChrAnimId: 0,
			 defAdjustDmyId: -1,
			 defCheckDmyId: -1,
			 diffAngMyToDef: 0.,
			 dist: 0.,
			 upperYRange: 0.,
			 lowerYRange: 0.,
			 diffAngMin: 0.,
			 diffAngMax: 0.,
			 pad: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ROLE_PARAM_ST {
	pub teamType: u8,
	pub pad10: [u8;3],
	pub phantomParamId: i32,
	pub spEffectID0: i32,
	pub spEffectID1: i32,
	pub spEffectID2: i32,
	pub spEffectID3: i32,
	pub spEffectID4: i32,
	pub spEffectID5: i32,
	pub spEffectID6: i32,
	pub spEffectID7: i32,
	pub spEffectID8: i32,
	pub spEffectID9: i32,
	pub sosSignSfxId: i32,
	pub mySosSignSfxId: i32,
	pub summonStartAnimId: i32,
	pub itemlotParamId: i32,
	pub voiceChatGroup: u8,
	pub roleNameColor: u8,
	pub pad1: [u8;2],
	pub roleNameId: i32,
	pub threatLv: i32,
	pub phantomParamId_vowRank1: i32,
	pub phantomParamId_vowRank2: i32,
	pub phantomParamId_vowRank3: i32,
	pub spEffectID_vowRank0: i32,
	pub spEffectID_vowRank1: i32,
	pub spEffectID_vowRank2: i32,
	pub spEffectID_vowRank3: i32,
	pub signPhantomId: i32,
	pub nonPlayerSummonStartAnimId: i32,
	pub pad2: [u8;16],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ROLE_PARAM_ST {
}
impl Default for ROLE_PARAM_ST {
	fn default() -> Self {
		Self {
			 teamType: 0,
			 pad10: [0;3],
			 phantomParamId: -1,
			 spEffectID0: -1,
			 spEffectID1: -1,
			 spEffectID2: -1,
			 spEffectID3: -1,
			 spEffectID4: -1,
			 spEffectID5: -1,
			 spEffectID6: -1,
			 spEffectID7: -1,
			 spEffectID8: -1,
			 spEffectID9: -1,
			 sosSignSfxId: 0,
			 mySosSignSfxId: 0,
			 summonStartAnimId: 0,
			 itemlotParamId: -1,
			 voiceChatGroup: 0,
			 roleNameColor: 0,
			 pad1: [0;2],
			 roleNameId: 0,
			 threatLv: 0,
			 phantomParamId_vowRank1: -1,
			 phantomParamId_vowRank2: -1,
			 phantomParamId_vowRank3: -1,
			 spEffectID_vowRank0: -1,
			 spEffectID_vowRank1: -1,
			 spEffectID_vowRank2: -1,
			 spEffectID_vowRank3: -1,
			 signPhantomId: -1,
			 nonPlayerSummonStartAnimId: 0,
			 pad2: [0;16],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct ROLLING_OBJ_LOT_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub AssetId_0: i32,
	pub AssetId_1: i32,
	pub AssetId_2: i32,
	pub AssetId_3: i32,
	pub AssetId_4: i32,
	pub AssetId_5: i32,
	pub AssetId_6: i32,
	pub AssetId_7: i32,
	pub CreateWeight_0: u8,
	pub CreateWeight_1: u8,
	pub CreateWeight_2: u8,
	pub CreateWeight_3: u8,
	pub CreateWeight_4: u8,
	pub CreateWeight_5: u8,
	pub CreateWeight_6: u8,
	pub CreateWeight_7: u8,
	pub Reserve_0: [u8;20],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl ROLLING_OBJ_LOT_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for ROLLING_OBJ_LOT_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 AssetId_0: -1,
			 AssetId_1: -1,
			 AssetId_2: -1,
			 AssetId_3: -1,
			 AssetId_4: -1,
			 AssetId_5: -1,
			 AssetId_6: -1,
			 AssetId_7: -1,
			 CreateWeight_0: 0,
			 CreateWeight_1: 0,
			 CreateWeight_2: 0,
			 CreateWeight_3: 0,
			 CreateWeight_4: 0,
			 CreateWeight_5: 0,
			 CreateWeight_6: 0,
			 CreateWeight_7: 0,
			 Reserve_0: [0;20],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct RUNTIME_BONE_CONTROL_PARAM_ST {
	pub chrId: i32,
	pub ctrlType: u8,
	pub pad: [u8;11],
	pub applyBone: [u8;32],
	pub targetBone1: [u8;32],
	pub targetBone2: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl RUNTIME_BONE_CONTROL_PARAM_ST {
}
impl Default for RUNTIME_BONE_CONTROL_PARAM_ST {
	fn default() -> Self {
		Self {
			 chrId: 0,
			 ctrlType: 0,
			 pad: [0;11],
			 applyBone: [0;32],
			 targetBone1: [0;32],
			 targetBone2: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SE_ACTIVATION_RANGE_PARAM_ST {
	pub activateRange: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SE_ACTIVATION_RANGE_PARAM_ST {
}
impl Default for SE_ACTIVATION_RANGE_PARAM_ST {
	fn default() -> Self {
		Self {
			 activateRange: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SE_MATERIAL_CONVERT_PARAM_ST {
	pub seMaterialId: u8,
	pub pad: [u8;3],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SE_MATERIAL_CONVERT_PARAM_ST {
}
impl Default for SE_MATERIAL_CONVERT_PARAM_ST {
	fn default() -> Self {
		Self {
			 seMaterialId: 0,
			 pad: [0;3],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SFX_BLOCK_RES_SHARE_PARAM {
	pub shareBlockRsMapUidVal: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SFX_BLOCK_RES_SHARE_PARAM {
}
impl Default for SFX_BLOCK_RES_SHARE_PARAM {
	fn default() -> Self {
		Self {
			 shareBlockRsMapUidVal: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SHOP_LINEUP_PARAM {
	pub equipId: i32,
	pub value: i32,
	pub mtrlId: i32,
	pub eventFlag_forStock: i32,
	pub eventFlag_forRelease: i32,
	pub sellQuantity: i16,
	pub pad3: [u8;1],
	pub equipType: u8,
	pub costType: u8,
	pub pad1: [u8;1],
	pub setNum: i16,
	pub value_Add: i32,
	pub value_Magnification: f32,
	pub iconId: i32,
	pub nameMsgId: i32,
	pub menuTitleMsgId: i32,
	pub menuIconId: i16,
	pub pad2: [u8;2],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SHOP_LINEUP_PARAM {
}
impl Default for SHOP_LINEUP_PARAM {
	fn default() -> Self {
		Self {
			 equipId: 0,
			 value: -1,
			 mtrlId: -1,
			 eventFlag_forStock: 0,
			 eventFlag_forRelease: 0,
			 sellQuantity: -1,
			 pad3: [0;1],
			 equipType: 0,
			 costType: 0,
			 pad1: [0;1],
			 setNum: 1,
			 value_Add: 0,
			 value_Magnification: 1.,
			 iconId: -1,
			 nameMsgId: -1,
			 menuTitleMsgId: -1,
			 menuIconId: -1,
			 pad2: [0;2],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SIGN_PUDDLE_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub matchAreaId: i32,
	pub pad1: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SIGN_PUDDLE_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for SIGN_PUDDLE_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 matchAreaId: 0,
			 pad1: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {
	pub SoundObjEnableDist: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {
}
impl Default for SOUND_ASSET_SOUND_OBJ_ENABLE_DIST_PARAM_ST {
	fn default() -> Self {
		Self {
			 SoundObjEnableDist: -1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
	pub SoundNo: i32,
	pub ExpandRange: f32,
	pub FollowSpeed: f32,
	pub FollowRate: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
}
impl Default for SOUND_AUTO_ENV_SOUND_GROUP_PARAM_ST {
	fn default() -> Self {
		Self {
			 SoundNo: -1,
			 ExpandRange: 100.,
			 FollowSpeed: 0.1,
			 FollowRate: 0.015,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST {
	pub NoHitDist: f32,
	pub isCollectNoHitPoint: u8,
	pub isCollectOutdoorPoint: u8,
	pub isCollectFloorPoint: u8,
	pub distValCalcType: u8,
	pub enableLifeTime: f32,
	pub maxDistRecordNum: i32,
	pub ignoreDistNumForMax: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST {
}
impl Default for SOUND_AUTO_REVERB_EVALUATION_DIST_PARAM_ST {
	fn default() -> Self {
		Self {
			 NoHitDist: -1.,
			 isCollectNoHitPoint: 0,
			 isCollectOutdoorPoint: 0,
			 isCollectFloorPoint: 0,
			 distValCalcType: 0,
			 enableLifeTime: -1.,
			 maxDistRecordNum: 20,
			 ignoreDistNumForMax: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_AUTO_REVERB_SELECT_PARAM_ST {
	pub reverbType: i32,
	pub AreaNo: i32,
	pub IndoorOutdoor: i8,
	pub useDistNoA: i8,
	pub useDistNoB: i8,
	pub pad0: [u8;1],
	pub DistMinA: f32,
	pub DistMaxA: f32,
	pub DistMinB: f32,
	pub DistMaxB: f32,
	pub NoHitNumMin: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_AUTO_REVERB_SELECT_PARAM_ST {
}
impl Default for SOUND_AUTO_REVERB_SELECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 reverbType: 0,
			 AreaNo: -1,
			 IndoorOutdoor: -1,
			 useDistNoA: -1,
			 useDistNoB: -1,
			 pad0: [0;1],
			 DistMinA: -1.,
			 DistMaxA: -1.,
			 DistMinB: -1.,
			 DistMaxB: -1.,
			 NoHitNumMin: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_CHR_PHYSICS_SE_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub ContactLandSeId: i32,
	pub ContactLandAddSeId: i32,
	pub ContactLandPlayNum: i32,
	pub IsEnablePlayCountPerRigid: u8,
	pub pad: [u8;3],
	pub ContactLandMinImpuse: f32,
	pub ContactLandMinSpeed: f32,
	pub ContactPlayerSeId: i32,
	pub ContactPlayerMinImpuse: f32,
	pub ContactPlayerMinSpeed: f32,
	pub ContactCheckRigidIdx0: i8,
	pub ContactCheckRigidIdx1: i8,
	pub ContactCheckRigidIdx2: i8,
	pub ContactCheckRigidIdx3: i8,
	pub ContactCheckRigidIdx4: i8,
	pub ContactCheckRigidIdx5: i8,
	pub ContactCheckRigidIdx6: i8,
	pub ContactCheckRigidIdx7: i8,
	pub ContactCheckRigidIdx8: i8,
	pub ContactCheckRigidIdx9: i8,
	pub ContactCheckRigidIdx10: i8,
	pub ContactCheckRigidIdx11: i8,
	pub ContactCheckRigidIdx12: i8,
	pub ContactCheckRigidIdx13: i8,
	pub ContactCheckRigidIdx14: i8,
	pub ContactCheckRigidIdx15: i8,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_CHR_PHYSICS_SE_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for SOUND_CHR_PHYSICS_SE_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 ContactLandSeId: -1,
			 ContactLandAddSeId: -1,
			 ContactLandPlayNum: 1,
			 IsEnablePlayCountPerRigid: 0,
			 pad: [0;3],
			 ContactLandMinImpuse: 20.,
			 ContactLandMinSpeed: 0.,
			 ContactPlayerSeId: -1,
			 ContactPlayerMinImpuse: 20.,
			 ContactPlayerMinSpeed: 0.,
			 ContactCheckRigidIdx0: -1,
			 ContactCheckRigidIdx1: -1,
			 ContactCheckRigidIdx2: -1,
			 ContactCheckRigidIdx3: -1,
			 ContactCheckRigidIdx4: -1,
			 ContactCheckRigidIdx5: -1,
			 ContactCheckRigidIdx6: -1,
			 ContactCheckRigidIdx7: -1,
			 ContactCheckRigidIdx8: -1,
			 ContactCheckRigidIdx9: -1,
			 ContactCheckRigidIdx10: -1,
			 ContactCheckRigidIdx11: -1,
			 ContactCheckRigidIdx12: -1,
			 ContactCheckRigidIdx13: -1,
			 ContactCheckRigidIdx14: -1,
			 ContactCheckRigidIdx15: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_COMMON_INGAME_PARAM_ST {
	pub ParamKeyStr: [u8;32],
	pub ParamValueStr: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_COMMON_INGAME_PARAM_ST {
}
impl Default for SOUND_COMMON_INGAME_PARAM_ST {
	fn default() -> Self {
		Self {
			 ParamKeyStr: [0;32],
			 ParamValueStr: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_COMMON_SYSTEM_PARAM_ST {
	pub ParamKeyStr: [u8;32],
	pub ParamValueStr: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_COMMON_SYSTEM_PARAM_ST {
}
impl Default for SOUND_COMMON_SYSTEM_PARAM_ST {
	fn default() -> Self {
		Self {
			 ParamKeyStr: [0;32],
			 ParamValueStr: [0;32],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SOUND_CUTSCENE_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub ReverbType: u8,
	pub pad0: [u8;3],
	pub BgmBehaviorTypeOnStart: i16,
	pub OneShotBgmBehaviorOnStart: i16,
	pub PostPlaySeId: i32,
	pub PostPlaySeIdForSkip: i32,
	pub EnterMapMuteStopTimeOnDrawCutscene: f32,
	pub reserved: [u8;8],
	pub reserved2: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SOUND_CUTSCENE_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for SOUND_CUTSCENE_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 ReverbType: 0,
			 pad0: [0;3],
			 BgmBehaviorTypeOnStart: 0,
			 OneShotBgmBehaviorOnStart: 0,
			 PostPlaySeId: -1,
			 PostPlaySeIdForSkip: -1,
			 EnterMapMuteStopTimeOnDrawCutscene: -1.,
			 reserved: [0;8],
			 reserved2: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SPEEDTREE_MODEL_PARAM_ST {
	pub MinFadeLeaf: f32,
	pub MinFadeFrond: f32,
	pub MinFadeBranch: f32,
	pub MinTranslucencyLeaf: f32,
	pub MaxTranslucencyLeaf: f32,
	pub MinTranslucencyFrond: f32,
	pub MaxTranslucencyFrond: f32,
	pub MinTranslucencyBranch: f32,
	pub MaxTranslucencyBranch: f32,
	pub BillboardBackSpecularWeakenParam: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SPEEDTREE_MODEL_PARAM_ST {
}
impl Default for SPEEDTREE_MODEL_PARAM_ST {
	fn default() -> Self {
		Self {
			 MinFadeLeaf: 0.,
			 MinFadeFrond: 0.,
			 MinFadeBranch: 0.,
			 MinTranslucencyLeaf: 0.,
			 MaxTranslucencyLeaf: 5.,
			 MinTranslucencyFrond: 0.,
			 MaxTranslucencyFrond: 5.,
			 MinTranslucencyBranch: 0.,
			 MaxTranslucencyBranch: 5.,
			 BillboardBackSpecularWeakenParam: 1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SP_EFFECT_PARAM_ST {
	pub iconId: i32,
	pub conditionHp: f32,
	pub effectEndurance: f32,
	pub motionInterval: f32,
	pub maxHpRate: f32,
	pub maxMpRate: f32,
	pub maxStaminaRate: f32,
	pub slashDamageCutRate: f32,
	pub blowDamageCutRate: f32,
	pub thrustDamageCutRate: f32,
	pub neutralDamageCutRate: f32,
	pub magicDamageCutRate: f32,
	pub fireDamageCutRate: f32,
	pub thunderDamageCutRate: f32,
	pub physicsAttackRate: f32,
	pub magicAttackRate: f32,
	pub fireAttackRate: f32,
	pub thunderAttackRate: f32,
	pub physicsAttackPowerRate: f32,
	pub magicAttackPowerRate: f32,
	pub fireAttackPowerRate: f32,
	pub thunderAttackPowerRate: f32,
	pub physicsAttackPower: i32,
	pub magicAttackPower: i32,
	pub fireAttackPower: i32,
	pub thunderAttackPower: i32,
	pub physicsDiffenceRate: f32,
	pub magicDiffenceRate: f32,
	pub fireDiffenceRate: f32,
	pub thunderDiffenceRate: f32,
	pub physicsDiffence: i32,
	pub magicDiffence: i32,
	pub fireDiffence: i32,
	pub thunderDiffence: i32,
	pub NoGuardDamageRate: f32,
	pub vitalSpotChangeRate: f32,
	pub normalSpotChangeRate: f32,
	pub lookAtTargetPosOffset: f32,
	pub behaviorId: i32,
	pub changeHpRate: f32,
	pub changeHpPoint: i32,
	pub changeMpRate: f32,
	pub changeMpPoint: i32,
	pub mpRecoverChangeSpeed: i32,
	pub changeStaminaRate: f32,
	pub changeStaminaPoint: i32,
	pub staminaRecoverChangeSpeed: i32,
	pub magicEffectTimeChange: f32,
	pub insideDurability: i32,
	pub maxDurability: i32,
	pub staminaAttackRate: f32,
	pub poizonAttackPower: i32,
	pub diseaseAttackPower: i32,
	pub bloodAttackPower: i32,
	pub curseAttackPower: i32,
	pub fallDamageRate: f32,
	pub soulRate: f32,
	pub equipWeightChangeRate: f32,
	pub allItemWeightChangeRate: f32,
	pub soul: i32,
	pub animIdOffset: i32,
	pub haveSoulRate: f32,
	pub targetPriority: f32,
	pub sightSearchEnemyRate: f32,
	pub hearingSearchEnemyRate: f32,
	pub grabityRate: f32,
	pub registPoizonChangeRate: f32,
	pub registDiseaseChangeRate: f32,
	pub registBloodChangeRate: f32,
	pub registCurseChangeRate: f32,
	pub soulStealRate: f32,
	pub lifeReductionRate: f32,
	pub hpRecoverRate: f32,
	pub replaceSpEffectId: i32,
	pub cycleOccurrenceSpEffectId: i32,
	pub atkOccurrenceSpEffectId: i32,
	pub guardDefFlickPowerRate: f32,
	pub guardStaminaCutRate: f32,
	pub rayCastPassedTime: i16,
	pub magicSubCategoryChange1: u8,
	pub magicSubCategoryChange2: u8,
	pub bowDistRate: i16,
	pub spCategory: i16,
	pub categoryPriority: u8,
	pub saveCategory: i8,
	pub changeMagicSlot: u8,
	pub changeMiracleSlot: u8,
	pub heroPointDamage: i8,
	pub defFlickPower: u8,
	pub flickDamageCutRate: u8,
	pub bloodDamageRate: u8,
	pub dmgLv_None: i8,
	pub dmgLv_S: i8,
	pub dmgLv_M: i8,
	pub dmgLv_L: i8,
	pub dmgLv_BlowM: i8,
	pub dmgLv_Push: i8,
	pub dmgLv_Strike: i8,
	pub dmgLv_BlowS: i8,
	pub dmgLv_Min: i8,
	pub dmgLv_Uppercut: i8,
	pub dmgLv_BlowLL: i8,
	pub dmgLv_Breath: i8,
	pub atkAttribute: u8,
	pub spAttribute: u8,
	pub stateInfo: i16,
	pub wepParamChange: u8,
	pub moveType: u8,
	pub lifeReductionType: i16,
	pub throwCondition: u8,
	pub addBehaviorJudgeId_condition: i8,
	pub freezeDamageRate: u8,
	bits_0: u8,
	bits_1: u8,
	bits_2: u8,
	bits_3: u8,
	bits_4: u8,
	bits_5: u8,
	bits_6: u8,
	bits_7: u8,
	pub repAtkDmgLv: i8,
	pub sightSearchRate: f32,
	bits_8: u8,
	pub changeTeamType: i8,
	pub dmypolyId: i16,
	pub vfxId: i32,
	pub accumuOverFireId: i32,
	pub accumuOverVal: i32,
	pub accumuUnderFireId: i32,
	pub accumuUnderVal: i32,
	pub accumuVal: i32,
	pub eye_angX: u8,
	pub eye_angY: u8,
	pub addDeceasedLv: i16,
	pub vfxId1: i32,
	pub vfxId2: i32,
	pub vfxId3: i32,
	pub vfxId4: i32,
	pub vfxId5: i32,
	pub vfxId6: i32,
	pub vfxId7: i32,
	pub freezeAttackPower: i32,
	pub AppearAiSoundId: i32,
	pub addFootEffectSfxId: i16,
	pub dexterityCancelSystemOnlyAddDexterity: i8,
	pub teamOffenseEffectivity: i8,
	pub toughnessDamageCutRate: f32,
	pub weakDmgRateA: f32,
	pub weakDmgRateB: f32,
	pub weakDmgRateC: f32,
	pub weakDmgRateD: f32,
	pub weakDmgRateE: f32,
	pub weakDmgRateF: f32,
	pub darkDamageCutRate: f32,
	pub darkDiffenceRate: f32,
	pub darkDiffence: i32,
	pub darkAttackRate: f32,
	pub darkAttackPowerRate: f32,
	pub darkAttackPower: i32,
	pub antiDarkSightRadius: f32,
	pub antiDarkSightDmypolyId: i32,
	pub conditionHpRate: f32,
	pub consumeStaminaRate: f32,
	pub itemDropRate: f32,
	pub changePoisonResistPoint: i32,
	pub changeDiseaseResistPoint: i32,
	pub changeBloodResistPoint: i32,
	pub changeCurseResistPoint: i32,
	pub changeFreezeResistPoint: i32,
	pub slashAttackRate: f32,
	pub blowAttackRate: f32,
	pub thrustAttackRate: f32,
	pub neutralAttackRate: f32,
	pub slashAttackPowerRate: f32,
	pub blowAttackPowerRate: f32,
	pub thrustAttackPowerRate: f32,
	pub neutralAttackPowerRate: f32,
	pub slashAttackPower: i32,
	pub blowAttackPower: i32,
	pub thrustAttackPower: i32,
	pub neutralAttackPower: i32,
	pub changeStrengthPoint: i32,
	pub changeAgilityPoint: i32,
	pub changeMagicPoint: i32,
	pub changeFaithPoint: i32,
	pub changeLuckPoint: i32,
	pub recoverArtsPoint_Str: i8,
	pub recoverArtsPoint_Dex: i8,
	pub recoverArtsPoint_Magic: i8,
	pub recoverArtsPoint_Miracle: i8,
	pub madnessDamageRate: u8,
	bits_9: u8,
	pub addBehaviorJudgeId_add: i16,
	pub saReceiveDamageRate: f32,
	pub defPlayerDmgCorrectRate_Physics: f32,
	pub defPlayerDmgCorrectRate_Magic: f32,
	pub defPlayerDmgCorrectRate_Fire: f32,
	pub defPlayerDmgCorrectRate_Thunder: f32,
	pub defPlayerDmgCorrectRate_Dark: f32,
	pub defEnemyDmgCorrectRate_Physics: f32,
	pub defEnemyDmgCorrectRate_Magic: f32,
	pub defEnemyDmgCorrectRate_Fire: f32,
	pub defEnemyDmgCorrectRate_Thunder: f32,
	pub defEnemyDmgCorrectRate_Dark: f32,
	pub defObjDmgCorrectRate: f32,
	pub atkPlayerDmgCorrectRate_Physics: f32,
	pub atkPlayerDmgCorrectRate_Magic: f32,
	pub atkPlayerDmgCorrectRate_Fire: f32,
	pub atkPlayerDmgCorrectRate_Thunder: f32,
	pub atkPlayerDmgCorrectRate_Dark: f32,
	pub atkEnemyDmgCorrectRate_Physics: f32,
	pub atkEnemyDmgCorrectRate_Magic: f32,
	pub atkEnemyDmgCorrectRate_Fire: f32,
	pub atkEnemyDmgCorrectRate_Thunder: f32,
	pub atkEnemyDmgCorrectRate_Dark: f32,
	pub registFreezeChangeRate: f32,
	pub invocationConditionsStateChange1: i16,
	pub invocationConditionsStateChange2: i16,
	pub invocationConditionsStateChange3: i16,
	pub hearingAiSoundLevel: i16,
	pub chrProxyHeightRate: f32,
	pub addAwarePointCorrectValue_forMe: f32,
	pub addAwarePointCorrectValue_forTarget: f32,
	pub sightSearchEnemyAdd: f32,
	pub sightSearchAdd: f32,
	pub hearingSearchAdd: f32,
	pub hearingSearchRate: f32,
	pub hearingSearchEnemyAdd: f32,
	pub value_Magnification: f32,
	pub artsConsumptionRate: f32,
	pub magicConsumptionRate: f32,
	pub shamanConsumptionRate: f32,
	pub miracleConsumptionRate: f32,
	pub changeHpEstusFlaskRate: i32,
	pub changeHpEstusFlaskPoint: i32,
	pub changeMpEstusFlaskRate: i32,
	pub changeMpEstusFlaskPoint: i32,
	pub changeHpEstusFlaskCorrectRate: f32,
	pub changeMpEstusFlaskCorrectRate: f32,
	pub applyIdOnGetSoul: i32,
	pub extendLifeRate: f32,
	pub contractLifeRate: f32,
	pub defObjectAttackPowerRate: f32,
	pub effectEndDeleteDecalGroupId: i16,
	pub addLifeForceStatus: i8,
	pub addWillpowerStatus: i8,
	pub addEndureStatus: i8,
	pub addVitalityStatus: i8,
	pub addStrengthStatus: i8,
	pub addDexterityStatus: i8,
	pub addMagicStatus: i8,
	pub addFaithStatus: i8,
	pub addLuckStatus: i8,
	pub deleteCriteriaDamage: u8,
	pub magicSubCategoryChange3: u8,
	pub spAttributeVariationValue: u8,
	pub atkFlickPower: u8,
	pub wetConditionDepth: u8,
	pub changeSaRecoveryVelocity: f32,
	pub regainRate: f32,
	pub saAttackPowerRate: f32,
	pub sleepAttackPower: i32,
	pub madnessAttackPower: i32,
	pub registSleepChangeRate: f32,
	pub registMadnessChangeRate: f32,
	pub changeSleepResistPoint: i32,
	pub changeMadnessResistPoint: i32,
	pub sleepDamageRate: u8,
	pub applyPartsGroup: u8,
	bits_10: u8,
	pub changeSuperArmorPoint: f32,
	pub changeSaPoint: f32,
	pub hugeEnemyPickupHeightOverwrite: f32,
	pub poisonDefDamageRate: f32,
	pub diseaseDefDamageRate: f32,
	pub bloodDefDamageRate: f32,
	pub curseDefDamageRate: f32,
	pub freezeDefDamageRate: f32,
	pub sleepDefDamageRate: f32,
	pub madnessDefDamageRate: f32,
	pub overwrite_maxBackhomeDist: i16,
	pub overwrite_backhomeDist: i16,
	pub overwrite_backhomeBattleDist: i16,
	pub overwrite_BackHome_LookTargetDist: i16,
	pub goodsConsumptionRate: f32,
	pub guardStaminaMult: f32,
	pub unk3: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SP_EFFECT_PARAM_ST {
	pub fn effectTargetSelf(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn effectTargetFriend(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn effectTargetEnemy(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn effectTargetPlayer(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn effectTargetAI(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn effectTargetLive(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn effectTargetGhost(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn disableSleep(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn disableMadness(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn effectTargetAttacker(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn dispIconNonactive(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn regainGaugeDamage(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn bAdjustMagicAblity(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn bAdjustFaithAblity(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn bGameClearBonus(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn magParamChange(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
	pub fn miracleParamChange(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn clearSoul(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn requestSOS(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
	pub fn requestBlackSOS(&self) -> bool {	
			self.bits_2 & (1 << 3) != 0
	}
	pub fn requestForceJoinBlackSOS(&self) -> bool {	
			self.bits_2 & (1 << 4) != 0
	}
	pub fn requestKickSession(&self) -> bool {	
			self.bits_2 & (1 << 5) != 0
	}
	pub fn requestLeaveSession(&self) -> bool {	
			self.bits_2 & (1 << 6) != 0
	}
	pub fn requestNpcInveda(&self) -> bool {	
			self.bits_2 & (1 << 7) != 0
	}
	pub fn noDead(&self) -> bool {	
			self.bits_3 & (1 << 0) != 0
	}
	pub fn bCurrHPIndependeMaxHP(&self) -> bool {	
			self.bits_3 & (1 << 1) != 0
	}
	pub fn corrosionIgnore(&self) -> bool {	
			self.bits_3 & (1 << 2) != 0
	}
	pub fn sightSearchCutIgnore(&self) -> bool {	
			self.bits_3 & (1 << 3) != 0
	}
	pub fn hearingSearchCutIgnore(&self) -> bool {	
			self.bits_3 & (1 << 4) != 0
	}
	pub fn antiMagicIgnore(&self) -> bool {	
			self.bits_3 & (1 << 5) != 0
	}
	pub fn fakeTargetIgnore(&self) -> bool {	
			self.bits_3 & (1 << 6) != 0
	}
	pub fn fakeTargetIgnoreUndead(&self) -> bool {	
			self.bits_3 & (1 << 7) != 0
	}
	pub fn fakeTargetIgnoreAnimal(&self) -> bool {	
			self.bits_4 & (1 << 0) != 0
	}
	pub fn grabityIgnore(&self) -> bool {	
			self.bits_4 & (1 << 1) != 0
	}
	pub fn disablePoison(&self) -> bool {	
			self.bits_4 & (1 << 2) != 0
	}
	pub fn disableDisease(&self) -> bool {	
			self.bits_4 & (1 << 3) != 0
	}
	pub fn disableBlood(&self) -> bool {	
			self.bits_4 & (1 << 4) != 0
	}
	pub fn disableCurse(&self) -> bool {	
			self.bits_4 & (1 << 5) != 0
	}
	pub fn enableCharm(&self) -> bool {	
			self.bits_4 & (1 << 6) != 0
	}
	pub fn enableLifeTime(&self) -> bool {	
			self.bits_4 & (1 << 7) != 0
	}
	pub fn bAdjustStrengthAblity(&self) -> bool {	
			self.bits_5 & (1 << 0) != 0
	}
	pub fn bAdjustAgilityAblity(&self) -> bool {	
			self.bits_5 & (1 << 1) != 0
	}
	pub fn eraseOnBonfireRecover(&self) -> bool {	
			self.bits_5 & (1 << 2) != 0
	}
	pub fn throwAttackParamChange(&self) -> bool {	
			self.bits_5 & (1 << 3) != 0
	}
	pub fn requestLeaveColiseumSession(&self) -> bool {	
			self.bits_5 & (1 << 4) != 0
	}
	pub fn isExtendSpEffectLife(&self) -> bool {	
			self.bits_5 & (1 << 5) != 0
	}
	pub fn hasTarget(&self) -> bool {	
			self.bits_5 & (1 << 6) != 0
	}
	pub fn replanningOnFire(&self) -> bool {	
			self.bits_5 & (1 << 7) != 0
	}
	pub fn vowType0(&self) -> bool {	
			self.bits_6 & (1 << 0) != 0
	}
	pub fn vowType1(&self) -> bool {	
			self.bits_6 & (1 << 1) != 0
	}
	pub fn vowType2(&self) -> bool {	
			self.bits_6 & (1 << 2) != 0
	}
	pub fn vowType3(&self) -> bool {	
			self.bits_6 & (1 << 3) != 0
	}
	pub fn vowType4(&self) -> bool {	
			self.bits_6 & (1 << 4) != 0
	}
	pub fn vowType5(&self) -> bool {	
			self.bits_6 & (1 << 5) != 0
	}
	pub fn vowType6(&self) -> bool {	
			self.bits_6 & (1 << 6) != 0
	}
	pub fn vowType7(&self) -> bool {	
			self.bits_6 & (1 << 7) != 0
	}
	pub fn vowType8(&self) -> bool {	
			self.bits_7 & (1 << 0) != 0
	}
	pub fn vowType9(&self) -> bool {	
			self.bits_7 & (1 << 1) != 0
	}
	pub fn vowType10(&self) -> bool {	
			self.bits_7 & (1 << 2) != 0
	}
	pub fn vowType11(&self) -> bool {	
			self.bits_7 & (1 << 3) != 0
	}
	pub fn vowType12(&self) -> bool {	
			self.bits_7 & (1 << 4) != 0
	}
	pub fn vowType13(&self) -> bool {	
			self.bits_7 & (1 << 5) != 0
	}
	pub fn vowType14(&self) -> bool {	
			self.bits_7 & (1 << 6) != 0
	}
	pub fn vowType15(&self) -> bool {	
			self.bits_7 & (1 << 7) != 0
	}
	pub fn effectTargetOpposeTarget(&self) -> bool {	
			self.bits_8 & (1 << 0) != 0
	}
	pub fn effectTargetFriendlyTarget(&self) -> bool {	
			self.bits_8 & (1 << 1) != 0
	}
	pub fn effectTargetSelfTarget(&self) -> bool {	
			self.bits_8 & (1 << 2) != 0
	}
	pub fn effectTargetPcHorse(&self) -> bool {	
			self.bits_8 & (1 << 3) != 0
	}
	pub fn effectTargetPcDeceased(&self) -> bool {	
			self.bits_8 & (1 << 4) != 0
	}
	pub fn isContractSpEffectLife(&self) -> bool {	
			self.bits_8 & (1 << 5) != 0
	}
	pub fn isWaitModeDelete(&self) -> bool {	
			self.bits_8 & (1 << 6) != 0
	}
	pub fn isIgnoreNoDamage(&self) -> bool {	
			self.bits_8 & (1 << 7) != 0
	}
	pub fn isUseStatusAilmentAtkPowerCorrect(&self) -> bool {	
			self.bits_9 & (1 << 0) != 0
	}
	pub fn isUseAtkParamAtkPowerCorrect(&self) -> bool {	
			self.bits_9 & (1 << 1) != 0
	}
	pub fn dontDeleteOnDead(&self) -> bool {	
			self.bits_9 & (1 << 2) != 0
	}
	pub fn disableFreeze(&self) -> bool {	
			self.bits_9 & (1 << 3) != 0
	}
	pub fn isDisableNetSync(&self) -> bool {	
			self.bits_9 & (1 << 4) != 0
	}
	pub fn shamanParamChange(&self) -> bool {	
			self.bits_9 & (1 << 5) != 0
	}
	pub fn isStopSearchedNotify(&self) -> bool {	
			self.bits_9 & (1 << 6) != 0
	}
	pub fn isCheckAboveShadowTest(&self) -> bool {	
			self.bits_9 & (1 << 7) != 0
	}
	pub fn clearTarget(&self) -> bool {	
			self.bits_10 & (1 << 0) != 0
	}
	pub fn fakeTargetIgnoreAjin(&self) -> bool {	
			self.bits_10 & (1 << 1) != 0
	}
	pub fn fakeTargetIgnoreMirageArts(&self) -> bool {	
			self.bits_10 & (1 << 2) != 0
	}
	pub fn requestForceJoinBlackSOS_B(&self) -> bool {	
			self.bits_10 & (1 << 3) != 0
	}
	pub fn isDestinedDeathHpMult(&self) -> bool {	
			self.bits_10 & (1 << 4) != 0
	}
}
impl Default for SP_EFFECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 iconId: -1,
			 conditionHp: -1.,
			 effectEndurance: 0.,
			 motionInterval: 0.,
			 maxHpRate: 1.,
			 maxMpRate: 1.,
			 maxStaminaRate: 1.,
			 slashDamageCutRate: 1.,
			 blowDamageCutRate: 1.,
			 thrustDamageCutRate: 1.,
			 neutralDamageCutRate: 1.,
			 magicDamageCutRate: 1.,
			 fireDamageCutRate: 1.,
			 thunderDamageCutRate: 1.,
			 physicsAttackRate: 1.,
			 magicAttackRate: 1.,
			 fireAttackRate: 1.,
			 thunderAttackRate: 1.,
			 physicsAttackPowerRate: 1.,
			 magicAttackPowerRate: 1.,
			 fireAttackPowerRate: 1.,
			 thunderAttackPowerRate: 1.,
			 physicsAttackPower: 0,
			 magicAttackPower: 0,
			 fireAttackPower: 0,
			 thunderAttackPower: 0,
			 physicsDiffenceRate: 1.,
			 magicDiffenceRate: 1.,
			 fireDiffenceRate: 1.,
			 thunderDiffenceRate: 1.,
			 physicsDiffence: 0,
			 magicDiffence: 0,
			 fireDiffence: 0,
			 thunderDiffence: 0,
			 NoGuardDamageRate: 1.,
			 vitalSpotChangeRate: -1.,
			 normalSpotChangeRate: -1.,
			 lookAtTargetPosOffset: 0.,
			 behaviorId: -1,
			 changeHpRate: 0.,
			 changeHpPoint: 0,
			 changeMpRate: 0.,
			 changeMpPoint: 0,
			 mpRecoverChangeSpeed: 0,
			 changeStaminaRate: 0.,
			 changeStaminaPoint: 0,
			 staminaRecoverChangeSpeed: 0,
			 magicEffectTimeChange: 0.,
			 insideDurability: 0,
			 maxDurability: 0,
			 staminaAttackRate: 1.,
			 poizonAttackPower: 0,
			 diseaseAttackPower: 0,
			 bloodAttackPower: 0,
			 curseAttackPower: 0,
			 fallDamageRate: 0.,
			 soulRate: 0.,
			 equipWeightChangeRate: 0.,
			 allItemWeightChangeRate: 0.,
			 soul: 0,
			 animIdOffset: -1,
			 haveSoulRate: 1.,
			 targetPriority: 0.,
			 sightSearchEnemyRate: 1.,
			 hearingSearchEnemyRate: 1.,
			 grabityRate: 1.,
			 registPoizonChangeRate: 1.,
			 registDiseaseChangeRate: 1.,
			 registBloodChangeRate: 1.,
			 registCurseChangeRate: 1.,
			 soulStealRate: 0.,
			 lifeReductionRate: 0.,
			 hpRecoverRate: 0.,
			 replaceSpEffectId: -1,
			 cycleOccurrenceSpEffectId: -1,
			 atkOccurrenceSpEffectId: -1,
			 guardDefFlickPowerRate: 1.,
			 guardStaminaCutRate: 1.,
			 rayCastPassedTime: -1,
			 magicSubCategoryChange1: 0,
			 magicSubCategoryChange2: 0,
			 bowDistRate: 0,
			 spCategory: 0,
			 categoryPriority: 0,
			 saveCategory: -1,
			 changeMagicSlot: 0,
			 changeMiracleSlot: 0,
			 heroPointDamage: 0,
			 defFlickPower: 0,
			 flickDamageCutRate: 0,
			 bloodDamageRate: 100,
			 dmgLv_None: 0,
			 dmgLv_S: 0,
			 dmgLv_M: 0,
			 dmgLv_L: 0,
			 dmgLv_BlowM: 0,
			 dmgLv_Push: 0,
			 dmgLv_Strike: 0,
			 dmgLv_BlowS: 0,
			 dmgLv_Min: 0,
			 dmgLv_Uppercut: 0,
			 dmgLv_BlowLL: 0,
			 dmgLv_Breath: 0,
			 atkAttribute: 254,
			 spAttribute: 254,
			 stateInfo: 0,
			 wepParamChange: 0,
			 moveType: 0,
			 lifeReductionType: 0,
			 throwCondition: 0,
			 addBehaviorJudgeId_condition: -1,
			 freezeDamageRate: 100,
			 bits_0: 0,
			 bits_1: 0,
			 bits_2: 0,
			 bits_3: 0,
			 bits_4: 0,
			 bits_5: 0,
			 bits_6: 0,
			 bits_7: 0,
			 repAtkDmgLv: 0,
			 sightSearchRate: 1.,
			 bits_8: 0,
			 changeTeamType: -1,
			 dmypolyId: -1,
			 vfxId: -1,
			 accumuOverFireId: -1,
			 accumuOverVal: -1,
			 accumuUnderFireId: -1,
			 accumuUnderVal: -1,
			 accumuVal: 0,
			 eye_angX: 0,
			 eye_angY: 0,
			 addDeceasedLv: 0,
			 vfxId1: -1,
			 vfxId2: -1,
			 vfxId3: -1,
			 vfxId4: -1,
			 vfxId5: -1,
			 vfxId6: -1,
			 vfxId7: -1,
			 freezeAttackPower: 0,
			 AppearAiSoundId: 0,
			 addFootEffectSfxId: -1,
			 dexterityCancelSystemOnlyAddDexterity: 0,
			 teamOffenseEffectivity: -1,
			 toughnessDamageCutRate: 1.,
			 weakDmgRateA: 1.,
			 weakDmgRateB: 1.,
			 weakDmgRateC: 1.,
			 weakDmgRateD: 1.,
			 weakDmgRateE: 1.,
			 weakDmgRateF: 1.,
			 darkDamageCutRate: 1.,
			 darkDiffenceRate: 1.,
			 darkDiffence: 0,
			 darkAttackRate: 1.,
			 darkAttackPowerRate: 1.,
			 darkAttackPower: 0,
			 antiDarkSightRadius: 0.,
			 antiDarkSightDmypolyId: -1,
			 conditionHpRate: -1.,
			 consumeStaminaRate: 1.,
			 itemDropRate: 0.,
			 changePoisonResistPoint: 0,
			 changeDiseaseResistPoint: 0,
			 changeBloodResistPoint: 0,
			 changeCurseResistPoint: 0,
			 changeFreezeResistPoint: 0,
			 slashAttackRate: 1.,
			 blowAttackRate: 1.,
			 thrustAttackRate: 1.,
			 neutralAttackRate: 1.,
			 slashAttackPowerRate: 1.,
			 blowAttackPowerRate: 1.,
			 thrustAttackPowerRate: 1.,
			 neutralAttackPowerRate: 1.,
			 slashAttackPower: 0,
			 blowAttackPower: 0,
			 thrustAttackPower: 0,
			 neutralAttackPower: 0,
			 changeStrengthPoint: 0,
			 changeAgilityPoint: 0,
			 changeMagicPoint: 0,
			 changeFaithPoint: 0,
			 changeLuckPoint: 0,
			 recoverArtsPoint_Str: 0,
			 recoverArtsPoint_Dex: 0,
			 recoverArtsPoint_Magic: 0,
			 recoverArtsPoint_Miracle: 0,
			 madnessDamageRate: 100,
			 bits_9: 0,
			 addBehaviorJudgeId_add: 0,
			 saReceiveDamageRate: 1.,
			 defPlayerDmgCorrectRate_Physics: 1.,
			 defPlayerDmgCorrectRate_Magic: 1.,
			 defPlayerDmgCorrectRate_Fire: 1.,
			 defPlayerDmgCorrectRate_Thunder: 1.,
			 defPlayerDmgCorrectRate_Dark: 1.,
			 defEnemyDmgCorrectRate_Physics: 1.,
			 defEnemyDmgCorrectRate_Magic: 1.,
			 defEnemyDmgCorrectRate_Fire: 1.,
			 defEnemyDmgCorrectRate_Thunder: 1.,
			 defEnemyDmgCorrectRate_Dark: 1.,
			 defObjDmgCorrectRate: 1.,
			 atkPlayerDmgCorrectRate_Physics: 1.,
			 atkPlayerDmgCorrectRate_Magic: 1.,
			 atkPlayerDmgCorrectRate_Fire: 1.,
			 atkPlayerDmgCorrectRate_Thunder: 1.,
			 atkPlayerDmgCorrectRate_Dark: 1.,
			 atkEnemyDmgCorrectRate_Physics: 1.,
			 atkEnemyDmgCorrectRate_Magic: 1.,
			 atkEnemyDmgCorrectRate_Fire: 1.,
			 atkEnemyDmgCorrectRate_Thunder: 1.,
			 atkEnemyDmgCorrectRate_Dark: 1.,
			 registFreezeChangeRate: 1.,
			 invocationConditionsStateChange1: 0,
			 invocationConditionsStateChange2: 0,
			 invocationConditionsStateChange3: 0,
			 hearingAiSoundLevel: -1,
			 chrProxyHeightRate: 1.,
			 addAwarePointCorrectValue_forMe: 0.,
			 addAwarePointCorrectValue_forTarget: 0.,
			 sightSearchEnemyAdd: 0.,
			 sightSearchAdd: 0.,
			 hearingSearchAdd: 0.,
			 hearingSearchRate: 1.,
			 hearingSearchEnemyAdd: 0.,
			 value_Magnification: 1.,
			 artsConsumptionRate: 1.,
			 magicConsumptionRate: 1.,
			 shamanConsumptionRate: 1.,
			 miracleConsumptionRate: 1.,
			 changeHpEstusFlaskRate: 0,
			 changeHpEstusFlaskPoint: 0,
			 changeMpEstusFlaskRate: 0,
			 changeMpEstusFlaskPoint: 0,
			 changeHpEstusFlaskCorrectRate: 1.,
			 changeMpEstusFlaskCorrectRate: 1.,
			 applyIdOnGetSoul: 0,
			 extendLifeRate: 1.,
			 contractLifeRate: 1.,
			 defObjectAttackPowerRate: 1.,
			 effectEndDeleteDecalGroupId: -1,
			 addLifeForceStatus: 0,
			 addWillpowerStatus: 0,
			 addEndureStatus: 0,
			 addVitalityStatus: 0,
			 addStrengthStatus: 0,
			 addDexterityStatus: 0,
			 addMagicStatus: 0,
			 addFaithStatus: 0,
			 addLuckStatus: 0,
			 deleteCriteriaDamage: 0,
			 magicSubCategoryChange3: 0,
			 spAttributeVariationValue: 0,
			 atkFlickPower: 0,
			 wetConditionDepth: 0,
			 changeSaRecoveryVelocity: 1.,
			 regainRate: 1.,
			 saAttackPowerRate: 1.,
			 sleepAttackPower: 0,
			 madnessAttackPower: 0,
			 registSleepChangeRate: 1.,
			 registMadnessChangeRate: 1.,
			 changeSleepResistPoint: 0,
			 changeMadnessResistPoint: 0,
			 sleepDamageRate: 100,
			 applyPartsGroup: 0,
			 bits_10: 0,
			 changeSuperArmorPoint: 0.,
			 changeSaPoint: 0.,
			 hugeEnemyPickupHeightOverwrite: 0.,
			 poisonDefDamageRate: 1.,
			 diseaseDefDamageRate: 1.,
			 bloodDefDamageRate: 1.,
			 curseDefDamageRate: 1.,
			 freezeDefDamageRate: 1.,
			 sleepDefDamageRate: 1.,
			 madnessDefDamageRate: 1.,
			 overwrite_maxBackhomeDist: 0,
			 overwrite_backhomeDist: 0,
			 overwrite_backhomeBattleDist: 0,
			 overwrite_BackHome_LookTargetDist: 0,
			 goodsConsumptionRate: 1.,
			 guardStaminaMult: 1.,
			 unk3: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SP_EFFECT_SET_PARAM_ST {
	pub spEffectId1: i32,
	pub spEffectId2: i32,
	pub spEffectId3: i32,
	pub spEffectId4: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SP_EFFECT_SET_PARAM_ST {
}
impl Default for SP_EFFECT_SET_PARAM_ST {
	fn default() -> Self {
		Self {
			 spEffectId1: -1,
			 spEffectId2: -1,
			 spEffectId3: -1,
			 spEffectId4: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SP_EFFECT_VFX_PARAM_ST {
	pub midstSfxId: i32,
	pub midstSeId: i32,
	pub initSfxId: i32,
	pub initSeId: i32,
	pub finishSfxId: i32,
	pub finishSeId: i32,
	pub camouflageBeginDist: f32,
	pub camouflageEndDist: f32,
	pub transformProtectorId: i32,
	pub midstDmyId: i16,
	pub initDmyId: i16,
	pub finishDmyId: i16,
	pub effectType: u8,
	pub soulParamIdForWepEnchant: u8,
	pub playCategory: u8,
	pub playPriority: u8,
	bits_0: u8,
	bits_1: u8,
	pub decalId1: i32,
	pub decalId2: i32,
	pub footEffectPriority: u8,
	pub footEffectOffset: u8,
	pub traceSfxIdOffsetType: u8,
	pub forceDeceasedType: u8,
	pub enchantStartDmyId_0: i32,
	pub enchantEndDmyId_0: i32,
	pub enchantStartDmyId_1: i32,
	pub enchantEndDmyId_1: i32,
	pub enchantStartDmyId_2: i32,
	pub enchantEndDmyId_2: i32,
	pub enchantStartDmyId_3: i32,
	pub enchantEndDmyId_3: i32,
	pub enchantStartDmyId_4: i32,
	pub enchantEndDmyId_4: i32,
	pub enchantStartDmyId_5: i32,
	pub enchantEndDmyId_5: i32,
	pub enchantStartDmyId_6: i32,
	pub enchantEndDmyId_6: i32,
	pub enchantStartDmyId_7: i32,
	pub enchantEndDmyId_7: i32,
	pub SfxIdOffsetType: u8,
	pub phantomParamOverwriteType: u8,
	pub camouflageMinAlpha: u8,
	pub wetAspectType: u8,
	pub phantomParamOverwriteId: i32,
	pub materialParamId: i32,
	pub materialParamInitValue: f32,
	pub materialParamTargetValue: f32,
	pub materialParamFadeTime: f32,
	pub footDecalMaterialOffsetOverwriteId: i16,
	pub pad: [u8;14],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SP_EFFECT_VFX_PARAM_ST {
	pub fn existEffectForLarge(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn existEffectForSoul(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn effectInvisibleAtCamouflage(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn useCamouflage(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn invisibleAtFriendCamouflage(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isHideFootEffect_forCamouflage(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn halfCamouflage(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
	pub fn isFullBodyTransformProtectorId(&self) -> bool {	
			self.bits_0 & (1 << 7) != 0
	}
	pub fn isInvisibleWeapon(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isSilence(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn isMidstFullbody(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isInitFullbody(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn isFinishFullbody(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
	pub fn isVisibleDeadChr(&self) -> bool {	
			self.bits_1 & (1 << 5) != 0
	}
	pub fn isUseOffsetEnchantSfxSize(&self) -> bool {	
			self.bits_1 & (1 << 6) != 0
	}
	pub fn pad_1(&self) -> bool {	
			self.bits_1 & (1 << 7) != 0
	}
}
impl Default for SP_EFFECT_VFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 midstSfxId: -1,
			 midstSeId: -1,
			 initSfxId: -1,
			 initSeId: -1,
			 finishSfxId: -1,
			 finishSeId: -1,
			 camouflageBeginDist: -1.,
			 camouflageEndDist: -1.,
			 transformProtectorId: -1,
			 midstDmyId: -1,
			 initDmyId: -1,
			 finishDmyId: -1,
			 effectType: 0,
			 soulParamIdForWepEnchant: 0,
			 playCategory: 0,
			 playPriority: 0,
			 bits_0: 0,
			 bits_1: 0,
			 decalId1: -1,
			 decalId2: -1,
			 footEffectPriority: 0,
			 footEffectOffset: 0,
			 traceSfxIdOffsetType: 0,
			 forceDeceasedType: 0,
			 enchantStartDmyId_0: -1,
			 enchantEndDmyId_0: -1,
			 enchantStartDmyId_1: -1,
			 enchantEndDmyId_1: -1,
			 enchantStartDmyId_2: -1,
			 enchantEndDmyId_2: -1,
			 enchantStartDmyId_3: -1,
			 enchantEndDmyId_3: -1,
			 enchantStartDmyId_4: -1,
			 enchantEndDmyId_4: -1,
			 enchantStartDmyId_5: -1,
			 enchantEndDmyId_5: -1,
			 enchantStartDmyId_6: -1,
			 enchantEndDmyId_6: -1,
			 enchantStartDmyId_7: -1,
			 enchantEndDmyId_7: -1,
			 SfxIdOffsetType: 0,
			 phantomParamOverwriteType: 0,
			 camouflageMinAlpha: 0,
			 wetAspectType: 0,
			 phantomParamOverwriteId: 0,
			 materialParamId: -1,
			 materialParamInitValue: 0.,
			 materialParamTargetValue: 0.,
			 materialParamFadeTime: 0.,
			 footDecalMaterialOffsetOverwriteId: -1,
			 pad: [0;14],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct SWORD_ARTS_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub swordArtsType: u8,
	pub artsSpeedType: u8,
	pub refStatus: i8,
	bits_1: u8,
	pub usePoint_L1: i8,
	pub usePoint_L2: i8,
	pub usePoint_R1: i8,
	pub usePoint_R2: i8,
	pub textId: i32,
	pub useMagicPoint_L1: i16,
	pub useMagicPoint_L2: i16,
	pub useMagicPoint_R1: i16,
	pub useMagicPoint_R2: i16,
	pub shieldIconType: i8,
	pub swordArtsTypeNew: u8,
	pub pad: [u8;1],
	pub iconId: i16,
	pub aiUsageId: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl SWORD_ARTS_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isRefRightArts(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isGrayoutLeftHand(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn isGrayoutRightHand(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn isGrayoutBothHand(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn reserve2(&self) -> bool {	
			self.bits_1 & (1 << 4) != 0
	}
}
impl Default for SWORD_ARTS_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 swordArtsType: 0,
			 artsSpeedType: 0,
			 refStatus: 0,
			 bits_1: 0,
			 usePoint_L1: 0,
			 usePoint_L2: 0,
			 usePoint_R1: 0,
			 usePoint_R2: 0,
			 textId: 0,
			 useMagicPoint_L1: 0,
			 useMagicPoint_L2: 0,
			 useMagicPoint_R1: 0,
			 useMagicPoint_R2: 0,
			 shieldIconType: 0,
			 swordArtsTypeNew: 0,
			 pad: [0;1],
			 iconId: 0,
			 aiUsageId: -1,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct TALK_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub msgId: i32,
	pub voiceId: i32,
	pub spEffectId0: i32,
	pub motionId0: i32,
	pub spEffectId1: i32,
	pub motionId1: i32,
	pub returnPos: i32,
	pub reactionId: i32,
	pub eventId: i32,
	pub msgId_female: i32,
	pub voiceId_female: i32,
	pub lipSyncStart: i16,
	pub lipSyncTime: i16,
	pub pad2: [u8;4],
	pub timeout: f32,
	pub talkAnimationId: i32,
	bits_1: u8,
	pub pad1: [u8;31],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl TALK_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isForceDisp(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn pad3(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
}
impl Default for TALK_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 msgId: -1,
			 voiceId: -1,
			 spEffectId0: -1,
			 motionId0: -1,
			 spEffectId1: -1,
			 motionId1: -1,
			 returnPos: -1,
			 reactionId: -1,
			 eventId: -1,
			 msgId_female: -1,
			 voiceId_female: -1,
			 lipSyncStart: -1,
			 lipSyncTime: -1,
			 pad2: [0;4],
			 timeout: -1.,
			 talkAnimationId: -1,
			 bits_1: 0,
			 pad1: [0;31],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct THROW_DIRECTION_SFX_PARAM_ST {
	pub sfxId_00: i32,
	pub sfxId_01: i32,
	pub sfxId_02: i32,
	pub sfxId_03: i32,
	pub sfxId_04: i32,
	pub sfxId_05: i32,
	pub sfxId_06: i32,
	pub sfxId_07: i32,
	pub sfxId_08: i32,
	pub sfxId_09: i32,
	pub sfxId_10: i32,
	pub sfxId_11: i32,
	pub sfxId_12: i32,
	pub sfxId_13: i32,
	pub sfxId_14: i32,
	pub sfxId_15: i32,
	pub sfxId_16: i32,
	pub sfxId_17: i32,
	pub sfxId_18: i32,
	pub sfxId_19: i32,
	pub sfxId_20: i32,
	pub sfxId_21: i32,
	pub sfxId_22: i32,
	pub sfxId_23: i32,
	pub sfxId_24: i32,
	pub sfxId_25: i32,
	pub sfxId_26: i32,
	pub sfxId_27: i32,
	pub sfxId_28: i32,
	pub sfxId_29: i32,
	pub sfxId_30: i32,
	pub pad1: [u8;20],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl THROW_DIRECTION_SFX_PARAM_ST {
}
impl Default for THROW_DIRECTION_SFX_PARAM_ST {
	fn default() -> Self {
		Self {
			 sfxId_00: 0,
			 sfxId_01: 0,
			 sfxId_02: 0,
			 sfxId_03: 0,
			 sfxId_04: 0,
			 sfxId_05: 0,
			 sfxId_06: 0,
			 sfxId_07: 0,
			 sfxId_08: 0,
			 sfxId_09: 0,
			 sfxId_10: 0,
			 sfxId_11: 0,
			 sfxId_12: 0,
			 sfxId_13: 0,
			 sfxId_14: 0,
			 sfxId_15: 0,
			 sfxId_16: 0,
			 sfxId_17: 0,
			 sfxId_18: 0,
			 sfxId_19: 0,
			 sfxId_20: 0,
			 sfxId_21: 0,
			 sfxId_22: 0,
			 sfxId_23: 0,
			 sfxId_24: 0,
			 sfxId_25: 0,
			 sfxId_26: 0,
			 sfxId_27: 0,
			 sfxId_28: 0,
			 sfxId_29: 0,
			 sfxId_30: 0,
			 pad1: [0;20],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct THROW_PARAM_ST {
	pub AtkChrId: i32,
	pub DefChrId: i32,
	pub Dist: f32,
	pub DiffAngMin: f32,
	pub DiffAngMax: f32,
	pub upperYRange: f32,
	pub lowerYRange: f32,
	pub diffAngMyToDef: f32,
	pub throwTypeId: i32,
	pub atkAnimId: i32,
	pub defAnimId: i32,
	pub escHp: i16,
	pub selfEscCycleTime: i16,
	pub sphereCastRadiusRateTop: i16,
	pub sphereCastRadiusRateLow: i16,
	pub PadType: u8,
	pub AtkEnableState: u8,
	pub throwFollowingType: u8,
	pub pad2: [u8;1],
	pub throwType: u8,
	pub selfEscCycleCnt: u8,
	pub dmyHasChrDirType: u8,
	bits_0: u8,
	pub atkSorbDmyId: i16,
	pub defSorbDmyId: i16,
	pub Dist_start: f32,
	pub DiffAngMin_start: f32,
	pub DiffAngMax_start: f32,
	pub upperYRange_start: f32,
	pub lowerYRange_start: f32,
	pub diffAngMyToDef_start: f32,
	pub judgeRangeBasePosDmyId1: i32,
	pub judgeRangeBasePosDmyId2: i32,
	pub adsrobModelPosInterpolationTime: f32,
	pub throwFollowingEndEasingTime: f32,
	pub pad1: [u8;24],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl THROW_PARAM_ST {
	pub fn isTurnAtker(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn isSkipWepCate(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isSkipSphereCast(&self) -> bool {	
			self.bits_0 & (1 << 2) != 0
	}
	pub fn isEnableCorrectPos_forThrowAdjust(&self) -> bool {	
			self.bits_0 & (1 << 3) != 0
	}
	pub fn isEnableThrowFollowingFallAssist(&self) -> bool {	
			self.bits_0 & (1 << 4) != 0
	}
	pub fn isEnableThrowFollowingFeedback(&self) -> bool {	
			self.bits_0 & (1 << 5) != 0
	}
	pub fn pad0(&self) -> bool {	
			self.bits_0 & (1 << 6) != 0
	}
}
impl Default for THROW_PARAM_ST {
	fn default() -> Self {
		Self {
			 AtkChrId: 0,
			 DefChrId: 0,
			 Dist: 0.,
			 DiffAngMin: 0.,
			 DiffAngMax: 0.,
			 upperYRange: 0.2,
			 lowerYRange: 0.2,
			 diffAngMyToDef: 60.,
			 throwTypeId: 0,
			 atkAnimId: 0,
			 defAnimId: 0,
			 escHp: 0,
			 selfEscCycleTime: 0,
			 sphereCastRadiusRateTop: 80,
			 sphereCastRadiusRateLow: 80,
			 PadType: 1,
			 AtkEnableState: 0,
			 throwFollowingType: 0,
			 pad2: [0;1],
			 throwType: 0,
			 selfEscCycleCnt: 0,
			 dmyHasChrDirType: 0,
			 bits_0: 0,
			 atkSorbDmyId: 0,
			 defSorbDmyId: 0,
			 Dist_start: 0.,
			 DiffAngMin_start: 0.,
			 DiffAngMax_start: 0.,
			 upperYRange_start: 0.,
			 lowerYRange_start: 0.,
			 diffAngMyToDef_start: 0.,
			 judgeRangeBasePosDmyId1: -1,
			 judgeRangeBasePosDmyId2: -1,
			 adsrobModelPosInterpolationTime: 0.5,
			 throwFollowingEndEasingTime: 0.5,
			 pad1: [0;24],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct TOUGHNESS_PARAM_ST {
	pub correctionRate: f32,
	pub minToughness: i16,
	pub isNonEffectiveCorrectionForMin: u8,
	pub pad2: [u8;1],
	pub spEffectId: i32,
	pub proCorrectionRate: f32,
	pub unk1: f32,
	pub unk2: f32,
	pub pad1: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl TOUGHNESS_PARAM_ST {
}
impl Default for TOUGHNESS_PARAM_ST {
	fn default() -> Self {
		Self {
			 correctionRate: 1.,
			 minToughness: 0,
			 isNonEffectiveCorrectionForMin: 0,
			 pad2: [0;1],
			 spEffectId: -1,
			 proCorrectionRate: 1.,
			 unk1: 1.,
			 unk2: 1.,
			 pad1: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct TUTORIAL_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub menuType: u8,
	pub triggerType: u8,
	pub repeatType: u8,
	pub pad1: [u8;1],
	pub imageId: i16,
	pub pad2: [u8;2],
	pub unlockEventFlagId: i32,
	pub textId: i32,
	pub displayMinTime: f32,
	pub displayTime: f32,
	pub pad3: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl TUTORIAL_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for TUTORIAL_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 menuType: 0,
			 triggerType: 0,
			 repeatType: 0,
			 pad1: [0;1],
			 imageId: 0,
			 pad2: [0;2],
			 unlockEventFlagId: 0,
			 textId: -1,
			 displayMinTime: 1.,
			 displayTime: 3.,
			 pad3: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WAYPOINT_PARAM_ST {
	pub attribute1: i16,
	pub attribute2: i16,
	pub attribute3: i16,
	pub attribute4: i16,
	pub padding4: [u8;8],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WAYPOINT_PARAM_ST {
}
impl Default for WAYPOINT_PARAM_ST {
	fn default() -> Self {
		Self {
			 attribute1: -1,
			 attribute2: -1,
			 attribute3: -1,
			 attribute4: -1,
			 padding4: [0;8],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WEATHER_ASSET_CREATE_PARAM_ST {
	pub AssetId: i32,
	pub SlotNo: i32,
	pub CreateConditionType: u8,
	pub padding0: [u8;3],
	pub TransitionSrcWeather: i16,
	pub TransitionDstWeather: i16,
	pub ElapsedTimeCheckweather: i16,
	pub padding1: [u8;2],
	pub ElapsedTime: f32,
	pub CreateDelayTime: f32,
	pub CreateProbability: i32,
	pub LifeTime: f32,
	pub FadeTime: f32,
	pub EnableCreateTimeMin: f32,
	pub EnableCreateTimeMax: f32,
	pub CreatePoint0: i16,
	pub CreatePoint1: i16,
	pub CreatePoint2: i16,
	pub CreatePoint3: i16,
	pub CreateAssetLimitId0: i8,
	pub CreateAssetLimitId1: i8,
	pub CreateAssetLimitId2: i8,
	pub CreateAssetLimitId3: i8,
	pub Reserved2: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WEATHER_ASSET_CREATE_PARAM_ST {
}
impl Default for WEATHER_ASSET_CREATE_PARAM_ST {
	fn default() -> Self {
		Self {
			 AssetId: 0,
			 SlotNo: 0,
			 CreateConditionType: 0,
			 padding0: [0;3],
			 TransitionSrcWeather: 0,
			 TransitionDstWeather: 0,
			 ElapsedTimeCheckweather: 0,
			 padding1: [0;2],
			 ElapsedTime: 0.,
			 CreateDelayTime: -1.,
			 CreateProbability: 100,
			 LifeTime: 600.,
			 FadeTime: 1.,
			 EnableCreateTimeMin: -1.,
			 EnableCreateTimeMax: -1.,
			 CreatePoint0: -1,
			 CreatePoint1: -1,
			 CreatePoint2: -1,
			 CreatePoint3: -1,
			 CreateAssetLimitId0: -1,
			 CreateAssetLimitId1: -1,
			 CreateAssetLimitId2: -1,
			 CreateAssetLimitId3: -1,
			 Reserved2: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WEATHER_ASSET_REPLACE_PARAM_ST {
	pub mapId: i32,
	pub TransitionSrcWeather: i16,
	pub padding0: [u8;2],
	pub isFireAsh: u8,
	pub padding1: [u8;3],
	pub reserved2: i32,
	pub AssetId0: i32,
	pub AssetId1: i32,
	pub AssetId2: i32,
	pub AssetId3: i32,
	pub AssetId4: i32,
	pub AssetId5: i32,
	pub AssetId6: i32,
	pub AssetId7: i32,
	pub reserved0: [u8;8],
	pub CreateAssetLimitId0: i8,
	pub CreateAssetLimitId1: i8,
	pub CreateAssetLimitId2: i8,
	pub CreateAssetLimitId3: i8,
	pub reserved1: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WEATHER_ASSET_REPLACE_PARAM_ST {
}
impl Default for WEATHER_ASSET_REPLACE_PARAM_ST {
	fn default() -> Self {
		Self {
			 mapId: 0,
			 TransitionSrcWeather: 0,
			 padding0: [0;2],
			 isFireAsh: 0,
			 padding1: [0;3],
			 reserved2: 0,
			 AssetId0: -1,
			 AssetId1: -1,
			 AssetId2: -1,
			 AssetId3: -1,
			 AssetId4: -1,
			 AssetId5: -1,
			 AssetId6: -1,
			 AssetId7: -1,
			 reserved0: [0;8],
			 CreateAssetLimitId0: -1,
			 CreateAssetLimitId1: -1,
			 CreateAssetLimitId2: -1,
			 CreateAssetLimitId3: -1,
			 reserved1: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WEATHER_LOT_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub weatherType0: i16,
	pub weatherType1: i16,
	pub weatherType2: i16,
	pub weatherType3: i16,
	pub weatherType4: i16,
	pub weatherType5: i16,
	pub weatherType6: i16,
	pub weatherType7: i16,
	pub weatherType8: i16,
	pub weatherType9: i16,
	pub weatherType10: i16,
	pub weatherType11: i16,
	pub weatherType12: i16,
	pub weatherType13: i16,
	pub weatherType14: i16,
	pub weatherType15: i16,
	pub lotteryWeight0: i16,
	pub lotteryWeight1: i16,
	pub lotteryWeight2: i16,
	pub lotteryWeight3: i16,
	pub lotteryWeight4: i16,
	pub lotteryWeight5: i16,
	pub lotteryWeight6: i16,
	pub lotteryWeight7: i16,
	pub lotteryWeight8: i16,
	pub lotteryWeight9: i16,
	pub lotteryWeight10: i16,
	pub lotteryWeight11: i16,
	pub lotteryWeight12: i16,
	pub lotteryWeight13: i16,
	pub lotteryWeight14: i16,
	pub lotteryWeight15: i16,
	pub timezoneLimit: u8,
	pub timezoneStartHour: u8,
	pub timezoneStartMinute: u8,
	pub timezoneEndHour: u8,
	pub timezoneEndMinute: u8,
	pub reserve: [u8;9],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WEATHER_LOT_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for WEATHER_LOT_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 weatherType0: -1,
			 weatherType1: -1,
			 weatherType2: -1,
			 weatherType3: -1,
			 weatherType4: -1,
			 weatherType5: -1,
			 weatherType6: -1,
			 weatherType7: -1,
			 weatherType8: -1,
			 weatherType9: -1,
			 weatherType10: -1,
			 weatherType11: -1,
			 weatherType12: -1,
			 weatherType13: -1,
			 weatherType14: -1,
			 weatherType15: -1,
			 lotteryWeight0: 0,
			 lotteryWeight1: 0,
			 lotteryWeight2: 0,
			 lotteryWeight3: 0,
			 lotteryWeight4: 0,
			 lotteryWeight5: 0,
			 lotteryWeight6: 0,
			 lotteryWeight7: 0,
			 lotteryWeight8: 0,
			 lotteryWeight9: 0,
			 lotteryWeight10: 0,
			 lotteryWeight11: 0,
			 lotteryWeight12: 0,
			 lotteryWeight13: 0,
			 lotteryWeight14: 0,
			 lotteryWeight15: 0,
			 timezoneLimit: 0,
			 timezoneStartHour: 0,
			 timezoneStartMinute: 0,
			 timezoneEndHour: 0,
			 timezoneEndMinute: 0,
			 reserve: [0;9],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WEATHER_LOT_TEX_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub srcR: u8,
	pub srcG: u8,
	pub srcB: u8,
	pub pad1: [u8;1],
	pub weatherLogId: i32,
	pub pad2: [u8;4],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WEATHER_LOT_TEX_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for WEATHER_LOT_TEX_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 srcR: 0,
			 srcG: 0,
			 srcB: 0,
			 pad1: [0;1],
			 weatherLogId: -1,
			 pad2: [0;4],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WEATHER_PARAM_ST {
	pub SfxId: i32,
	pub WindSfxId: i32,
	pub GroundHitSfxId: i32,
	pub SoundId: i32,
	pub WetTime: f32,
	pub GparamId: i32,
	pub NextLotIngameSecondsMin: i32,
	pub NextLotIngameSecondsMax: i32,
	pub WetSpEffectId00: i32,
	pub WetSpEffectId01: i32,
	pub WetSpEffectId02: i32,
	pub WetSpEffectId03: i32,
	pub WetSpEffectId04: i32,
	pub SfxIdInoor: i32,
	pub SfxIdOutdoor: i32,
	pub aiSightRate: f32,
	pub DistViewWeatherGparamOverrideWeight: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WEATHER_PARAM_ST {
}
impl Default for WEATHER_PARAM_ST {
	fn default() -> Self {
		Self {
			 SfxId: -1,
			 WindSfxId: -1,
			 GroundHitSfxId: -1,
			 SoundId: -1,
			 WetTime: -1.,
			 GparamId: 0,
			 NextLotIngameSecondsMin: 3600,
			 NextLotIngameSecondsMax: 7200,
			 WetSpEffectId00: -1,
			 WetSpEffectId01: -1,
			 WetSpEffectId02: -1,
			 WetSpEffectId03: -1,
			 WetSpEffectId04: -1,
			 SfxIdInoor: -1,
			 SfxIdOutdoor: -1,
			 aiSightRate: 1.,
			 DistViewWeatherGparamOverrideWeight: -1.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WEP_ABSORP_POS_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub hangPosType: u8,
	pub isSkeletonBind: u8,
	pub pad0: [u8;2],
	pub right_0: i16,
	pub left_0: i16,
	pub both_0: i16,
	pub leftHang_0: i16,
	pub rightHang_0: i16,
	pub right_1: i16,
	pub left_1: i16,
	pub both_1: i16,
	pub leftHang_1: i16,
	pub rightHang_1: i16,
	pub right_2: i16,
	pub left_2: i16,
	pub both_2: i16,
	pub leftHang_2: i16,
	pub rightHang_2: i16,
	pub right_3: i16,
	pub left_3: i16,
	pub both_3: i16,
	pub leftHang_3: i16,
	pub rightHang_3: i16,
	pub wepInvisibleType_0: u8,
	pub wepInvisibleType_1: u8,
	pub wepInvisibleType_2: u8,
	pub wepInvisibleType_3: u8,
	pub leftBoth_0: i16,
	pub leftBoth_1: i16,
	pub leftBoth_2: i16,
	pub leftBoth_3: i16,
	pub dispPosType_right_0: u8,
	pub dispPosType_left_0: u8,
	pub dispPosType_rightBoth_0: u8,
	pub dispPosType_leftBoth_0: u8,
	pub dispPosType_rightHang_0: u8,
	pub dispPosType_leftHang_0: u8,
	pub dispPosType_right_1: u8,
	pub dispPosType_left_1: u8,
	pub dispPosType_rightBoth_1: u8,
	pub dispPosType_leftBoth_1: u8,
	pub dispPosType_rightHang_1: u8,
	pub dispPosType_leftHang_1: u8,
	pub dispPosType_right_2: u8,
	pub dispPosType_left_2: u8,
	pub dispPosType_rightBoth_2: u8,
	pub dispPosType_leftBoth_2: u8,
	pub dispPosType_rightHang_2: u8,
	pub dispPosType_leftHang_2: u8,
	pub dispPosType_right_3: u8,
	pub dispPosType_left_3: u8,
	pub dispPosType_rightBoth_3: u8,
	pub dispPosType_leftBoth_3: u8,
	pub dispPosType_rightHang_3: u8,
	pub dispPosType_leftHang_3: u8,
	pub reserve: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WEP_ABSORP_POS_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for WEP_ABSORP_POS_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 hangPosType: 0,
			 isSkeletonBind: 0,
			 pad0: [0;2],
			 right_0: 0,
			 left_0: 0,
			 both_0: 0,
			 leftHang_0: 0,
			 rightHang_0: 0,
			 right_1: 0,
			 left_1: 0,
			 both_1: 0,
			 leftHang_1: 0,
			 rightHang_1: 0,
			 right_2: 0,
			 left_2: 0,
			 both_2: 0,
			 leftHang_2: 0,
			 rightHang_2: 0,
			 right_3: 0,
			 left_3: 0,
			 both_3: 0,
			 leftHang_3: 0,
			 rightHang_3: 0,
			 wepInvisibleType_0: 0,
			 wepInvisibleType_1: 0,
			 wepInvisibleType_2: 0,
			 wepInvisibleType_3: 0,
			 leftBoth_0: 0,
			 leftBoth_1: 0,
			 leftBoth_2: 0,
			 leftBoth_3: 0,
			 dispPosType_right_0: 0,
			 dispPosType_left_0: 0,
			 dispPosType_rightBoth_0: 0,
			 dispPosType_leftBoth_0: 0,
			 dispPosType_rightHang_0: 0,
			 dispPosType_leftHang_0: 0,
			 dispPosType_right_1: 0,
			 dispPosType_left_1: 0,
			 dispPosType_rightBoth_1: 0,
			 dispPosType_leftBoth_1: 0,
			 dispPosType_rightHang_1: 0,
			 dispPosType_leftHang_1: 0,
			 dispPosType_right_2: 0,
			 dispPosType_left_2: 0,
			 dispPosType_rightBoth_2: 0,
			 dispPosType_leftBoth_2: 0,
			 dispPosType_rightHang_2: 0,
			 dispPosType_leftHang_2: 0,
			 dispPosType_right_3: 0,
			 dispPosType_left_3: 0,
			 dispPosType_rightBoth_3: 0,
			 dispPosType_leftBoth_3: 0,
			 dispPosType_rightHang_3: 0,
			 dispPosType_leftHang_3: 0,
			 reserve: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WET_ASPECT_PARAM_ST {
	pub baseColorR: u8,
	pub baseColorG: u8,
	pub baseColorB: u8,
	pub reserve_0: [u8;1],
	pub baseColorA: f32,
	pub metallic: u8,
	pub reserve_1: [u8;1],
	pub reserve_2: [u8;1],
	pub reserve_3: [u8;1],
	pub metallicRate: f32,
	pub shininessRate: f32,
	pub shininess: u8,
	pub reserve_4: [u8;11],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WET_ASPECT_PARAM_ST {
}
impl Default for WET_ASPECT_PARAM_ST {
	fn default() -> Self {
		Self {
			 baseColorR: 0,
			 baseColorG: 0,
			 baseColorB: 0,
			 reserve_0: [0;1],
			 baseColorA: 0.,
			 metallic: 0,
			 reserve_1: [0;1],
			 reserve_2: [0;1],
			 reserve_3: [0;1],
			 metallicRate: 0.,
			 shininessRate: 0.,
			 shininess: 0,
			 reserve_4: [0;11],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WHITE_SIGN_COOL_TIME_PARAM_ST {
	pub limitationTime_Normal: f32,
	pub limitationTime_NormalDriedFinger: f32,
	pub limitationTime_Guardian: f32,
	pub limitationTime_GuardianDriedFinger: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WHITE_SIGN_COOL_TIME_PARAM_ST {
}
impl Default for WHITE_SIGN_COOL_TIME_PARAM_ST {
	fn default() -> Self {
		Self {
			 limitationTime_Normal: 0.,
			 limitationTime_NormalDriedFinger: 0.,
			 limitationTime_Guardian: 0.,
			 limitationTime_GuardianDriedFinger: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WORLD_MAP_LEGACY_CONV_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub srcAreaNo: u8,
	pub srcGridXNo: u8,
	pub srcGridZNo: u8,
	pub pad1: [u8;1],
	pub srcPosX: f32,
	pub srcPosY: f32,
	pub srcPosZ: f32,
	pub dstAreaNo: u8,
	pub dstGridXNo: u8,
	pub dstGridZNo: u8,
	pub pad2: [u8;1],
	pub dstPosX: f32,
	pub dstPosY: f32,
	pub dstPosZ: f32,
	bits_1: u8,
	pub pad4: [u8;11],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WORLD_MAP_LEGACY_CONV_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isBasePoint(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn pad3(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
}
impl Default for WORLD_MAP_LEGACY_CONV_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 srcAreaNo: 0,
			 srcGridXNo: 0,
			 srcGridZNo: 0,
			 pad1: [0;1],
			 srcPosX: 0.,
			 srcPosY: 0.,
			 srcPosZ: 0.,
			 dstAreaNo: 0,
			 dstGridXNo: 0,
			 dstGridZNo: 0,
			 pad2: [0;1],
			 dstPosX: 0.,
			 dstPosY: 0.,
			 dstPosZ: 0.,
			 bits_1: 0,
			 pad4: [0;11],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WORLD_MAP_PIECE_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub openEventFlagId: i32,
	pub openTravelAreaLeft: f32,
	pub openTravelAreaRight: f32,
	pub openTravelAreaTop: f32,
	pub openTravelAreaBottom: f32,
	pub acquisitionEventFlagId: i32,
	pub acquisitionEventScale: f32,
	pub acquisitionEventCenterX: f32,
	pub acquisitionEventCenterY: f32,
	pub acquisitionEventResScale: f32,
	pub acquisitionEventResOffsetX: f32,
	pub acquisitionEventResOffsetY: f32,
	pub pad: [u8;12],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WORLD_MAP_PIECE_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for WORLD_MAP_PIECE_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 openEventFlagId: 0,
			 openTravelAreaLeft: 0.,
			 openTravelAreaRight: 0.,
			 openTravelAreaTop: 0.,
			 openTravelAreaBottom: 0.,
			 acquisitionEventFlagId: 0,
			 acquisitionEventScale: 1.,
			 acquisitionEventCenterX: 0.,
			 acquisitionEventCenterY: 0.,
			 acquisitionEventResScale: 1.,
			 acquisitionEventResOffsetX: 0.,
			 acquisitionEventResOffsetY: 0.,
			 pad: [0;12],
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WORLD_MAP_PLACE_NAME_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub worldMapPieceId: i32,
	pub textId: i32,
	pub pad1: [u8;4],
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad2: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WORLD_MAP_PLACE_NAME_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for WORLD_MAP_PLACE_NAME_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 worldMapPieceId: -1,
			 textId: -1,
			 pad1: [0;4],
			 areaNo: 0,
			 gridXNo: 0,
			 gridZNo: 0,
			 pad2: [0;1],
			 posX: 0.,
			 posY: 0.,
			 posZ: 0.,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WORLD_MAP_POINT_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub eventFlagId: i32,
	pub distViewEventFlagId: i32,
	pub iconId: i16,
	pub bgmPlaceType: i16,
	bits_1: u8,
	pub areaNo_forDistViewMark: u8,
	pub gridXNo_forDistViewMark: u8,
	pub gridZNo_forDistViewMark: u8,
	pub clearedEventFlagId: i32,
	bits_2: u8,
	pub pad2: [u8;1],
	pub distViewIconId: i16,
	pub angle: f32,
	pub areaNo: u8,
	pub gridXNo: u8,
	pub gridZNo: u8,
	pub pad: [u8;1],
	pub posX: f32,
	pub posY: f32,
	pub posZ: f32,
	pub textId1: i32,
	pub textEnableFlagId1: i32,
	pub textDisableFlagId1: i32,
	pub textId2: i32,
	pub textEnableFlagId2: i32,
	pub textDisableFlagId2: i32,
	pub textId3: i32,
	pub textEnableFlagId3: i32,
	pub textDisableFlagId3: i32,
	pub textId4: i32,
	pub textEnableFlagId4: i32,
	pub textDisableFlagId4: i32,
	pub textId5: i32,
	pub textEnableFlagId5: i32,
	pub textDisableFlagId5: i32,
	pub textId6: i32,
	pub textEnableFlagId6: i32,
	pub textDisableFlagId6: i32,
	pub textId7: i32,
	pub textEnableFlagId7: i32,
	pub textDisableFlagId7: i32,
	pub textId8: i32,
	pub textEnableFlagId8: i32,
	pub textDisableFlagId8: i32,
	pub textType1: u8,
	pub textType2: u8,
	pub textType3: u8,
	pub textType4: u8,
	pub textType5: u8,
	pub textType6: u8,
	pub textType7: u8,
	pub textType8: u8,
	pub distViewId: i32,
	pub posX_forDistViewMark: f32,
	pub posY_forDistViewMark: f32,
	pub posZ_forDistViewMark: f32,
	pub distViewId1: i32,
	pub distViewId2: i32,
	pub distViewId3: i32,
	pub dispMinZoomStep: u8,
	pub selectMinZoomStep: u8,
	pub entryFEType: u8,
	pub pad4: [u8;9],
	pub unkC0: i32,
	pub unkC4: i32,
	pub unkC8: i32,
	pub unkCC: i32,
	pub unkD0: i32,
	pub unkD4: i32,
	pub unkD8: i32,
	pub unkDC: i32,
	pub unkE0: i32,
	pub unkE4: i32,
	pub unkE8: i32,
	pub unkEC: i32,
	pub unkF0: i32,
	pub unkF4: i32,
	pub unkF8: i32,
	pub unkFC: i32,
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WORLD_MAP_POINT_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
	pub fn isAreaIcon(&self) -> bool {	
			self.bits_1 & (1 << 0) != 0
	}
	pub fn isOverrideDistViewMarkPos(&self) -> bool {	
			self.bits_1 & (1 << 1) != 0
	}
	pub fn isEnableNoText(&self) -> bool {	
			self.bits_1 & (1 << 2) != 0
	}
	pub fn pad3(&self) -> bool {	
			self.bits_1 & (1 << 3) != 0
	}
	pub fn dispMask00(&self) -> bool {	
			self.bits_2 & (1 << 0) != 0
	}
	pub fn dispMask01(&self) -> bool {	
			self.bits_2 & (1 << 1) != 0
	}
	pub fn pad2_0(&self) -> bool {	
			self.bits_2 & (1 << 2) != 0
	}
}
impl Default for WORLD_MAP_POINT_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 eventFlagId: 0,
			 distViewEventFlagId: 0,
			 iconId: 0,
			 bgmPlaceType: 0,
			 bits_1: 0,
			 areaNo_forDistViewMark: 0,
			 gridXNo_forDistViewMark: 0,
			 gridZNo_forDistViewMark: 0,
			 clearedEventFlagId: 0,
			 bits_2: 0,
			 pad2: [0;1],
			 distViewIconId: 0,
			 angle: 0.,
			 areaNo: 0,
			 gridXNo: 0,
			 gridZNo: 0,
			 pad: [0;1],
			 posX: 0.,
			 posY: 0.,
			 posZ: 0.,
			 textId1: -1,
			 textEnableFlagId1: 0,
			 textDisableFlagId1: 0,
			 textId2: -1,
			 textEnableFlagId2: 0,
			 textDisableFlagId2: 0,
			 textId3: -1,
			 textEnableFlagId3: 0,
			 textDisableFlagId3: 0,
			 textId4: -1,
			 textEnableFlagId4: 0,
			 textDisableFlagId4: 0,
			 textId5: -1,
			 textEnableFlagId5: 0,
			 textDisableFlagId5: 0,
			 textId6: -1,
			 textEnableFlagId6: 0,
			 textDisableFlagId6: 0,
			 textId7: -1,
			 textEnableFlagId7: 0,
			 textDisableFlagId7: 0,
			 textId8: -1,
			 textEnableFlagId8: 0,
			 textDisableFlagId8: 0,
			 textType1: 0,
			 textType2: 0,
			 textType3: 0,
			 textType4: 0,
			 textType5: 0,
			 textType6: 0,
			 textType7: 0,
			 textType8: 0,
			 distViewId: -1,
			 posX_forDistViewMark: 0.,
			 posY_forDistViewMark: 0.,
			 posZ_forDistViewMark: 0.,
			 distViewId1: -1,
			 distViewId2: -1,
			 distViewId3: -1,
			 dispMinZoomStep: 0,
			 selectMinZoomStep: 0,
			 entryFEType: 0,
			 pad4: [0;9],
			 unkC0: 0,
			 unkC4: 0,
			 unkC8: 0,
			 unkCC: 0,
			 unkD0: 0,
			 unkD4: 0,
			 unkD8: 0,
			 unkDC: 0,
			 unkE0: 0,
			 unkE4: 0,
			 unkE8: 0,
			 unkEC: 0,
			 unkF0: 0,
			 unkF4: 0,
			 unkF8: 0,
			 unkFC: 0,
		}
	}
}

#[repr(C, packed)]
#[derive(Clone)]
#[allow(unused,non_snake_case, non_camel_case_types)]
pub struct WWISE_VALUE_TO_STR_CONVERT_PARAM_ST {
	bits_0: u8,
	pub disableParamReserve2: [u8;3],
	pub ParamStr: [u8;32],
}
#[allow(unused,non_snake_case, non_camel_case_types)]
impl WWISE_VALUE_TO_STR_CONVERT_PARAM_ST {
	pub fn disableParam_NT(&self) -> bool {	
			self.bits_0 & (1 << 0) != 0
	}
	pub fn disableParamReserve1(&self) -> bool {	
			self.bits_0 & (1 << 1) != 0
	}
}
impl Default for WWISE_VALUE_TO_STR_CONVERT_PARAM_ST {
	fn default() -> Self {
		Self {
			 bits_0: 0,
			 disableParamReserve2: [0;3],
			 ParamStr: [0;32],
		}
	}
}

