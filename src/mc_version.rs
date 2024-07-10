use std::collections::HashMap;

use once_cell::sync::Lazy;
use phf::phf_map;

// Source: https://minecraft.wiki/w/Data_version
//
// To add a new mapping between id (aka "data version") and version string (aka "name"),
// add the mapping in ID2STR.
// For ease of editing, add mappings in numeric order on id.
//
// ID2STR is populated compile-time to eliminate initialization at runtime.
static ID2STR: phf::Map<i32, &'static str> = phf_map! {
    // Released in 2015
    100i32 => "15w32a",
    103i32 => "15w32b",
    104i32 => "15w32c",
    111i32 => "15w33a",
    // 15w33b reportedly also has an ID of 111, but we can only have one mapping.
    112i32 => "15w33c",
    114i32 => "15w34a",
    115i32 => "15w34b",
    116i32 => "15w34c",
    117i32 => "15w34d",
    118i32 => "15w35a",
    119i32 => "15w35b",
    120i32 => "15w35c",
    121i32 => "15w35d",
    122i32 => "15w35e",
    123i32 => "15w36a",
    124i32 => "15w36b",
    125i32 => "15w36c",
    126i32 => "15w36d",
    127i32 => "15w37a",
    128i32 => "15w38a",
    129i32 => "15w38b",
    130i32 => "15w39a",
    131i32 => "15w39b",
    132i32 => "15w39c",
    133i32 => "15w40a",
    134i32 => "15w40b",
    136i32 => "15w41a",
    137i32 => "15w41b",
    138i32 => "15w42a",
    139i32 => "15w43a",
    140i32 => "15w43b",
    141i32 => "15w43c",
    142i32 => "15w44a",
    143i32 => "15w44b",
    145i32 => "15w45a",
    146i32 => "15w46a",
    148i32 => "15w47a",
    149i32 => "15w47b",
    150i32 => "15w47c",
    151i32 => "15w49a",
    152i32 => "15w49b",
    153i32 => "15w50a",
    154i32 => "15w51a",
    155i32 => "15w51b",
    // Released in 2016
    156i32 => "16w02a",
    157i32 => "16w03a",
    158i32 => "16w04a",
    159i32 => "16w05a",
    160i32 => "16w05b",
    161i32 => "16w06a",
    162i32 => "16w07a",
    163i32 => "16w07b",
    164i32 => "1.9-pre1",
    165i32 => "1.9-pre2",
    167i32 => "1.9-pre3",
    168i32 => "1.9-pre4",
    169i32 => "1.9",
    170i32 => "1.9.1-pre1",
    171i32 => "1.9.1-pre2",
    172i32 => "1.9.1-pre3",
    175i32 => "1.9.1",
    176i32 => "1.9.2",
    177i32 => "16w14a",
    178i32 => "16w15a",
    179i32 => "16w15b",
    180i32 => "1.9.3-pre1",
    181i32 => "1.9.3-pre2",
    182i32 => "1.9.3-pre3",
    183i32 => "1.9.3",
    184i32 => "1.9.4",
    501i32 => "16w20a",
    503i32 => "16w21a",
    504i32 => "16w21b",
    506i32 => "1.10-pre1",
    507i32 => "1.10-pre2",
    510i32 => "1.10",
    511i32 => "1.10.1",
    512i32 => "1.10.2",
    800i32 => "16w32a",
    801i32 => "16w32b",
    802i32 => "16w33a",
    803i32 => "16w35a",
    805i32 => "16w36a",
    807i32 => "16w38a",
    809i32 => "16w39a",
    811i32 => "16w39b",
    812i32 => "16w39c",
    813i32 => "16w40a",
    814i32 => "16w41a",
    815i32 => "16w42a",
    816i32 => "16w43a",
    817i32 => "16w44a",
    818i32 => "1.11-pre1",
    819i32 => "1.11",
    920i32 => "16w50a",
    921i32 => "1.11.1",
    922i32 => "1.11.2",
    // Released in 2017
    1022i32 => "17w06a",
    1122i32 => "17w13a",
    1123i32 => "17w13b",
    1124i32 => "17w14a",
    1125i32 => "17w15a",
    1126i32 => "17w16a",
    1127i32 => "17w16b",
    1128i32 => "17w17a",
    1129i32 => "17w17b",
    1130i32 => "17w18a",
    1131i32 => "17w18b",
    1132i32 => "1.12-pre1",
    1133i32 => "1.12-pre2",
    1134i32 => "1.12-pre3",
    1135i32 => "1.12-pre4",
    1136i32 => "1.12-pre5",
    1137i32 => "1.12-pre6",
    1138i32 => "1.12-pre7",
    1139i32 => "1.12",
    1239i32 => "17w31a",
    1240i32 => "1.12.1-pre1",
    1241i32 => "1.12.1",
    1341i32 => "1.12.2-pre1",
    1342i32 => "1.12.2-pre2",
    1343i32 => "1.12.2",
    1444i32 => "17w43a",
    1445i32 => "17w43b",
    1447i32 => "17w45a",
    1448i32 => "17w45b",
    1449i32 => "17w46a",
    1451i32 => "17w47a", // This is when "the Flattening" happened.
    1452i32 => "17w47b",
    1453i32 => "17w48a",
    1454i32 => "17w49a",
    1455i32 => "17w49b",
    1457i32 => "17w50a",
    // Released in 2018
    1459i32 => "18w01a",
    1461i32 => "18w02a",
    1462i32 => "18w03a",
    1463i32 => "18w03b",
    1464i32 => "18w05a",
    1466i32 => "18w06a",
    1467i32 => "18w07a",
    1468i32 => "18w07b",
    1469i32 => "18w07c",
    1470i32 => "18w08a",
    1471i32 => "18w08b",
    1472i32 => "18w09a",
    1473i32 => "18w10a",
    1474i32 => "18w10b",
    1476i32 => "18w10c",
    1477i32 => "18w10d",
    1478i32 => "18w11a",
    1479i32 => "18w14a",
    1481i32 => "18w14b",
    1482i32 => "18w15a",
    1483i32 => "18w16a",
    1484i32 => "18w19a",
    1485i32 => "18w19b",
    1489i32 => "18w20a",
    1491i32 => "18w20b",
    1493i32 => "18w20c",
    1495i32 => "18w21a",
    1496i32 => "18w21b",
    1497i32 => "18w22a",
    1498i32 => "18w22b",
    1499i32 => "18w22c",
    1501i32 => "1.13-pre1",
    1502i32 => "1.13-pre2",
    1503i32 => "1.13-pre3",
    1504i32 => "1.13-pre4",
    1511i32 => "1.13-pre5",
    1512i32 => "1.13-pre6",
    1513i32 => "1.13-pre7",
    1516i32 => "1.13-pre8",
    1517i32 => "1.13-pre9",
    1518i32 => "1.13-pre10",
    1519i32 => "1.13",
    1620i32 => "18w30a",
    1621i32 => "18w30b",
    1622i32 => "18w31a",
    1623i32 => "18w32a",
    1625i32 => "18w33a",
    1626i32 => "1.13.1-pre1",
    1627i32 => "1.13.1-pre2",
    1628i32 => "1.13.1",
    1629i32 => "1.13.2-pre1",
    1630i32 => "1.13.2-pre2",
    1631i32 => "1.13.2",
    1901i32 => "18w43a",
    1902i32 => "18w43b",
    1903i32 => "18w43c",
    1907i32 => "18w44a",
    1908i32 => "18w45a",
    1910i32 => "18w46a",
    1912i32 => "18w47a",
    1913i32 => "18w47b",
    1914i32 => "18w48a",
    1915i32 => "18w48b",
    1916i32 => "18w49a",
    1919i32 => "18w50a",
    // Released in 2019
    1921i32 => "19w02a",
    1922i32 => "19w03a",
    1923i32 => "19w03b",
    1924i32 => "19w03c",
    1926i32 => "19w04a",
    1927i32 => "19w04b",
    1930i32 => "19w05a",
    1931i32 => "19w06a",
    1932i32 => "19w07a",
    1933i32 => "19w08a",
    1934i32 => "19w08b",
    1935i32 => "19w09a",
    1937i32 => "19w11a",
    1938i32 => "19w11b",
    1940i32 => "19w12a",
    1941i32 => "19w12b",
    1942i32 => "19w13a",
    1943i32 => "19w13b",
    1944i32 => "19w14a",
    1945i32 => "19w14b",
    1947i32 => "1.14 Pre-Release 1",
    1948i32 => "1.14 Pre-Release 2",
    1949i32 => "1.14 Pre-Release 3",
    1950i32 => "1.14 Pre-Release 4",
    1951i32 => "1.14 Pre-Release 5",
    1952i32 => "1.14",
    1955i32 => "1.14.1 Pre-Release 1",
    1956i32 => "1.14.1 Pre-Release 2",
    1957i32 => "1.14.1",
    1958i32 => "1.14.2 Pre-Release 1",
    1959i32 => "1.14.2 Pre-Release 2",
    1960i32 => "1.14.2 Pre-Release 3",
    1962i32 => "1.14.2 Pre-Release 4",
    1963i32 => "1.14.2",
    1964i32 => "1.14.3 Pre-Release 1",
    1965i32 => "1.14.3 Pre-Release 2",
    1966i32 => "1.14.3 Pre-Release 3",
    1967i32 => "1.14.3 Pre-Release 4",
    1968i32 => "1.14.3",
    1969i32 => "1.14.4 Pre-Release 1",
    1970i32 => "1.14.4 Pre-Release 2",
    1971i32 => "1.14.4 Pre-Release 3",
    1972i32 => "1.14.4 Pre-Release 4",
    1973i32 => "1.14.4 Pre-Release 5",
    1974i32 => "1.14.4 Pre-Release 6",
    1975i32 => "1.14.4 Pre-Release 7",
    1976i32 => "1.14.4",
    // NOTE 2067 "1.14.3 - Combat Test" not supported.
    // NOTE 2068 "Combat Test 2" not supported.
    // NOTE 2069 "Combat Test 3" not supported.
    2200i32 => "19w34a",
    2201i32 => "19w35a",
    2203i32 => "19w36a",
    2204i32 => "19w37a",
    2205i32 => "19w38a",
    2206i32 => "19w38b",
    2207i32 => "19w39a",
    2208i32 => "19w40a",
    2210i32 => "19w41a",
    2212i32 => "19w42a",
    2213i32 => "19w44a",
    2214i32 => "19w45a",
    2215i32 => "19w45b",
    2216i32 => "19w46a",
    2217i32 => "19w46b",
    2218i32 => "1.15 Pre-Release 1",
    2219i32 => "1.15 Pre-Release 2",
    2220i32 => "1.15 Pre-Release 3",
    2221i32 => "1.15 Pre-Release 4",
    2222i32 => "1.15 Pre-Release 5",
    2223i32 => "1.15 Pre-Release 6",
    2224i32 => "1.15 Pre-Release 7",
    2225i32 => "1.15",
    2226i32 => "1.15.1 Pre-Release 1",
    2227i32 => "1.15.1",
    // Released in 2020
    2228i32 => "1.15.2 Pre-Release 1",
    2229i32 => "1.15.2 Pre-Release 2",
    2230i32 => "1.15.2",
    // NOTE 2320 "Combat Test 4" not supported.
    // NOTE 2321 "Combat Test 5" not supported.
    2504i32 => "20w06a",
    2506i32 => "20w07a",
    2507i32 => "20w08a",
    2510i32 => "20w09a",
    2512i32 => "20w10a",
    2513i32 => "20w11a",
    2515i32 => "20w12a",
    2520i32 => "20w13a",
    2521i32 => "20w13b",
    2524i32 => "20w14a",
    2525i32 => "20w15a",
    2526i32 => "20w16a",
    2529i32 => "20w17a",
    2532i32 => "20w18a",
    2534i32 => "20w19a",
    2536i32 => "20w20a",
    2537i32 => "20w20b",
    2554i32 => "20w21a",
    2555i32 => "20w22a",
    2556i32 => "1.16 Pre-release 1",
    2557i32 => "1.16 Pre-release 2",
    2559i32 => "1.16 Pre-release 3",
    2560i32 => "1.16 Pre-release 4",
    2561i32 => "1.16 Pre-release 5",
    2562i32 => "1.16 Pre-release 6",
    2563i32 => "1.16 Pre-release 7",
    2564i32 => "1.16 Pre-release 8",
    2565i32 => "1.16 Release Candidate 1",
    2566i32 => "1.16",
    2567i32 => "1.16.1",
    2569i32 => "20w27a",
    2570i32 => "20w28a",
    2571i32 => "20w29a",
    2572i32 => "20w30a",
    2573i32 => "1.16.2 Pre-release 1",
    2574i32 => "1.16.2 Pre-release 2",
    2575i32 => "1.16.2 Pre-release 3",
    2576i32 => "1.16.2 Release Candidate 1",
    2577i32 => "1.16.2 Release Candidate 2",
    2578i32 => "1.16.2",
    2579i32 => "1.16.3 Release Candidate 1",
    2580i32 => "1.16.3",
    2581i32 => "1.16.4 Pre-release 1",
    2582i32 => "1.16.4 Pre-release 2",
    2583i32 => "1.16.4 Release Candidate 1",
    2584i32 => "1.16.4",
    2681i32 => "20w45a",
    2682i32 => "20w46a",
    2683i32 => "20w48a",
    2685i32 => "20w49a",
    2687i32 => "20w51a",
    // NOTE 2701 "Combat Test 6" not supported
    // NOTE 2704 "Combat Test 7c" not supported
    // NOTE 2706 "Combat Test 8b" not supported
    // NOTE 2707 "Combat Test 8c" not supported
    // Released in 2021
    2585i32 => "1.16.5 Release Candidate 1",
    2586i32 => "1.16.5",
    2689i32 => "21w03a",
    2690i32 => "21w05a",
    2692i32 => "21w05b",
    2694i32 => "21w06a",
    2695i32 => "21w07a",
    2697i32 => "21w08a",
    2698i32 => "21w08b",
    2699i32 => "21w10a",
    2703i32 => "21w11a",
    2705i32 => "21w13a",
    2706i32 => "21w14a",
    2709i32 => "21w15a",
    2711i32 => "21w16a",
    2712i32 => "21w17a",
    2713i32 => "21w18a",
    2714i32 => "21w19a",
    2715i32 => "21w20a",
    2716i32 => "1.17 Pre-release 1",
    2718i32 => "1.17 Pre-release 2",
    2719i32 => "1.17 Pre-release 3",
    2720i32 => "1.17 Pre-release 4",
    2721i32 => "1.17 Pre-release 5",
    2722i32 => "1.17 Release Candidate 1",
    2723i32 => "1.17 Release Candidate 2",
    2724i32 => "1.17",
    2725i32 => "1.17.1 Pre-release 1",
    2726i32 => "1.17.1 Pre-release 2",
    2727i32 => "1.17.1 Pre-release 3",
    2728i32 => "1.17.1 Release Candidate 1",
    2729i32 => "1.17.1 Release Candidate 2",
    2730i32 => "1.17.1",
    2825i32 => "1.18 Experimental Snapshot 1",
    2826i32 => "1.18 Experimental Snapshot 2",
    2827i32 => "1.18 Experimental Snapshot 3",
    2828i32 => "1.18 Experimental Snapshot 4",
    2829i32 => "1.18 Experimental Snapshot 5",
    2830i32 => "1.18 Experimental Snapshot 6",
    2831i32 => "1.18 Experimental Snapshot 7",
    2834i32 => "21w37a",
    2835i32 => "21w38a",
    2836i32 => "21w39a",
    2838i32 => "21w40a",
    2839i32 => "21w41a",
    2840i32 => "21w42a",
    2844i32 => "21w43a",
    2845i32 => "21w44a",
    2847i32 => "1.18 Pre-release 1",
    2848i32 => "1.18 Pre-release 2",
    2849i32 => "1.18 Pre-release 3",
    2850i32 => "1.18 Pre-release 4",
    2851i32 => "1.18 Pre-release 5",
    2853i32 => "1.18 Pre-release 6",
    2854i32 => "1.18 Pre-release 7",
    2855i32 => "1.18 Pre-release 8",
    2856i32 => "1.18 Release Candidate 1",
    2857i32 => "1.18 Release Candidate 2",
    2858i32 => "1.18 Release Candidate 3",
    2859i32 => "1.18 Release Candidate 4",
    2860i32 => "1.18",
    2861i32 => "1.18.1 Pre-release 1",
    2862i32 => "1.18.1 Release Candidate 1",
    2863i32 => "1.18.1 Release Candidate 2",
    2864i32 => "1.18.1 Release Candidate 3",
    2865i32 => "1.18.1",
    // Released in 2022
    2966i32 => "22w03a",
    2967i32 => "22w05a",
    2968i32 => "22w06a",
    2969i32 => "22w07a",
    2971i32 => "1.18.2 Pre-release 1",
    2972i32 => "1.18.2 Pre-release 2",
    2973i32 => "1.18.2 Pre-release 3",
    2974i32 => "1.18.2 Release Candidate 1",
    2975i32 => "1.18.2",
    3066i32 => "Deep Dark Experimental Snapshot 1",
    3080i32 => "22w11a",
    3082i32 => "22w12a",
    3085i32 => "22w13a",
    3088i32 => "22w14a",
    3089i32 => "22w15a",
    3091i32 => "22w16a",
    3092i32 => "22w16b",
    3093i32 => "22w17a",
    3095i32 => "22w18a",
    3096i32 => "22w19a",
    3098i32 => "1.19 Pre-release 1",
    3099i32 => "1.19 Pre-release 2",
    3100i32 => "1.19 Pre-release 3",
    3101i32 => "1.19 Pre-release 4",
    3102i32 => "1.19 Pre-release 5",
    3103i32 => "1.19 Release Candidate 1",
    3104i32 => "1.19 Release Candidate 2",
    3105i32 => "1.19",
    3106i32 => "22w24a",
    3107i32 => "1.19.1 Pre-release 1",
    3109i32 => "1.19.1 Release Candidate 1",
    3110i32 => "1.19.1 Pre-release 2",
    3111i32 => "1.19.1 Pre-release 3",
    3112i32 => "1.19.1 Pre-release 4",
    3113i32 => "1.19.1 Pre-release 5",
    3114i32 => "1.19.1 Pre-release 6",
    3115i32 => "1.19.1 Release Candidate 2",
    3116i32 => "1.19.1 Release Candidate 3",
    3117i32 => "1.19.1",
    3118i32 => "1.19.2 Release Candidate 1",
    3119i32 => "1.19.2 Release Candidate 2",
    3120i32 => "1.19.2",
    3205i32 => "22w42a",
    3206i32 => "22w43a",
    3207i32 => "22w44a",
    3208i32 => "22w45a",
    3210i32 => "22w46a",
    3211i32 => "1.19.3 Pre-release 1",
    3212i32 => "1.19.3 Pre-release 2",
    3213i32 => "1.19.3 Pre-release 3",
    3215i32 => "1.19.3 Release Candidate 1",
    3216i32 => "1.19.3 Release Candidate 2",
    3217i32 => "1.19.3 Release Candidate 3",
    3218i32 => "1.19.3",
    // Released in 2023
    3320i32 => "23w03a",
    3321i32 => "23w04a",
    3323i32 => "23w05a",
    3326i32 => "23w06a",
    3329i32 => "23w07a",
    3330i32 => "1.19.4 Pre-release 1",
    3331i32 => "1.19.4 Pre-release 2",
    3332i32 => "1.19.4 Pre-release 3",
    3333i32 => "1.19.4 Pre-release 4",
    3334i32 => "1.19.4 Release Candidate 1",
    3335i32 => "1.19.4 Release Candidate 2",
    3336i32 => "1.19.4 Release Candidate 3",
    3337i32 => "1.19.4",
    3442i32 => "23w12a",
    3443i32 => "23w13a",
    3445i32 => "23w14a",
    3449i32 => "23w16a",
    3452i32 => "23w17a",
    3453i32 => "23w18a",
    3454i32 => "1.20 Pre-release 1",
    3455i32 => "1.20 Pre-release 2",
    3456i32 => "1.20 Pre-release 3",
    3457i32 => "1.20 Pre-release 4",
    3458i32 => "1.20 Pre-release 5",
    3460i32 => "1.20 Pre-release 6",
    3461i32 => "1.20 Pre-release 7",
    3462i32 => "1.20 Release Candidate 1",
    3463i32 => "1.20",
    3464i32 => "1.20.1 Release Candidate 1",
    3465i32 => "1.20.1",
    3567i32 => "23w31a",
    3569i32 => "23w32a",
    3570i32 => "23w33a",
    3571i32 => "23w35a",
    3572i32 => "1.20.2 Pre-release 1",
    3573i32 => "1.20.2 Pre-release 2",
    3574i32 => "1.20.2 Pre-release 3",
    3575i32 => "1.20.2 Pre-release 4",
    3576i32 => "1.20.2 Release Candidate 1",
    3577i32 => "1.20.2 Release Candidate 2",
    3578i32 => "1.20.2",
    3679i32 => "23w40a",
    3681i32 => "23w41a",
    3684i32 => "23w42a",
    3686i32 => "23w43a",
    3687i32 => "23w43b",
    3688i32 => "23w44a",
    3690i32 => "23w45a",
    3691i32 => "23w46a",
    3693i32 => "1.20.3 Pre-release 1",
    3694i32 => "1.20.3 Pre-release 2",
    3695i32 => "1.20.3 Pre-release 3",
    3696i32 => "1.20.3 Pre-release 4",
    3697i32 => "1.20.3 Release Candidate 1",
    3698i32 => "1.20.3",
    3699i32 => "1.20.4 Release Candidate 1",
    3700i32 => "1.20.4",
    3801i32 => "23w51a",
    3802i32 => "23w51b",
    // Released in 2024
    3804i32 => "24w03a",
    3805i32 => "24w03b",
    3806i32 => "24w04a",
    3809i32 => "24w05a",
    3811i32 => "24w05b",
    3815i32 => "24w06a",
    3817i32 => "24w07a",
    3819i32 => "24w09a",
    3821i32 => "24w10a",
    3823i32 => "24w11a",
    3824i32 => "24w12a",
    3826i32 => "24w13a",
    3827i32 => "24w14a",
    3829i32 => "1.20.5 Pre-release 1",
    3830i32 => "1.20.5 Pre-release 2",
    3831i32 => "1.20.5 Pre-release 3",
    3832i32 => "1.20.5 Pre-release 4",
    3834i32 => "1.20.5 Release Candidate 1",
    3835i32 => "1.20.5 Release Candidate 2",
    3836i32 => "1.20.5 Release Candidate 3",
    3837i32 => "1.20.5",
    3838i32 => "1.20.6 Release Candidate 1",
    3839i32 => "1.20.6",
    3940i32 => "24w18a",
    3941i32 => "24w19a",
    3942i32 => "24w19b",
    3944i32 => "24w20a",
    3946i32 => "24w21a",
    3947i32 => "24w21b",
    3948i32 => "1.21 Pre-release 1",
    3949i32 => "1.21 Pre-release 1",
    3950i32 => "1.21 Pre-release 1",
    3951i32 => "1.21 Pre-release 1",
    3952i32 => "1.21 Release Candidate 1",
    3953i32 => "1.21",
};

