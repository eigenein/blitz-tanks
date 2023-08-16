//! Auto-generated tankopedia, to update run `blitz-tanks bundle-tankopedia`.

use crate::models::{TankId, Vehicle, VehicleAvailability::*, VehicleType::*};

pub static ALL_TANK_IDS: [TankId; 612] = [
    TankId(1),
    TankId(17),
    TankId(33),
    TankId(49),
    TankId(81),
    TankId(113),
    TankId(257),
    TankId(289),
    TankId(321),
    TankId(337),
    TankId(353),
    TankId(385),
    TankId(513),
    TankId(529),
    TankId(545),
    TankId(577),
    TankId(593),
    TankId(609),
    TankId(625),
    TankId(641),
    TankId(769),
    TankId(785),
    TankId(801),
    TankId(817),
    TankId(849),
    TankId(865),
    TankId(881),
    TankId(897),
    TankId(1025),
    TankId(1041),
    TankId(1057),
    TankId(1073),
    TankId(1089),
    TankId(1105),
    TankId(1121),
    TankId(1137),
    TankId(1153),
    TankId(1297),
    TankId(1313),
    TankId(1329),
    TankId(1361),
    TankId(1377),
    TankId(1393),
    TankId(1409),
    TankId(1537),
    TankId(1553),
    TankId(1569),
    TankId(1585),
    TankId(1601),
    TankId(1617),
    TankId(1633),
    TankId(1649),
    TankId(1665),
    TankId(1809),
    TankId(1825),
    TankId(1841),
    TankId(1857),
    TankId(1889),
    TankId(1905),
    TankId(1921),
    TankId(2049),
    TankId(2065),
    TankId(2097),
    TankId(2129),
    TankId(2145),
    TankId(2161),
    TankId(2177),
    TankId(2305),
    TankId(2321),
    TankId(2353),
    TankId(2369),
    TankId(2385),
    TankId(2401),
    TankId(2433),
    TankId(2561),
    TankId(2577),
    TankId(2593),
    TankId(2609),
    TankId(2625),
    TankId(2657),
    TankId(2689),
    TankId(2817),
    TankId(2849),
    TankId(2865),
    TankId(2881),
    TankId(2897),
    TankId(2913),
    TankId(2945),
    TankId(3073),
    TankId(3089),
    TankId(3105),
    TankId(3121),
    TankId(3137),
    TankId(3153),
    TankId(3201),
    TankId(3329),
    TankId(3345),
    TankId(3361),
    TankId(3377),
    TankId(3425),
    TankId(3457),
    TankId(3585),
    TankId(3601),
    TankId(3633),
    TankId(3649),
    TankId(3681),
    TankId(3697),
    TankId(3713),
    TankId(3857),
    TankId(3873),
    TankId(3889),
    TankId(3905),
    TankId(3921),
    TankId(3937),
    TankId(3953),
    TankId(3969),
    TankId(4113),
    TankId(4145),
    TankId(4193),
    TankId(4209),
    TankId(4225),
    TankId(4353),
    TankId(4369),
    TankId(4385),
    TankId(4401),
    TankId(4417),
    TankId(4433),
    TankId(4449),
    TankId(4465),
    TankId(4481),
    TankId(4609),
    TankId(4657),
    TankId(4689),
    TankId(4705),
    TankId(4721),
    TankId(4737),
    TankId(4881),
    TankId(4897),
    TankId(4913),
    TankId(4929),
    TankId(4945),
    TankId(4961),
    TankId(4977),
    TankId(4993),
    TankId(5121),
    TankId(5137),
    TankId(5153),
    TankId(5169),
    TankId(5185),
    TankId(5201),
    TankId(5217),
    TankId(5233),
    TankId(5249),
    TankId(5377),
    TankId(5393),
    TankId(5409),
    TankId(5425),
    TankId(5441),
    TankId(5457),
    TankId(5473),
    TankId(5489),
    TankId(5505),
    TankId(5665),
    TankId(5681),
    TankId(5713),
    TankId(5729),
    TankId(5745),
    TankId(5761),
    TankId(5889),
    TankId(5921),
    TankId(5937),
    TankId(5953),
    TankId(5969),
    TankId(5985),
    TankId(6001),
    TankId(6017),
    TankId(6145),
    TankId(6161),
    TankId(6177),
    TankId(6193),
    TankId(6209),
    TankId(6225),
    TankId(6241),
    TankId(6257),
    TankId(6273),
    TankId(6401),
    TankId(6417),
    TankId(6433),
    TankId(6449),
    TankId(6465),
    TankId(6481),
    TankId(6497),
    TankId(6529),
    TankId(6657),
    TankId(6673),
    TankId(6689),
    TankId(6705),
    TankId(6721),
    TankId(6753),
    TankId(6785),
    TankId(6913),
    TankId(6929),
    TankId(6945),
    TankId(6961),
    TankId(6977),
    TankId(6993),
    TankId(7009),
    TankId(7025),
    TankId(7041),
    TankId(7169),
    TankId(7185),
    TankId(7201),
    TankId(7217),
    TankId(7249),
    TankId(7265),
    TankId(7281),
    TankId(7297),
    TankId(7425),
    TankId(7441),
    TankId(7473),
    TankId(7505),
    TankId(7537),
    TankId(7553),
    TankId(7697),
    TankId(7713),
    TankId(7729),
    TankId(7745),
    TankId(7761),
    TankId(7793),
    TankId(7809),
    TankId(7937),
    TankId(7953),
    TankId(7985),
    TankId(8001),
    TankId(8017),
    TankId(8049),
    TankId(8065),
    TankId(8193),
    TankId(8209),
    TankId(8225),
    TankId(8241),
    TankId(8257),
    TankId(8273),
    TankId(8305),
    TankId(8321),
    TankId(8465),
    TankId(8497),
    TankId(8513),
    TankId(8529),
    TankId(8561),
    TankId(8577),
    TankId(8737),
    TankId(8753),
    TankId(8785),
    TankId(8817),
    TankId(8833),
    TankId(8961),
    TankId(8993),
    TankId(9009),
    TankId(9041),
    TankId(9073),
    TankId(9089),
    TankId(9217),
    TankId(9249),
    TankId(9265),
    TankId(9297),
    TankId(9329),
    TankId(9345),
    TankId(9489),
    TankId(9505),
    TankId(9521),
    TankId(9553),
    TankId(9601),
    TankId(9745),
    TankId(9761),
    TankId(9777),
    TankId(9793),
    TankId(9809),
    TankId(9841),
    TankId(9857),
    TankId(9985),
    TankId(10001),
    TankId(10017),
    TankId(10033),
    TankId(10049),
    TankId(10065),
    TankId(10097),
    TankId(10113),
    TankId(10241),
    TankId(10257),
    TankId(10273),
    TankId(10289),
    TankId(10353),
    TankId(10369),
    TankId(10497),
    TankId(10513),
    TankId(10529),
    TankId(10545),
    TankId(10609),
    TankId(10625),
    TankId(10753),
    TankId(10769),
    TankId(10785),
    TankId(10801),
    TankId(10817),
    TankId(10865),
    TankId(10881),
    TankId(11009),
    TankId(11025),
    TankId(11041),
    TankId(11057),
    TankId(11073),
    TankId(11121),
    TankId(11137),
    TankId(11265),
    TankId(11281),
    TankId(11297),
    TankId(11393),
    TankId(11521),
    TankId(11537),
    TankId(11553),
    TankId(11585),
    TankId(11649),
    TankId(11777),
    TankId(11793),
    TankId(11809),
    TankId(11905),
    TankId(12033),
    TankId(12049),
    TankId(12065),
    TankId(12097),
    TankId(12161),
    TankId(12305),
    TankId(12321),
    TankId(12417),
    TankId(12545),
    TankId(12673),
    TankId(12929),
    TankId(13073),
    TankId(13089),
    TankId(13185),
    TankId(13329),
    TankId(13345),
    TankId(13441),
    TankId(13569),
    TankId(13697),
    TankId(13825),
    TankId(13841),
    TankId(13857),
    TankId(13889),
    TankId(13953),
    TankId(14097),
    TankId(14113),
    TankId(14145),
    TankId(14209),
    TankId(14337),
    TankId(14609),
    TankId(14625),
    TankId(14721),
    TankId(14865),
    TankId(14881),
    TankId(14977),
    TankId(15137),
    TankId(15393),
    TankId(15441),
    TankId(15617),
    TankId(15649),
    TankId(15697),
    TankId(15889),
    TankId(15905),
    TankId(15937),
    TankId(15953),
    TankId(16145),
    TankId(16193),
    TankId(16209),
    TankId(16257),
    TankId(16401),
    TankId(16449),
    TankId(16465),
    TankId(16641),
    TankId(16657),
    TankId(16673),
    TankId(16705),
    TankId(16897),
    TankId(17169),
    TankId(17217),
    TankId(17233),
    TankId(17425),
    TankId(17473),
    TankId(17489),
    TankId(17729),
    TankId(17745),
    TankId(17953),
    TankId(17985),
    TankId(18001),
    TankId(18177),
    TankId(18209),
    TankId(18241),
    TankId(18257),
    TankId(18433),
    TankId(18449),
    TankId(18465),
    TankId(18497),
    TankId(18513),
    TankId(18689),
    TankId(18721),
    TankId(18753),
    TankId(18769),
    TankId(18945),
    TankId(18961),
    TankId(18977),
    TankId(19009),
    TankId(19025),
    TankId(19201),
    TankId(19217),
    TankId(19233),
    TankId(19265),
    TankId(19281),
    TankId(19457),
    TankId(19473),
    TankId(19489),
    TankId(19521),
    TankId(19537),
    TankId(19713),
    TankId(19729),
    TankId(19745),
    TankId(19777),
    TankId(19793),
    TankId(19969),
    TankId(19985),
    TankId(20001),
    TankId(20033),
    TankId(20049),
    TankId(20241),
    TankId(20257),
    TankId(20289),
    TankId(20305),
    TankId(20481),
    TankId(20497),
    TankId(20513),
    TankId(20545),
    TankId(20561),
    TankId(20737),
    TankId(20753),
    TankId(20769),
    TankId(20817),
    TankId(20993),
    TankId(21009),
    TankId(21025),
    TankId(21073),
    TankId(21249),
    TankId(21265),
    TankId(21281),
    TankId(21329),
    TankId(21505),
    TankId(21521),
    TankId(21537),
    TankId(21585),
    TankId(21761),
    TankId(21777),
    TankId(21793),
    TankId(21841),
    TankId(22017),
    TankId(22033),
    TankId(22049),
    TankId(22097),
    TankId(22273),
    TankId(22289),
    TankId(22305),
    TankId(22353),
    TankId(22529),
    TankId(22545),
    TankId(22561),
    TankId(22609),
    TankId(22785),
    TankId(22801),
    TankId(22817),
    TankId(22865),
    TankId(23041),
    TankId(23057),
    TankId(23073),
    TankId(23121),
    TankId(23297),
    TankId(23313),
    TankId(23329),
    TankId(23553),
    TankId(23569),
    TankId(23585),
    TankId(23809),
    TankId(23825),
    TankId(23841),
    TankId(24065),
    TankId(24081),
    TankId(24097),
    TankId(24321),
    TankId(24337),
    TankId(24577),
    TankId(24593),
    TankId(24609),
    TankId(24849),
    TankId(24865),
    TankId(25089),
    TankId(25105),
    TankId(25345),
    TankId(25361),
    TankId(25377),
    TankId(25601),
    TankId(25617),
    TankId(25633),
    TankId(25857),
    TankId(25889),
    TankId(26113),
    TankId(26129),
    TankId(26145),
    TankId(26401),
    TankId(26641),
    TankId(26657),
    TankId(26913),
    TankId(27169),
    TankId(27425),
    TankId(27681),
    TankId(27937),
    TankId(28193),
    TankId(28449),
    TankId(51201),
    TankId(51457),
    TankId(51473),
    TankId(51489),
    TankId(51713),
    TankId(51729),
    TankId(51745),
    TankId(51809),
    TankId(51985),
    TankId(52065),
    TankId(52225),
    TankId(52241),
    TankId(52257),
    TankId(52481),
    TankId(52497),
    TankId(52513),
    TankId(52561),
    TankId(52737),
    TankId(52769),
    TankId(52993),
    TankId(53025),
    TankId(53249),
    TankId(53505),
    TankId(53537),
    TankId(53585),
    TankId(53761),
    TankId(53841),
    TankId(54097),
    TankId(54273),
    TankId(54289),
    TankId(54353),
    TankId(54529),
    TankId(54545),
    TankId(54785),
    TankId(54801),
    TankId(54865),
    TankId(55057),
    TankId(55073),
    TankId(55297),
    TankId(55313),
    TankId(55889),
    TankId(56097),
    TankId(56577),
    TankId(56609),
    TankId(57105),
    TankId(57361),
    TankId(57617),
    TankId(58641),
    TankId(58881),
    TankId(59137),
    TankId(59649),
    TankId(59665),
    TankId(59905),
    TankId(60161),
    TankId(60177),
    TankId(60417),
    TankId(60929),
    TankId(62737),
    TankId(62977),
    TankId(62993),
    TankId(63553),
    TankId(63585),
    TankId(63601),
    TankId(63841),
    TankId(64001),
    TankId(64017),
    TankId(64065),
    TankId(64081),
    TankId(64257),
    TankId(64273),
    TankId(64337),
    TankId(64529),
    TankId(64561),
    TankId(64593),
    TankId(64769),
    TankId(64785),
    TankId(64801),
    TankId(64849),
    TankId(65025),
    TankId(65041),
    TankId(65057),
    TankId(65105),
    TankId(65281),
    TankId(65297),
    TankId(65313),
    TankId(65329),
    TankId(65361),
    TankId(65377),
];

