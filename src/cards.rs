use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct BlowupCard {
    pub id: usize,
    pub theme: CardType,
    pub crop: [([f64; 2], [f64; 2]); 2],
}

#[derive(Clone,Debug,PartialEq)]
pub enum CardType {
    Normal(usize, usize), //page_i,index
    Rotatable(usize, usize, usize, usize), //(normal_page_i,normal_index,rotate_page_index,rotate_index)
}
fn sprite_config(ct: CardType) -> [([f64; 2], [f64; 2]); 2] {
    let portrait_info = SpriteInfo{
        first: (90.0, 2130.0),
        num_in_row: 3.0,
        w_h: (510.0, 675.0),
        pad: (10.0, 10.0, 0.0, 0.0),
    };
    let landscape_info = SpriteInfo {
        first: (100.0, 1610.0),
        num_in_row: 3.0,
        w_h: (675.0, 480.0),
        pad: (0.0, 0.0, 0.0, 30.0),
    };
    match ct {
        CardType::Normal(_, index) => {
            [portrait_info.src_rect(index as f64), portrait_info.src_rect(index as f64)]
        }
        CardType::Rotatable(_, index, _, index2) => {
            [portrait_info.src_rect(index as f64), landscape_info.src_rect(index2 as f64)]
        }
    }
}
macro_rules! blowupcard_map {
    ($(($id:expr,$theme:expr)),* $(,)*) => {{
  let cards: HashMap<usize,BlowupCard> =[
        $(($id,BlowupCard{
                  id:$id,
                  theme:$theme,
                  crop:sprite_config($theme),
             }),)*
               ].iter().cloned().collect();
               cards
    }}
}
#[cfg(feature = "english")]
pub fn populate() -> HashMap<usize, BlowupCard> {
    blowupcard_map!{
       (0,CardType::Normal(0,0)),
       (1,CardType::Normal(0,1)),
       (2,CardType::Normal(0,2)),
       (3,CardType::Normal(0,3)),
       (4,CardType::Normal(0,4)),
       (5,CardType::Normal(0,5)),
       (6,CardType::Normal(0,6)),
       (7,CardType::Normal(0,7)),
       (8,CardType::Normal(0,8)),
       (9,CardType::Normal(1,0)),
       (10,CardType::Normal(1,1)),
       (11,CardType::Normal(1,2)),
       (12,CardType::Normal(1,3)),
       (13,CardType::Normal(1,4)),
       (14,CardType::Normal(1,5)),
       (15,CardType::Normal(1,6)),
       (16,CardType::Normal(1,7)),
       (18,CardType::Normal(2,0)),
       (19,CardType::Normal(2,1)),
       (20,CardType::Normal(2,2)),
       (21,CardType::Normal(2,3)),
       (22,CardType::Normal(2,4)),
       (23,CardType::Normal(2,5)),
       (24,CardType::Normal(2,6)),
       (25,CardType::Normal(2,7)),
       (26,CardType::Normal(2,8)),
       (27,CardType::Rotatable(3,0,22,6)),
       (28,CardType::Normal(3,1)),
       (29,CardType::Rotatable(3,2,22,0)),
       (30,CardType::Normal(3,3)),
       (31,CardType::Rotatable(3,4,22,4)),
       (32,CardType::Normal(3,5)),
       (33,CardType::Rotatable(3,6,22,8)),
       (34,CardType::Normal(3,7)), //end of adventure
       (35,CardType::Normal(3,8)),
       (36,CardType::Normal(4,0)),
       (37,CardType::Normal(4,1)),
       (38,CardType::Normal(4,2)),
       (39,CardType::Normal(4,3)),
       (40,CardType::Normal(4,4)),
       (41,CardType::Normal(4,5)),
       (42,CardType::Normal(4,6)),
       (43,CardType::Normal(4,7)),
       (44,CardType::Normal(4,8)),
       (45,CardType::Normal(5,0)),
       (46,CardType::Normal(5,1)),
       (47,CardType::Normal(5,2)),
       (48,CardType::Normal(5,3)),
       (49,CardType::Normal(5,4)),
       (50,CardType::Normal(5,5)),
       (51,CardType::Normal(5,6)),
       (52,CardType::Normal(5,7)),
       (53,CardType::Normal(5,8)),
       (54,CardType::Normal(6,0)),
       (55,CardType::Normal(6,1)),
       (56,CardType::Normal(6,2)),
       (57,CardType::Normal(6,3)),
       (58,CardType::Normal(6,4)),
       (59,CardType::Normal(6,5)),
       (60,CardType::Rotatable(6,6,23,8)),
       (61,CardType::Normal(6,7)),
       (62,CardType::Normal(6,8)),
       (63,CardType::Normal(7,0)),
       (64,CardType::Rotatable(7,1,24,3)),
       (65,CardType::Normal(7,2)),
       (66,CardType::Normal(7,3)),
       (67,CardType::Rotatable(7,4,24,4)),
       (68,CardType::Rotatable(7,5,24,1)),
       (69,CardType::Normal(7,6)),
       (70,CardType::Normal(7,7)),
       (71,CardType::Normal(7,8)),
       (72,CardType::Normal(8,0)),
       (73,CardType::Normal(8,1)),
       (74,CardType::Normal(8,2)),
       (75,CardType::Normal(8,3)),
       (76,CardType::Normal(8,4)),
       (77,CardType::Normal(8,5)),
       (78,CardType::Normal(8,6)),
       (79,CardType::Normal(8,7)),
       (80,CardType::Normal(8,8)),
       (81,CardType::Normal(9,0)),
       (82,CardType::Normal(9,1)),
       (83,CardType::Normal(9,2)),
       (84,CardType::Normal(9,3)),
       (85,CardType::Normal(9,4)),
       (86,CardType::Normal(9,5)),
       (87,CardType::Normal(9,6)),
       (88,CardType::Normal(9,7)),
       (89,CardType::Normal(9,8)),
       (90,CardType::Normal(10,0)),
       (91,CardType::Normal(10,1)),
       (92,CardType::Normal(10,2)),
       (93,CardType::Normal(10,3)),
       (94,CardType::Normal(10,4)),
       (95,CardType::Normal(10,5)),
       (96,CardType::Rotatable(10,6,25,8)),
       (97,CardType::Rotatable(10,7,25,5)),
       (98,CardType::Rotatable(10,8,25,2)),
       (99,CardType::Normal(11,0)),
       (100,CardType::Normal(11,1)),
       (101,CardType::Rotatable(11,2,26,0)),
       (102,CardType::Normal(11,3)),
       (103,CardType::Normal(11,4)),
       (104,CardType::Normal(11,5)),
       (105,CardType::Normal(11,6)),
       (106,CardType::Normal(11,7)),
       (107,CardType::Normal(11,8)),
       (108,CardType::Normal(12,0)),
       (109,CardType::Normal(12,1)),
       (110,CardType::Normal(12,2)),
       (111,CardType::Normal(12,3)),
       (112,CardType::Normal(12,4)),
       (113,CardType::Normal(12,5)),
       (114,CardType::Normal(12,6)),
       (115,CardType::Normal(12,7)),
       (116,CardType::Normal(12,8)),
       (117,CardType::Normal(13,0)),
       (118,CardType::Normal(13,1)),
       (119,CardType::Normal(13,2)),
       (120,CardType::Normal(13,3)),
       (121,CardType::Normal(13,4)),
       (122,CardType::Normal(13,5)),
       (123,CardType::Normal(13,6)),
       (124,CardType::Normal(13,7)),
       (125,CardType::Normal(13,8)),
       (126,CardType::Normal(14,0)),
       (127,CardType::Normal(14,1)),
       (128,CardType::Normal(14,2)),
       (129,CardType::Rotatable(14,3,27,7)),
       (130,CardType::Normal(14,4)),
       (131,CardType::Normal(14,5)),
       (132,CardType::Normal(14,6)),
       (133,CardType::Rotatable(14,7,27,5)),
       (134,CardType::Normal(14,8)),
       (135,CardType::Rotatable(15,0,28,6)),
       (136,CardType::Rotatable(15,1,28,3)),
       (137,CardType::Normal(15,2)),
       (138,CardType::Normal(15,3)),
       (139,CardType::Normal(15,4)),
       (140,CardType::Normal(15,5)),
       (141,CardType::Normal(15,6)),
       (142,CardType::Normal(15,7)),
       (143,CardType::Normal(15,8)),
       (144,CardType::Normal(16,0)),
       (145,CardType::Normal(16,1)),
       (146,CardType::Normal(16,2)),
       (147,CardType::Normal(16,3)),
       (148,CardType::Normal(16,4)),
       (149,CardType::Normal(16,5)),
       (150,CardType::Normal(16,6)),
       (151,CardType::Normal(16,7)),
       (152,CardType::Normal(16,8)),
       (153,CardType::Normal(17,0)),
       (154,CardType::Normal(17,1)),
       (155,CardType::Normal(17,2)),
       (156,CardType::Normal(17,3)),
       (157,CardType::Normal(17,4)),
       (158,CardType::Normal(17,5)),
       (159,CardType::Normal(17,6)),
       (160,CardType::Normal(17,7)),
       (161,CardType::Normal(17,8)),
       (162,CardType::Normal(18,0)),
       (163,CardType::Normal(18,1)),
       (164,CardType::Normal(18,2)),
       (165,CardType::Normal(18,3)),
       (166,CardType::Normal(18,4)),
       (167,CardType::Normal(18,5)),
       (168,CardType::Normal(18,6)),
       (169,CardType::Normal(18,7)),
       (170,CardType::Normal(18,8)),
       (171,CardType::Normal(19,0)),
       (172,CardType::Normal(19,1)),
       (173,CardType::Normal(19,2)),
       (174,CardType::Normal(19,3)),
       (175,CardType::Normal(19,4)),
       (176,CardType::Normal(19,5)),
       (177,CardType::Normal(19,6)),
       (178,CardType::Normal(19,7)),
       (179,CardType::Normal(19,8)),
       (180,CardType::Normal(20,0)),
       (181,CardType::Normal(20,1)),
       (182,CardType::Normal(20,2)),
       (183,CardType::Normal(20,3)),
       (184,CardType::Normal(20,4)),
       (185,CardType::Normal(20,5)),
       (186,CardType::Normal(20,6)),
       (189,CardType::Normal(20,7)),
       (190,CardType::Normal(20,8)),
       (191,CardType::Normal(21,0)),
       (192,CardType::Normal(21,1)),
       (193,CardType::Normal(21,2)),
       (194,CardType::Normal(21,3)),
       (195,CardType::Normal(21,4)),
       (196,CardType::Normal(21,5)),
       (197,CardType::Normal(21,6)),
       (198,CardType::Normal(21,7)),
       (199,CardType::Normal(21,8)),
       
   }
}
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct SpriteInfo {
    pub first: (f64, f64), //left corner of first
    pub num_in_row: f64,
    pub w_h: (f64, f64),
    pub pad: (f64, f64, f64, f64),
}
impl SpriteInfo {
    pub fn src_rect(&self, index: f64) -> ([f64; 2], [f64; 2]) {
        let s = self;
        let (x, y) = (index % s.num_in_row as f64, (index / (s.num_in_row)).floor());
        ([s.first.0 + x * s.w_h.0 + s.pad.0, s.first.1 - y * s.w_h.1 - s.pad.2],
         [s.first.0 + (x + 1.0) * s.w_h.0 - s.pad.1, s.first.1 - (y + 1.0) * s.w_h.1 + s.pad.3])

    }
}