// For now, STR2ID is initialized on the first access and cached for subsequent accesses.
static STR2ID: Lazy<HashMap<&'static str, i32>> = Lazy::new(|| {
    ID2STR
        .entries()
        .map(|(key, value)| (*value, *key))
        .collect::<HashMap<_, _>>()
});

/// Minecraft version.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub struct McVersion {
    id: i32,
}

impl McVersion {
    /// Creates a new McVersion from a "data version" / "version id" value.
    ///
    /// # Example
    ///
    /// ```
    /// use mcprogedit::mc_version::McVersion;
    ///
    /// let version = McVersion::from_id(100);
    /// ```
    pub fn from_id<I>(id: I) -> Self
    where
        I: Into<i32>,
    {
        McVersion { id: id.into() }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &'static str {
        ID2STR[&self.id]
    }
}

impl std::str::FromStr for McVersion {
    type Err = ();

    /// Creates a new McVersion from a version name string.
    ///
    /// # Examples
    ///
    /// ```
    /// use mcprogedit::mc_version::McVersion;
    /// use std::str::FromStr;
    ///
    /// let snapshot_version = McVersion::from_str("15w32a").unwrap();
    /// let release_version = McVersion::from_str("1.12.2").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match STR2ID.get(s) {
            Some(id) => Ok(McVersion { id: *id }),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn mc_version_name_yields_same_name_as_the_input_to_mc_version_fromstr() {
        let version_strs = ["15w37a", "1.9-pre1", "16w42a", "1.12.1-pre1", "18w21b"];
        assert!(!version_strs.is_empty());

        for version_str in version_strs {
            assert_eq!(
                McVersion::from_str(version_str).unwrap().name(),
                version_str
            )
        }
    }

    fn ids_in_id_list_can_convert_to_str_and_back_again(ids: &[i32]) {
        for id in ids {
            let version = McVersion::from_id(*id);
            let name = version.name();
            let version_from_str = McVersion::from_str(name).unwrap();
            assert_eq!(version, version_from_str);
        }
    }

    #[test]
    fn ids_in_id2str_list_has_reverse_mappings() {
        let ids = ID2STR.keys().map(|k| *k).collect::<Vec<i32>>();
        assert!(!ids.is_empty());
        ids_in_id_list_can_convert_to_str_and_back_again(&ids);
    }

    #[test]
    fn ids_in_str2id_list_has_reverse_mappings() {
        let ids = STR2ID
            .entries()
            .map(|(_, value)| *value)
            .collect::<Vec<i32>>();
        assert!(!ids.is_empty());
        ids_in_id_list_can_convert_to_str_and_back_again(&ids);
    }
}
