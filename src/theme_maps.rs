// manual mapping of background to foreground color
// this is designed to be compatibile with powerline-go
// which has the colors hardcoded into the system to
// ensure they're very pretty

use std::collections::HashMap;
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub static THEME_MAP: Lazy<HashMap<&'static str, HashMap<u8, u8>>> = Lazy::new(|| {
    HashMap::from([
    ("default", HashMap::from([
        (0, 250),     (1, 250),     (2, 120),     (3, 228),     (4, 250),     
        (5, 250),     (6, 123),     (7, 238),     (8, 0),       (9, 0),       
        (10, 0),      (11, 0),      (12, 250),    (13, 0),      (14, 0),      
        (15, 242),    (16, 250),    (17, 250),    (18, 250),    (19, 189),    
        (20, 254),    (21, 250),    (22, 83),     (23, 87),     (24, 117),    
        (25, 188),    (26, 254),    (27, 0),      (28, 120),    (29, 122),    
        (30, 123),    (31, 159),    (32, 255),    (33, 0),      (34, 157),    
        (35, 158),    (36, 159),    (37, 159),    (38, 195),    (39, 0),      
        (40, 194),    (41, 194),    (42, 195),    (43, 195),    (44, 195),    
        (45, 0),      (46, 0),      (47, 0),      (48, 0),      (49, 0),      
        (50, 0),      (51, 0),      (52, 250),    (53, 250),    (54, 250),    
        (55, 189),    (56, 254),    (57, 250),    (58, 227),    (59, 253),    
        (60, 255),    (61, 0),      (62, 233),    (63, 17),     (64, 192),    
        (65, 255),    (66, 195),    (67, 232),    (68, 233),    (69, 17),     
        (70, 193),    (71, 232),    (72, 232),    (73, 232),    (74, 234),    
        (75, 236),    (76, 194),    (77, 235),    (78, 235),    (79, 235),    
        (80, 235),    (81, 237),    (82, 0),      (83, 237),    (84, 237),    
        (85, 237),    (86, 237),    (87, 237),    (88, 250),    (89, 250),    
        (90, 250),    (91, 189),    (92, 254),    (93, 0),      (94, 222),    
        (95, 255),    (96, 255),    (97, 232),    (98, 233),    (99, 17),     
        (100, 228),   (101, 15),    (102, 232),   (103, 233),   (104, 17),    
        (105, 18),    (106, 229),   (107, 232),   (108, 234),   (109, 234),   
        (110, 236),   (111, 54),    (112, 230),   (113, 235),   (114, 22),    
        (115, 237),   (116, 238),   (117, 238),   (118, 0),     (119, 237),   
        (120, 22),    (121, 23),    (122, 23),    (123, 23),    (124, 252),   
        (125, 252),   (126, 189),   (127, 189),   (128, 254),   (129, 0),     
        (130, 223),   (131, 232),   (132, 232),   (133, 232),   (134, 233),   
        (135, 17),    (136, 229),   (137, 232),   (138, 233),   (139, 234),   
        (140, 53),    (141, 18),    (142, 229),   (143, 232),   (144, 234),   
        (145, 236),   (146, 17),    (147, 19),    (148, 230),   (149, 235),   
        (150, 238),   (151, 22),    (152, 23),    (153, 24),    (154, 0),     
        (155, 237),   (156, 22),    (157, 2),     (158, 29),    (159, 6),     
        (160, 254),   (161, 254),   (162, 254),   (163, 254),   (164, 254),   
        (165, 0),     (166, 255),   (167, 233),   (168, 233),   (169, 234),   
        (170, 234),   (171, 235),   (172, 230),   (173, 234),   (174, 52),    
        (175, 235),   (176, 53),    (177, 53),    (178, 230),   (179, 235),   
        (180, 236),   (181, 52),    (182, 53),    (183, 55),    (184, 230),   
        (185, 235),   (186, 238),   (187, 58),    (188, 240),   (189, 20),    
        (190, 0),     (191, 238),   (192, 58),    (193, 64),    (194, 35),    
        (195, 66),    (196, 0),     (197, 0),     (198, 0),     (199, 0),     
        (200, 0),     (201, 0),     (202, 0),     (203, 235),   (204, 235),   
        (205, 235),   (206, 235),   (207, 53),    (208, 0),     (209, 236),   
        (210, 52),    (211, 237),   (212, 53),    (213, 53),    (214, 0),     
        (215, 236),   (216, 238),   (217, 1),     (218, 89),    (219, 5),     
        (220, 0),     (221, 237),   (222, 58),    (223, 95),    (224, 131),   
        (225, 126),   (226, 0),     (227, 238),   (228, 58),    (229, 3),     
        (230, 143),   (231, 242),   (232, 250),   (233, 250),   (234, 250),   
        (235, 250),   (236, 250),   (237, 250),   (238, 251),   (239, 252),   
        (240, 188),   (241, 254),   (242, 254),   (243, 255),   (244, 0),     
        (245, 232),   (246, 233),   (247, 234),   (248, 235),   (249, 236),   
        (250, 237),   (251, 238),   (252, 239),   (253, 240),   (254, 242),   
        (255, 243),   
    ])),
    ("low-contrast", HashMap::from([    
        (0, 250),     (1, 250),     (2, 120),     (3, 228),     (4, 250),     
        (5, 250),     (6, 123),     (7, 238),     (8, 0),       (9, 0),       
        (10, 0),      (11, 0),      (12, 250),    (13, 0),      (14, 0),      
        (15, 242),    (16, 250),    (17, 250),    (18, 250),    (19, 189),    
        (20, 254),    (21, 250),    (22, 83),     (23, 87),     (24, 117),    
        (25, 188),    (26, 254),    (27, 0),      (28, 120),    (29, 122),    
        (30, 123),    (31, 159),    (32, 255),    (33, 0),      (34, 157),    
        (35, 158),    (36, 159),    (37, 159),    (38, 195),    (39, 0),      
        (40, 194),    (41, 194),    (42, 195),    (43, 195),    (44, 195),    
        (45, 0),      (46, 0),      (47, 0),      (48, 0),      (49, 0),      
        (50, 0),      (51, 0),      (52, 250),    (53, 250),    (54, 250),    
        (55, 189),    (56, 254),    (57, 250),    (58, 227),    (59, 253),    
        (60, 255),    (61, 0),      (62, 233),    (63, 17),     (64, 192),    
        (65, 255),    (66, 195),    (67, 232),    (68, 233),    (69, 17),     
        (70, 193),    (71, 232),    (72, 232),    (73, 232),    (74, 234),    
        (75, 236),    (76, 194),    (77, 235),    (78, 235),    (79, 235),    
        (80, 235),    (81, 237),    (82, 0),      (83, 237),    (84, 237),    
        (85, 237),    (86, 237),    (87, 237),    (88, 250),    (89, 250),    
        (90, 250),    (91, 189),    (92, 254),    (93, 0),      (94, 222),    
        (95, 255),    (96, 255),    (97, 232),    (98, 233),    (99, 17),     
        (100, 228),   (101, 15),    (102, 232),   (103, 233),   (104, 17),    
        (105, 18),    (106, 229),   (107, 232),   (108, 234),   (109, 234),   
        (110, 236),   (111, 54),    (112, 230),   (113, 235),   (114, 22),    
        (115, 237),   (116, 238),   (117, 238),   (118, 0),     (119, 237),   
        (120, 22),    (121, 23),    (122, 23),    (123, 23),    (124, 252),   
        (125, 252),   (126, 189),   (127, 189),   (128, 254),   (129, 0),     
        (130, 223),   (131, 232),   (132, 232),   (133, 232),   (134, 233),   
        (135, 17),    (136, 229),   (137, 232),   (138, 233),   (139, 234),   
        (140, 53),    (141, 18),    (142, 229),   (143, 232),   (144, 234),   
        (145, 236),   (146, 17),    (147, 19),    (148, 230),   (149, 235),   
        (150, 238),   (151, 22),    (152, 23),    (153, 24),    (154, 0),     
        (155, 237),   (156, 22),    (157, 2),     (158, 29),    (159, 6),     
        (160, 254),   (161, 254),   (162, 254),   (163, 254),   (164, 254),   
        (165, 0),     (166, 255),   (167, 233),   (168, 233),   (169, 234),   
        (170, 234),   (171, 235),   (172, 230),   (173, 234),   (174, 52),    
        (175, 235),   (176, 53),    (177, 53),    (178, 230),   (179, 235),   
        (180, 236),   (181, 52),    (182, 53),    (183, 55),    (184, 230),   
        (185, 235),   (186, 238),   (187, 58),    (188, 240),   (189, 20),    
        (190, 0),     (191, 238),   (192, 58),    (193, 64),    (194, 35),    
        (195, 66),    (196, 0),     (197, 0),     (198, 0),     (199, 0),     
        (200, 0),     (201, 0),     (202, 0),     (203, 235),   (204, 235),   
        (205, 235),   (206, 235),   (207, 53),    (208, 0),     (209, 236),   
        (210, 52),    (211, 237),   (212, 53),    (213, 53),    (214, 0),     
        (215, 236),   (216, 238),   (217, 1),     (218, 89),    (219, 5),     
        (220, 0),     (221, 237),   (222, 58),    (223, 95),    (224, 131),   
        (225, 126),   (226, 0),     (227, 238),   (228, 58),    (229, 3),     
        (230, 143),   (231, 242),   (232, 250),   (233, 250),   (234, 250),   
        (235, 250),   (236, 250),   (237, 250),   (238, 251),   (239, 252),   
        (240, 188),   (241, 254),   (242, 254),   (243, 255),   (244, 0),     
        (245, 232),   (246, 233),   (247, 234),   (248, 235),   (249, 236),   
        (250, 237),   (251, 238),   (252, 239),   (253, 240),   (254, 242),   
        (255, 243),   
    ])),
    ("solarized-dark16", HashMap::from([    
        (0, 14),      (1, 14),      (2, 120),     (3, 228),     (4, 14),      
        (5, 14),      (6, 123),     (7, 0),       (8, 8),       (9, 8),       
        (10, 8),      (11, 8),      (12, 14),     (13, 8),      (14, 8),      
        (15, 242),    (16, 14),     (17, 14),     (18, 14),     (19, 189),    
        (20, 8),      (21, 14),     (22, 83),     (23, 87),     (24, 117),    
        (25, 188),    (26, 8),      (27, 8),      (28, 120),    (29, 122),    
        (30, 123),    (31, 159),    (32, 8),      (33, 8),      (34, 157),    
        (35, 158),    (36, 159),    (37, 159),    (38, 195),    (39, 8),      
        (40, 194),    (41, 194),    (42, 195),    (43, 195),    (44, 195),    
        (45, 8),      (46, 8),      (47, 8),      (48, 8),      (49, 8),      
        (50, 8),      (51, 8),      (52, 14),     (53, 14),     (54, 14),     
        (55, 189),    (56, 8),      (57, 14),     (58, 227),    (59, 253),    
        (60, 8),      (61, 8),      (62, 233),    (63, 17),     (64, 192),    
        (65, 8),      (66, 195),    (67, 232),    (68, 233),    (69, 17),     
        (70, 193),    (71, 232),    (72, 232),    (73, 232),    (74, 8),      
        (75, 0),      (76, 194),    (77, 0),      (78, 0),      (79, 0),      
        (80, 0),      (81, 0),      (82, 8),      (83, 0),      (84, 0),      
        (85, 0),      (86, 0),      (87, 0),      (88, 14),     (89, 14),     
        (90, 14),     (91, 189),    (92, 8),      (93, 8),      (94, 222),    
        (95, 8),      (96, 8),      (97, 232),    (98, 233),    (99, 17),     
        (100, 228),   (101, 15),    (102, 232),   (103, 233),   (104, 17),    
        (105, 18),    (106, 229),   (107, 232),   (108, 8),     (109, 8),     
        (110, 0),     (111, 54),    (112, 15),    (113, 0),     (114, 2),     
        (115, 0),     (116, 0),     (117, 0),     (118, 8),     (119, 0),     
        (120, 2),     (121, 23),    (122, 23),    (123, 23),    (124, 252),   
        (125, 252),   (126, 189),   (127, 189),   (128, 8),     (129, 8),     
        (130, 223),   (131, 232),   (132, 232),   (133, 232),   (134, 233),   
        (135, 17),    (136, 229),   (137, 232),   (138, 233),   (139, 8),     
        (140, 53),    (141, 18),    (142, 229),   (143, 232),   (144, 8),     
        (145, 0),     (146, 17),    (147, 19),    (148, 15),    (149, 0),     
        (150, 0),     (151, 2),     (152, 23),    (153, 24),    (154, 8),     
        (155, 0),     (156, 2),     (157, 2),     (158, 29),    (159, 6),     
        (160, 8),     (161, 8),     (162, 8),     (163, 8),     (164, 8),     
        (165, 8),     (166, 8),     (167, 233),   (168, 233),   (169, 8),     
        (170, 8),     (171, 0),     (172, 15),    (173, 8),     (174, 1),     
        (175, 0),     (176, 53),    (177, 53),    (178, 15),    (179, 0),     
        (180, 0),     (181, 1),     (182, 53),    (183, 55),    (184, 15),    
        (185, 0),     (186, 0),     (187, 58),    (188, 10),    (189, 4),     
        (190, 8),     (191, 0),     (192, 58),    (193, 2),     (194, 35),    
        (195, 66),    (196, 8),     (197, 8),     (198, 8),     (199, 8),     
        (200, 8),     (201, 8),     (202, 8),     (203, 0),     (204, 0),     
        (205, 0),     (206, 0),     (207, 53),    (208, 8),     (209, 0),     
        (210, 1),     (211, 0),     (212, 53),    (213, 53),    (214, 8),     
        (215, 0),     (216, 0),     (217, 1),     (218, 89),    (219, 5),     
        (220, 8),     (221, 0),     (222, 58),    (223, 95),    (224, 131),   
        (225, 126),   (226, 8),     (227, 0),     (228, 58),    (229, 3),     
        (230, 143),   (231, 242),   (232, 14),    (233, 14),    (234, 14),    
        (235, 14),    (236, 14),    (237, 14),    (238, 251),   (239, 252),   
        (240, 188),   (241, 8),     (242, 8),     (243, 8),     (244, 8),     
        (245, 232),   (246, 233),   (247, 8),     (248, 0),     (249, 0),     
        (250, 0),     (251, 0),     (252, 239),   (253, 10),    (254, 242),   
        (255, 243),   
    ])),
    ("solarized-light16", HashMap::from([    
        (0, 14),      (1, 14),      (2, 120),     (3, 228),     (4, 14),      
        (5, 14),      (6, 123),     (7, 0),       (8, 8),       (9, 8),       
        (10, 8),      (11, 8),      (12, 14),     (13, 8),      (14, 8),      
        (15, 242),    (16, 14),     (17, 14),     (18, 14),     (19, 189),    
        (20, 8),      (21, 14),     (22, 83),     (23, 87),     (24, 117),    
        (25, 188),    (26, 8),      (27, 8),      (28, 120),    (29, 122),    
        (30, 123),    (31, 159),    (32, 8),      (33, 8),      (34, 157),    
        (35, 158),    (36, 159),    (37, 159),    (38, 195),    (39, 8),      
        (40, 194),    (41, 194),    (42, 195),    (43, 195),    (44, 195),    
        (45, 8),      (46, 8),      (47, 8),      (48, 8),      (49, 8),      
        (50, 8),      (51, 8),      (52, 14),     (53, 14),     (54, 14),     
        (55, 189),    (56, 8),      (57, 14),     (58, 227),    (59, 253),    
        (60, 8),      (61, 8),      (62, 233),    (63, 17),     (64, 192),    
        (65, 8),      (66, 195),    (67, 232),    (68, 233),    (69, 17),     
        (70, 193),    (71, 232),    (72, 232),    (73, 232),    (74, 8),      
        (75, 0),      (76, 194),    (77, 0),      (78, 0),      (79, 0),      
        (80, 0),      (81, 0),      (82, 8),      (83, 0),      (84, 0),      
        (85, 0),      (86, 0),      (87, 0),      (88, 14),     (89, 14),     
        (90, 14),     (91, 189),    (92, 8),      (93, 8),      (94, 222),    
        (95, 8),      (96, 8),      (97, 232),    (98, 233),    (99, 17),     
        (100, 228),   (101, 15),    (102, 232),   (103, 233),   (104, 17),    
        (105, 18),    (106, 229),   (107, 232),   (108, 8),     (109, 8),     
        (110, 0),     (111, 54),    (112, 15),    (113, 0),     (114, 2),     
        (115, 0),     (116, 0),     (117, 0),     (118, 8),     (119, 0),     
        (120, 2),     (121, 23),    (122, 23),    (123, 23),    (124, 252),   
        (125, 252),   (126, 189),   (127, 189),   (128, 8),     (129, 8),     
        (130, 223),   (131, 232),   (132, 232),   (133, 232),   (134, 233),   
        (135, 17),    (136, 229),   (137, 232),   (138, 233),   (139, 8),     
        (140, 53),    (141, 18),    (142, 229),   (143, 232),   (144, 8),     
        (145, 0),     (146, 17),    (147, 19),    (148, 15),    (149, 0),     
        (150, 0),     (151, 2),     (152, 23),    (153, 24),    (154, 8),     
        (155, 0),     (156, 2),     (157, 2),     (158, 29),    (159, 6),     
        (160, 8),     (161, 8),     (162, 8),     (163, 8),     (164, 8),     
        (165, 8),     (166, 8),     (167, 233),   (168, 233),   (169, 8),     
        (170, 8),     (171, 0),     (172, 15),    (173, 8),     (174, 1),     
        (175, 0),     (176, 53),    (177, 53),    (178, 15),    (179, 0),     
        (180, 0),     (181, 1),     (182, 53),    (183, 55),    (184, 15),    
        (185, 0),     (186, 0),     (187, 58),    (188, 10),    (189, 4),     
        (190, 8),     (191, 0),     (192, 58),    (193, 2),     (194, 35),    
        (195, 66),    (196, 8),     (197, 8),     (198, 8),     (199, 8),     
        (200, 8),     (201, 8),     (202, 8),     (203, 0),     (204, 0),     
        (205, 0),     (206, 0),     (207, 53),    (208, 8),     (209, 0),     
        (210, 1),     (211, 0),     (212, 53),    (213, 53),    (214, 8),     
        (215, 0),     (216, 0),     (217, 1),     (218, 89),    (219, 5),     
        (220, 8),     (221, 0),     (222, 58),    (223, 95),    (224, 131),   
        (225, 126),   (226, 8),     (227, 0),     (228, 58),    (229, 3),     
        (230, 143),   (231, 242),   (232, 14),    (233, 14),    (234, 14),    
        (235, 14),    (236, 14),    (237, 14),    (238, 251),   (239, 252),   
        (240, 188),   (241, 8),     (242, 8),     (243, 8),     (244, 8),     
        (245, 232),   (246, 233),   (247, 8),     (248, 0),     (249, 0),     
        (250, 0),     (251, 0),     (252, 239),   (253, 10),    (254, 242),   
        (255, 243),   
    ])),
])
});
