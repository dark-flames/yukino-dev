use generic_array::typenum::*;

use crate::view::ValueCount;

pub trait ResultIndex: ValueCount {
    fn index() -> &'static str;
}

macro_rules! generate {
    [$($n: ident), *] => {
        $(
            impl ResultIndex for $n {
                fn index() -> &'static str {
                    stringify!($n)
                }
            }
        )*
    }
}

generate![
    U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20,
    U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31, U32, U33, U34, U35, U36, U37, U38, U39,
    U40, U41, U42, U43, U44, U45, U46, U47, U48, U49, U50, U51, U52, U53, U54, U55, U56, U57, U58,
    U59, U60, U61, U62, U63, U64, U65, U66, U67, U68, U69, U70, U71, U72, U73, U74, U75, U76, U77,
    U78, U79, U80, U81, U82, U83, U84, U85, U86, U87, U88, U89, U90, U91, U92, U93, U94, U95, U96,
    U97, U98, U99, U100, U101, U102, U103, U104, U105, U106, U107, U108, U109, U110, U111, U112,
    U113, U114, U115, U116, U117, U118, U119, U120, U121, U122, U123, U124, U125, U126, U127, U128,
    U129, U130, U131, U132, U133, U134, U135, U136, U137, U138, U139, U140, U141, U142, U143, U144,
    U145, U146, U147, U148, U149, U150, U151, U152, U153, U154, U155, U156, U157, U158, U159, U160,
    U161, U162, U163, U164, U165, U166, U167, U168, U169, U170, U171, U172, U173, U174, U175, U176,
    U177, U178, U179, U180, U181, U182, U183, U184, U185, U186, U187, U188, U189, U190, U191, U192,
    U193, U194, U195, U196, U197, U198, U199, U200, U201, U202, U203, U204, U205, U206, U207, U208,
    U209, U210, U211, U212, U213, U214, U215, U216, U217, U218, U219, U220, U221, U222, U223, U224,
    U225, U226, U227, U228, U229, U230, U231, U232, U233, U234, U235, U236, U237, U238, U239, U240,
    U241, U242, U243, U244, U245, U246, U247, U248, U249, U250, U251, U252, U253, U254, U255, U256,
    U257, U258, U259, U260, U261, U262, U263, U264, U265, U266, U267, U268, U269, U270, U271, U272,
    U273, U274, U275, U276, U277, U278, U279, U280, U281, U282, U283, U284, U285, U286, U287, U288,
    U289, U290, U291, U292, U293, U294, U295, U296, U297, U298, U299, U300, U301, U302, U303, U304,
    U305, U306, U307, U308, U309, U310, U311, U312, U313, U314, U315, U316, U317, U318, U319, U320,
    U321, U322, U323, U324, U325, U326, U327, U328, U329, U330, U331, U332, U333, U334, U335, U336,
    U337, U338, U339, U340, U341, U342, U343, U344, U345, U346, U347, U348, U349, U350, U351, U352,
    U353, U354, U355, U356, U357, U358, U359, U360, U361, U362, U363, U364, U365, U366, U367, U368,
    U369, U370, U371, U372, U373, U374, U375, U376, U377, U378, U379, U380, U381, U382, U383, U384,
    U385, U386, U387, U388, U389, U390, U391, U392, U393, U394, U395, U396, U397, U398, U399, U400,
    U401, U402, U403, U404, U405, U406, U407, U408, U409, U410, U411, U412, U413, U414, U415, U416,
    U417, U418, U419, U420, U421, U422, U423, U424, U425, U426, U427, U428, U429, U430, U431, U432,
    U433, U434, U435, U436, U437, U438, U439, U440, U441, U442, U443, U444, U445, U446, U447, U448,
    U449, U450, U451, U452, U453, U454, U455, U456, U457, U458, U459, U460, U461, U462, U463, U464,
    U465, U466, U467, U468, U469, U470, U471, U472, U473, U474, U475, U476, U477, U478, U479, U480,
    U481, U482, U483, U484, U485, U486, U487, U488, U489, U490, U491, U492, U493, U494, U495, U496,
    U497, U498, U499, U500, U501, U502, U503, U504, U505, U506, U507, U508, U509, U510, U511, U512,
    U513, U514, U515, U516, U517, U518, U519, U520, U521, U522, U523, U524, U525, U526, U527, U528,
    U529, U530, U531, U532, U533, U534, U535, U536, U537, U538, U539, U540, U541, U542, U543, U544,
    U545, U546, U547, U548, U549, U550, U551, U552, U553, U554, U555, U556, U557, U558, U559, U560,
    U561, U562, U563, U564, U565, U566, U567, U568, U569, U570, U571, U572, U573, U574, U575, U576,
    U577, U578, U579, U580, U581, U582, U583, U584, U585, U586, U587, U588, U589, U590, U591, U592,
    U593, U594, U595, U596, U597, U598, U599, U600, U601, U602, U603, U604, U605, U606, U607, U608,
    U609, U610, U611, U612, U613, U614, U615, U616, U617, U618, U619, U620, U621, U622, U623, U624,
    U625, U626, U627, U628, U629, U630, U631, U632, U633, U634, U635, U636, U637, U638, U639, U640,
    U641, U642, U643, U644, U645, U646, U647, U648, U649, U650, U651, U652, U653, U654, U655, U656,
    U657, U658, U659, U660, U661, U662, U663, U664, U665, U666, U667, U668, U669, U670, U671, U672,
    U673, U674, U675, U676, U677, U678, U679, U680, U681, U682, U683, U684, U685, U686, U687, U688,
    U689, U690, U691, U692, U693, U694, U695, U696, U697, U698, U699, U700, U701, U702, U703, U704,
    U705, U706, U707, U708, U709, U710, U711, U712, U713, U714, U715, U716, U717, U718, U719, U720,
    U721, U722, U723, U724, U725, U726, U727, U728, U729, U730, U731, U732, U733, U734, U735, U736,
    U737, U738, U739, U740, U741, U742, U743, U744, U745, U746, U747, U748, U749, U750, U751, U752,
    U753, U754, U755, U756, U757, U758, U759, U760, U761, U762, U763, U764, U765, U766, U767, U768,
    U769, U770, U771, U772, U773, U774, U775, U776, U777, U778, U779, U780, U781, U782, U783, U784,
    U785, U786, U787, U788, U789, U790, U791, U792, U793, U794, U795, U796, U797, U798, U799, U800,
    U801, U802, U803, U804, U805, U806, U807, U808, U809, U810, U811, U812, U813, U814, U815, U816,
    U817, U818, U819, U820, U821, U822, U823, U824, U825, U826, U827, U828, U829, U830, U831, U832,
    U833, U834, U835, U836, U837, U838, U839, U840, U841, U842, U843, U844, U845, U846, U847, U848,
    U849, U850, U851, U852, U853, U854, U855, U856, U857, U858, U859, U860, U861, U862, U863, U864,
    U865, U866, U867, U868, U869, U870, U871, U872, U873, U874, U875, U876, U877, U878, U879, U880,
    U881, U882, U883, U884, U885, U886, U887, U888, U889, U890, U891, U892, U893, U894, U895, U896,
    U897, U898, U899, U900, U901, U902, U903, U904, U905, U906, U907, U908, U909, U910, U911, U912,
    U913, U914, U915, U916, U917, U918, U919, U920, U921, U922, U923, U924, U925, U926, U927, U928,
    U929, U930, U931, U932, U933, U934, U935, U936, U937, U938, U939, U940, U941, U942, U943, U944,
    U945, U946, U947, U948, U949, U950, U951, U952, U953, U954, U955, U956, U957, U958, U959, U960,
    U961, U962, U963, U964, U965, U966, U967, U968, U969, U970, U971, U972, U973, U974, U975, U976,
    U977, U978, U979, U980, U981, U982, U983, U984, U985, U986, U987, U988, U989, U990, U991, U992,
    U993, U994, U995, U996, U997, U998, U999, U1000, U1001, U1002, U1003, U1004, U1005, U1006,
    U1007, U1008, U1009, U1010, U1011, U1012, U1013, U1014, U1015, U1016, U1017, U1018, U1019,
    U1020, U1021, U1022, U1023, U1024
];