pub const fn is_known_tank_id(tank_id: TankId) -> bool {
    matches!(
        tank_id,
        TankId(1)
            | TankId(17)
            | TankId(33)
            | TankId(49)
            | TankId(81)
            | TankId(113)
            | TankId(257)
            | TankId(289)
            | TankId(321)
            | TankId(337)
            | TankId(353)
            | TankId(385)
            | TankId(513)
            | TankId(529)
            | TankId(545)
            | TankId(577)
            | TankId(593)
            | TankId(609)
            | TankId(625)
            | TankId(641)
            | TankId(769)
            | TankId(785)
            | TankId(801)
            | TankId(817)
            | TankId(849)
            | TankId(865)
            | TankId(881)
            | TankId(897)
            | TankId(1025)
            | TankId(1041)
            | TankId(1057)
            | TankId(1073)
            | TankId(1089)
            | TankId(1105)
            | TankId(1121)
            | TankId(1137)
            | TankId(1153)
            | TankId(1297)
            | TankId(1313)
            | TankId(1329)
            | TankId(1361)
            | TankId(1377)
            | TankId(1393)
            | TankId(1409)
            | TankId(1537)
            | TankId(1553)
            | TankId(1569)
            | TankId(1585)
            | TankId(1601)
            | TankId(1617)
            | TankId(1633)
            | TankId(1649)
            | TankId(1665)
            | TankId(1809)
            | TankId(1825)
            | TankId(1841)
            | TankId(1857)
            | TankId(1889)
            | TankId(1905)
            | TankId(1921)
            | TankId(2049)
            | TankId(2065)
            | TankId(2097)
            | TankId(2129)
            | TankId(2145)
            | TankId(2161)
            | TankId(2177)
            | TankId(2305)
            | TankId(2321)
            | TankId(2353)
            | TankId(2369)
            | TankId(2385)
            | TankId(2401)
            | TankId(2433)
            | TankId(2561)
            | TankId(2577)
            | TankId(2593)
            | TankId(2609)
            | TankId(2625)
            | TankId(2657)
            | TankId(2689)
            | TankId(2817)
            | TankId(2849)
            | TankId(2865)
            | TankId(2881)
            | TankId(2897)
            | TankId(2913)
            | TankId(2945)
            | TankId(3073)
            | TankId(3089)
            | TankId(3105)
            | TankId(3121)
            | TankId(3137)
            | TankId(3153)
            | TankId(3201)
            | TankId(3329)
            | TankId(3345)
            | TankId(3361)
            | TankId(3377)
            | TankId(3425)
            | TankId(3457)
            | TankId(3585)
            | TankId(3601)
            | TankId(3633)
            | TankId(3649)
            | TankId(3681)
            | TankId(3697)
            | TankId(3713)
            | TankId(3857)
            | TankId(3873)
            | TankId(3889)
            | TankId(3905)
            | TankId(3921)
            | TankId(3937)
            | TankId(3953)
            | TankId(3969)
            | TankId(4113)
            | TankId(4145)
            | TankId(4193)
            | TankId(4209)
            | TankId(4225)
            | TankId(4353)
            | TankId(4369)
            | TankId(4385)
            | TankId(4401)
            | TankId(4417)
            | TankId(4433)
            | TankId(4449)
            | TankId(4465)
            | TankId(4481)
            | TankId(4609)
            | TankId(4657)
            | TankId(4689)
            | TankId(4705)
            | TankId(4721)
            | TankId(4737)
            | TankId(4881)
            | TankId(4897)
            | TankId(4913)
            | TankId(4929)
            | TankId(4945)
            | TankId(4961)
            | TankId(4977)
            | TankId(4993)
            | TankId(5121)
            | TankId(5137)
            | TankId(5153)
            | TankId(5169)
            | TankId(5185)
            | TankId(5201)
            | TankId(5217)
            | TankId(5233)
            | TankId(5249)
            | TankId(5377)
            | TankId(5393)
            | TankId(5409)
            | TankId(5425)
            | TankId(5441)
            | TankId(5457)
            | TankId(5473)
            | TankId(5489)
            | TankId(5505)
            | TankId(5665)
            | TankId(5681)
            | TankId(5713)
            | TankId(5729)
            | TankId(5745)
            | TankId(5761)
            | TankId(5889)
            | TankId(5921)
            | TankId(5937)
            | TankId(5953)
            | TankId(5969)
            | TankId(5985)
            | TankId(6001)
            | TankId(6017)
            | TankId(6145)
            | TankId(6161)
            | TankId(6177)
            | TankId(6193)
            | TankId(6209)
            | TankId(6225)
            | TankId(6241)
            | TankId(6257)
            | TankId(6273)
            | TankId(6401)
            | TankId(6417)
            | TankId(6433)
            | TankId(6449)
            | TankId(6465)
            | TankId(6481)
            | TankId(6497)
            | TankId(6529)
            | TankId(6657)
            | TankId(6673)
            | TankId(6689)
            | TankId(6705)
            | TankId(6721)
            | TankId(6753)
            | TankId(6785)
            | TankId(6913)
            | TankId(6929)
            | TankId(6945)
            | TankId(6961)
            | TankId(6977)
            | TankId(6993)
            | TankId(7009)
            | TankId(7025)
            | TankId(7041)
            | TankId(7169)
            | TankId(7185)
            | TankId(7201)
            | TankId(7217)
            | TankId(7249)
            | TankId(7265)
            | TankId(7281)
            | TankId(7297)
            | TankId(7425)
            | TankId(7441)
            | TankId(7473)
            | TankId(7505)
            | TankId(7537)
            | TankId(7553)
            | TankId(7697)
            | TankId(7713)
            | TankId(7729)
            | TankId(7745)
            | TankId(7761)
            | TankId(7793)
            | TankId(7809)
            | TankId(7937)
            | TankId(7953)
            | TankId(7985)
            | TankId(8001)
            | TankId(8017)
            | TankId(8049)
            | TankId(8065)
            | TankId(8193)
            | TankId(8209)
            | TankId(8225)
            | TankId(8241)
            | TankId(8257)
            | TankId(8273)
            | TankId(8305)
            | TankId(8321)
            | TankId(8465)
            | TankId(8497)
            | TankId(8513)
            | TankId(8529)
            | TankId(8561)
            | TankId(8577)
            | TankId(8737)
            | TankId(8753)
            | TankId(8785)
            | TankId(8817)
            | TankId(8833)
            | TankId(8961)
            | TankId(8993)
            | TankId(9009)
            | TankId(9041)
            | TankId(9073)
            | TankId(9089)
            | TankId(9217)
            | TankId(9249)
            | TankId(9265)
            | TankId(9297)
            | TankId(9329)
            | TankId(9345)
            | TankId(9489)
            | TankId(9505)
            | TankId(9521)
            | TankId(9553)
            | TankId(9601)
            | TankId(9745)
            | TankId(9761)
            | TankId(9777)
            | TankId(9793)
            | TankId(9809)
            | TankId(9841)
            | TankId(9857)
            | TankId(9985)
            | TankId(10001)
            | TankId(10017)
            | TankId(10033)
            | TankId(10049)
            | TankId(10065)
            | TankId(10097)
            | TankId(10113)
            | TankId(10241)
            | TankId(10257)
            | TankId(10273)
            | TankId(10289)
            | TankId(10353)
            | TankId(10369)
            | TankId(10497)
            | TankId(10513)
            | TankId(10529)
            | TankId(10545)
            | TankId(10609)
            | TankId(10625)
            | TankId(10753)
            | TankId(10769)
            | TankId(10785)
            | TankId(10801)
            | TankId(10817)
            | TankId(10865)
            | TankId(10881)
            | TankId(11009)
            | TankId(11025)
            | TankId(11041)
            | TankId(11057)
            | TankId(11073)
            | TankId(11121)
            | TankId(11137)
            | TankId(11265)
            | TankId(11281)
            | TankId(11297)
            | TankId(11393)
            | TankId(11521)
            | TankId(11537)
            | TankId(11553)
            | TankId(11585)
            | TankId(11649)
            | TankId(11777)
            | TankId(11793)
            | TankId(11809)
            | TankId(11905)
            | TankId(12033)
            | TankId(12049)
            | TankId(12065)
            | TankId(12097)
            | TankId(12161)
            | TankId(12305)
            | TankId(12321)
            | TankId(12417)
            | TankId(12545)
            | TankId(12673)
            | TankId(12929)
            | TankId(13073)
            | TankId(13089)
            | TankId(13185)
            | TankId(13329)
            | TankId(13345)
            | TankId(13441)
            | TankId(13569)
            | TankId(13697)
            | TankId(13825)
            | TankId(13841)
            | TankId(13857)
            | TankId(13889)
            | TankId(13953)
            | TankId(14097)
            | TankId(14113)
            | TankId(14145)
            | TankId(14209)
            | TankId(14337)
            | TankId(14609)
            | TankId(14625)
            | TankId(14721)
            | TankId(14865)
            | TankId(14881)
            | TankId(14977)
            | TankId(15137)
            | TankId(15393)
            | TankId(15441)
            | TankId(15617)
            | TankId(15649)
            | TankId(15697)
            | TankId(15889)
            | TankId(15905)
            | TankId(15937)
            | TankId(15953)
            | TankId(16145)
            | TankId(16193)
            | TankId(16209)
            | TankId(16257)
            | TankId(16401)
            | TankId(16449)
            | TankId(16465)
            | TankId(16641)
            | TankId(16657)
            | TankId(16673)
            | TankId(16705)
            | TankId(16897)
            | TankId(17169)
            | TankId(17217)
            | TankId(17233)
            | TankId(17425)
            | TankId(17473)
            | TankId(17489)
            | TankId(17729)
            | TankId(17745)
            | TankId(17953)
            | TankId(17985)
            | TankId(18001)
            | TankId(18177)
            | TankId(18209)
            | TankId(18241)
            | TankId(18257)
            | TankId(18433)
            | TankId(18449)
            | TankId(18465)
            | TankId(18497)
            | TankId(18513)
            | TankId(18689)
            | TankId(18721)
            | TankId(18753)
            | TankId(18769)
            | TankId(18945)
            | TankId(18961)
            | TankId(18977)
            | TankId(19009)
            | TankId(19025)
            | TankId(19201)
            | TankId(19217)
            | TankId(19233)
            | TankId(19265)
            | TankId(19281)
            | TankId(19457)
            | TankId(19473)
            | TankId(19489)
            | TankId(19521)
            | TankId(19537)
            | TankId(19713)
            | TankId(19729)
            | TankId(19745)
            | TankId(19777)
            | TankId(19793)
            | TankId(19969)
            | TankId(19985)
            | TankId(20001)
            | TankId(20033)
            | TankId(20049)
            | TankId(20241)
            | TankId(20257)
            | TankId(20289)
            | TankId(20305)
            | TankId(20481)
            | TankId(20497)
            | TankId(20513)
            | TankId(20545)
            | TankId(20561)
            | TankId(20737)
            | TankId(20753)
            | TankId(20769)
            | TankId(20817)
            | TankId(20993)
            | TankId(21009)
            | TankId(21025)
            | TankId(21073)
            | TankId(21249)
            | TankId(21265)
            | TankId(21281)
            | TankId(21329)
            | TankId(21505)
            | TankId(21521)
            | TankId(21537)
            | TankId(21585)
            | TankId(21761)
            | TankId(21777)
            | TankId(21793)
            | TankId(21841)
            | TankId(22017)
            | TankId(22033)
            | TankId(22049)
            | TankId(22097)
            | TankId(22273)
            | TankId(22289)
            | TankId(22305)
            | TankId(22353)
            | TankId(22529)
            | TankId(22545)
            | TankId(22561)
            | TankId(22609)
            | TankId(22785)
            | TankId(22801)
            | TankId(22817)
            | TankId(22865)
            | TankId(23041)
            | TankId(23057)
            | TankId(23073)
            | TankId(23121)
            | TankId(23297)
            | TankId(23313)
            | TankId(23329)
            | TankId(23553)
            | TankId(23569)
            | TankId(23585)
            | TankId(23809)
            | TankId(23825)
            | TankId(23841)
            | TankId(24065)
            | TankId(24081)
            | TankId(24097)
            | TankId(24321)
            | TankId(24337)
            | TankId(24577)
            | TankId(24593)
            | TankId(24609)
            | TankId(24849)
            | TankId(24865)
            | TankId(25089)
            | TankId(25105)
            | TankId(25345)
            | TankId(25361)
            | TankId(25377)
            | TankId(25601)
            | TankId(25617)
            | TankId(25633)
            | TankId(25857)
            | TankId(25889)
            | TankId(26113)
            | TankId(26129)
            | TankId(26145)
            | TankId(26401)
            | TankId(26641)
            | TankId(26657)
            | TankId(26913)
            | TankId(27169)
            | TankId(27425)
            | TankId(27681)
            | TankId(27937)
            | TankId(28193)
            | TankId(28449)
            | TankId(51201)
            | TankId(51457)
            | TankId(51473)
            | TankId(51489)
            | TankId(51713)
            | TankId(51729)
            | TankId(51745)
            | TankId(51809)
            | TankId(51985)
            | TankId(52065)
            | TankId(52225)
            | TankId(52241)
            | TankId(52257)
            | TankId(52481)
            | TankId(52497)
            | TankId(52513)
            | TankId(52561)
            | TankId(52737)
            | TankId(52769)
            | TankId(52993)
            | TankId(53025)
            | TankId(53249)
            | TankId(53505)
            | TankId(53537)
            | TankId(53585)
            | TankId(53761)
            | TankId(53841)
            | TankId(54097)
            | TankId(54273)
            | TankId(54289)
            | TankId(54353)
            | TankId(54529)
            | TankId(54545)
            | TankId(54785)
            | TankId(54801)
            | TankId(54865)
            | TankId(55057)
            | TankId(55073)
            | TankId(55297)
            | TankId(55313)
            | TankId(55889)
            | TankId(56097)
            | TankId(56577)
            | TankId(56609)
            | TankId(57105)
            | TankId(57361)
            | TankId(57617)
            | TankId(58641)
            | TankId(58881)
            | TankId(59137)
            | TankId(59649)
            | TankId(59665)
            | TankId(59905)
            | TankId(60161)
            | TankId(60177)
            | TankId(60417)
            | TankId(60929)
            | TankId(62737)
            | TankId(62977)
            | TankId(62993)
            | TankId(63553)
            | TankId(63585)
            | TankId(63601)
            | TankId(63841)
            | TankId(64001)
            | TankId(64017)
            | TankId(64065)
            | TankId(64081)
            | TankId(64257)
            | TankId(64273)
            | TankId(64337)
            | TankId(64529)
            | TankId(64561)
            | TankId(64593)
            | TankId(64769)
            | TankId(64785)
            | TankId(64801)
            | TankId(64849)
            | TankId(65025)
            | TankId(65041)
            | TankId(65057)
            | TankId(65105)
            | TankId(65281)
            | TankId(65297)
            | TankId(65313)
            | TankId(65329)
            | TankId(65361)
            | TankId(65377)
    )
}

pub const fn get_vehicle(tank_id: TankId) -> Option<Vehicle> {
    match tank_id {
        TankId(1) => Some(Vehicle {
            tank_id: TankId(1),
            name: "T-34",
            tier: 5,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1.webp"),
        }),
        TankId(17) => Some(Vehicle {
            tank_id: TankId(17),
            name: "Pz. IV G",
            tier: 5,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/17.webp"),
        }),
        TankId(33) => Some(Vehicle {
            tank_id: TankId(33),
            name: "T14",
            tier: 5,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/33.webp"),
        }),
        TankId(49) => Some(Vehicle {
            tank_id: TankId(49),
            name: "Type 59",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/49.webp"),
        }),
        TankId(81) => Some(Vehicle {
            tank_id: TankId(81),
            name: "Medium I",
            tier: 1,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/81.webp"),
        }),
        TankId(113) => Some(Vehicle {
            tank_id: TankId(113),
            name: "Vindicator UM",
            tier: 7,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/113.webp"),
        }),
        TankId(257) => Some(Vehicle {
            tank_id: TankId(257),
            name: "SU-85",
            tier: 5,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/257.webp"),
        }),
        TankId(289) => Some(Vehicle {
            tank_id: TankId(289),
            name: "M3 Stuart",
            tier: 2,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/289.webp"),
        }),
        TankId(321) => Some(Vehicle {
            tank_id: TankId(321),
            name: "D2",
            tier: 3,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/321.webp"),
        }),
        TankId(337) => Some(Vehicle {
            tank_id: TankId(337),
            name: "Medium II",
            tier: 2,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/337.webp"),
        }),
        TankId(353) => Some(Vehicle {
            tank_id: TankId(353),
            name: "Chi-Ni",
            tier: 2,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/353.webp"),
        }),
        TankId(385) => Some(Vehicle {
            tank_id: TankId(385),
            name: "Progetto 65",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/385.webp"),
        }),
        TankId(513) => Some(Vehicle {
            tank_id: TankId(513),
            name: "IS",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/513.webp"),
        }),
        TankId(529) => Some(Vehicle {
            tank_id: TankId(529),
            name: "Tiger I",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/529.webp"),
        }),
        TankId(545) => Some(Vehicle {
            tank_id: TankId(545),
            name: "T1",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/545.webp"),
        }),
        TankId(577) => Some(Vehicle {
            tank_id: TankId(577),
            name: "FT",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/577.webp"),
        }),
        TankId(593) => Some(Vehicle {
            tank_id: TankId(593),
            name: "Sherman Firefly",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/593.webp"),
        }),
        TankId(609) => Some(Vehicle {
            tank_id: TankId(609),
            name: "R. Otsu",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/609.webp"),
        }),
        TankId(625) => Some(Vehicle {
            tank_id: TankId(625),
            name: "Strv 74A2",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/625.webp"),
        }),
        TankId(641) => Some(Vehicle {
            tank_id: TankId(641),
            name: "Standard B",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/641.webp"),
        }),
        TankId(769) => Some(Vehicle {
            tank_id: TankId(769),
            name: "BT-7",
            tier: 3,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/769.webp"),
        }),
        TankId(785) => Some(Vehicle {
            tank_id: TankId(785),
            name: "Pz. 35 (t)",
            tier: 2,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/785.webp"),
        }),
        TankId(801) => Some(Vehicle {
            tank_id: TankId(801),
            name: "M6",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/801.webp"),
        }),
        TankId(817) => Some(Vehicle {
            tank_id: TankId(817),
            name: "WZ-111",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/817.webp"),
        }),
        TankId(849) => Some(Vehicle {
            tank_id: TankId(849),
            name: "Matilda",
            tier: 4,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/849.webp"),
        }),
        TankId(865) => Some(Vehicle {
            tank_id: TankId(865),
            name: "Ha-Go",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/865.webp"),
        }),
        TankId(881) => Some(Vehicle {
            tank_id: TankId(881),
            name: "Edelweiss",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/881.webp"),
        }),
        TankId(897) => Some(Vehicle {
            tank_id: TankId(897),
            name: "P.44 Pantera",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/897.webp"),
        }),
        TankId(1025) => Some(Vehicle {
            tank_id: TankId(1025),
            name: "BT-2",
            tier: 2,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/1025.webp"),
        }),
        TankId(1041) => Some(Vehicle {
            tank_id: TankId(1041),
            name: "StuG III G",
            tier: 5,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/1041.webp"),
        }),
        TankId(1057) => Some(Vehicle {
            tank_id: TankId(1057),
            name: "M4 Sherman",
            tier: 5,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1057.webp"),
        }),
        TankId(1073) => Some(Vehicle {
            tank_id: TankId(1073),
            name: "T-34-1",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1073.webp"),
        }),
        TankId(1089) => Some(Vehicle {
            tank_id: TankId(1089),
            name: "B1",
            tier: 4,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/1089.webp"),
        }),
        TankId(1105) => Some(Vehicle {
            tank_id: TankId(1105),
            name: "Cromwell",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1105.webp"),
        }),
        TankId(1121) => Some(Vehicle {
            tank_id: TankId(1121),
            name: "Chi-Ri",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1121.webp"),
        }),
        TankId(1137) => Some(Vehicle {
            tank_id: TankId(1137),
            name: "Predator UM",
            tier: 7,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/1137.webp"),
        }),
        TankId(1153) => Some(Vehicle {
            tank_id: TankId(1153),
            name: "P.43 ter",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1153.webp"),
        }),
        TankId(1297) => Some(Vehicle {
            tank_id: TankId(1297),
            name: "Panther",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1297.webp"),
        }),
        TankId(1313) => Some(Vehicle {
            tank_id: TankId(1313),
            name: "M4A3E8",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1313.webp"),
        }),
        TankId(1329) => Some(Vehicle {
            tank_id: TankId(1329),
            name: "NC-31",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/1329.webp"),
        }),
        TankId(1361) => Some(Vehicle {
            tank_id: TankId(1361),
            name: "Churchill Mk. VI",
            tier: 6,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/1361.webp"),
        }),
        TankId(1377) => Some(Vehicle {
            tank_id: TankId(1377),
            name: "Chi-Nu",
            tier: 5,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1377.webp"),
        }),
        TankId(1393) => Some(Vehicle {
            tank_id: TankId(1393),
            name: "Nameless",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/1393.webp"),
        }),
        TankId(1409) => Some(Vehicle {
            tank_id: TankId(1409),
            name: "P.43 bis",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1409.webp"),
        }),
        TankId(1537) => Some(Vehicle {
            tank_id: TankId(1537),
            name: "T-28 mod. 1940",
            tier: 4,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/1537.webp"),
        }),
        TankId(1553) => Some(Vehicle {
            tank_id: TankId(1553),
            name: "Jg.Pz. IV",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/1553.webp"),
        }),
        TankId(1569) => Some(Vehicle {
            tank_id: TankId(1569),
            name: "T20",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1569.webp"),
        }),
        TankId(1585) => Some(Vehicle {
            tank_id: TankId(1585),
            name: "T-34-2",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1585.webp"),
        }),
        TankId(1601) => Some(Vehicle {
            tank_id: TankId(1601),
            name: "D1",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/1601.webp"),
        }),
        TankId(1617) => Some(Vehicle {
            tank_id: TankId(1617),
            name: "Sherman V",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/1617.webp"),
        }),
        TankId(1633) => Some(Vehicle {
            tank_id: TankId(1633),
            name: "Chi-He",
            tier: 4,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1633.webp"),
        }),
        TankId(1649) => Some(Vehicle {
            tank_id: TankId(1649),
            name: "Helsing",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/1649.webp"),
        }),
        TankId(1665) => Some(Vehicle {
            tank_id: TankId(1665),
            name: "Lago",
            tier: 4,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1665.webp"),
        }),
        TankId(1809) => Some(Vehicle {
            tank_id: TankId(1809),
            name: "Hetzer",
            tier: 4,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/1809.webp"),
        }),
        TankId(1825) => Some(Vehicle {
            tank_id: TankId(1825),
            name: "M2 Light",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/1825.webp"),
        }),
        TankId(1841) => Some(Vehicle {
            tank_id: TankId(1841),
            name: "WZ-120",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1841.webp"),
        }),
        TankId(1857) => Some(Vehicle {
            tank_id: TankId(1857),
            name: "B-C 25 t AP",
            tier: 9,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/1857.webp"),
        }),
        TankId(1889) => Some(Vehicle {
            tank_id: TankId(1889),
            name: "Chi-To",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1889.webp"),
        }),
        TankId(1905) => Some(Vehicle {
            tank_id: TankId(1905),
            name: "O-47",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/1905.webp"),
        }),
        TankId(1921) => Some(Vehicle {
            tank_id: TankId(1921),
            name: "Strv m/42",
            tier: 5,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/1921.webp"),
        }),
        TankId(2049) => Some(Vehicle {
            tank_id: TankId(2049),
            name: "A-20",
            tier: 4,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2049.webp"),
        }),
        TankId(2065) => Some(Vehicle {
            tank_id: TankId(2065),
            name: "Pz. II",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2065.webp"),
        }),
        TankId(2097) => Some(Vehicle {
            tank_id: TankId(2097),
            name: "WZ-111 1-4",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/2097.webp"),
        }),
        TankId(2129) => Some(Vehicle {
            tank_id: TankId(2129),
            name: "Crusader",
            tier: 5,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2129.webp"),
        }),
        TankId(2145) => Some(Vehicle {
            tank_id: TankId(2145),
            name: "Chi-Ha",
            tier: 3,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/2145.webp"),
        }),
        TankId(2161) => Some(Vehicle {
            tank_id: TankId(2161),
            name: "WZ Blaze",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/2161.webp"),
        }),
        TankId(2177) => Some(Vehicle {
            tank_id: TankId(2177),
            name: "14TP",
            tier: 3,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2177.webp"),
        }),
        TankId(2305) => Some(Vehicle {
            tank_id: TankId(2305),
            name: "SU-152",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/2305.webp"),
        }),
        TankId(2321) => Some(Vehicle {
            tank_id: TankId(2321),
            name: "VK 36.01 H",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/2321.webp"),
        }),
        TankId(2353) => Some(Vehicle {
            tank_id: TankId(2353),
            name: "VAE Type B",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2353.webp"),
        }),
        TankId(2369) => Some(Vehicle {
            tank_id: TankId(2369),
            name: "FCM36Pak40",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/2369.webp"),
        }),
        TankId(2385) => Some(Vehicle {
            tank_id: TankId(2385),
            name: "Medium III",
            tier: 3,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/2385.webp"),
        }),
        TankId(2401) => Some(Vehicle {
            tank_id: TankId(2401),
            name: "Ke-Ni",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/2401.webp"),
        }),
        TankId(2433) => Some(Vehicle {
            tank_id: TankId(2433),
            name: "10TP",
            tier: 2,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2433.webp"),
        }),
        TankId(2561) => Some(Vehicle {
            tank_id: TankId(2561),
            name: "T-34-85",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/2561.webp"),
        }),
        TankId(2577) => Some(Vehicle {
            tank_id: TankId(2577),
            name: "VK 30.01 H",
            tier: 5,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/2577.webp"),
        }),
        TankId(2593) => Some(Vehicle {
            tank_id: TankId(2593),
            name: "T30",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/2593.webp"),
        }),
        TankId(2609) => Some(Vehicle {
            tank_id: TankId(2609),
            name: "Type 64",
            tier: 6,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/2609.webp"),
        }),
        TankId(2625) => Some(Vehicle {
            tank_id: TankId(2625),
            name: "ARL 44",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/2625.webp"),
        }),
        TankId(2657) => Some(Vehicle {
            tank_id: TankId(2657),
            name: "STA-1",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/2657.webp"),
        }),
        TankId(2689) => Some(Vehicle {
            tank_id: TankId(2689),
            name: "Vickers Mk. F",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/2689.webp"),
        }),
        TankId(2817) => Some(Vehicle {
            tank_id: TankId(2817),
            name: "KV-1S",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/2817.webp"),
        }),
        TankId(2849) => Some(Vehicle {
            tank_id: TankId(2849),
            name: "T34",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/2849.webp"),
        }),
        TankId(2865) => Some(Vehicle {
            tank_id: TankId(2865),
            name: "WZ-110",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/2865.webp"),
        }),
        TankId(2881) => Some(Vehicle {
            tank_id: TankId(2881),
            name: "AMX 40",
            tier: 4,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/2881.webp"),
        }),
        TankId(2897) => Some(Vehicle {
            tank_id: TankId(2897),
            name: "Churchill I",
            tier: 5,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/2897.webp"),
        }),
        TankId(2913) => Some(Vehicle {
            tank_id: TankId(2913),
            name: "Ke-Ho",
            tier: 4,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/2913.webp"),
        }),
        TankId(2945) => Some(Vehicle {
            tank_id: TankId(2945),
            name: "Progetto 46",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/2945.webp"),
        }),
        TankId(3073) => Some(Vehicle {
            tank_id: TankId(3073),
            name: "T-46",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/3073.webp"),
        }),
        TankId(3089) => Some(Vehicle {
            tank_id: TankId(3089),
            name: "L.Tr.",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/3089.webp"),
        }),
        TankId(3105) => Some(Vehicle {
            tank_id: TankId(3105),
            name: "M3 Lee",
            tier: 4,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/3105.webp"),
        }),
        TankId(3121) => Some(Vehicle {
            tank_id: TankId(3121),
            name: "M5A1 Stuart",
            tier: 4,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/3121.webp"),
        }),
        TankId(3137) => Some(Vehicle {
            tank_id: TankId(3137),
            name: "AMX 50 100",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3137.webp"),
        }),
        TankId(3153) => Some(Vehicle {
            tank_id: TankId(3153),
            name: "Black Prince",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3153.webp"),
        }),
        TankId(3201) => Some(Vehicle {
            tank_id: TankId(3201),
            name: "50TP prototyp",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/3201.webp"),
        }),
        TankId(3329) => Some(Vehicle {
            tank_id: TankId(3329),
            name: "MS-1 mod. 1",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/3329.webp"),
        }),
        TankId(3345) => Some(Vehicle {
            tank_id: TankId(3345),
            name: "Pz. 38 (t)",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/3345.webp"),
        }),
        TankId(3361) => Some(Vehicle {
            tank_id: TankId(3361),
            name: "T1 Heavy",
            tier: 5,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3361.webp"),
        }),
        TankId(3377) => Some(Vehicle {
            tank_id: TankId(3377),
            name: "WZ-131",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/3377.webp"),
        }),
        TankId(3425) => Some(Vehicle {
            tank_id: TankId(3425),
            name: "Type 61",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/3425.webp"),
        }),
        TankId(3457) => Some(Vehicle {
            tank_id: TankId(3457),
            name: "Emil I",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3457.webp"),
        }),
        TankId(3585) => Some(Vehicle {
            tank_id: TankId(3585),
            name: "SU-100",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/3585.webp"),
        }),
        TankId(3601) => Some(Vehicle {
            tank_id: TankId(3601),
            name: "Pz.Jäg. I",
            tier: 2,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/3601.webp"),
        }),
        TankId(3633) => Some(Vehicle {
            tank_id: TankId(3633),
            name: "IS-2",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3633.webp"),
        }),
        TankId(3649) => Some(Vehicle {
            tank_id: TankId(3649),
            name: "B-C 25 t",
            tier: 10,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/3649.webp"),
        }),
        TankId(3681) => Some(Vehicle {
            tank_id: TankId(3681),
            name: "STB-1",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/3681.webp"),
        }),
        TankId(3697) => Some(Vehicle {
            tank_id: TankId(3697),
            name: "Lupus",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/3697.webp"),
        }),
        TankId(3713) => Some(Vehicle {
            tank_id: TankId(3713),
            name: "Strv 74",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/3713.webp"),
        }),
        TankId(3857) => Some(Vehicle {
            tank_id: TankId(3857),
            name: "JPanther",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/3857.webp"),
        }),
        TankId(3873) => Some(Vehicle {
            tank_id: TankId(3873),
            name: "T29",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3873.webp"),
        }),
        TankId(3889) => Some(Vehicle {
            tank_id: TankId(3889),
            name: "WZ-132",
            tier: 8,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/3889.webp"),
        }),
        TankId(3905) => Some(Vehicle {
            tank_id: TankId(3905),
            name: "AMX 50 120",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3905.webp"),
        }),
        TankId(3921) => Some(Vehicle {
            tank_id: TankId(3921),
            name: "Caernarvon",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/3921.webp"),
        }),
        TankId(3937) => Some(Vehicle {
            tank_id: TankId(3937),
            name: "Ho-Ri",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/3937.webp"),
        }),
        TankId(3953) => Some(Vehicle {
            tank_id: TankId(3953),
            name: "T 55A",
            tier: 9,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/3953.webp"),
        }),
        TankId(3969) => Some(Vehicle {
            tank_id: TankId(3969),
            name: "Leo",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/3969.webp"),
        }),
        TankId(4113) => Some(Vehicle {
            tank_id: TankId(4113),
            name: "VK 30.02 D",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4113.webp"),
        }),
        TankId(4145) => Some(Vehicle {
            tank_id: TankId(4145),
            name: "WZ-121",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4145.webp"),
        }),
        TankId(4193) => Some(Vehicle {
            tank_id: TankId(4193),
            name: "Ho-Ri T.II",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/4193.webp"),
        }),
        TankId(4209) => Some(Vehicle {
            tank_id: TankId(4209),
            name: "WarDuck",
            tier: 1,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4209.webp"),
        }),
        TankId(4225) => Some(Vehicle {
            tank_id: TankId(4225),
            name: "Emil II",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/4225.webp"),
        }),
        TankId(4353) => Some(Vehicle {
            tank_id: TankId(4353),
            name: "T-44",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4353.webp"),
        }),
        TankId(4369) => Some(Vehicle {
            tank_id: TankId(4369),
            name: "Pz. III",
            tier: 3,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4369.webp"),
        }),
        TankId(4385) => Some(Vehicle {
            tank_id: TankId(4385),
            name: "T32",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/4385.webp"),
        }),
        TankId(4401) => Some(Vehicle {
            tank_id: TankId(4401),
            name: "Chi-Ha",
            tier: 3,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4401.webp"),
        }),
        TankId(4417) => Some(Vehicle {
            tank_id: TankId(4417),
            name: "AMX M4 mle. 54",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/4417.webp"),
        }),
        TankId(4433) => Some(Vehicle {
            tank_id: TankId(4433),
            name: "Conqueror",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/4433.webp"),
        }),
        TankId(4449) => Some(Vehicle {
            tank_id: TankId(4449),
            name: "IS-2 Pravda SP",
            tier: 7,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/4449.webp"),
        }),
        TankId(4465) => Some(Vehicle {
            tank_id: TankId(4465),
            name: "Hafen",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/4465.webp"),
        }),
        TankId(4481) => Some(Vehicle {
            tank_id: TankId(4481),
            name: "Kranvagn",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/4481.webp"),
        }),
        TankId(4609) => Some(Vehicle {
            tank_id: TankId(4609),
            name: "T-26",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/4609.webp"),
        }),
        TankId(4657) => Some(Vehicle {
            tank_id: TankId(4657),
            name: "Type T-34",
            tier: 5,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/4657.webp"),
        }),
        TankId(4689) => Some(Vehicle {
            tank_id: TankId(4689),
            name: "Churchill VII",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/4689.webp"),
        }),
        TankId(4705) => Some(Vehicle {
            tank_id: TankId(4705),
            name: "Firefly Saunders SP",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/4705.webp"),
        }),
        TankId(4721) => Some(Vehicle {
            tank_id: TankId(4721),
            name: "Gravedigger",
            tier: 7,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/4721.webp"),
        }),
        TankId(4737) => Some(Vehicle {
            tank_id: TankId(4737),
            name: "EMIL 1951",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/4737.webp"),
        }),
        TankId(4881) => Some(Vehicle {
            tank_id: TankId(4881),
            name: "Pz. III A",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/4881.webp"),
        }),
        TankId(4897) => Some(Vehicle {
            tank_id: TankId(4897),
            name: "M2 Medium",
            tier: 3,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/4897.webp"),
        }),
        TankId(4913) => Some(Vehicle {
            tank_id: TankId(4913),
            name: "59-16",
            tier: 6,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/4913.webp"),
        }),
        TankId(4929) => Some(Vehicle {
            tank_id: TankId(4929),
            name: "AMX 13 90",
            tier: 8,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/4929.webp"),
        }),
        TankId(4945) => Some(Vehicle {
            tank_id: TankId(4945),
            name: "Valentine Mk. IX",
            tier: 4,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/4945.webp"),
        }),
        TankId(4961) => Some(Vehicle {
            tank_id: TankId(4961),
            name: "Ho-Ri T.I",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/4961.webp"),
        }),
        TankId(4977) => Some(Vehicle {
            tank_id: TankId(4977),
            name: "Scavenger",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/4977.webp"),
        }),
        TankId(4993) => Some(Vehicle {
            tank_id: TankId(4993),
            name: "P.43/06 ann.",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/4993.webp"),
        }),
        TankId(5121) => Some(Vehicle {
            tank_id: TankId(5121),
            name: "AT-1",
            tier: 2,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/5121.webp"),
        }),
        TankId(5137) => Some(Vehicle {
            tank_id: TankId(5137),
            name: "Tiger II",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5137.webp"),
        }),
        TankId(5153) => Some(Vehicle {
            tank_id: TankId(5153),
            name: "M5 Stuart",
            tier: 3,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/5153.webp"),
        }),
        TankId(5169) => Some(Vehicle {
            tank_id: TankId(5169),
            name: "Type 58",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5169.webp"),
        }),
        TankId(5185) => Some(Vehicle {
            tank_id: TankId(5185),
            name: "AMX 13 75",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/5185.webp"),
        }),
        TankId(5201) => Some(Vehicle {
            tank_id: TankId(5201),
            name: "Cruiser I",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/5201.webp"),
        }),
        TankId(5217) => Some(Vehicle {
            tank_id: TankId(5217),
            name: "Chi-To SPG",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/5217.webp"),
        }),
        TankId(5233) => Some(Vehicle {
            tank_id: TankId(5233),
            name: "Smasher",
            tier: 7,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/5233.webp"),
        }),
        TankId(5249) => Some(Vehicle {
            tank_id: TankId(5249),
            name: "Pudel",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/5249.webp"),
        }),
        TankId(5377) => Some(Vehicle {
            tank_id: TankId(5377),
            name: "IS-3",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5377.webp"),
        }),
        TankId(5393) => Some(Vehicle {
            tank_id: TankId(5393),
            name: "Leopard",
            tier: 5,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/5393.webp"),
        }),
        TankId(5409) => Some(Vehicle {
            tank_id: TankId(5409),
            name: "M7",
            tier: 4,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5409.webp"),
        }),
        TankId(5425) => Some(Vehicle {
            tank_id: TankId(5425),
            name: "WZ-113",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5425.webp"),
        }),
        TankId(5441) => Some(Vehicle {
            tank_id: TankId(5441),
            name: "AMX 30 1er prot.",
            tier: 9,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/5441.webp"),
        }),
        TankId(5457) => Some(Vehicle {
            tank_id: TankId(5457),
            name: "Comet",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5457.webp"),
        }),
        TankId(5473) => Some(Vehicle {
            tank_id: TankId(5473),
            name: "Mitsu 108",
            tier: 5,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5473.webp"),
        }),
        TankId(5489) => Some(Vehicle {
            tank_id: TankId(5489),
            name: "Y5 T-34",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/5489.webp"),
        }),
        TankId(5505) => Some(Vehicle {
            tank_id: TankId(5505),
            name: "TVP T 50/51",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5505.webp"),
        }),
        TankId(5665) => Some(Vehicle {
            tank_id: TankId(5665),
            name: "T2 Medium",
            tier: 2,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/5665.webp"),
        }),
        TankId(5681) => Some(Vehicle {
            tank_id: TankId(5681),
            name: "121B",
            tier: 10,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/5681.webp"),
        }),
        TankId(5713) => Some(Vehicle {
            tank_id: TankId(5713),
            name: "Centurion 7/1",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5713.webp"),
        }),
        TankId(5729) => Some(Vehicle {
            tank_id: TankId(5729),
            name: "Ju-Nu",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5729.webp"),
        }),
        TankId(5745) => Some(Vehicle {
            tank_id: TankId(5745),
            name: "Y5 Firefly",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/5745.webp"),
        }),
        TankId(5761) => Some(Vehicle {
            tank_id: TankId(5761),
            name: "Škoda T 50",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5761.webp"),
        }),
        TankId(5889) => Some(Vehicle {
            tank_id: TankId(5889),
            name: "KV-3",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5889.webp"),
        }),
        TankId(5921) => Some(Vehicle {
            tank_id: TankId(5921),
            name: "Pershing",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5921.webp"),
        }),
        TankId(5937) => Some(Vehicle {
            tank_id: TankId(5937),
            name: "59-Patton",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/5937.webp"),
        }),
        TankId(5953) => Some(Vehicle {
            tank_id: TankId(5953),
            name: "AMX 38",
            tier: 2,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5953.webp"),
        }),
        TankId(5969) => Some(Vehicle {
            tank_id: TankId(5969),
            name: "Centurion I",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/5969.webp"),
        }),
        TankId(5985) => Some(Vehicle {
            tank_id: TankId(5985),
            name: "Ju-To",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/5985.webp"),
        }),
        TankId(6001) => Some(Vehicle {
            tank_id: TankId(6001),
            name: "Y5 ELC bis",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/6001.webp"),
        }),
        TankId(6017) => Some(Vehicle {
            tank_id: TankId(6017),
            name: "TVP VTU",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/6017.webp"),
        }),
        TankId(6145) => Some(Vehicle {
            tank_id: TankId(6145),
            name: "IS-4",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6145.webp"),
        }),
        TankId(6161) => Some(Vehicle {
            tank_id: TankId(6161),
            name: "Luchs",
            tier: 4,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/6161.webp"),
        }),
        TankId(6177) => Some(Vehicle {
            tank_id: TankId(6177),
            name: "T18",
            tier: 2,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/6177.webp"),
        }),
        TankId(6193) => Some(Vehicle {
            tank_id: TankId(6193),
            name: "T-34-3",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/6193.webp"),
        }),
        TankId(6209) => Some(Vehicle {
            tank_id: TankId(6209),
            name: "AMX 50 B",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6209.webp"),
        }),
        TankId(6225) => Some(Vehicle {
            tank_id: TankId(6225),
            name: "FV215b",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6225.webp"),
        }),
        TankId(6241) => Some(Vehicle {
            tank_id: TankId(6241),
            name: "Chi-Se",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6241.webp"),
        }),
        TankId(6257) => Some(Vehicle {
            tank_id: TankId(6257),
            name: "M4/FL10",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/6257.webp"),
        }),
        TankId(6273) => Some(Vehicle {
            tank_id: TankId(6273),
            name: "T-34/100",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/6273.webp"),
        }),
        TankId(6401) => Some(Vehicle {
            tank_id: TankId(6401),
            name: "SU-76",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/6401.webp"),
        }),
        TankId(6417) => Some(Vehicle {
            tank_id: TankId(6417),
            name: "Pz. III/IV",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/6417.webp"),
        }),
        TankId(6433) => Some(Vehicle {
            tank_id: TankId(6433),
            name: "T82",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/6433.webp"),
        }),
        TankId(6449) => Some(Vehicle {
            tank_id: TankId(6449),
            name: "WZ-113G FT",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/6449.webp"),
        }),
        TankId(6465) => Some(Vehicle {
            tank_id: TankId(6465),
            name: "AMX 12 t",
            tier: 6,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/6465.webp"),
        }),
        TankId(6481) => Some(Vehicle {
            tank_id: TankId(6481),
            name: "Covenanter",
            tier: 4,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/6481.webp"),
        }),
        TankId(6497) => Some(Vehicle {
            tank_id: TankId(6497),
            name: "Type 68",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6497.webp"),
        }),
        TankId(6529) => Some(Vehicle {
            tank_id: TankId(6529),
            name: "Škoda T 25",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/6529.webp"),
        }),
        TankId(6657) => Some(Vehicle {
            tank_id: TankId(6657),
            name: "T-43",
            tier: 7,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/6657.webp"),
        }),
        TankId(6673) => Some(Vehicle {
            tank_id: TankId(6673),
            name: "Marder II",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/6673.webp"),
        }),
        TankId(6689) => Some(Vehicle {
            tank_id: TankId(6689),
            name: "T49 A",
            tier: 7,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/6689.webp"),
        }),
        TankId(6705) => Some(Vehicle {
            tank_id: TankId(6705),
            name: "LT vz. 38",
            tier: 2,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/6705.webp"),
        }),
        TankId(6721) => Some(Vehicle {
            tank_id: TankId(6721),
            name: "BDR G1 B",
            tier: 5,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6721.webp"),
        }),
        TankId(6753) => Some(Vehicle {
            tank_id: TankId(6753),
            name: "Type 71",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6753.webp"),
        }),
        TankId(6785) => Some(Vehicle {
            tank_id: TankId(6785),
            name: "Škoda T 27",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/6785.webp"),
        }),
        TankId(6913) => Some(Vehicle {
            tank_id: TankId(6913),
            name: "SU-85B",
            tier: 4,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/6913.webp"),
        }),
        TankId(6929) => Some(Vehicle {
            tank_id: TankId(6929),
            name: "Maus",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6929.webp"),
        }),
        TankId(6945) => Some(Vehicle {
            tank_id: TankId(6945),
            name: "Wolverine",
            tier: 5,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/6945.webp"),
        }),
        TankId(6961) => Some(Vehicle {
            tank_id: TankId(6961),
            name: "WZ-120-1G FT",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/6961.webp"),
        }),
        TankId(6977) => Some(Vehicle {
            tank_id: TankId(6977),
            name: "AMX M4 45",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/6977.webp"),
        }),
        TankId(6993) => Some(Vehicle {
            tank_id: TankId(6993),
            name: "Cruiser II",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/6993.webp"),
        }),
        TankId(7009) => Some(Vehicle {
            tank_id: TankId(7009),
            name: "Type 57",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/7009.webp"),
        }),
        TankId(7025) => Some(Vehicle {
            tank_id: TankId(7025),
            name: "Vulcan",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/7025.webp"),
        }),
        TankId(7041) => Some(Vehicle {
            tank_id: TankId(7041),
            name: "Turbo",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/7041.webp"),
        }),
        TankId(7169) => Some(Vehicle {
            tank_id: TankId(7169),
            name: "IS-7",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/7169.webp"),
        }),
        TankId(7185) => Some(Vehicle {
            tank_id: TankId(7185),
            name: "VK 30.01 P",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/7185.webp"),
        }),
        TankId(7201) => Some(Vehicle {
            tank_id: TankId(7201),
            name: "Jackson",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7201.webp"),
        }),
        TankId(7217) => Some(Vehicle {
            tank_id: TankId(7217),
            name: "WZ-112-2",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/7217.webp"),
        }),
        TankId(7249) => Some(Vehicle {
            tank_id: TankId(7249),
            name: "FV4202",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/7249.webp"),
        }),
        TankId(7265) => Some(Vehicle {
            tank_id: TankId(7265),
            name: "Ferrum",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/7265.webp"),
        }),
        TankId(7281) => Some(Vehicle {
            tank_id: TankId(7281),
            name: "Lycan",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/7281.webp"),
        }),
        TankId(7297) => Some(Vehicle {
            tank_id: TankId(7297),
            name: "60TP Lewandowskiego",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/7297.webp"),
        }),
        TankId(7425) => Some(Vehicle {
            tank_id: TankId(7425),
            name: "ISU-152",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7425.webp"),
        }),
        TankId(7441) => Some(Vehicle {
            tank_id: TankId(7441),
            name: "VK 45.02 B",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/7441.webp"),
        }),
        TankId(7473) => Some(Vehicle {
            tank_id: TankId(7473),
            name: "T-34-2G FT",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7473.webp"),
        }),
        TankId(7505) => Some(Vehicle {
            tank_id: TankId(7505),
            name: "Cruiser IV",
            tier: 3,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/7505.webp"),
        }),
        TankId(7537) => Some(Vehicle {
            tank_id: TankId(7537),
            name: "Nightmare",
            tier: 5,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/7537.webp"),
        }),
        TankId(7553) => Some(Vehicle {
            tank_id: TankId(7553),
            name: "50TP Tyszkiewicza",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/7553.webp"),
        }),
        TankId(7697) => Some(Vehicle {
            tank_id: TankId(7697),
            name: "Ferdinand",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7697.webp"),
        }),
        TankId(7713) => Some(Vehicle {
            tank_id: TankId(7713),
            name: "T40",
            tier: 4,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/7713.webp"),
        }),
        TankId(7729) => Some(Vehicle {
            tank_id: TankId(7729),
            name: "WZ-131G FT",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7729.webp"),
        }),
        TankId(7745) => Some(Vehicle {
            tank_id: TankId(7745),
            name: "FT AC",
            tier: 2,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/7745.webp"),
        }),
        TankId(7761) => Some(Vehicle {
            tank_id: TankId(7761),
            name: "Cruiser III",
            tier: 2,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/7761.webp"),
        }),
        TankId(7793) => Some(Vehicle {
            tank_id: TankId(7793),
            name: "Annihilator",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/7793.webp"),
        }),
        TankId(7809) => Some(Vehicle {
            tank_id: TankId(7809),
            name: "53TP Markowskiego",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/7809.webp"),
        }),
        TankId(7937) => Some(Vehicle {
            tank_id: TankId(7937),
            name: "T-54",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/7937.webp"),
        }),
        TankId(7953) => Some(Vehicle {
            tank_id: TankId(7953),
            name: "Jagdtiger",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7953.webp"),
        }),
        TankId(7985) => Some(Vehicle {
            tank_id: TankId(7985),
            name: "WZ-111-1G FT",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/7985.webp"),
        }),
        TankId(8001) => Some(Vehicle {
            tank_id: TankId(8001),
            name: "Lorraine 40 t",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/8001.webp"),
        }),
        TankId(8017) => Some(Vehicle {
            tank_id: TankId(8017),
            name: "Valentine AT",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/8017.webp"),
        }),
        TankId(8049) => Some(Vehicle {
            tank_id: TankId(8049),
            name: "Spike",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/8049.webp"),
        }),
        TankId(8065) => Some(Vehicle {
            tank_id: TankId(8065),
            name: "40TP Habicha",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/8065.webp"),
        }),
        TankId(8193) => Some(Vehicle {
            tank_id: TankId(8193),
            name: "Obj. 704",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/8193.webp"),
        }),
        TankId(8209) => Some(Vehicle {
            tank_id: TankId(8209),
            name: "Pz. 38 nA",
            tier: 4,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/8209.webp"),
        }),
        TankId(8225) => Some(Vehicle {
            tank_id: TankId(8225),
            name: "T28",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/8225.webp"),
        }),
        TankId(8241) => Some(Vehicle {
            tank_id: TankId(8241),
            name: "WZ-111G FT",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/8241.webp"),
        }),
        TankId(8257) => Some(Vehicle {
            tank_id: TankId(8257),
            name: "UE 57",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/8257.webp"),
        }),
        TankId(8273) => Some(Vehicle {
            tank_id: TankId(8273),
            name: "UC 2-pdr",
            tier: 2,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/8273.webp"),
        }),
        TankId(8305) => Some(Vehicle {
            tank_id: TankId(8305),
            name: "Titan H-Nd",
            tier: 7,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/8305.webp"),
        }),
        TankId(8321) => Some(Vehicle {
            tank_id: TankId(8321),
            name: "45TP Habicha",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/8321.webp"),
        }),
        TankId(8465) => Some(Vehicle {
            tank_id: TankId(8465),
            name: "Panther II",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/8465.webp"),
        }),
        TankId(8497) => Some(Vehicle {
            tank_id: TankId(8497),
            name: "WZ-111 5A",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/8497.webp"),
        }),
        TankId(8513) => Some(Vehicle {
            tank_id: TankId(8513),
            name: "AMX 30 B",
            tier: 10,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/8513.webp"),
        }),
        TankId(8529) => Some(Vehicle {
            tank_id: TankId(8529),
            name: "AT 15",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/8529.webp"),
        }),
        TankId(8561) => Some(Vehicle {
            tank_id: TankId(8561),
            name: "Titan T24 57",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/8561.webp"),
        }),
        TankId(8577) => Some(Vehicle {
            tank_id: TankId(8577),
            name: "Lansen C",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/8577.webp"),
        }),
        TankId(8737) => Some(Vehicle {
            tank_id: TankId(8737),
            name: "T95",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/8737.webp"),
        }),
        TankId(8753) => Some(Vehicle {
            tank_id: TankId(8753),
            name: "M41D",
            tier: 7,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/8753.webp"),
        }),
        TankId(8785) => Some(Vehicle {
            tank_id: TankId(8785),
            name: "AT 2",
            tier: 5,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/8785.webp"),
        }),
        TankId(8817) => Some(Vehicle {
            tank_id: TankId(8817),
            name: "Titan Mk. I",
            tier: 5,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/8817.webp"),
        }),
        TankId(8833) => Some(Vehicle {
            tank_id: TankId(8833),
            name: "Spark",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/8833.webp"),
        }),
        TankId(8961) => Some(Vehicle {
            tank_id: TankId(8961),
            name: "KV-13",
            tier: 7,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/8961.webp"),
        }),
        TankId(8993) => Some(Vehicle {
            tank_id: TankId(8993),
            name: "M46 Patton",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/8993.webp"),
        }),
        TankId(9009) => Some(Vehicle {
            tank_id: TankId(9009),
            name: "Ox",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/9009.webp"),
        }),
        TankId(9041) => Some(Vehicle {
            tank_id: TankId(9041),
            name: "Alecto",
            tier: 4,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/9041.webp"),
        }),
        TankId(9073) => Some(Vehicle {
            tank_id: TankId(9073),
            name: "Titan-54d",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/9073.webp"),
        }),
        TankId(9089) => Some(Vehicle {
            tank_id: TankId(9089),
            name: "Škoda T 56",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/9089.webp"),
        }),
        TankId(9217) => Some(Vehicle {
            tank_id: TankId(9217),
            name: "IS-6",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/9217.webp"),
        }),
        TankId(9249) => Some(Vehicle {
            tank_id: TankId(9249),
            name: "T25 AT",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/9249.webp"),
        }),
        TankId(9265) => Some(Vehicle {
            tank_id: TankId(9265),
            name: "Type 58",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/9265.webp"),
        }),
        TankId(9297) => Some(Vehicle {
            tank_id: TankId(9297),
            name: "FV215b 183",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/9297.webp"),
        }),
        TankId(9329) => Some(Vehicle {
            tank_id: TankId(9329),
            name: "Titan-150",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/9329.webp"),
        }),
        TankId(9345) => Some(Vehicle {
            tank_id: TankId(9345),
            name: "Svear",
            tier: 7,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/9345.webp"),
        }),
        TankId(9489) => Some(Vehicle {
            tank_id: TankId(9489),
            name: "E 100",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/9489.webp"),
        }),
        TankId(9505) => Some(Vehicle {
            tank_id: TankId(9505),
            name: "M103",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/9505.webp"),
        }),
        TankId(9521) => Some(Vehicle {
            tank_id: TankId(9521),
            name: "WZ-122 TM",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/9521.webp"),
        }),
        TankId(9553) => Some(Vehicle {
            tank_id: TankId(9553),
            name: "AT 8",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/9553.webp"),
        }),
        TankId(9601) => Some(Vehicle {
            tank_id: TankId(9601),
            name: "CS-52 LIS",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/9601.webp"),
        }),
        TankId(9745) => Some(Vehicle {
            tank_id: TankId(9745),
            name: "E 75",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/9745.webp"),
        }),
        TankId(9761) => Some(Vehicle {
            tank_id: TankId(9761),
            name: "Chaffee",
            tier: 6,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/9761.webp"),
        }),
        TankId(9777) => Some(Vehicle {
            tank_id: TankId(9777),
            name: "WZ-114",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/9777.webp"),
        }),
        TankId(9793) => Some(Vehicle {
            tank_id: TankId(9793),
            name: "SAu 40",
            tier: 4,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/9793.webp"),
        }),
        TankId(9809) => Some(Vehicle {
            tank_id: TankId(9809),
            name: "Churchill GC",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/9809.webp"),
        }),
        TankId(9841) => Some(Vehicle {
            tank_id: TankId(9841),
            name: "Rover",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/9841.webp"),
        }),
        TankId(9857) => Some(Vehicle {
            tank_id: TankId(9857),
            name: "Škoda T 45",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/9857.webp"),
        }),
        TankId(9985) => Some(Vehicle {
            tank_id: TankId(9985),
            name: "SU-101",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/9985.webp"),
        }),
        TankId(10001) => Some(Vehicle {
            tank_id: TankId(10001),
            name: "VK 28.01",
            tier: 6,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/10001.webp"),
        }),
        TankId(10017) => Some(Vehicle {
            tank_id: TankId(10017),
            name: "M4A3E2",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/10017.webp"),
        }),
        TankId(10033) => Some(Vehicle {
            tank_id: TankId(10033),
            name: "WZ-132A",
            tier: 9,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/10033.webp"),
        }),
        TankId(10049) => Some(Vehicle {
            tank_id: TankId(10049),
            name: "S35 CA",
            tier: 5,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10049.webp"),
        }),
        TankId(10065) => Some(Vehicle {
            tank_id: TankId(10065),
            name: "AT 7",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10065.webp"),
        }),
        TankId(10097) => Some(Vehicle {
            tank_id: TankId(10097),
            name: "Medjay",
            tier: 5,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/10097.webp"),
        }),
        TankId(10113) => Some(Vehicle {
            tank_id: TankId(10113),
            name: "Carro 45t",
            tier: 10,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/10113.webp"),
        }),
        TankId(10241) => Some(Vehicle {
            tank_id: TankId(10241),
            name: "SU-100M1",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10241.webp"),
        }),
        TankId(10257) => Some(Vehicle {
            tank_id: TankId(10257),
            name: "E 50",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/10257.webp"),
        }),
        TankId(10273) => Some(Vehicle {
            tank_id: TankId(10273),
            name: "M8A1",
            tier: 4,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10273.webp"),
        }),
        TankId(10289) => Some(Vehicle {
            tank_id: TankId(10289),
            name: "WZ-132-1",
            tier: 10,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/10289.webp"),
        }),
        TankId(10353) => Some(Vehicle {
            tank_id: TankId(10353),
            name: "Pharaoh",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/10353.webp"),
        }),
        TankId(10369) => Some(Vehicle {
            tank_id: TankId(10369),
            name: "Minotauro",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10369.webp"),
        }),
        TankId(10497) => Some(Vehicle {
            tank_id: TankId(10497),
            name: "KV-2",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/10497.webp"),
        }),
        TankId(10513) => Some(Vehicle {
            tank_id: TankId(10513),
            name: "VK 45.02 A",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/10513.webp"),
        }),
        TankId(10529) => Some(Vehicle {
            tank_id: TankId(10529),
            name: "T67",
            tier: 5,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/10529.webp"),
        }),
        TankId(10545) => Some(Vehicle {
            tank_id: TankId(10545),
            name: "Wind",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/10545.webp"),
        }),
        TankId(10609) => Some(Vehicle {
            tank_id: TankId(10609),
            name: "Magnate",
            tier: 7,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/10609.webp"),
        }),
        TankId(10625) => Some(Vehicle {
            tank_id: TankId(10625),
            name: "CC 1 Mk. 2",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10625.webp"),
        }),
        TankId(10753) => Some(Vehicle {
            tank_id: TankId(10753),
            name: "ST-I",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/10753.webp"),
        }),
        TankId(10769) => Some(Vehicle {
            tank_id: TankId(10769),
            name: "Tiger (P)",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/10769.webp"),
        }),
        TankId(10785) => Some(Vehicle {
            tank_id: TankId(10785),
            name: "T110E5",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/10785.webp"),
        }),
        TankId(10801) => Some(Vehicle {
            tank_id: TankId(10801),
            name: "Panlong",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/10801.webp"),
        }),
        TankId(10817) => Some(Vehicle {
            tank_id: TankId(10817),
            name: "AMX AC 46",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10817.webp"),
        }),
        TankId(10865) => Some(Vehicle {
            tank_id: TankId(10865),
            name: "Fixer",
            tier: 8,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/10865.webp"),
        }),
        TankId(10881) => Some(Vehicle {
            tank_id: TankId(10881),
            name: "SMV CC-64",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/10881.webp"),
        }),
        TankId(11009) => Some(Vehicle {
            tank_id: TankId(11009),
            name: "KV-4",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/11009.webp"),
        }),
        TankId(11025) => Some(Vehicle {
            tank_id: TankId(11025),
            name: "St. Emil",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11025.webp"),
        }),
        TankId(11041) => Some(Vehicle {
            tank_id: TankId(11041),
            name: "T25/2",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11041.webp"),
        }),
        TankId(11057) => Some(Vehicle {
            tank_id: TankId(11057),
            name: "114 SP2",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/11057.webp"),
        }),
        TankId(11073) => Some(Vehicle {
            tank_id: TankId(11073),
            name: "Foch",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11073.webp"),
        }),
        TankId(11121) => Some(Vehicle {
            tank_id: TankId(11121),
            name: "Regressor",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/11121.webp"),
        }),
        TankId(11137) => Some(Vehicle {
            tank_id: TankId(11137),
            name: "SMV CC-56",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11137.webp"),
        }),
        TankId(11265) => Some(Vehicle {
            tank_id: TankId(11265),
            name: "T-150",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/11265.webp"),
        }),
        TankId(11281) => Some(Vehicle {
            tank_id: TankId(11281),
            name: "Kpz 70",
            tier: 9,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/11281.webp"),
        }),
        TankId(11297) => Some(Vehicle {
            tank_id: TankId(11297),
            name: "T28 Prot.",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11297.webp"),
        }),
        TankId(11393) => Some(Vehicle {
            tank_id: TankId(11393),
            name: "Bassotto",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11393.webp"),
        }),
        TankId(11521) => Some(Vehicle {
            tank_id: TankId(11521),
            name: "IS-8",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/11521.webp"),
        }),
        TankId(11537) => Some(Vehicle {
            tank_id: TankId(11537),
            name: "JPanther II",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11537.webp"),
        }),
        TankId(11553) => Some(Vehicle {
            tank_id: TankId(11553),
            name: "Hellcat",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11553.webp"),
        }),
        TankId(11585) => Some(Vehicle {
            tank_id: TankId(11585),
            name: "ARL V39",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11585.webp"),
        }),
        TankId(11649) => Some(Vehicle {
            tank_id: TankId(11649),
            name: "Semovente M41",
            tier: 5,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11649.webp"),
        }),
        TankId(11777) => Some(Vehicle {
            tank_id: TankId(11777),
            name: "KV-1",
            tier: 5,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/11777.webp"),
        }),
        TankId(11793) => Some(Vehicle {
            tank_id: TankId(11793),
            name: "Nashorn",
            tier: 6,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/11793.webp"),
        }),
        TankId(11809) => Some(Vehicle {
            tank_id: TankId(11809),
            name: "T23E3",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/11809.webp"),
        }),
        TankId(11905) => Some(Vehicle {
            tank_id: TankId(11905),
            name: "Shadowhunter",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/11905.webp"),
        }),
        TankId(12033) => Some(Vehicle {
            tank_id: TankId(12033),
            name: "SU-122-54",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/12033.webp"),
        }),
        TankId(12049) => Some(Vehicle {
            tank_id: TankId(12049),
            name: "Jg.Pz. E 100",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/12049.webp"),
        }),
        TankId(12065) => Some(Vehicle {
            tank_id: TankId(12065),
            name: "T95E2",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/12065.webp"),
        }),
        TankId(12097) => Some(Vehicle {
            tank_id: TankId(12097),
            name: "AMX AC 48",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/12097.webp"),
        }),
        TankId(12161) => Some(Vehicle {
            tank_id: TankId(12161),
            name: "Strv K",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/12161.webp"),
        }),
        TankId(12305) => Some(Vehicle {
            tank_id: TankId(12305),
            name: "E 50 M",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/12305.webp"),
        }),
        TankId(12321) => Some(Vehicle {
            tank_id: TankId(12321),
            name: "T6E1 Grizzly",
            tier: 4,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/12321.webp"),
        }),
        TankId(12417) => Some(Vehicle {
            tank_id: TankId(12417),
            name: "Bisonte C45",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/12417.webp"),
        }),
        TankId(12545) => Some(Vehicle {
            tank_id: TankId(12545),
            name: "K-91",
            tier: 9,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/12545.webp"),
        }),
        TankId(12673) => Some(Vehicle {
            tank_id: TankId(12673),
            name: "Tornvagn",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/12673.webp"),
        }),
        TankId(12929) => Some(Vehicle {
            tank_id: TankId(12929),
            name: "TNH T Vz. 51",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/12929.webp"),
        }),
        TankId(13073) => Some(Vehicle {
            tank_id: TankId(13073),
            name: "Pz. II G",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/13073.webp"),
        }),
        TankId(13089) => Some(Vehicle {
            tank_id: TankId(13089),
            name: "T110E4",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/13089.webp"),
        }),
        TankId(13185) => Some(Vehicle {
            tank_id: TankId(13185),
            name: "Vz. 55",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/13185.webp"),
        }),
        TankId(13329) => Some(Vehicle {
            tank_id: TankId(13329),
            name: "D.W. 2",
            tier: 4,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/13329.webp"),
        }),
        TankId(13345) => Some(Vehicle {
            tank_id: TankId(13345),
            name: "T26E4",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/13345.webp"),
        }),
        TankId(13441) => Some(Vehicle {
            tank_id: TankId(13441),
            name: "Aeonix",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/13441.webp"),
        }),
        TankId(13569) => Some(Vehicle {
            tank_id: TankId(13569),
            name: "Obj. 268",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/13569.webp"),
        }),
        TankId(13697) => Some(Vehicle {
            tank_id: TankId(13697),
            name: "TNH 105/1000",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/13697.webp"),
        }),
        TankId(13825) => Some(Vehicle {
            tank_id: TankId(13825),
            name: "T-62A",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/13825.webp"),
        }),
        TankId(13841) => Some(Vehicle {
            tank_id: TankId(13841),
            name: "Indien-Pz.",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/13841.webp"),
        }),
        TankId(13857) => Some(Vehicle {
            tank_id: TankId(13857),
            name: "T110E3",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/13857.webp"),
        }),
        TankId(13889) => Some(Vehicle {
            tank_id: TankId(13889),
            name: "Foch 155",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/13889.webp"),
        }),
        TankId(13953) => Some(Vehicle {
            tank_id: TankId(13953),
            name: "Vz. 44-1",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/13953.webp"),
        }),
        TankId(14097) => Some(Vehicle {
            tank_id: TankId(14097),
            name: "VK 30.01 D",
            tier: 6,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/14097.webp"),
        }),
        TankId(14113) => Some(Vehicle {
            tank_id: TankId(14113),
            name: "M48 Patton",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/14113.webp"),
        }),
        TankId(14145) => Some(Vehicle {
            tank_id: TankId(14145),
            name: "AMX ELC bis",
            tier: 5,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/14145.webp"),
        }),
        TankId(14209) => Some(Vehicle {
            tank_id: TankId(14209),
            name: "Škoda P-JS",
            tier: 6,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/14209.webp"),
        }),
        TankId(14337) => Some(Vehicle {
            tank_id: TankId(14337),
            name: "Obj. 263",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/14337.webp"),
        }),
        TankId(14609) => Some(Vehicle {
            tank_id: TankId(14609),
            name: "Leopard 1",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/14609.webp"),
        }),
        TankId(14625) => Some(Vehicle {
            tank_id: TankId(14625),
            name: "T69",
            tier: 8,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/14625.webp"),
        }),
        TankId(14721) => Some(Vehicle {
            tank_id: TankId(14721),
            name: "Strv 81",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/14721.webp"),
        }),
        TankId(14865) => Some(Vehicle {
            tank_id: TankId(14865),
            name: "Leopard PT A",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/14865.webp"),
        }),
        TankId(14881) => Some(Vehicle {
            tank_id: TankId(14881),
            name: "T57 Heavy",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/14881.webp"),
        }),
        TankId(14977) => Some(Vehicle {
            tank_id: TankId(14977),
            name: "CS-63",
            tier: 10,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/14977.webp"),
        }),
        TankId(15137) => Some(Vehicle {
            tank_id: TankId(15137),
            name: "T21",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/15137.webp"),
        }),
        TankId(15393) => Some(Vehicle {
            tank_id: TankId(15393),
            name: "T54E1",
            tier: 9,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/15393.webp"),
        }),
        TankId(15441) => Some(Vehicle {
            tank_id: TankId(15441),
            name: "Chieftain",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/15441.webp"),
        }),
        TankId(15617) => Some(Vehicle {
            tank_id: TankId(15617),
            name: "Obj. 907",
            tier: 10,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/15617.webp"),
        }),
        TankId(15649) => Some(Vehicle {
            tank_id: TankId(15649),
            name: "T71",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/15649.webp"),
        }),
        TankId(15697) => Some(Vehicle {
            tank_id: TankId(15697),
            name: "Chieftain Mk. 6",
            tier: 10,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/15697.webp"),
        }),
        TankId(15889) => Some(Vehicle {
            tank_id: TankId(15889),
            name: "VK 30.02 M",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/15889.webp"),
        }),
        TankId(15905) => Some(Vehicle {
            tank_id: TankId(15905),
            name: "M60",
            tier: 10,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/15905.webp"),
        }),
        TankId(15937) => Some(Vehicle {
            tank_id: TankId(15937),
            name: "R35",
            tier: 1,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/15937.webp"),
        }),
        TankId(15953) => Some(Vehicle {
            tank_id: TankId(15953),
            name: "FV201 (A45)",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/15953.webp"),
        }),
        TankId(16145) => Some(Vehicle {
            tank_id: TankId(16145),
            name: "Pz.Sfl. IVc",
            tier: 5,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/16145.webp"),
        }),
        TankId(16193) => Some(Vehicle {
            tank_id: TankId(16193),
            name: "M4A1 Rev.",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/16193.webp"),
        }),
        TankId(16209) => Some(Vehicle {
            tank_id: TankId(16209),
            name: "Cruiser I",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/16209.webp"),
        }),
        TankId(16257) => Some(Vehicle {
            tank_id: TankId(16257),
            name: "Outcast",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/16257.webp"),
        }),
        TankId(16401) => Some(Vehicle {
            tank_id: TankId(16401),
            name: "WT auf Pz. IV",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/16401.webp"),
        }),
        TankId(16449) => Some(Vehicle {
            tank_id: TankId(16449),
            name: "AMX 13 57",
            tier: 7,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/16449.webp"),
        }),
        TankId(16465) => Some(Vehicle {
            tank_id: TankId(16465),
            name: "Cruiser I",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/16465.webp"),
        }),
        TankId(16641) => Some(Vehicle {
            tank_id: TankId(16641),
            name: "MT-25",
            tier: 6,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/16641.webp"),
        }),
        TankId(16657) => Some(Vehicle {
            tank_id: TankId(16657),
            name: "Rhm.-B. WT",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/16657.webp"),
        }),
        TankId(16673) => Some(Vehicle {
            tank_id: TankId(16673),
            name: "T37",
            tier: 6,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/16673.webp"),
        }),
        TankId(16705) => Some(Vehicle {
            tank_id: TankId(16705),
            name: "AMX M4 49",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/16705.webp"),
        }),
        TankId(16897) => Some(Vehicle {
            tank_id: TankId(16897),
            name: "Obj. 140",
            tier: 10,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/16897.webp"),
        }),
        TankId(17169) => Some(Vehicle {
            tank_id: TankId(17169),
            name: "Pz. IV A",
            tier: 3,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/17169.webp"),
        }),
        TankId(17217) => Some(Vehicle {
            tank_id: TankId(17217),
            name: "Eraser BP44",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/17217.webp"),
        }),
        TankId(17233) => Some(Vehicle {
            tank_id: TankId(17233),
            name: "Conway",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/17233.webp"),
        }),
        TankId(17425) => Some(Vehicle {
            tank_id: TankId(17425),
            name: "Pz. IV D",
            tier: 4,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/17425.webp"),
        }),
        TankId(17473) => Some(Vehicle {
            tank_id: TankId(17473),
            name: "AMX Defender",
            tier: 8,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/17473.webp"),
        }),
        TankId(17489) => Some(Vehicle {
            tank_id: TankId(17489),
            name: "Charioteer",
            tier: 8,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/17489.webp"),
        }),
        TankId(17729) => Some(Vehicle {
            tank_id: TankId(17729),
            name: "Somua SM",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/17729.webp"),
        }),
        TankId(17745) => Some(Vehicle {
            tank_id: TankId(17745),
            name: "FV217 Badger",
            tier: 10,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/17745.webp"),
        }),
        TankId(17953) => Some(Vehicle {
            tank_id: TankId(17953),
            name: "M41 Bulldog",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/17953.webp"),
        }),
        TankId(17985) => Some(Vehicle {
            tank_id: TankId(17985),
            name: "Bretagne Panther",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/17985.webp"),
        }),
        TankId(18001) => Some(Vehicle {
            tank_id: TankId(18001),
            name: "FV4005",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/18001.webp"),
        }),
        TankId(18177) => Some(Vehicle {
            tank_id: TankId(18177),
            name: "T-54 ltwt.",
            tier: 8,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18177.webp"),
        }),
        TankId(18209) => Some(Vehicle {
            tank_id: TankId(18209),
            name: "T49",
            tier: 8,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18209.webp"),
        }),
        TankId(18241) => Some(Vehicle {
            tank_id: TankId(18241),
            name: "B-C Bourrasque",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/18241.webp"),
        }),
        TankId(18257) => Some(Vehicle {
            tank_id: TankId(18257),
            name: "Challenger",
            tier: 7,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/18257.webp"),
        }),
        TankId(18433) => Some(Vehicle {
            tank_id: TankId(18433),
            name: "LTTB",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18433.webp"),
        }),
        TankId(18449) => Some(Vehicle {
            tank_id: TankId(18449),
            name: "Ru 251",
            tier: 8,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18449.webp"),
        }),
        TankId(18465) => Some(Vehicle {
            tank_id: TankId(18465),
            name: "M2 Light",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18465.webp"),
        }),
        TankId(18497) => Some(Vehicle {
            tank_id: TankId(18497),
            name: "Lorraine Fearless",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/18497.webp"),
        }),
        TankId(18513) => Some(Vehicle {
            tank_id: TankId(18513),
            name: "Chimera",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/18513.webp"),
        }),
        TankId(18689) => Some(Vehicle {
            tank_id: TankId(18689),
            name: "T-70/57",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/18689.webp"),
        }),
        TankId(18721) => Some(Vehicle {
            tank_id: TankId(18721),
            name: "M2 Light",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18721.webp"),
        }),
        TankId(18753) => Some(Vehicle {
            tank_id: TankId(18753),
            name: "AMX CDA 105",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/18753.webp"),
        }),
        TankId(18769) => Some(Vehicle {
            tank_id: TankId(18769),
            name: "Action X",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/18769.webp"),
        }),
        TankId(18945) => Some(Vehicle {
            tank_id: TankId(18945),
            name: "ISU-130",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/18945.webp"),
        }),
        TankId(18961) => Some(Vehicle {
            tank_id: TankId(18961),
            name: "SP I C",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/18961.webp"),
        }),
        TankId(18977) => Some(Vehicle {
            tank_id: TankId(18977),
            name: "T95E6",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/18977.webp"),
        }),
        TankId(19009) => Some(Vehicle {
            tank_id: TankId(19009),
            name: "AMXmas",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/19009.webp"),
        }),
        TankId(19025) => Some(Vehicle {
            tank_id: TankId(19025),
            name: "Defender Mk. 1",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/19025.webp"),
        }),
        TankId(19201) => Some(Vehicle {
            tank_id: TankId(19201),
            name: "T-26",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/19201.webp"),
        }),
        TankId(19217) => Some(Vehicle {
            tank_id: TankId(19217),
            name: "Grille 15",
            tier: 10,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/19217.webp"),
        }),
        TankId(19233) => Some(Vehicle {
            tank_id: TankId(19233),
            name: "Chrysler K",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/19233.webp"),
        }),
        TankId(19265) => Some(Vehicle {
            tank_id: TankId(19265),
            name: "Charles",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/19265.webp"),
        }),
        TankId(19281) => Some(Vehicle {
            tank_id: TankId(19281),
            name: "Super Conqueror",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/19281.webp"),
        }),
        TankId(19457) => Some(Vehicle {
            tank_id: TankId(19457),
            name: "T-26",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/19457.webp"),
        }),
        TankId(19473) => Some(Vehicle {
            tank_id: TankId(19473),
            name: "Krupp-38(D)",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/19473.webp"),
        }),
        TankId(19489) => Some(Vehicle {
            tank_id: TankId(19489),
            name: "T28 Defender",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/19489.webp"),
        }),
        TankId(19521) => Some(Vehicle {
            tank_id: TankId(19521),
            name: "#france_vehicles:Bat-Chat-25CL",
            tier: 9,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/19521.webp"),
        }),
        TankId(19537) => Some(Vehicle {
            tank_id: TankId(19537),
            name: "Vickers Light",
            tier: 10,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/19537.webp"),
        }),
        TankId(19713) => Some(Vehicle {
            tank_id: TankId(19713),
            name: "Loza's Sherman",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/19713.webp"),
        }),
        TankId(19729) => Some(Vehicle {
            tank_id: TankId(19729),
            name: "Tiger 131",
            tier: 6,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/19729.webp"),
        }),
        TankId(19745) => Some(Vehicle {
            tank_id: TankId(19745),
            name: "T26E5",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/19745.webp"),
        }),
        TankId(19777) => Some(Vehicle {
            tank_id: TankId(19777),
            name: "A.P. AMX 30",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/19777.webp"),
        }),
        TankId(19793) => Some(Vehicle {
            tank_id: TankId(19793),
            name: "Vickers CR",
            tier: 9,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/19793.webp"),
        }),
        TankId(19969) => Some(Vehicle {
            tank_id: TankId(19969),
            name: "T-22 medium",
            tier: 10,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/19969.webp"),
        }),
        TankId(19985) => Some(Vehicle {
            tank_id: TankId(19985),
            name: "Skorpion G",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/19985.webp"),
        }),
        TankId(20001) => Some(Vehicle {
            tank_id: TankId(20001),
            name: "T92E1",
            tier: 9,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/20001.webp"),
        }),
        TankId(20033) => Some(Vehicle {
            tank_id: TankId(20033),
            name: "Char Futur 4",
            tier: 9,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/20033.webp"),
        }),
        TankId(20049) => Some(Vehicle {
            tank_id: TankId(20049),
            name: "FV301",
            tier: 8,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/20049.webp"),
        }),
        TankId(20241) => Some(Vehicle {
            tank_id: TankId(20241),
            name: "Pz. II",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/20241.webp"),
        }),
        TankId(20257) => Some(Vehicle {
            tank_id: TankId(20257),
            name: "Sheridan",
            tier: 10,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/20257.webp"),
        }),
        TankId(20289) => Some(Vehicle {
            tank_id: TankId(20289),
            name: "Pirate",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/20289.webp"),
        }),
        TankId(20305) => Some(Vehicle {
            tank_id: TankId(20305),
            name: "Centurion Mk. 5/1 RAAC",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/20305.webp"),
        }),
        TankId(20481) => Some(Vehicle {
            tank_id: TankId(20481),
            name: "Obj. 252U",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/20481.webp"),
        }),
        TankId(20497) => Some(Vehicle {
            tank_id: TankId(20497),
            name: "VK 100.01 (P)",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/20497.webp"),
        }),
        TankId(20513) => Some(Vehicle {
            tank_id: TankId(20513),
            name: "T54E2",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/20513.webp"),
        }),
        TankId(20545) => Some(Vehicle {
            tank_id: TankId(20545),
            name: "Renault G1",
            tier: 5,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/20545.webp"),
        }),
        TankId(20561) => Some(Vehicle {
            tank_id: TankId(20561),
            name: "Turtle Mk. I",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/20561.webp"),
        }),
        TankId(20737) => Some(Vehicle {
            tank_id: TankId(20737),
            name: "SU-130PM",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/20737.webp"),
        }),
        TankId(20753) => Some(Vehicle {
            tank_id: TankId(20753),
            name: "Mäuschen",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/20753.webp"),
        }),
        TankId(20769) => Some(Vehicle {
            tank_id: TankId(20769),
            name: "T25 Pilot 1",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/20769.webp"),
        }),
        TankId(20817) => Some(Vehicle {
            tank_id: TankId(20817),
            name: "Explorer",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/20817.webp"),
        }),
        TankId(20993) => Some(Vehicle {
            tank_id: TankId(20993),
            name: "T-2020",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/20993.webp"),
        }),
        TankId(21009) => Some(Vehicle {
            tank_id: TankId(21009),
            name: "Panzer 58",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/21009.webp"),
        }),
        TankId(21025) => Some(Vehicle {
            tank_id: TankId(21025),
            name: "T26E3 Eagle 7",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/21025.webp"),
        }),
        TankId(21073) => Some(Vehicle {
            tank_id: TankId(21073),
            name: "Dreadnought",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/21073.webp"),
        }),
        TankId(21249) => Some(Vehicle {
            tank_id: TankId(21249),
            name: "Thunder",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/21249.webp"),
        }),
        TankId(21265) => Some(Vehicle {
            tank_id: TankId(21265),
            name: "VK 168.01 (P)",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/21265.webp"),
        }),
        TankId(21281) => Some(Vehicle {
            tank_id: TankId(21281),
            name: "Rudolph",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/21281.webp"),
        }),
        TankId(21329) => Some(Vehicle {
            tank_id: TankId(21329),
            name: "GSOR 1008",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/21329.webp"),
        }),
        TankId(21505) => Some(Vehicle {
            tank_id: TankId(21505),
            name: "T-44-85",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/21505.webp"),
        }),
        TankId(21521) => Some(Vehicle {
            tank_id: TankId(21521),
            name: "E 75 TS",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/21521.webp"),
        }),
        TankId(21537) => Some(Vehicle {
            tank_id: TankId(21537),
            name: "Sherman Easy 8",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/21537.webp"),
        }),
        TankId(21585) => Some(Vehicle {
            tank_id: TankId(21585),
            name: "Blasteroid",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/21585.webp"),
        }),
        TankId(21761) => Some(Vehicle {
            tank_id: TankId(21761),
            name: "STG",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/21761.webp"),
        }),
        TankId(21777) => Some(Vehicle {
            tank_id: TankId(21777),
            name: "VK 90.01 (P)",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/21777.webp"),
        }),
        TankId(21793) => Some(Vehicle {
            tank_id: TankId(21793),
            name: "Sheridan Missile",
            tier: 10,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/21793.webp"),
        }),
        TankId(21841) => Some(Vehicle {
            tank_id: TankId(21841),
            name: "Caliban",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/21841.webp"),
        }),
        TankId(22017) => Some(Vehicle {
            tank_id: TankId(22017),
            name: "T-34-85",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/22017.webp"),
        }),
        TankId(22033) => Some(Vehicle {
            tank_id: TankId(22033),
            name: "Agent",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/22033.webp"),
        }),
        TankId(22049) => Some(Vehicle {
            tank_id: TankId(22049),
            name: "Magnus",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/22049.webp"),
        }),
        TankId(22097) => Some(Vehicle {
            tank_id: TankId(22097),
            name: "Churchill VIII",
            tier: 6,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/22097.webp"),
        }),
        TankId(22273) => Some(Vehicle {
            tank_id: TankId(22273),
            name: "Obj. 260",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/22273.webp"),
        }),
        TankId(22289) => Some(Vehicle {
            tank_id: TankId(22289),
            name: "Tiger I",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/22289.webp"),
        }),
        TankId(22305) => Some(Vehicle {
            tank_id: TankId(22305),
            name: "AE Phase I",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/22305.webp"),
        }),
        TankId(22353) => Some(Vehicle {
            tank_id: TankId(22353),
            name: "Churchill W",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/22353.webp"),
        }),
        TankId(22529) => Some(Vehicle {
            tank_id: TankId(22529),
            name: "LT-432",
            tier: 8,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/22529.webp"),
        }),
        TankId(22545) => Some(Vehicle {
            tank_id: TankId(22545),
            name: "Kanonenjagdpanzer",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/22545.webp"),
        }),
        TankId(22561) => Some(Vehicle {
            tank_id: TankId(22561),
            name: "TS-5",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/22561.webp"),
        }),
        TankId(22609) => Some(Vehicle {
            tank_id: TankId(22609),
            name: "Caernarvon Defender",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/22609.webp"),
        }),
        TankId(22785) => Some(Vehicle {
            tank_id: TankId(22785),
            name: "Triumphant",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/22785.webp"),
        }),
        TankId(22801) => Some(Vehicle {
            tank_id: TankId(22801),
            name: "Icebreaker",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/22801.webp"),
        }),
        TankId(22817) => Some(Vehicle {
            tank_id: TankId(22817),
            name: "M-VI-Yoh",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/22817.webp"),
        }),
        TankId(22865) => Some(Vehicle {
            tank_id: TankId(22865),
            name: "Charlemagne",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/22865.webp"),
        }),
        TankId(23041) => Some(Vehicle {
            tank_id: TankId(23041),
            name: "T-34 shielded",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/23041.webp"),
        }),
        TankId(23057) => Some(Vehicle {
            tank_id: TankId(23057),
            name: "Kunze Panzer",
            tier: 7,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/23057.webp"),
        }),
        TankId(23073) => Some(Vehicle {
            tank_id: TankId(23073),
            name: "M-V-Yoh",
            tier: 9,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/23073.webp"),
        }),
        TankId(23121) => Some(Vehicle {
            tank_id: TankId(23121),
            name: "Cobra",
            tier: 9,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/23121.webp"),
        }),
        TankId(23297) => Some(Vehicle {
            tank_id: TankId(23297),
            name: "Object 244",
            tier: 6,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/23297.webp"),
        }),
        TankId(23313) => Some(Vehicle {
            tank_id: TankId(23313),
            name: "Kpz 50 t",
            tier: 10,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/23313.webp"),
        }),
        TankId(23329) => Some(Vehicle {
            tank_id: TankId(23329),
            name: "M-III-Yoh",
            tier: 8,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/23329.webp"),
        }),
        TankId(23553) => Some(Vehicle {
            tank_id: TankId(23553),
            name: "MS-1",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/23553.webp"),
        }),
        TankId(23569) => Some(Vehicle {
            tank_id: TankId(23569),
            name: "Pz. IV Gargoyle",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/23569.webp"),
        }),
        TankId(23585) => Some(Vehicle {
            tank_id: TankId(23585),
            name: "M-VII-Yoh",
            tier: 7,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/23585.webp"),
        }),
        TankId(23809) => Some(Vehicle {
            tank_id: TankId(23809),
            name: "Object 84",
            tier: 9,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/23809.webp"),
        }),
        TankId(23825) => Some(Vehicle {
            tank_id: TankId(23825),
            name: "Steyr WT",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/23825.webp"),
        }),
        TankId(23841) => Some(Vehicle {
            tank_id: TankId(23841),
            name: "Super Hellcat",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/23841.webp"),
        }),
        TankId(24065) => Some(Vehicle {
            tank_id: TankId(24065),
            name: "LTG",
            tier: 7,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/24065.webp"),
        }),
        TankId(24081) => Some(Vehicle {
            tank_id: TankId(24081),
            name: "U-Panzer",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/24081.webp"),
        }),
        TankId(24097) => Some(Vehicle {
            tank_id: TankId(24097),
            name: "BLTZ9000",
            tier: 6,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/24097.webp"),
        }),
        TankId(24321) => Some(Vehicle {
            tank_id: TankId(24321),
            name: "T-100 LT",
            tier: 10,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/24321.webp"),
        }),
        TankId(24337) => Some(Vehicle {
            tank_id: TankId(24337),
            name: "M48A2 Räumpanzer",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/24337.webp"),
        }),
        TankId(24577) => Some(Vehicle {
            tank_id: TankId(24577),
            name: "Object 268/4",
            tier: 10,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/24577.webp"),
        }),
        TankId(24593) => Some(Vehicle {
            tank_id: TankId(24593),
            name: "Keiler",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/24593.webp"),
        }),
        TankId(24609) => Some(Vehicle {
            tank_id: TankId(24609),
            name: "Concept 1B",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/24609.webp"),
        }),
        TankId(24849) => Some(Vehicle {
            tank_id: TankId(24849),
            name: "Kryos",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/24849.webp"),
        }),
        TankId(24865) => Some(Vehicle {
            tank_id: TankId(24865),
            name: "Scepter",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/24865.webp"),
        }),
        TankId(25089) => Some(Vehicle {
            tank_id: TankId(25089),
            name: "Object 752",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/25089.webp"),
        }),
        TankId(25105) => Some(Vehicle {
            tank_id: TankId(25105),
            name: "Barkhan",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/25105.webp"),
        }),
        TankId(25345) => Some(Vehicle {
            tank_id: TankId(25345),
            name: "Object 274a",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/25345.webp"),
        }),
        TankId(25361) => Some(Vehicle {
            tank_id: TankId(25361),
            name: "Waffenträger Ritter",
            tier: 9,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/25361.webp"),
        }),
        TankId(25377) => Some(Vehicle {
            tank_id: TankId(25377),
            name: "T77",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/25377.webp"),
        }),
        TankId(25601) => Some(Vehicle {
            tank_id: TankId(25601),
            name: "IS-2 shielded",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/25601.webp"),
        }),
        TankId(25617) => Some(Vehicle {
            tank_id: TankId(25617),
            name: "HWK 30",
            tier: 8,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/25617.webp"),
        }),
        TankId(25633) => Some(Vehicle {
            tank_id: TankId(25633),
            name: "M-IV-Y",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/25633.webp"),
        }),
        TankId(25857) => Some(Vehicle {
            tank_id: TankId(25857),
            name: "Obj. 777 II",
            tier: 10,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/25857.webp"),
        }),
        TankId(25889) => Some(Vehicle {
            tank_id: TankId(25889),
            name: "Ranger",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/25889.webp"),
        }),
        TankId(26113) => Some(Vehicle {
            tank_id: TankId(26113),
            name: "Object 452K",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/26113.webp"),
        }),
        TankId(26129) => Some(Vehicle {
            tank_id: TankId(26129),
            name: "Epsilon",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/26129.webp"),
        }),
        TankId(26145) => Some(Vehicle {
            tank_id: TankId(26145),
            name: "High Score",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/26145.webp"),
        }),
        TankId(26401) => Some(Vehicle {
            tank_id: TankId(26401),
            name: "Enforcer",
            tier: 6,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/26401.webp"),
        }),
        TankId(26641) => Some(Vehicle {
            tank_id: TankId(26641),
            name: "Kpz 07 RH",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/26641.webp"),
        }),
        TankId(26657) => Some(Vehicle {
            tank_id: TankId(26657),
            name: "ASTRON Rex",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/26657.webp"),
        }),
        TankId(26913) => Some(Vehicle {
            tank_id: TankId(26913),
            name: "Frosty",
            tier: 6,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/26913.webp"),
        }),
        TankId(27169) => Some(Vehicle {
            tank_id: TankId(27169),
            name: "Cyborg",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/27169.webp"),
        }),
        TankId(27425) => Some(Vehicle {
            tank_id: TankId(27425),
            name: "TL-7-120",
            tier: 9,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/27425.webp"),
        }),
        TankId(27681) => Some(Vehicle {
            tank_id: TankId(27681),
            name: "Pawlack Tank",
            tier: 6,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/27681.webp"),
        }),
        TankId(27937) => Some(Vehicle {
            tank_id: TankId(27937),
            name: "T49 Fearless",
            tier: 8,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/27937.webp"),
        }),
        TankId(28193) => Some(Vehicle {
            tank_id: TankId(28193),
            name: "TS-60",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/28193.webp"),
        }),
        TankId(28449) => Some(Vehicle {
            tank_id: TankId(28449),
            name: "T42",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/28449.webp"),
        }),
        TankId(51201) => Some(Vehicle {
            tank_id: TankId(51201),
            name: "KV-220 T",
            tier: 5,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/51201.webp"),
        }),
        TankId(51457) => Some(Vehicle {
            tank_id: TankId(51457),
            name: "Matilda IV",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/51457.webp"),
        }),
        TankId(51473) => Some(Vehicle {
            tank_id: TankId(51473),
            name: "Pz. V/IV",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/51473.webp"),
        }),
        TankId(51489) => Some(Vehicle {
            tank_id: TankId(51489),
            name: "T2 Light",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/51489.webp"),
        }),
        TankId(51713) => Some(Vehicle {
            tank_id: TankId(51713),
            name: "Churchill III",
            tier: 5,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/51713.webp"),
        }),
        TankId(51729) => Some(Vehicle {
            tank_id: TankId(51729),
            name: "Pz. II J",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/51729.webp"),
        }),
        TankId(51745) => Some(Vehicle {
            tank_id: TankId(51745),
            name: "Ram II",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/51745.webp"),
        }),
        TankId(51809) => Some(Vehicle {
            tank_id: TankId(51809),
            name: "Ke-Ni Otsu",
            tier: 3,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/51809.webp"),
        }),
        TankId(51985) => Some(Vehicle {
            tank_id: TankId(51985),
            name: "Pz. S35",
            tier: 3,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/51985.webp"),
        }),
        TankId(52065) => Some(Vehicle {
            tank_id: TankId(52065),
            name: "Hetzer Kame SP",
            tier: 4,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/52065.webp"),
        }),
        TankId(52225) => Some(Vehicle {
            tank_id: TankId(52225),
            name: "BT-SV",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/52225.webp"),
        }),
        TankId(52241) => Some(Vehicle {
            tank_id: TankId(52241),
            name: "Pz. B2",
            tier: 4,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/52241.webp"),
        }),
        TankId(52257) => Some(Vehicle {
            tank_id: TankId(52257),
            name: "M4A2E4",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/52257.webp"),
        }),
        TankId(52481) => Some(Vehicle {
            tank_id: TankId(52481),
            name: "Valentine II",
            tier: 4,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/52481.webp"),
        }),
        TankId(52497) => Some(Vehicle {
            tank_id: TankId(52497),
            name: "Pz. 38H",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/52497.webp"),
        }),
        TankId(52513) => Some(Vehicle {
            tank_id: TankId(52513),
            name: "M6A2E1",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/52513.webp"),
        }),
        TankId(52561) => Some(Vehicle {
            tank_id: TankId(52561),
            name: "Tortoise",
            tier: 9,
            type_: AntiTank,
            availability: Researchable,
            image_content: include_bytes!("vendored/52561.webp"),
        }),
        TankId(52737) => Some(Vehicle {
            tank_id: TankId(52737),
            name: "M3 Light",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/52737.webp"),
        }),
        TankId(52769) => Some(Vehicle {
            tank_id: TankId(52769),
            name: "Locust",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/52769.webp"),
        }),
        TankId(52993) => Some(Vehicle {
            tank_id: TankId(52993),
            name: "A-32",
            tier: 4,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/52993.webp"),
        }),
        TankId(53025) => Some(Vehicle {
            tank_id: TankId(53025),
            name: "M6A2E1 EXP",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/53025.webp"),
        }),
        TankId(53249) => Some(Vehicle {
            tank_id: TankId(53249),
            name: "KV-5",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/53249.webp"),
        }),
        TankId(53505) => Some(Vehicle {
            tank_id: TankId(53505),
            name: "T-127",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/53505.webp"),
        }),
        TankId(53537) => Some(Vehicle {
            tank_id: TankId(53537),
            name: "T1E6",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/53537.webp"),
        }),
        TankId(53585) => Some(Vehicle {
            tank_id: TankId(53585),
            name: "Matilda BP",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/53585.webp"),
        }),
        TankId(53761) => Some(Vehicle {
            tank_id: TankId(53761),
            name: "SU-85I",
            tier: 5,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/53761.webp"),
        }),
        TankId(53841) => Some(Vehicle {
            tank_id: TankId(53841),
            name: "TOG II*",
            tier: 6,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/53841.webp"),
        }),
        TankId(54097) => Some(Vehicle {
            tank_id: TankId(54097),
            name: "AT 15A",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/54097.webp"),
        }),
        TankId(54273) => Some(Vehicle {
            tank_id: TankId(54273),
            name: "SU-76I",
            tier: 3,
            type_: AntiTank,
            availability: Collectible,
            image_content: include_bytes!("vendored/54273.webp"),
        }),
        TankId(54289) => Some(Vehicle {
            tank_id: TankId(54289),
            name: "Löwe",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/54289.webp"),
        }),
        TankId(54353) => Some(Vehicle {
            tank_id: TankId(54353),
            name: "Excelsior",
            tier: 5,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/54353.webp"),
        }),
        TankId(54529) => Some(Vehicle {
            tank_id: TankId(54529),
            name: "Tetrarch",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/54529.webp"),
        }),
        TankId(54545) => Some(Vehicle {
            tank_id: TankId(54545),
            name: "T-25",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/54545.webp"),
        }),
        TankId(54785) => Some(Vehicle {
            tank_id: TankId(54785),
            name: "SU-100Y",
            tier: 6,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/54785.webp"),
        }),
        TankId(54801) => Some(Vehicle {
            tank_id: TankId(54801),
            name: "T-15",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/54801.webp"),
        }),
        TankId(54865) => Some(Vehicle {
            tank_id: TankId(54865),
            name: "Light VIC",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/54865.webp"),
        }),
        TankId(55057) => Some(Vehicle {
            tank_id: TankId(55057),
            name: "Pz. IV hydr.",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/55057.webp"),
        }),
        TankId(55073) => Some(Vehicle {
            tank_id: TankId(55073),
            name: "T7 Car",
            tier: 2,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/55073.webp"),
        }),
        TankId(55297) => Some(Vehicle {
            tank_id: TankId(55297),
            name: "SU-122-44",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/55297.webp"),
        }),
        TankId(55313) => Some(Vehicle {
            tank_id: TankId(55313),
            name: "JgTig.8,8 cm",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/55313.webp"),
        }),
        TankId(55889) => Some(Vehicle {
            tank_id: TankId(55889),
            name: "Cromwell B",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/55889.webp"),
        }),
        TankId(56097) => Some(Vehicle {
            tank_id: TankId(56097),
            name: "Fury",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/56097.webp"),
        }),
        TankId(56577) => Some(Vehicle {
            tank_id: TankId(56577),
            name: "LTP",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/56577.webp"),
        }),
        TankId(56609) => Some(Vehicle {
            tank_id: TankId(56609),
            name: "T28 HTC",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/56609.webp"),
        }),
        TankId(57105) => Some(Vehicle {
            tank_id: TankId(57105),
            name: "Dicker Max",
            tier: 6,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/57105.webp"),
        }),
        TankId(57361) => Some(Vehicle {
            tank_id: TankId(57361),
            name: "Pz. IV S.",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/57361.webp"),
        }),
        TankId(57617) => Some(Vehicle {
            tank_id: TankId(57617),
            name: "Panther/M10",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/57617.webp"),
        }),
        TankId(58641) => Some(Vehicle {
            tank_id: TankId(58641),
            name: "VK 72.01 K",
            tier: 10,
            type_: Heavy,
            availability: Researchable,
            image_content: include_bytes!("vendored/58641.webp"),
        }),
        TankId(58881) => Some(Vehicle {
            tank_id: TankId(58881),
            name: "IS-5",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/58881.webp"),
        }),
        TankId(59137) => Some(Vehicle {
            tank_id: TankId(59137),
            name: "IS-2 (1945)",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/59137.webp"),
        }),
        TankId(59649) => Some(Vehicle {
            tank_id: TankId(59649),
            name: "ISU-122S",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/59649.webp"),
        }),
        TankId(59665) => Some(Vehicle {
            tank_id: TankId(59665),
            name: "Großtraktor",
            tier: 3,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/59665.webp"),
        }),
        TankId(59905) => Some(Vehicle {
            tank_id: TankId(59905),
            name: "T-54 mod. 1",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/59905.webp"),
        }),
        TankId(60161) => Some(Vehicle {
            tank_id: TankId(60161),
            name: "IS-2Sh",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/60161.webp"),
        }),
        TankId(60177) => Some(Vehicle {
            tank_id: TankId(60177),
            name: "Panther 8,8",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/60177.webp"),
        }),
        TankId(60417) => Some(Vehicle {
            tank_id: TankId(60417),
            name: "IS-3 Defender",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/60417.webp"),
        }),
        TankId(60929) => Some(Vehicle {
            tank_id: TankId(60929),
            name: "BT-7 art.",
            tier: 3,
            type_: Light,
            availability: Collectible,
            image_content: include_bytes!("vendored/60929.webp"),
        }),
        TankId(62737) => Some(Vehicle {
            tank_id: TankId(62737),
            name: "leKpz M 41 90 mm",
            tier: 8,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/62737.webp"),
        }),
        TankId(62977) => Some(Vehicle {
            tank_id: TankId(62977),
            name: "T-44-100",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/62977.webp"),
        }),
        TankId(62993) => Some(Vehicle {
            tank_id: TankId(62993),
            name: "VK 45.03",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/62993.webp"),
        }),
        TankId(63553) => Some(Vehicle {
            tank_id: TankId(63553),
            name: "AMX CDC",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/63553.webp"),
        }),
        TankId(63585) => Some(Vehicle {
            tank_id: TankId(63585),
            name: "Tiger Kuromorimine SP",
            tier: 6,
            type_: Heavy,
            availability: Collectible,
            image_content: include_bytes!("vendored/63585.webp"),
        }),
        TankId(63601) => Some(Vehicle {
            tank_id: TankId(63601),
            name: "Dracula",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/63601.webp"),
        }),
        TankId(63841) => Some(Vehicle {
            tank_id: TankId(63841),
            name: "Pz. IV Ankou SP",
            tier: 5,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/63841.webp"),
        }),
        TankId(64001) => Some(Vehicle {
            tank_id: TankId(64001),
            name: "T-34-85 Rudy",
            tier: 7,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/64001.webp"),
        }),
        TankId(64017) => Some(Vehicle {
            tank_id: TankId(64017),
            name: "Tankenstein",
            tier: 7,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/64017.webp"),
        }),
        TankId(64065) => Some(Vehicle {
            tank_id: TankId(64065),
            name: "FCM 50 t",
            tier: 8,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/64065.webp"),
        }),
        TankId(64081) => Some(Vehicle {
            tank_id: TankId(64081),
            name: "Mk I* Heavy Tank",
            tier: 1,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/64081.webp"),
        }),
        TankId(64257) => Some(Vehicle {
            tank_id: TankId(64257),
            name: "T-34-85 Victory",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/64257.webp"),
        }),
        TankId(64273) => Some(Vehicle {
            tank_id: TankId(64273),
            name: "JgTig.8,8 cm (2015)",
            tier: 8,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/64273.webp"),
        }),
        TankId(64337) => Some(Vehicle {
            tank_id: TankId(64337),
            name: "AC IV Sentinel",
            tier: 6,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/64337.webp"),
        }),
        TankId(64529) => Some(Vehicle {
            tank_id: TankId(64529),
            name: "E 25",
            tier: 7,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/64529.webp"),
        }),
        TankId(64561) => Some(Vehicle {
            tank_id: TankId(64561),
            name: "112 Glacial",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/64561.webp"),
        }),
        TankId(64593) => Some(Vehicle {
            tank_id: TankId(64593),
            name: "Angry Connor",
            tier: 5,
            type_: AntiTank,
            availability: Premium,
            image_content: include_bytes!("vendored/64593.webp"),
        }),
        TankId(64769) => Some(Vehicle {
            tank_id: TankId(64769),
            name: "IS-6 Fearless",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/64769.webp"),
        }),
        TankId(64785) => Some(Vehicle {
            tank_id: TankId(64785),
            name: "Pz. II",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/64785.webp"),
        }),
        TankId(64801) => Some(Vehicle {
            tank_id: TankId(64801),
            name: "T34 (1776)",
            tier: 8,
            type_: Heavy,
            availability: Premium,
            image_content: include_bytes!("vendored/64801.webp"),
        }),
        TankId(64849) => Some(Vehicle {
            tank_id: TankId(64849),
            name: "AC 1 Sentinel",
            tier: 4,
            type_: Medium,
            availability: Collectible,
            image_content: include_bytes!("vendored/64849.webp"),
        }),
        TankId(65025) => Some(Vehicle {
            tank_id: TankId(65025),
            name: "MS-1 mod. 1",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/65025.webp"),
        }),
        TankId(65041) => Some(Vehicle {
            tank_id: TankId(65041),
            name: "L.Tr.",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/65041.webp"),
        }),
        TankId(65057) => Some(Vehicle {
            tank_id: TankId(65057),
            name: "T1",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/65057.webp"),
        }),
        TankId(65105) => Some(Vehicle {
            tank_id: TankId(65105),
            name: "Medium I",
            tier: 1,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/65105.webp"),
        }),
        TankId(65281) => Some(Vehicle {
            tank_id: TankId(65281),
            name: "MS-1 mod. 1",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/65281.webp"),
        }),
        TankId(65297) => Some(Vehicle {
            tank_id: TankId(65297),
            name: "L.Tr.",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/65297.webp"),
        }),
        TankId(65313) => Some(Vehicle {
            tank_id: TankId(65313),
            name: "T1",
            tier: 1,
            type_: Light,
            availability: Researchable,
            image_content: include_bytes!("vendored/65313.webp"),
        }),
        TankId(65329) => Some(Vehicle {
            tank_id: TankId(65329),
            name: "Type 62",
            tier: 7,
            type_: Light,
            availability: Premium,
            image_content: include_bytes!("vendored/65329.webp"),
        }),
        TankId(65361) => Some(Vehicle {
            tank_id: TankId(65361),
            name: "Medium I",
            tier: 1,
            type_: Medium,
            availability: Researchable,
            image_content: include_bytes!("vendored/65361.webp"),
        }),
        TankId(65377) => Some(Vehicle {
            tank_id: TankId(65377),
            name: "Chi-Nu Kai",
            tier: 5,
            type_: Medium,
            availability: Premium,
            image_content: include_bytes!("vendored/65377.webp"),
        }),
        _ => None,
    }
}
